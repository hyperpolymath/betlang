// SPDX-License-Identifier: MIT OR Apache-2.0
//// Server state management for Betlang LSP

import gleam/dict.{type Dict}
import gleam/option.{type Option, None, Some}

/// Document state
pub type Document {
  Document(
    uri: String,
    version: Int,
    content: String,
    language_id: String,
  )
}

/// Diagnostic severity
pub type DiagnosticSeverity {
  Error
  Warning
  Information
  Hint
}

/// A diagnostic message
pub type Diagnostic {
  Diagnostic(
    range: Range,
    severity: DiagnosticSeverity,
    message: String,
    source: String,
  )
}

/// Text range (line/character positions)
pub type Range {
  Range(
    start_line: Int,
    start_char: Int,
    end_line: Int,
    end_char: Int,
  )
}

/// Position in document
pub type Position {
  Position(line: Int, character: Int)
}

/// REPL session state
pub type ReplSession {
  ReplSession(
    id: String,
    history: List(String),
    bindings: Dict(String, String),
  )
}

/// Server state
pub type ServerState {
  ServerState(
    initialized: Bool,
    shutdown_requested: Bool,
    documents: Dict(String, Document),
    diagnostics: Dict(String, List(Diagnostic)),
    repl_session: Option(ReplSession),
    root_uri: Option(String),
    capabilities: ServerCapabilities,
  )
}

/// Server capabilities configuration
pub type ServerCapabilities {
  ServerCapabilities(
    hover: Bool,
    completion: Bool,
    definition: Bool,
    references: Bool,
    document_symbol: Bool,
    formatting: Bool,
    diagnostics: Bool,
  )
}

/// Create new server state
pub fn new() -> ServerState {
  ServerState(
    initialized: False,
    shutdown_requested: False,
    documents: dict.new(),
    diagnostics: dict.new(),
    repl_session: None,
    root_uri: None,
    capabilities: default_capabilities(),
  )
}

/// Default capabilities
fn default_capabilities() -> ServerCapabilities {
  ServerCapabilities(
    hover: True,
    completion: True,
    definition: True,
    references: True,
    document_symbol: True,
    formatting: True,
    diagnostics: True,
  )
}

/// Check if server should exit
pub fn should_exit(state: ServerState) -> Bool {
  state.shutdown_requested
}

/// Mark as initialized
pub fn set_initialized(state: ServerState, root_uri: Option(String)) -> ServerState {
  ServerState(..state, initialized: True, root_uri: root_uri)
}

/// Request shutdown
pub fn request_shutdown(state: ServerState) -> ServerState {
  ServerState(..state, shutdown_requested: True)
}

/// Open a document
pub fn open_document(
  state: ServerState,
  uri: String,
  version: Int,
  content: String,
  language_id: String,
) -> ServerState {
  let doc = Document(
    uri: uri,
    version: version,
    content: content,
    language_id: language_id,
  )
  ServerState(..state, documents: dict.insert(state.documents, uri, doc))
}

/// Update a document
pub fn update_document(
  state: ServerState,
  uri: String,
  version: Int,
  content: String,
) -> ServerState {
  case dict.get(state.documents, uri) {
    Ok(doc) -> {
      let updated = Document(..doc, version: version, content: content)
      ServerState(..state, documents: dict.insert(state.documents, uri, updated))
    }
    Error(_) -> state
  }
}

/// Close a document
pub fn close_document(state: ServerState, uri: String) -> ServerState {
  ServerState(
    ..state,
    documents: dict.delete(state.documents, uri),
    diagnostics: dict.delete(state.diagnostics, uri),
  )
}

/// Get a document
pub fn get_document(state: ServerState, uri: String) -> Option(Document) {
  case dict.get(state.documents, uri) {
    Ok(doc) -> Some(doc)
    Error(_) -> None
  }
}

/// Set diagnostics for a document
pub fn set_diagnostics(
  state: ServerState,
  uri: String,
  diagnostics: List(Diagnostic),
) -> ServerState {
  ServerState(..state, diagnostics: dict.insert(state.diagnostics, uri, diagnostics))
}

/// Get diagnostics for a document
pub fn get_diagnostics(state: ServerState, uri: String) -> List(Diagnostic) {
  case dict.get(state.diagnostics, uri) {
    Ok(diags) -> diags
    Error(_) -> []
  }
}

/// Start REPL session
pub fn start_repl(state: ServerState, id: String) -> ServerState {
  let session = ReplSession(id: id, history: [], bindings: dict.new())
  ServerState(..state, repl_session: Some(session))
}

/// Stop REPL session
pub fn stop_repl(state: ServerState) -> ServerState {
  ServerState(..state, repl_session: None)
}

/// Add to REPL history
pub fn add_repl_history(state: ServerState, input: String) -> ServerState {
  case state.repl_session {
    Some(session) -> {
      let updated = ReplSession(..session, history: [input, ..session.history])
      ServerState(..state, repl_session: Some(updated))
    }
    None -> state
  }
}
