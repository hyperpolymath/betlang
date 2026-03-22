// SPDX-License-Identifier: MIT OR Apache-2.0
//! Document symbol handler — lists all definitions in a betlang file.

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Handle document symbol request.
pub async fn document_symbols(
    backend: &Backend,
    params: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>> {
    let uri = &params.text_document.uri;

    let doc = match backend.document_map.get(uri) {
        Some(d) => d,
        None => return Ok(None),
    };

    let mut symbols: Vec<SymbolInformation> = Vec::new();

    for (line_idx, line) in doc.source.lines().enumerate() {
        let trimmed = line.trim();
        let ln = line_idx as u32;

        // let name = ... / let rec name ...
        if let Some(rest) = trimmed.strip_prefix("let ") {
            let maybe_rec = rest.strip_prefix("rec ").unwrap_or(rest);
            if let Some((name, has_params)) = extract_def_name(maybe_rec) {
                let col = line.find(&name).unwrap_or(0) as u32;
                let kind = if has_params {
                    SymbolKind::FUNCTION
                } else {
                    SymbolKind::VARIABLE
                };
                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name: name.clone(),
                    kind,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(ln, col),
                            end: Position::new(ln, col + name.len() as u32),
                        },
                    },
                    container_name: None,
                });
            }
        }

        // (define name ...) or (define (name ...) ...)
        if trimmed.starts_with("(define") {
            let inner = trimmed.trim_start_matches("(define").trim_start();
            let (name, is_fn) = if inner.starts_with('(') {
                match extract_def_name(inner.trim_start_matches('(')) {
                    Some((n, _)) => (Some(n), true),
                    None => (None, false),
                }
            } else {
                match extract_def_name(inner) {
                    Some((n, p)) => (Some(n), p),
                    None => (None, false),
                }
            };
            if let Some(n) = name {
                let col = line.find(&n).unwrap_or(0) as u32;
                let kind = if is_fn {
                    SymbolKind::FUNCTION
                } else {
                    SymbolKind::VARIABLE
                };
                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name: n.clone(),
                    kind,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(ln, col),
                            end: Position::new(ln, col + n.len() as u32),
                        },
                    },
                    container_name: None,
                });
            }
        }

        // type Name = ...
        if let Some(rest) = trimmed.strip_prefix("type ") {
            if let Some((name, _)) = extract_def_name(rest) {
                let col = line.find(&name).unwrap_or(0) as u32;
                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name: name.clone(),
                    kind: SymbolKind::TYPE_PARAMETER,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(ln, col),
                            end: Position::new(ln, col + name.len() as u32),
                        },
                    },
                    container_name: None,
                });
            }
        }

        // import Module
        if let Some(rest) = trimmed.strip_prefix("import ") {
            if let Some((name, _)) = extract_def_name(rest) {
                let col = line.find(&name).unwrap_or(0) as u32;
                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name: name.clone(),
                    kind: SymbolKind::MODULE,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(ln, col),
                            end: Position::new(ln, col + name.len() as u32),
                        },
                    },
                    container_name: None,
                });
            }
        }
    }

    Ok(Some(DocumentSymbolResponse::Flat(symbols)))
}

/// Extract the first identifier from a definition line.
/// Returns (name, has_parameters).
fn extract_def_name(s: &str) -> Option<(String, bool)> {
    let s = s.trim();
    let end = s
        .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    let name = s[..end].to_string();
    let rest = s[end..].trim_start();
    let has_params = rest.starts_with('(') || rest.starts_with(|c: char| c.is_alphabetic());
    Some((name, has_params))
}
