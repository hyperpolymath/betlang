// @taxonomy: compiler/parser
// SPDX-License-Identifier: PMPL-1.0-or-later
//! Parser for Betlang
//!
//! This module provides lexing and parsing for betlang source code.

#![forbid(unsafe_code)]
pub mod lexer;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub use lexer::{lex, Lexer, LexError, Token, SpannedToken};

use bet_syntax::ast::Module;
use thiserror::Error;

/// Parse error type
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    Lexer(#[from] LexError),

    #[error("Parse error at {location}: {message}")]
    Parse {
        location: usize,
        message: String,
    },

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Unexpected token: {found}, expected one of: {expected:?}")]
    UnexpectedToken {
        found: String,
        expected: Vec<String>,
    },
}

/// Result of parsing with error recovery: partial AST + diagnostics.
pub struct ParseOutput {
    pub module: Module,
    pub diagnostics: Vec<ParseError>,
}

/// Parse a betlang source string into an AST
pub fn parse(source: &str) -> Result<Module, ParseError> {
    let lexer = Lexer::new(source);
    grammar::ModuleParser::new()
        .parse(lexer)
        .map_err(convert_lalrpop_error)
}

/// Convert a lalrpop error into our ParseError type.
fn convert_lalrpop_error(e: lalrpop_util::ParseError<usize, Token, LexError>) -> ParseError {
    match e {
        lalrpop_util::ParseError::InvalidToken { location } => ParseError::Parse {
            location,
            message: "Invalid token".to_string(),
        },
        lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
            ParseError::Parse {
                location,
                message: format!("Unexpected end of input, expected one of: {:?}", expected),
            }
        }
        lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
            ParseError::UnexpectedToken {
                found: format!("{:?}", token.1),
                expected,
            }
        }
        lalrpop_util::ParseError::ExtraToken { token } => ParseError::Parse {
            location: token.0,
            message: format!("Extra token: {:?}", token.1),
        },
        lalrpop_util::ParseError::User { error } => ParseError::Lexer(error),
    }
}

/// Find the next item boundary in source text.
/// Scans from `start` for keywords that begin a new top-level item.
fn find_next_item_boundary(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut i = start;
    while i < len {
        // Skip to next potential keyword start (alphabetic after whitespace/newline)
        if i == 0 || bytes[i - 1].is_ascii_whitespace() {
            let rest = &source[i..];
            // Check for keywords that start items
            for kw in &["let ", "type ", "import "] {
                if rest.starts_with(kw) {
                    return Some(i);
                }
            }
        }
        i += 1;
    }
    None
}

/// Parse with error recovery. Since lalrpop does not natively support
/// error recovery, we catch errors, record them as diagnostics, skip to
/// the next item boundary in the source, and retry parsing the remainder.
///
/// Returns partial AST + all diagnostics collected.
pub fn parse_recovering(source: &str) -> ParseOutput {
    use bet_syntax::ast::Item;
    use bet_syntax::span::Span;

    let mut all_items: Vec<bet_syntax::span::Spanned<Item>> = Vec::new();
    let mut diagnostics: Vec<ParseError> = Vec::new();
    let mut offset = 0;

    while offset < source.len() {
        let remainder = &source[offset..];
        if remainder.trim().is_empty() {
            break;
        }

        let lexer = Lexer::new(remainder);
        match grammar::ModuleParser::new().parse(lexer) {
            Ok(module) => {
                // Adjust spans by offset and add items
                all_items.extend(module.items.into_iter());
                break;
            }
            Err(e) => {
                let error_location = match &e {
                    lalrpop_util::ParseError::InvalidToken { location } => *location,
                    lalrpop_util::ParseError::UnrecognizedEof { location, .. } => *location,
                    lalrpop_util::ParseError::UnrecognizedToken { token, .. } => token.0,
                    lalrpop_util::ParseError::ExtraToken { token } => token.0,
                    lalrpop_util::ParseError::User { .. } => 0,
                };

                diagnostics.push(convert_lalrpop_error(e));

                // Find the next item boundary after the error
                let abs_error = offset + error_location;
                match find_next_item_boundary(source, abs_error + 1) {
                    Some(next) => {
                        offset = next;
                    }
                    None => break,
                }
            }
        }
    }

    ParseOutput {
        module: Module {
            name: None,
            items: all_items,
            span: Span::new(0, source.len() as u32),
        },
        diagnostics,
    }
}

