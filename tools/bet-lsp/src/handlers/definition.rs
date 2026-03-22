// SPDX-License-Identifier: MIT OR Apache-2.0
//! Go-to-definition handler.
//!
//! Searches the document for `let`, `define`, `type`, and `import`
//! declarations that match the word under the cursor.

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Handle go-to-definition request.
pub async fn goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let doc = match backend.document_map.get(&uri) {
        Some(d) => d,
        None => return Ok(None),
    };

    let word = match doc.word_at_position(position.line, position.character) {
        Some(w) => w,
        None => return Ok(None),
    };

    // Search for definitions of the word in the document
    for (line_idx, line) in doc.source.lines().enumerate() {
        let trimmed = line.trim();

        // let name = ...
        if let Some(rest) = trimmed.strip_prefix("let ") {
            let maybe_rec = rest.strip_prefix("rec ").unwrap_or(rest);
            if let Some(name) = extract_name(maybe_rec) {
                if name == word {
                    let col = line.find(&name).unwrap_or(0) as u32;
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(line_idx as u32, col),
                            end: Position::new(line_idx as u32, col + name.len() as u32),
                        },
                    })));
                }
            }
        }

        // (define name ...) or (define (name ...) ...)
        if trimmed.starts_with("(define") {
            let inner = trimmed
                .trim_start_matches("(define")
                .trim_start();
            let name = if inner.starts_with('(') {
                extract_name(inner.trim_start_matches('('))
            } else {
                extract_name(inner)
            };
            if let Some(n) = name {
                if n == word {
                    let col = line.find(&n).unwrap_or(0) as u32;
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(line_idx as u32, col),
                            end: Position::new(line_idx as u32, col + n.len() as u32),
                        },
                    })));
                }
            }
        }

        // type Name = ...
        if let Some(rest) = trimmed.strip_prefix("type ") {
            if let Some(name) = extract_name(rest) {
                if name == word {
                    let col = line.find(&name).unwrap_or(0) as u32;
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position::new(line_idx as u32, col),
                            end: Position::new(line_idx as u32, col + name.len() as u32),
                        },
                    })));
                }
            }
        }
    }

    Ok(None)
}

/// Extract the first identifier from a string (letters, digits, underscores,
/// hyphens).
fn extract_name(s: &str) -> Option<String> {
    let s = s.trim();
    let end = s
        .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .unwrap_or(s.len());
    if end == 0 {
        None
    } else {
        Some(s[..end].to_string())
    }
}
