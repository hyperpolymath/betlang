// SPDX-License-Identifier: MIT OR Apache-2.0
//! Betlang Language Server Protocol (LSP) implementation
//!
//! This is optional tooling and NOT authoritative for betlang semantics.
//! The authoritative implementation is core/betlang.rkt (Racket).

use tower_lsp::{LspService, Server};

mod backend;
mod document;
mod handlers;
mod utils;

use backend::Backend;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
