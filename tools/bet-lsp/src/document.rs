// SPDX-License-Identifier: MIT OR Apache-2.0
//! Document state management and caching

use once_cell::sync::OnceCell;
use tower_lsp::lsp_types::Url;

use crate::utils::LineIndex;

/// Document state with lazy-evaluated caches
pub struct DocumentState {
    pub uri: Url,
    pub version: i32,
    pub source: String,
    pub line_index: LineIndex,

    // Lazy-evaluated caches
    // Note: Actual parsing would use bet-parse crate
    // For now, we store parse results as strings (simplified)
    tokens: OnceCell<Result<Vec<String>, String>>,
    ast: OnceCell<Result<String, String>>,
}

impl DocumentState {
    /// Create a new document state
    pub fn new(uri: Url, source: String, version: i32) -> Self {
        let line_index = LineIndex::new(&source);

        Self {
            uri,
            version,
            source,
            line_index,
            tokens: OnceCell::new(),
            ast: OnceCell::new(),
        }
    }

    /// Get tokens (lazy evaluation)
    pub fn tokens(&self) -> &Result<Vec<String>, String> {
        self.tokens.get_or_init(|| {
            // Simplified tokenization - would use bet-parse in production
            let tokens: Vec<String> = self
                .source
                .split_whitespace()
                .map(String::from)
                .collect();
            Ok(tokens)
        })
    }

    /// Get AST (lazy evaluation)
    pub fn ast(&self) -> &Result<String, String> {
        self.ast.get_or_init(|| {
            // Simplified AST parsing - would use bet-parse in production
            match self.tokens() {
                Ok(_) => Ok(self.source.clone()),
                Err(e) => Err(e.clone()),
            }
        })
    }

    /// Get word at position (for completion/hover)
    pub fn word_at_position(&self, line: u32, character: u32) -> Option<String> {
        let offset = self.line_index.position_to_offset(line, character)?;

        // Find word boundaries
        let start = self.source[..offset]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
            .map(|i| i + 1)
            .unwrap_or(0);

        let end = self.source[offset..]
            .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
            .map(|i| offset + i)
            .unwrap_or(self.source.len());

        if start < end {
            Some(self.source[start..end].to_string())
        } else {
            None
        }
    }
}
