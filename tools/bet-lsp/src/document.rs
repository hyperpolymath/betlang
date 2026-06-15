// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Document state management and caching

use once_cell::sync::OnceCell;
use tower_lsp::lsp_types::Url;

use bet_parse::ParseError;
use bet_syntax::ast::Module;

use crate::utils::LineIndex;

/// Document state with lazy-evaluated caches
pub struct DocumentState {
    pub uri: Url,
    pub version: i32,
    pub source: String,
    pub line_index: LineIndex,

    /// Lazy, cached parse result from the real `bet-parse` parser.
    parsed: OnceCell<Result<Module, ParseError>>,
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
            parsed: OnceCell::new(),
        }
    }

    /// Parse the document with the real betlang parser (lazy, cached).
    ///
    /// Returns the parsed `Module` on success, or the `ParseError` (which
    /// carries byte offsets via [`ParseError::offsets`]) on failure.
    pub fn parsed(&self) -> &Result<Module, ParseError> {
        self.parsed.get_or_init(|| bet_parse::parse(&self.source))
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
