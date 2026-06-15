// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Diagnostics handler — surfaces real parse and type-check diagnostics for
//! betlang documents, produced by the `bet-parse` parser and `bet-check`
//! type checker (not lexical heuristics).

use tower_lsp::lsp_types::*;

use crate::backend::Backend;
use crate::utils::range_from_offsets;

/// Publish diagnostics for a document.
pub async fn publish_diagnostics(backend: &Backend, uri: &Url) {
    let diagnostics = collect_diagnostics(backend, uri);

    backend
        .client
        .publish_diagnostics(uri.clone(), diagnostics, None)
        .await;
}

/// A small default range at the start of the document, used when an error
/// carries no usable source span.
fn fallback_range() -> Range {
    Range {
        start: Position::new(0, 0),
        end: Position::new(0, 1),
    }
}

/// Collect parse + type diagnostics for a document.
///
/// Pipeline mirrors `bet run`/`bet check`: parse with `bet-parse`; on success,
/// type-check with `bet-check`. Each error is mapped to its real source span.
fn collect_diagnostics(backend: &Backend, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let doc = match backend.document_map.get(uri) {
        Some(d) => d,
        None => return diagnostics,
    };

    match doc.parsed() {
        Err(parse_err) => {
            let range = parse_err
                .offsets()
                .and_then(|(start, end)| range_from_offsets(start, end, &doc.line_index))
                .unwrap_or_else(fallback_range);

            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("parse-error".into())),
                source: Some("bet-lsp".into()),
                message: parse_err.to_string(),
                ..Default::default()
            });
        }
        Ok(module) => {
            // Parse succeeded — run the real type checker.
            if let Err(type_err) = bet_check::check_module(module) {
                let range = type_err
                    .span()
                    .and_then(|sp| {
                        range_from_offsets(sp.start as usize, sp.end as usize, &doc.line_index)
                    })
                    .unwrap_or_else(fallback_range);

                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("type-error".into())),
                    source: Some("bet-lsp".into()),
                    message: type_err.to_string(),
                    ..Default::default()
                });
            }
        }
    }

    diagnostics
}
