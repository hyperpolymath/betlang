// SPDX-License-Identifier: MIT OR Apache-2.0
//! Completion handler

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Betlang keywords for completion
pub const KEYWORDS: &[&str] = &[
    "bet",
    "bet/weighted",
    "bet/conditional",
    "bet/lazy",
    "bet-chain",
    "bet-compose",
    "bet-map",
    "bet-filter",
    "bet-repeat",
    "bet-until",
    "define",
    "lambda",
    "let",
    "cond",
    "if",
    "and",
    "or",
    "not",
    "true",
    "false",
    "maybe",
];

/// Standard library modules
const STDLIB_MODULES: &[&str] = &[
    "statistics",
    "distributions",
    "bayesian",
    "optimization",
    "sampling",
    "markov",
    "combinators",
    "ternary",
];

/// Built-in functions
const BUILTINS: &[&str] = &[
    "bet-probability",
    "bet-value",
    "bet-outcomes",
    "bet-entropy",
    "bet-expected-value",
    "bet-variance",
    "bet-sample",
    "bet-trace",
];

/// Handle completion request
pub async fn completion(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    // Get document
    let doc = match backend.document_map.get(&uri) {
        Some(d) => d,
        None => return Ok(None),
    };

    // Get word at cursor
    let word = doc
        .word_at_position(position.line, position.character)
        .unwrap_or_default();

    let mut items = Vec::new();

    // Add keywords
    for keyword in KEYWORDS {
        if keyword.starts_with(&word) || word.is_empty() {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Keyword".to_string()),
                sort_text: Some(format!("1_{}", keyword)),
                ..Default::default()
            });
        }
    }

    // Add builtins
    for builtin in BUILTINS {
        if builtin.starts_with(&word) || word.is_empty() {
            items.push(CompletionItem {
                label: builtin.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Built-in function".to_string()),
                sort_text: Some(format!("0_{}", builtin)),
                ..Default::default()
            });
        }
    }

    // Add stdlib modules
    for module in STDLIB_MODULES {
        if module.starts_with(&word) || word.is_empty() {
            items.push(CompletionItem {
                label: module.to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some(format!("Standard library module: {}", module)),
                sort_text: Some(format!("2_{}", module)),
                ..Default::default()
            });
        }
    }

    Ok(Some(CompletionResponse::Array(items)))
}
