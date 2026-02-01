// SPDX-License-Identifier: MIT OR Apache-2.0
//! Definition handler

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Handle go-to-definition request
pub async fn goto_definition(
    backend: &Backend,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    // Get document
    let doc = match backend.document_map.get(&uri) {
        Some(d) => d,
        None => return Ok(None),
    };

    // Get word at cursor
    let _word = match doc.word_at_position(position.line, position.character) {
        Some(w) => w,
        None => return Ok(None),
    };

    // Simplified: Would search AST for definitions
    // For now, return None (not implemented)
    Ok(None)
}
