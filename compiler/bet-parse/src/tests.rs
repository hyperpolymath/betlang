// SPDX-License-Identifier: PMPL-1.0-or-later
//! Comprehensive lexer and parser tests for Betlang.
//!
//! These tests exercise all token types in the logos lexer and all major
//! grammar productions in the LALRPOP parser.

use crate::lexer::{lex, Token};
use crate::{parse, parse_expr};
use bet_syntax::ast::*;

// ---------------------------------------------------------------------------
// Helper: collect token variants from a source string, ignoring spans
// ---------------------------------------------------------------------------

fn tokens(src: &str) -> Vec<Token> {
    lex(src).expect("lexer failed").into_iter().map(|t| t.token).collect()
}

// ===========================================================================
// 1. Lexer Tests
// ===========================================================================

// ---- 1.1 Keywords --------------------------------------------------------

#[test]
fn lex_keyword_bet() {
    assert_eq!(tokens("bet"), vec![Token::Bet]);
}

#[test]
fn lex_keyword_let() {
    assert_eq!(tokens("let"), vec![Token::Let]);
}

#[test]
fn lex_keyword_in() {
    assert_eq!(tokens("in"), vec![Token::In]);
}

#[test]
fn lex_keyword_fun() {
    assert_eq!(tokens("fun"), vec![Token::Fun]);
}

#[test]
fn lex_keyword_match() {
    assert_eq!(tokens("match"), vec![Token::Match]);
}

#[test]
fn lex_keyword_if_then_else_end() {
    assert_eq!(
        tokens("if then else end"),
        vec![Token::If, Token::Then, Token::Else, Token::End],
    );
}

#[test]
fn lex_keyword_do_return() {
    assert_eq!(tokens("do return"), vec![Token::Do, Token::Return]);
}

#[test]
fn lex_keyword_type_module_import_export() {
    assert_eq!(
        tokens("type module import export"),
        vec![Token::Type, Token::Module, Token::Import, Token::Export],
    );
}

#[test]
fn lex_keyword_rec() {
    assert_eq!(tokens("rec"), vec![Token::Rec]);
}

#[test]
fn lex_keyword_logical_ops() {
    assert_eq!(
        tokens("and or not xor"),
        vec![Token::And, Token::Or, Token::Not, Token::Xor],
    );
}

#[test]
fn lex_keyword_ternary_literals() {
    assert_eq!(
        tokens("true false unknown"),
        vec![Token::True, Token::False, Token::Unknown],
    );
}

#[test]
fn lex_keyword_probabilistic() {
    assert_eq!(
        tokens("sample observe infer parallel weighted"),
        vec![Token::Sample, Token::Observe, Token::Infer, Token::Parallel, Token::Weighted],
    );
}

#[test]
fn lex_keyword_inference_methods() {
    assert_eq!(
        tokens("MCMC HMC SMC VI"),
        vec![Token::MCMC, Token::HMC, Token::SMC, Token::VI],
    );
}

// ---- 1.2 Operators -------------------------------------------------------

#[test]
fn lex_arithmetic_operators() {
    assert_eq!(
        tokens("+ - * / % ^"),
        vec![Token::Plus, Token::Minus, Token::Star, Token::Slash, Token::Percent, Token::Caret],
    );
}

#[test]
fn lex_comparison_operators() {
    assert_eq!(
        tokens("== != < <= > >="),
        vec![Token::EqEq, Token::NotEq, Token::Lt, Token::Le, Token::Gt, Token::Ge],
    );
}

#[test]
fn lex_logical_operators() {
    assert_eq!(tokens("&& ||"), vec![Token::AndAnd, Token::OrOr]);
}

#[test]
fn lex_compound_operators() {
    assert_eq!(
        tokens("++ :: >> |>"),
        vec![Token::PlusPlus, Token::ColonColon, Token::GtGt, Token::Pipe],
    );
}

#[test]
fn lex_arrow_operators() {
    assert_eq!(
        tokens("<- -> =>"),
        vec![Token::LArrow, Token::RArrow, Token::FatArrow],
    );
}

// ---- 1.3 Delimiters and punctuation --------------------------------------

#[test]
fn lex_delimiters() {
    assert_eq!(
        tokens("( ) [ ] { }"),
        vec![Token::LParen, Token::RParen, Token::LBracket, Token::RBracket, Token::LBrace, Token::RBrace],
    );
}

