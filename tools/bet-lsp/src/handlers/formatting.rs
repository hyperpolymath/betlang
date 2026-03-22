// SPDX-License-Identifier: MIT OR Apache-2.0
//! Formatting handler

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Handle document formatting request
pub async fn formatting(
    backend: &Backend,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let uri = params.text_document.uri;

    // Get document
    let doc = match backend.document_map.get(&uri) {
        Some(d) => d,
        None => return Ok(None),
    };

    // Simplified formatting - would use raco fmt in production
    let formatted = format_racket(&doc.source);

    // Create a text edit that replaces the entire document
    let start = Position::new(0, 0);
    let end_line = doc.source.lines().count() as u32;
    let end_char = doc.source.lines().last().map(|l| l.len()).unwrap_or(0) as u32;
    let end = Position::new(end_line, end_char);

    let edit = TextEdit {
        range: Range { start, end },
        new_text: formatted,
    };

    Ok(Some(vec![edit]))
}

/// Simple Racket formatter (basic indentation)
pub fn format_racket(source: &str) -> String {
    let mut formatted = String::new();
    let mut indent_level: usize = 0;
    let mut in_string = false;

    for line in source.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            formatted.push('\n');
            continue;
        }

        // Track string literals
        for ch in trimmed.chars() {
            if ch == '"' && !in_string {
                in_string = true;
            } else if ch == '"' && in_string {
                in_string = false;
            }
        }

        // Adjust indent before line
        if !in_string && trimmed.starts_with(')') {
            indent_level = indent_level.saturating_sub(1);
        }

        // Add indented line
        let indent = "  ".repeat(indent_level);
        formatted.push_str(&indent);
        formatted.push_str(trimmed);
        formatted.push('\n');

        // Adjust indent after line
        if !in_string && trimmed.contains('(') {
            indent_level += trimmed.matches('(').count();
        }
        if !in_string && trimmed.contains(')') {
            indent_level = indent_level.saturating_sub(trimmed.matches(')').count());
        }
    }

    formatted
}
