// SPDX-License-Identifier: MIT OR Apache-2.0
//! Diagnostics handler

use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Publish diagnostics for a document
pub async fn publish_diagnostics(backend: &Backend, uri: &Url) {
    let diagnostics = collect_diagnostics(backend, uri);

    backend
        .client
        .publish_diagnostics(uri.clone(), diagnostics, None)
        .await;
}

/// Collect all diagnostics for a document
fn collect_diagnostics(backend: &Backend, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Get document
    let doc = match backend.document_map.get(uri) {
        Some(d) => d,
        None => return diagnostics,
    };

    // Try to tokenize (simplified - real tokenizer would use bet-lexer)
    if let Err(err) = doc.tokens() {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("lex-error".to_string())),
            source: Some("betlang-lsp".to_string()),
            message: err.clone(),
            ..Default::default()
        });
        return diagnostics;
    }

    // Try to parse (simplified - would use bet-parse once grammar conflicts are fixed)
    if let Err(err) = doc.ast() {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("parse-error".to_string())),
            source: Some("betlang-lsp".to_string()),
            message: err.clone(),
            ..Default::default()
        });
        return diagnostics;
    }

    // TODO: Add type checking diagnostics once bet-check is working
    diagnostics
}