#[test]
fn lex_punctuation() {
    assert_eq!(
        tokens(", ; : . = @ | _ ? \\"),
        vec![
            Token::Comma, Token::Semi, Token::Colon, Token::Dot,
            Token::Eq, Token::At, Token::Bar, Token::Underscore,
            Token::Question, Token::Backslash,
        ],
    );
}

// ---- 1.4 Literals --------------------------------------------------------

#[test]
fn lex_integer_literal() {
    assert_eq!(tokens("42"), vec![Token::Int(42)]);
}

#[test]
fn lex_integer_zero() {
    assert_eq!(tokens("0"), vec![Token::Int(0)]);
}

#[test]
fn lex_large_integer() {
    assert_eq!(tokens("9999999999"), vec![Token::Int(9_999_999_999)]);
}

#[test]
fn lex_float_literal() {
    assert_eq!(tokens("3.14"), vec![Token::Float(3.14)]);
}

#[test]
fn lex_float_with_exponent() {
    let toks = tokens("1.0e10");
    assert_eq!(toks.len(), 1);
    assert!(matches!(toks[0], Token::Float(f) if (f - 1.0e10).abs() < 1.0));
}

#[test]
fn lex_integer_with_exponent() {
    // Pattern: [0-9]+[eE][+-]?[0-9]+  (no dot) is also Float
    let toks = tokens("2e3");
    assert_eq!(toks.len(), 1);
    assert!(matches!(toks[0], Token::Float(f) if (f - 2000.0).abs() < 0.5));
}

#[test]
fn lex_float_negative_exponent() {
    let toks = tokens("5.0e-2");
    assert_eq!(toks.len(), 1);
    assert!(matches!(toks[0], Token::Float(f) if (f - 0.05).abs() < 1e-10));
}