/// Parse a single expression
pub fn parse_expr(source: &str) -> Result<bet_syntax::ast::Expr, ParseError> {
    let lexer = Lexer::new(source);
    grammar::ExprParser::new()
        .parse(lexer)
        .map_err(convert_lalrpop_error)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_parse_bet() {
        let result = parse_expr("bet { 1, 2, 3 }");
        assert!(result.is_ok(), "Failed to parse bet: {:?}", result.err());
    }

    #[test]
    fn test_parse_bet_end() {
        let result = parse_expr("bet 1, 2, 3 end");
        assert!(result.is_ok(), "Failed to parse bet end: {:?}", result.err());
    }

    #[test]
    fn test_parse_let() {
        let result = parse("let x = 42");
        assert!(result.is_ok(), "Failed to parse let: {:?}", result.err());
    }

    #[test]
    fn test_parse_function() {
        let result = parse("let add = fun x y -> x + y");
        assert!(result.is_ok(), "Failed to parse function: {:?}", result.err());
    }

    #[test]
    fn test_parse_do_notation() {
        let result = parse_expr("do { x <- sample normal; x }");
        assert!(result.is_ok(), "Failed to parse do: {:?}", result.err());
    }

    #[test]
    fn test_parse_do_end() {
        let result = parse_expr("do x <- sample normal; x end");
        assert!(result.is_ok(), "Failed to parse do end: {:?}", result.err());
    }

    #[test]
    fn test_parse_if_end() {
        let result = parse_expr("if x then y else z end");
        assert!(result.is_ok(), "Failed to parse if end: {:?}", result.err());
    }

    #[test]
    fn test_parse_match_end() {
        let result = parse_expr("match x a -> y; b -> z end");
        assert!(result.is_ok(), "Failed to parse match end: {:?}", result.err());
    }

    #[test]
    fn test_parse_parallel_end() {
        let result = parse_expr("parallel 4 do x + 1 end");
        assert!(result.is_ok(), "Failed to parse parallel end: {:?}", result.err());
    }

    #[test]
    fn test_parse_field_access() {
        let result = parse_expr("record.field");
        assert!(result.is_ok(), "Failed to parse field access: {:?}", result.err());
        match result.expect("TODO: handle error") {
            bet_syntax::ast::Expr::Field(_, field) => {
                assert_eq!(field.node.as_str(), "field");
            }
            other => panic!("Expected Field, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_chained_field_access() {
        let result = parse_expr("a.b.c");
        assert!(result.is_ok(), "Failed to parse chained field access: {:?}", result.err());
        // Should parse as (a.b).c
        match result.expect("TODO: handle error") {
            bet_syntax::ast::Expr::Field(base, field_c) => {
                assert_eq!(field_c.node.as_str(), "c");
                match &base.node {
                    bet_syntax::ast::Expr::Field(_, field_b) => {
                        assert_eq!(field_b.node.as_str(), "b");
                    }
                    other => panic!("Expected inner Field, got {:?}", other),
                }
            }
            other => panic!("Expected Field, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_index_access() {
        let result = parse_expr("arr.[0]");
        assert!(result.is_ok(), "Failed to parse index access: {:?}", result.err());
        match result.expect("TODO: handle error") {
            bet_syntax::ast::Expr::Index(_, _) => {}
            other => panic!("Expected Index, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_field_then_index() {
        let result = parse_expr("record.items.[0]");
        assert!(result.is_ok(), "Failed to parse field then index: {:?}", result.err());
        // Should parse as (record.items).[0]
        match result.expect("TODO: handle error") {
            bet_syntax::ast::Expr::Index(base, _) => {
                match &base.node {
                    bet_syntax::ast::Expr::Field(_, field) => {
                        assert_eq!(field.node.as_str(), "items");
                    }
                    other => panic!("Expected inner Field, got {:?}", other),
                }
            }
            other => panic!("Expected Index, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_field_access_in_binop() {
        // Field access should have higher precedence than binary operators
        let result = parse_expr("a.x + b.y");
        assert!(result.is_ok(), "Failed to parse field in binop: {:?}", result.err());
        match result.expect("TODO: handle error") {
            bet_syntax::ast::Expr::BinOp(bet_syntax::ast::BinOp::Add, _, _) => {}
            other => panic!("Expected BinOp(Add, ...), got {:?}", other),
        }
    }

    #[test]
    fn test_parse_field_access_in_match() {
        // Field access on match scrutinee (keyword form)
        let result = parse_expr("match x.y a -> 1; b -> 2 end");
        assert!(result.is_ok(), "Failed to parse field in match: {:?}", result.err());
    }
}
