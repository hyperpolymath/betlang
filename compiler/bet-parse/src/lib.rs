// SPDX-License-Identifier: MIT OR Apache-2.0
//! Parser for Betlang
//!
//! This module provides lexing and parsing for betlang source code.

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

/// Parse a betlang source string into an AST
pub fn parse(source: &str) -> Result<Module, ParseError> {
    let lexer = Lexer::new(source);
    grammar::ModuleParser::new()
        .parse(lexer)
        .map_err(|e| match e {
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
        })
}

/// Parse a single expression
pub fn parse_expr(source: &str) -> Result<bet_syntax::ast::Expr, ParseError> {
    let lexer = Lexer::new(source);
    grammar::ExprParser::new()
        .parse(lexer)
        .map_err(|e| match e {
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
        })
}

#[cfg(test)]
mod tests {
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
        let result = parse_expr("do { x <- sample normal; return x }");
        assert!(result.is_ok(), "Failed to parse do: {:?}", result.err());
    }

    #[test]
    fn test_parse_do_end() {
        let result = parse_expr("do x <- sample normal; return x end");
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
}