#[test]
fn lex_string_literal() {
    assert_eq!(tokens(r#""hello""#), vec![Token::String("hello".into())]);
}

#[test]
fn lex_string_with_escape() {
    assert_eq!(tokens(r#""foo\"bar""#), vec![Token::String(r#"foo\"bar"#.into())]);
}

#[test]
fn lex_empty_string() {
    assert_eq!(tokens(r#""""#), vec![Token::String(String::new())]);
}

// ---- 1.5 Identifiers and type variables ----------------------------------

#[test]
fn lex_identifier() {
    assert_eq!(tokens("myVar"), vec![Token::Ident("myVar".into())]);
}

#[test]
fn lex_identifier_with_underscores() {
    // Note: leading underscore creates Underscore + Ident because "_" matches
    // the Underscore token. A name like "foo_bar" is fine.
    assert_eq!(tokens("foo_bar"), vec![Token::Ident("foo_bar".into())]);
}

#[test]
fn lex_type_variable() {
    assert_eq!(tokens("'a"), vec![Token::TypeVar("a".into())]);
}

#[test]
fn lex_type_variable_longer() {
    assert_eq!(tokens("'alpha"), vec![Token::TypeVar("alpha".into())]);
}

// ---- 1.6 Comments --------------------------------------------------------

#[test]
fn lex_line_comment_skipped() {
    assert_eq!(tokens("42 -- this is a comment"), vec![Token::Int(42)]);
}

#[test]
fn lex_block_comment_skipped() {
    assert_eq!(tokens("{- block comment -} 7"), vec![Token::Int(7)]);
}

#[test]
fn lex_block_comment_multiline() {
    let src = "{- multi\n   line\n   comment -} 99";
    assert_eq!(tokens(src), vec![Token::Int(99)]);
}

// ---- 1.7 Whitespace handling ---------------------------------------------

#[test]
fn lex_whitespace_variants() {
    // Tabs, carriage returns, form feeds should all be skipped
    assert_eq!(tokens("\t\r\n 1"), vec![Token::Int(1)]);
}

// ---- 1.8 Token spans are correct -----------------------------------------

#[test]
fn lex_span_positions() {
    let spanned = lex("let x = 42").unwrap();
    // "let" is at bytes 0..3
    assert_eq!(spanned[0].span.start, 0);
    assert_eq!(spanned[0].span.end, 3);
    // "x" is at bytes 4..5
    assert_eq!(spanned[1].span.start, 4);
    assert_eq!(spanned[1].span.end, 5);
    // "=" is at byte 6..7
    assert_eq!(spanned[2].span.start, 6);
    assert_eq!(spanned[2].span.end, 7);
    // "42" is at bytes 8..10
    assert_eq!(spanned[3].span.start, 8);
    assert_eq!(spanned[3].span.end, 10);
}

// ---- 1.9 Multi-token sequences -------------------------------------------

#[test]
fn lex_bet_expression_tokens() {
    let toks = tokens("bet { a @ 0.5, b @ 0.3, c @ 0.2 }");
    assert_eq!(toks[0], Token::Bet);
    assert_eq!(toks[1], Token::LBrace);
    assert!(toks.iter().any(|t| matches!(t, Token::At)));
    assert!(toks.iter().any(|t| matches!(t, Token::Comma)));
    assert_eq!(*toks.last().unwrap(), Token::RBrace);
}

#[test]
fn lex_do_notation_tokens() {
    let toks = tokens("do { x <- sample normal; return x }");
    assert_eq!(toks[0], Token::Do);
    assert!(toks.iter().any(|t| matches!(t, Token::LArrow)));
    assert!(toks.iter().any(|t| matches!(t, Token::Sample)));
    assert!(toks.iter().any(|t| matches!(t, Token::Return)));
    assert!(toks.iter().any(|t| matches!(t, Token::Semi)));
}

// ---- 1.10 Lexer error on invalid token -----------------------------------

#[test]
fn lex_invalid_token_error() {
    let result = lex("`");
    assert!(result.is_err());
}

#[test]
fn lex_invalid_token_in_middle() {
    let result = lex("let x = `bad");
    assert!(result.is_err());
}

// ===========================================================================
// 2. Parser Tests -- Expressions
// ===========================================================================

// ---- 2.1 Literal expressions --------------------------------------------

#[test]
fn parse_int_literal() {
    let expr = parse_expr("42").unwrap();
    assert!(matches!(expr, Expr::Int(42)));
}

#[test]
fn parse_float_literal() {
    let expr = parse_expr("3.14").unwrap();
    assert!(matches!(expr, Expr::Float(f) if (f - 3.14).abs() < 1e-10));
}

#[test]
fn parse_string_literal() {
    let expr = parse_expr(r#""hello""#).unwrap();
    assert!(matches!(expr, Expr::String(ref s) if s == "hello"));
}

#[test]
fn parse_true_literal() {
    let expr = parse_expr("true").unwrap();
    assert!(matches!(expr, Expr::Ternary(TernaryValue::True)));
}

#[test]
fn parse_false_literal() {
    let expr = parse_expr("false").unwrap();
    assert!(matches!(expr, Expr::Ternary(TernaryValue::False)));
}

#[test]
fn parse_unknown_literal() {
    let expr = parse_expr("unknown").unwrap();
    assert!(matches!(expr, Expr::Ternary(TernaryValue::Unknown)));
}

#[test]
fn parse_unit_literal() {
    let expr = parse_expr("()").unwrap();
    assert!(matches!(expr, Expr::Unit));
}

// ---- 2.2 Variable references --------------------------------------------

#[test]
fn parse_variable() {
    let expr = parse_expr("x").unwrap();
    match expr {
        Expr::Var(sym) => assert_eq!(sym.as_str(), "x"),
        other => panic!("Expected Var, got {:?}", other),
    }
}

// ---- 2.3 Ternary bet expressions -----------------------------------------

#[test]
fn parse_bet_braces() {
    let expr = parse_expr("bet { 1, 2, 3 }").unwrap();
    assert!(matches!(expr, Expr::Bet(_)));
}

#[test]
fn parse_bet_end() {
    let expr = parse_expr("bet 1, 2, 3 end").unwrap();
    assert!(matches!(expr, Expr::Bet(_)));
}

#[test]
fn parse_bet_with_vars() {
    let expr = parse_expr("bet { x, y, z }").unwrap();
    match &expr {
        Expr::Bet(BetExpr { alternatives }) => {
            assert!(matches!(alternatives[0].node, Expr::Var(_)));
            assert!(matches!(alternatives[1].node, Expr::Var(_)));
            assert!(matches!(alternatives[2].node, Expr::Var(_)));
        }
        other => panic!("Expected Bet, got {:?}", other),
    }
}

#[test]
fn parse_weighted_bet() {
    let expr = parse_expr("bet { 1 @ 0.5, 2 @ 0.3, 3 @ 0.2 }").unwrap();
    assert!(matches!(expr, Expr::WeightedBet(_)));
}

#[test]
fn parse_weighted_bet_alternatives() {
    let expr = parse_expr("bet { 10 @ 0.6, 20 @ 0.3, 30 @ 0.1 }").unwrap();
    match &expr {
        Expr::WeightedBet(WeightedBetExpr { alternatives }) => {
            assert!(matches!(alternatives[0].0.node, Expr::Int(10)));
            assert!(matches!(alternatives[1].0.node, Expr::Int(20)));
            assert!(matches!(alternatives[2].0.node, Expr::Int(30)));
        }
        other => panic!("Expected WeightedBet, got {:?}", other),
    }
}

// ---- 2.4 Arithmetic and binary operators ---------------------------------

#[test]
fn parse_addition() {
    let expr = parse_expr("1 + 2").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Add, _, _)));
}

#[test]
fn parse_subtraction() {
    let expr = parse_expr("5 - 3").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Sub, _, _)));
}

#[test]
fn parse_multiplication() {
    let expr = parse_expr("4 * 2").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Mul, _, _)));
}

