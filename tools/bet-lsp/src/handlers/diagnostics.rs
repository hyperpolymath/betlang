// SPDX-License-Identifier: MIT OR Apache-2.0
//! Diagnostics handler — produces parse-level and structural diagnostics
//! for betlang documents.
//!
//! Performs lightweight lexical analysis to detect common errors without
//! requiring the full LALR parser (which has known grammar conflicts).

use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Publish diagnostics for a document.
pub async fn publish_diagnostics(backend: &Backend, uri: &Url) {
    let diagnostics = collect_diagnostics(backend, uri);

    backend
        .client
        .publish_diagnostics(uri.clone(), diagnostics, None)
        .await;
}

/// Collect all diagnostics for a document.
fn collect_diagnostics(backend: &Backend, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let doc = match backend.document_map.get(uri) {
        Some(d) => d,
        None => return diagnostics,
    };

    // Check tokenization
    if let Err(err) = doc.tokens() {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("lex-error".into())),
            source: Some("bet-lsp".into()),
            message: err.clone(),
            ..Default::default()
        });
        return diagnostics;
    }

    // Check basic parsing
    if let Err(err) = doc.ast() {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("parse-error".into())),
            source: Some("bet-lsp".into()),
            message: err.clone(),
            ..Default::default()
        });
        return diagnostics;
    }

    // Structural analysis
    let source = &doc.source;
    let mut paren_depth: i32 = 0;
    let mut brace_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut in_string = false;

    for (line_idx, line) in source.lines().enumerate() {
        let ln = line_idx as u32;

        for (col_idx, ch) in line.char_indices() {
            if ch == '"' {
                in_string = !in_string;
                continue;
            }
            if in_string {
                continue;
            }
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    paren_depth -= 1;
                    if paren_depth < 0 {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position::new(ln, col_idx as u32),
                                end: Position::new(ln, col_idx as u32 + 1),
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("unmatched-paren".into())),
                            source: Some("bet-lsp".into()),
                            message: "Unmatched closing parenthesis".into(),
                            ..Default::default()
                        });
                    }
                }
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth -= 1;
                    if brace_depth < 0 {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position::new(ln, col_idx as u32),
                                end: Position::new(ln, col_idx as u32 + 1),
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("unmatched-brace".into())),
                            source: Some("bet-lsp".into()),
                            message: "Unmatched closing brace".into(),
                            ..Default::default()
                        });
                    }
                }
                '[' => bracket_depth += 1,
                ']' => {
                    bracket_depth -= 1;
                    if bracket_depth < 0 {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position::new(ln, col_idx as u32),
                                end: Position::new(ln, col_idx as u32 + 1),
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("unmatched-bracket".into())),
                            source: Some("bet-lsp".into()),
                            message: "Unmatched closing bracket".into(),
                            ..Default::default()
                        });
                    }
                }
                _ => {}
            }
        }

        // Check for bet expressions with wrong arity (not exactly 3)
        let trimmed = line.trim();
        if trimmed.starts_with("bet ") || trimmed.starts_with("(bet ") {
            // Count comma-separated alternatives in braces
            if let Some(brace_start) = trimmed.find('{') {
                if let Some(brace_end) = trimmed.rfind('}') {
                    let inner = &trimmed[brace_start + 1..brace_end];
                    let alt_count = inner.split(',').count();
                    if alt_count != 3 && !inner.trim().is_empty() {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position::new(ln, 0),
                                end: Position::new(ln, line.len() as u32),
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            code: Some(NumberOrString::String("ternary-arity".into())),
                            source: Some("bet-lsp".into()),
                            message: format!(
                                "bet expression has {} alternatives (expected 3 — ternary)",
                                alt_count
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        // Check for unclosed string on a single line
        let quote_count = trimmed.chars().filter(|&c| c == '"').count();
        if quote_count % 2 != 0 {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position::new(ln, 0),
                    end: Position::new(ln, line.len() as u32),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("unclosed-string".into())),
                source: Some("bet-lsp".into()),
                message: "Unclosed string literal".into(),
                ..Default::default()
            });
        }
    }

    // Report unbalanced delimiters at end of document
    if paren_depth > 0 {
        let last_line = source.lines().count().saturating_sub(1) as u32;
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(last_line, 0),
                end: Position::new(last_line, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("unmatched-paren".into())),
            source: Some("bet-lsp".into()),
            message: format!("{} unclosed parenthesis(es)", paren_depth),
            ..Default::default()
        });
    }
    if brace_depth > 0 {
        let last_line = source.lines().count().saturating_sub(1) as u32;
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(last_line, 0),
                end: Position::new(last_line, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("unmatched-brace".into())),
            source: Some("bet-lsp".into()),
            message: format!("{} unclosed brace(s)", brace_depth),
            ..Default::default()
        });
    }
    if bracket_depth > 0 {
        let last_line = source.lines().count().saturating_sub(1) as u32;
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position::new(last_line, 0),
                end: Position::new(last_line, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("unmatched-bracket".into())),
            source: Some("bet-lsp".into()),
            message: format!("{} unclosed bracket(s)", bracket_depth),
            ..Default::default()
        });
    }

    diagnostics
}
