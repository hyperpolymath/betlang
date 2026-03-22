// SPDX-License-Identifier: MIT OR Apache-2.0
//! Utilities for LSP

use tower_lsp::lsp_types::{Position, Range};

/// Line index for mapping between byte offsets and LSP positions
pub struct LineIndex {
    /// Byte offset of the start of each line
    line_starts: Vec<usize>,
}

impl LineIndex {
    /// Create a new line index from source text
    pub fn new(text: &str) -> Self {
        let mut line_starts = vec![0];

        for (i, ch) in text.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }

        Self { line_starts }
    }

    /// Convert LSP position to byte offset
    pub fn position_to_offset(&self, line: u32, character: u32) -> Option<usize> {
        let line_start = *self.line_starts.get(line as usize)?;
        Some(line_start + character as usize)
    }

    /// Convert byte offset to LSP position
    pub fn offset_to_position(&self, offset: usize) -> Option<Position> {
        let line = self
            .line_starts
            .binary_search(&offset)
            .unwrap_or_else(|i| i - 1);

        let line_start = self.line_starts[line];
        let character = offset - line_start;

        Some(Position::new(line as u32, character as u32))
    }
}

/// Convert byte range to LSP Range (simplified)
pub fn range_from_offsets(
    start: usize,
    end: usize,
    line_index: &LineIndex,
) -> Option<Range> {
    let start_pos = line_index.offset_to_position(start)?;
    let end_pos = line_index.offset_to_position(end)?;
    Some(Range {
        start: start_pos,
        end: end_pos,
    })
}