#[test]
fn parse_division() {
    let expr = parse_expr("8 / 2").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Div, _, _)));
}

#[test]
fn parse_modulo() {
    let expr = parse_expr("7 % 3").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Mod, _, _)));
}

#[test]
fn parse_concat() {
    let expr = parse_expr(r#""a" ++ "b""#).unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Concat, _, _)));
}

#[test]
fn parse_comparison_eq() {
    let expr = parse_expr("x == y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Eq, _, _)));
}

#[test]
fn parse_comparison_ne() {
    let expr = parse_expr("x != y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Ne, _, _)));
}

#[test]
fn parse_comparison_lt() {
    let expr = parse_expr("x < y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Lt, _, _)));
}

#[test]
fn parse_comparison_le() {
    let expr = parse_expr("x <= y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Le, _, _)));
}

#[test]
fn parse_comparison_gt() {
    let expr = parse_expr("x > y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Gt, _, _)));
}

#[test]
fn parse_comparison_ge() {
    let expr = parse_expr("x >= y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Ge, _, _)));
}

#[test]
fn parse_logical_and() {
    let expr = parse_expr("x && y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::And, _, _)));
}

#[test]
fn parse_logical_or() {
    let expr = parse_expr("x || y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Or, _, _)));
}

#[test]
fn parse_logical_and_keyword() {
    let expr = parse_expr("x and y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::And, _, _)));
}

#[test]
fn parse_logical_or_keyword() {
    let expr = parse_expr("x or y").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Or, _, _)));
}

// ---- 2.5 Unary operators -------------------------------------------------

#[test]
fn parse_negation() {
    let expr = parse_expr("-1").unwrap();
    assert!(matches!(expr, Expr::UnOp(UnOp::Neg, _)));
}

#[test]
fn parse_not() {
    let expr = parse_expr("not true").unwrap();
    assert!(matches!(expr, Expr::UnOp(UnOp::Not, _)));
}

// ---- 2.6 Operator precedence --------------------------------------------

#[test]
fn parse_precedence_mul_before_add() {
    // 1 + 2 * 3 should parse as 1 + (2 * 3)
    let expr = parse_expr("1 + 2 * 3").unwrap();
    match expr {
        Expr::BinOp(BinOp::Add, lhs, rhs) => {
            assert!(matches!(lhs.node, Expr::Int(1)));
            assert!(matches!(rhs.node, Expr::BinOp(BinOp::Mul, _, _)));
        }
        other => panic!("Expected Add at top level, got {:?}", other),
    }
}

#[test]
fn parse_precedence_compare_before_logical() {
    // x && y == z should parse as x && (y == z)
    let expr = parse_expr("x && y == z").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::And, _, _)));
}

// ---- 2.7 Lambda / function expressions -----------------------------------

#[test]
fn parse_lambda_single_param() {
    let expr = parse_expr("fun x -> x").unwrap();
    match expr {
        Expr::Lambda(LambdaExpr { params, body }) => {
            assert_eq!(params.len(), 1);
            assert!(matches!(body.node, Expr::Var(_)));
        }
        other => panic!("Expected Lambda, got {:?}", other),
    }
}

#[test]
fn parse_lambda_multiple_params() {
    let expr = parse_expr("fun x y z -> x").unwrap();
    match expr {
        Expr::Lambda(LambdaExpr { params, .. }) => {
            assert_eq!(params.len(), 3);
        }
        other => panic!("Expected Lambda, got {:?}", other),
    }
}

// ---- 2.8 Let expressions -------------------------------------------------

#[test]
fn parse_let_in_expr() {
    let expr = parse_expr("let x = 1 in x").unwrap();
    match expr {
        Expr::Let(LetExpr { is_rec, .. }) => assert!(!is_rec),
        other => panic!("Expected Let, got {:?}", other),
    }
}

#[test]
fn parse_let_rec_in_expr() {
    let expr = parse_expr("let rec f = fun x -> x in f").unwrap();
    match expr {
        Expr::Let(LetExpr { is_rec, .. }) => assert!(is_rec),
        other => panic!("Expected Let(rec), got {:?}", other),
    }
}

#[test]
fn parse_let_with_type_annotation() {
    let expr = parse_expr("let x : Int = 42 in x").unwrap();
    match expr {
        Expr::Let(LetExpr { type_ann, .. }) => assert!(type_ann.is_some()),
        other => panic!("Expected Let with type_ann, got {:?}", other),
    }
}

// ---- 2.9 If expressions --------------------------------------------------

#[test]
fn parse_if_then_else_end() {
    let expr = parse_expr("if true then 1 else 0 end").unwrap();
    assert!(matches!(expr, Expr::If(_)));
}

#[test]
fn parse_nested_if() {
    let expr = parse_expr("if x then if y then 1 else 2 end else 3 end").unwrap();
    match expr {
        Expr::If(IfExpr { then_branch, .. }) => {
            assert!(matches!(then_branch.node, Expr::If(_)));
        }
        other => panic!("Expected If, got {:?}", other),
    }
}

// ---- 2.10 Match expressions ----------------------------------------------

#[test]
fn parse_match_braces() {
    let expr = parse_expr("match x { a -> 1, b -> 2 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => assert_eq!(arms.len(), 2),
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_match_end() {
    let expr = parse_expr("match x a -> 1; b -> 2 end").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => assert_eq!(arms.len(), 2),
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_match_three_arms() {
    let expr = parse_expr("match x { true -> 1, false -> 0, unknown -> 2 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => assert_eq!(arms.len(), 3),
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_match_wildcard_pattern() {
    let expr = parse_expr("match x { _ -> 0 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::Wildcard));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

// ---- 2.11 Do notation ----------------------------------------------------

#[test]
fn parse_do_braces() {
    // `return` is just a variable (not special syntax), `x` is a bind target
    let expr = parse_expr("do { x <- sample normal; x }").unwrap();
    match expr {
        Expr::Do(DoExpr { statements }) => assert_eq!(statements.len(), 2),
        other => panic!("Expected Do, got {:?}", other),
    }
}

#[test]
fn parse_do_end() {
    let expr = parse_expr("do x <- sample normal; x end").unwrap();
    match expr {
        Expr::Do(DoExpr { statements }) => assert_eq!(statements.len(), 2),
        other => panic!("Expected Do, got {:?}", other),
    }
}

#[test]
fn parse_do_let_statement() {
    let expr = parse_expr("do { let y = 10; y }").unwrap();
    match expr {
        Expr::Do(DoExpr { statements }) => {
            assert_eq!(statements.len(), 2);
            assert!(matches!(statements[0].node, DoStatement::Let(_, _)));
            assert!(matches!(statements[1].node, DoStatement::Expr(_)));
        }
        other => panic!("Expected Do, got {:?}", other),
    }
}

// ---- 2.12 Probabilistic expressions -------------------------------------

#[test]
fn parse_sample() {
    let expr = parse_expr("sample normal").unwrap();
    assert!(matches!(expr, Expr::Sample(_)));
}

#[test]
fn parse_sample_parenthesized() {
    // sample takes an atom; wrap complex exprs in parens
    let expr = parse_expr("sample (normal)").unwrap();
    assert!(matches!(expr, Expr::Sample(_)));
}

#[test]
fn parse_parallel_braces() {
    let expr = parse_expr("parallel 4 { x + 1 }").unwrap();
    assert!(matches!(expr, Expr::Parallel(_, _)));
}

#[test]
fn parse_parallel_end() {
    let expr = parse_expr("parallel 4 do x + 1 end").unwrap();
    assert!(matches!(expr, Expr::Parallel(_, _)));
}

// ---- 2.13 Data structures ------------------------------------------------

#[test]
fn parse_tuple() {
    let expr = parse_expr("(1, 2, 3)").unwrap();
    match expr {
        Expr::Tuple(elems) => assert_eq!(elems.len(), 3),
        other => panic!("Expected Tuple, got {:?}", other),
    }
}

#[test]
fn parse_pair() {
    let expr = parse_expr("(1, 2)").unwrap();
    match expr {
        Expr::Tuple(elems) => assert_eq!(elems.len(), 2),
        other => panic!("Expected Tuple, got {:?}", other),
    }
}

#[test]
fn parse_list_empty() {
    let expr = parse_expr("[]").unwrap();
    match expr {
        Expr::List(elems) => assert!(elems.is_empty()),
        other => panic!("Expected List, got {:?}", other),
    }
}

#[test]
fn parse_list_with_elements() {
    let expr = parse_expr("[1, 2, 3]").unwrap();
    match expr {
        Expr::List(elems) => assert_eq!(elems.len(), 3),
        other => panic!("Expected List, got {:?}", other),
    }
}

#[test]
fn parse_list_trailing_comma() {
    let expr = parse_expr("[1, 2, 3,]").unwrap();
    match expr {
        Expr::List(elems) => assert_eq!(elems.len(), 3),
        other => panic!("Expected List, got {:?}", other),
    }
}

#[test]
fn parse_record() {
    let expr = parse_expr("{ x = 1, y = 2 }").unwrap();
    match expr {
        Expr::Record(fields) => assert_eq!(fields.len(), 2),
        other => panic!("Expected Record, got {:?}", other),
    }
}

// ---- 2.14 Parenthesized expressions --------------------------------------

#[test]
fn parse_parenthesized() {
    let expr = parse_expr("(1 + 2)").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Add, _, _)));
}

#[test]
fn parse_type_annotation_expr() {
    let expr = parse_expr("(x : Int)").unwrap();
    assert!(matches!(expr, Expr::Annotate(_, _)));
}

// ---- 2.15 Holes -----------------------------------------------------------

#[test]
fn parse_anonymous_hole() {
    let expr = parse_expr("_").unwrap();
    assert!(matches!(expr, Expr::Hole(None)));
}

#[test]
fn parse_named_hole() {
    let expr = parse_expr("?myhole").unwrap();
    match expr {
        Expr::Hole(Some(sym)) => assert_eq!(sym.as_str(), "myhole"),
        other => panic!("Expected named Hole, got {:?}", other),
    }
}

// ---- 2.16 Function application -------------------------------------------

#[test]
fn parse_variable_reference() {
    // Without juxtaposition application, `f` is just a variable
    let expr = parse_expr("f").unwrap();
    assert!(matches!(expr, Expr::Var(_)));
}

#[test]
fn parse_complex_expression() {
    // More complex expressions use operators, not juxtaposition
    let expr = parse_expr("f + g").unwrap();
    assert!(matches!(expr, Expr::BinOp(BinOp::Add, _, _)));
}

// ===========================================================================
// 3. Parser Tests -- Top-Level Items (Module)
// ===========================================================================

// ---- 3.1 Simple let bindings (module level) ------------------------------

#[test]
fn parse_module_let_binding() {
    let module = parse("let x = 42").unwrap();
    assert_eq!(module.items.len(), 1);
    assert!(matches!(module.items[0].node, Item::Let(_)));
}

#[test]
fn parse_module_let_rec() {
    let module = parse("let rec fact n = n").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::Let(LetDef { is_rec, .. }) => assert!(is_rec),
        other => panic!("Expected Let, got {:?}", other),
    }
}

// ---- 3.2 Function definitions (module level) -----------------------------

#[test]
fn parse_module_function() {
    let module = parse("let add = fun x y -> x + y").unwrap();
    assert_eq!(module.items.len(), 1);
}

#[test]
fn parse_module_function_with_params() {
    let module = parse("let add x y = x + y").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::Let(LetDef { params, .. }) => assert_eq!(params.len(), 2),
        other => panic!("Expected Let with params, got {:?}", other),
    }
}

#[test]
fn parse_module_function_with_type_ann() {
    let module = parse("let double x : Int = x + x").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::Let(LetDef { type_ann, .. }) => assert!(type_ann.is_some()),
        other => panic!("Expected Let with type_ann, got {:?}", other),
    }
}

// ---- 3.3 Type definitions ------------------------------------------------

#[test]
fn parse_type_alias() {
    let module = parse("type Probability = Float").unwrap();
    assert_eq!(module.items.len(), 1);
    assert!(matches!(module.items[0].node, Item::TypeDef(_)));
}

#[test]
fn parse_type_with_params() {
    let module = parse("type Maybe 'a = 'a").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::TypeDef(TypeDef { params, .. }) => assert_eq!(params.len(), 1),
        other => panic!("Expected TypeDef with params, got {:?}", other),
    }
}

#[test]
fn parse_function_type_definition() {
    let module = parse("type Transformer 'a 'b = 'a -> 'b").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::TypeDef(TypeDef { params, body, .. }) => {
            assert_eq!(params.len(), 2);
            assert!(matches!(body.node, Type::Arrow(_, _)));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

// ---- 3.4 Imports ----------------------------------------------------------

#[test]
fn parse_import_simple() {
    let module = parse("import Stats").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::Import(Import { path, items }) => {
            assert_eq!(path.len(), 1);
            assert!(items.is_none());
        }
        other => panic!("Expected Import, got {:?}", other),
    }
}

#[test]
fn parse_import_qualified() {
    let module = parse("import Stats Distributions").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::Import(Import { path, items }) => {
            assert_eq!(path.len(), 2);
            assert!(items.is_none());
        }
        other => panic!("Expected Import, got {:?}", other),
    }
}

#[test]
fn parse_import_selective() {
    let module = parse("import Stats . { normal, uniform }").unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::Import(Import { items, .. }) => {
            let items = items.as_ref().unwrap();
            assert_eq!(items.len(), 2);
        }
        other => panic!("Expected Import with items, got {:?}", other),
    }
}

// ---- 3.5 Module with multiple items --------------------------------------

#[test]
fn parse_module_multiple_items() {
    let src = r#"
        type Prob = Float
        let x = 42
        let f x = x + 1
    "#;
    let module = parse(src).unwrap();
    assert_eq!(module.items.len(), 3);
    assert!(matches!(module.items[0].node, Item::TypeDef(_)));
    assert!(matches!(module.items[1].node, Item::Let(_)));
    assert!(matches!(module.items[2].node, Item::Let(_)));
}

#[test]
fn parse_empty_module() {
    let module = parse("").unwrap();
    assert!(module.items.is_empty());
    assert!(module.name.is_none());
}

// ---- 3.6 Expression at top level ----------------------------------------

#[test]
fn parse_module_expr_via_let() {
    // Top-level expressions require wrapping in `let _ = expr`
    let module = parse("let result = 42").unwrap();
    assert_eq!(module.items.len(), 1);
    assert!(matches!(module.items[0].node, Item::Let(_)));
}

// ===========================================================================
// 4. Pattern Tests
// ===========================================================================

#[test]
fn parse_pattern_wildcard_in_match() {
    let expr = parse_expr("match x { _ -> 1 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::Wildcard));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_pattern_literal_int() {
    let expr = parse_expr("match x { 42 -> 1 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::Literal(Literal::Int(42))));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_pattern_literal_string() {
    let expr = parse_expr(r#"match x { "hi" -> 1 }"#).unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::Literal(Literal::String(_))));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_pattern_ternary_true() {
    let expr = parse_expr("match x { true -> 1 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(
                arms[0].pattern.node,
                Pattern::Literal(Literal::Ternary(TernaryValue::True))
            ));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_pattern_tuple() {
    let expr = parse_expr("match x { (a, b) -> a }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::Tuple(_)));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_pattern_list() {
    let expr = parse_expr("match x { [a, b] -> a }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::List(_, _)));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_pattern_unit() {
    let expr = parse_expr("match x { () -> 1 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::Literal(Literal::Unit)));
        }
        other => panic!("Expected Match, got {:?}", other),
    }
}

#[test]
fn parse_pattern_or_ternary() {
    // Ternary or-pattern: p1 | p2 | p3
    let expr = parse_expr("match x { a | b | c -> 1 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert!(matches!(arms[0].pattern.node, Pattern::Or(_, _, _)));
        }
        other => panic!("Expected Match with Or pattern, got {:?}", other),
    }
}

// ===========================================================================
// 5. Type annotation tests
// ===========================================================================

#[test]
fn parse_type_named() {
    let module = parse("type X = Int").unwrap();
    match &module.items[0].node {
        Item::TypeDef(TypeDef { body, .. }) => {
            assert!(matches!(body.node, Type::Named(_)));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

#[test]
fn parse_type_arrow() {
    let module = parse("type F = Int -> Float").unwrap();
    match &module.items[0].node {
        Item::TypeDef(TypeDef { body, .. }) => {
            assert!(matches!(body.node, Type::Arrow(_, _)));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

#[test]
fn parse_type_tuple() {
    let module = parse("type Pair = (Int, Float)").unwrap();
    match &module.items[0].node {
        Item::TypeDef(TypeDef { body, .. }) => {
            assert!(matches!(body.node, Type::Tuple(_)));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

#[test]
fn parse_type_record() {
    let module = parse("type Point = { x : Float, y : Float }").unwrap();
    match &module.items[0].node {
        Item::TypeDef(TypeDef { body, .. }) => {
            assert!(matches!(body.node, Type::Record(_)));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

#[test]
fn parse_type_variable() {
    let module = parse("type Identity 'a = 'a").unwrap();
    match &module.items[0].node {
        Item::TypeDef(TypeDef { body, .. }) => {
            assert!(matches!(body.node, Type::Var(_)));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

#[test]
fn parse_type_application() {
    let module = parse("type IntList = List Int").unwrap();
    match &module.items[0].node {
        Item::TypeDef(TypeDef { body, .. }) => {
            assert!(matches!(body.node, Type::App(_, _)));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

#[test]
fn parse_type_hole() {
    let module = parse("type Inferred = _").unwrap();
    match &module.items[0].node {
        Item::TypeDef(TypeDef { body, .. }) => {
            assert!(matches!(body.node, Type::Hole));
        }
        other => panic!("Expected TypeDef, got {:?}", other),
    }
}

// ===========================================================================
// 6. Error handling tests
// ===========================================================================

#[test]
fn parse_error_unclosed_brace() {
    let result = parse_expr("bet { 1, 2, 3");
    assert!(result.is_err());
}

#[test]
fn parse_error_missing_else() {
    // if without else (and end) should fail
    let result = parse_expr("if true then 1 end");
    assert!(result.is_err());
}

#[test]
fn parse_error_empty_bet() {
    // bet must have exactly three alternatives
    let result = parse_expr("bet { }");
    assert!(result.is_err());
}

// ===========================================================================
// 7. Comments in parsed code
// ===========================================================================

#[test]
fn parse_with_line_comments() {
    let src = r#"
        -- this is a comment
        let x = 42
    "#;
    let module = parse(src).unwrap();
    assert_eq!(module.items.len(), 1);
}

#[test]
fn parse_with_block_comments() {
    let src = r#"
        {- a block comment -}
        let y = 99
    "#;
    let module = parse(src).unwrap();
    assert_eq!(module.items.len(), 1);
}

// ===========================================================================
// 8. Integration / realistic examples
// ===========================================================================

#[test]
fn parse_probabilistic_model() {
    let src = r#"
        let coin = bet { true, false, unknown }
    "#;
    let module = parse(src).unwrap();
    assert_eq!(module.items.len(), 1);
    match &module.items[0].node {
        Item::Let(LetDef { body, .. }) => {
            assert!(matches!(body.node, Expr::Bet(_)));
        }
        other => panic!("Expected Let with Bet body, got {:?}", other),
    }
}

#[test]
fn parse_do_sampling_pipeline() {
    let src = "do { x <- sample normal; let y = x + 1; y }";
    let expr = parse_expr(src).unwrap();
    match expr {
        Expr::Do(DoExpr { statements }) => {
            assert_eq!(statements.len(), 3);
            assert!(matches!(statements[0].node, DoStatement::Bind(_, _)));
            assert!(matches!(statements[1].node, DoStatement::Let(_, _)));
            assert!(matches!(statements[2].node, DoStatement::Expr(_)));
        }
        other => panic!("Expected Do, got {:?}", other),
    }
}

#[test]
fn parse_match_with_guard() {
    let expr = parse_expr("match x { n if n > 0 -> n, _ -> 0 }").unwrap();
    match expr {
        Expr::Match(MatchExpr { arms, .. }) => {
            assert_eq!(arms.len(), 2);
            assert!(arms[0].guard.is_some());
            assert!(arms[1].guard.is_none());
        }
        other => panic!("Expected Match with guard, got {:?}", other),
    }
}
