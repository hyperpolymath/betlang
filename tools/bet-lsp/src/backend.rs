// SPDX-License-Identifier: MIT OR Apache-2.0
//! LSP Backend - LanguageServer trait implementation

use dashmap::DashMap;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::document::DocumentState;
use crate::handlers;

/// LSP Backend for Betlang
pub struct Backend {
    /// LSP client for sending notifications
    pub client: Client,
    /// Map of document URIs to document state
    pub document_map: Arc<DashMap<Url, DocumentState>>,
}

impl Backend {
    /// Create a new LSP backend
    pub fn new(client: Client) -> Self {
        Self {
            client,
            document_map: Arc::new(DashMap::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["(".to_string(), " ".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("betlang".to_string()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "bet-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Betlang LSP server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;

        // Store document
        let doc = DocumentState::new(uri.clone(), text, version);
        self.document_map.insert(uri.clone(), doc);

        // Publish diagnostics
        handlers::diagnostics::publish_diagnostics(self, &uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        // Get new text (full sync mode)
        if let Some(change) = params.content_changes.first() {
            let new_text = change.text.clone();

            // Replace document state (invalidates all caches)
            let doc = DocumentState::new(uri.clone(), new_text, version);
            self.document_map.insert(uri.clone(), doc);

            // Publish diagnostics
            handlers::diagnostics::publish_diagnostics(self, &uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        // Remove from cache
        self.document_map.remove(&uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        handlers::completion::completion(self, params).await
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        handlers::hover::hover(self, params).await
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        handlers::definition::goto_definition(self, params).await
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        handlers::formatting::formatting(self, params).await
    }
}
