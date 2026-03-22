// SPDX-License-Identifier: MIT OR Apache-2.0
//! Integration tests for bet-lsp

use bet_lsp::document::DocumentState;
use bet_lsp::handlers::completion::KEYWORDS;
use bet_lsp::handlers::formatting::format_racket;
use bet_lsp::utils::LineIndex;
use tower_lsp::lsp_types::Url;

#[test]
fn test_document_state_creation() {
    let uri = Url::parse("file:///test.bet").unwrap();
    let source = "bet { a, b, c }".to_string();
    let doc = DocumentState::new(uri, source.clone(), 1);

    assert_eq!(doc.source, source);
}

#[test]
fn test_document_tokens() {
    let uri = Url::parse("file:///test.bet").unwrap();
    let source = "bet { a, b, c }".to_string();
    let doc = DocumentState::new(uri, source, 1);

    let tokens = doc.tokens();
    assert!(tokens.is_ok());
    assert!(tokens.as_ref().unwrap().len() > 0);
}

#[test]
fn test_document_ast() {
    let uri = Url::parse("file:///test.bet").unwrap();
    let source = "bet { a, b, c }".to_string();
    let doc = DocumentState::new(uri, source.clone(), 1);

    let ast = doc.ast();
    assert!(ast.is_ok());
}

#[test]
fn test_word_at_position() {
    let uri = Url::parse("file:///test.bet").unwrap();
    let source = "bet { hello, world, test }".to_string();
    let doc = DocumentState::new(uri, source, 1);

    // Position within "hello"
    let word = doc.word_at_position(0, 7);
    assert_eq!(word, Some("hello".to_string()));

    // Position within "world"
    let word = doc.word_at_position(0, 14);
    assert_eq!(word, Some("world".to_string()));
}

#[test]
fn test_line_index_position_to_offset() {
    let source = "line 1\nline 2\nline 3";
    let index = LineIndex::new(source);

    // Start of line 0
    assert_eq!(index.position_to_offset(0, 0), Some(0));

    // Start of line 1
    assert_eq!(index.position_to_offset(1, 0), Some(7));

    // Start of line 2
    assert_eq!(index.position_to_offset(2, 0), Some(14));

    // Middle of line 0
    assert_eq!(index.position_to_offset(0, 5), Some(5));
}

#[test]
fn test_keywords_completion() {
    // Test that betlang keywords are present
    assert!(KEYWORDS.contains(&"bet"));
    assert!(KEYWORDS.contains(&"bet/weighted"));
    assert!(KEYWORDS.contains(&"bet/conditional"));
    assert!(KEYWORDS.contains(&"bet-chain"));
    assert!(KEYWORDS.contains(&"lambda"));
    assert!(KEYWORDS.contains(&"define"));
}

#[test]
fn test_formatter_basic() {
    let input = "(define x 42)";
    let output = format_racket(input);
    assert!(output.contains("define"));
    assert!(output.contains("x"));
    assert!(output.contains("42"));
}

#[test]
fn test_formatter_nested() {
    let input = "(define (factorial n)\n  (if (= n 0)\n    1\n    (* n (factorial (- n 1)))))";
    let output = format_racket(input);

    // Should preserve the key words
    assert!(output.contains("define"));
    assert!(output.contains("factorial"));
    assert!(output.contains("if"));
}

#[test]
fn test_formatter_bet_expression() {
    let input = "(bet 'a 'b 'c)";
    let output = format_racket(input);
    assert!(output.contains("bet"));
    assert!(output.contains("'a"));
    assert!(output.contains("'b"));
    assert!(output.contains("'c"));
}

#[test]
fn test_word_at_position_edge_cases() {
    let uri = Url::parse("file:///test.bet").unwrap();

    // Empty document
    let doc = DocumentState::new(uri.clone(), "".to_string(), 1);
    assert_eq!(doc.word_at_position(0, 0), None);

    // Single word
    let doc = DocumentState::new(uri.clone(), "word".to_string(), 1);
    assert_eq!(doc.word_at_position(0, 2), Some("word".to_string()));

    // Word with dashes and underscores
    let doc = DocumentState::new(uri, "bet-weighted_choice".to_string(), 1);
    assert_eq!(doc.word_at_position(0, 5), Some("bet-weighted_choice".to_string()));
}
