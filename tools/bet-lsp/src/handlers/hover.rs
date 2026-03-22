// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hover handler

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::backend::Backend;

/// Handle hover request
pub async fn hover(backend: &Backend, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    // Get document
    let doc = match backend.document_map.get(&uri) {
        Some(d) => d,
        None => return Ok(None),
    };

    // Get word at cursor
    let word = match doc.word_at_position(position.line, position.character) {
        Some(w) => w,
        None => return Ok(None),
    };

    // Check if it's a keyword
    if let Some(keyword_doc) = get_keyword_documentation(&word) {
        return Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: keyword_doc,
            }),
            range: None,
        }));
    }

    Ok(None)
}

/// Get documentation for Betlang keywords
fn get_keyword_documentation(keyword: &str) -> Option<String> {
    let doc = match keyword {
        "bet" => "## bet\n\n3-way equal probability choice.\n\n```racket\n(bet 'a 'b 'c)  ; Returns a, b, or c with equal probability (1/3 each)\n```",
        "bet/weighted" => "## bet/weighted\n\nWeighted probability choice.\n\n```racket\n(bet/weighted '((0.7 'success) (0.3 'failure)))\n```",
        "bet/conditional" => "## bet/conditional\n\nConditional bet based on predicate.\n\n```racket\n(bet/conditional positive? 'a 'b 'c)\n```",
        "bet/lazy" => "## bet/lazy\n\nLazy bet evaluation.\n\n```racket\n(bet/lazy (thunk 'a) (thunk 'b) (thunk 'c))\n```",
        "bet-chain" => "## bet-chain\n\nChain bets together.\n\n```racket\n(bet-chain (bet 'a 'b) (lambda (x) (bet x 'c)))\n```",
        "bet-map" => "## bet-map\n\nMap function over bet outcomes.\n\n```racket\n(bet-map (lambda (x) (* x 2)) (bet 1 2 3))\n```",
        "define" => "## define\n\nDefine a variable or function.\n\n```racket\n(define x 42)\n(define (square x) (* x x))\n```",
        "lambda" => "## lambda\n\nAnonymous function.\n\n```racket\n(lambda (x) (* x x))\n```",
        "maybe" => "## maybe\n\nTernary logic value (true/false/maybe).\n\n```racket\n(ternary-and 'true 'maybe)  ; => 'maybe\n```",
        _ => return None,
    };

    Some(doc.to_string())
}
