// SPDX-License-Identifier: MIT OR Apache-2.0
//// LSP request handlers for Betlang

import gleam/dynamic.{type Dynamic}
import gleam/json
import gleam/option.{type Option, None, Some}
import gleam/result
import gleam/string
import gleam/list
import bet_lsp/jsonrpc.{type Request}
import bet_lsp/state.{type ServerState, type Diagnostic, type DiagnosticSeverity}
import bet_lsp/protocol
import bet_lsp/analyzer

// ============================================================================
// Lifecycle Handlers
// ============================================================================

/// Handle initialize request
pub fn handle_initialize(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  let root_uri = case request.params {
    Some(params) -> extract_root_uri(params)
    None -> None
  }

  let capabilities = json.object([
    #("textDocumentSync", json.object([
      #("openClose", json.bool(True)),
      #("change", json.int(1)),  // Full sync
      #("save", json.object([#("includeText", json.bool(True))])),
    ])),
    #("hoverProvider", json.bool(True)),
    #("completionProvider", json.object([
      #("triggerCharacters", json.array(["."], json.string)),
      #("resolveProvider", json.bool(False)),
    ])),
    #("definitionProvider", json.bool(True)),
    #("referencesProvider", json.bool(True)),
    #("documentSymbolProvider", json.bool(True)),
    #("documentFormattingProvider", json.bool(True)),
    #("diagnosticProvider", json.object([
      #("interFileDependencies", json.bool(False)),
      #("workspaceDiagnostics", json.bool(False)),
    ])),
  ])

  let server_info = json.object([
    #("name", json.string("bet-lsp")),
    #("version", json.string("0.1.0")),
  ])

  let result = json.object([
    #("capabilities", capabilities),
    #("serverInfo", server_info),
  ])

  let response = jsonrpc.success(request.id, result)
  let new_state = state.set_initialized(state, root_uri)

  #(Some(response), new_state)
}

fn extract_root_uri(params: Dynamic) -> Option(String) {
  case dynamic.field("rootUri", dynamic.string)(params) {
    Ok(uri) -> Some(uri)
    Error(_) -> None
  }
}

/// Handle initialized notification
pub fn handle_initialized(
  _request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  // No response needed for notifications
  #(None, state)
}

/// Handle shutdown request
pub fn handle_shutdown(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  let response = jsonrpc.success(request.id, json.null())
  let new_state = state.request_shutdown(state)
  #(Some(response), new_state)
}

/// Handle exit notification
pub fn handle_exit(
  _request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  #(None, state)
}

// ============================================================================
// Document Handlers
// ============================================================================

/// Handle textDocument/didOpen
pub fn handle_did_open(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case extract_text_document(params) {
        Ok(#(uri, version, content, language_id)) -> {
          let new_state = state.open_document(state, uri, version, content, language_id)
          // Run diagnostics
          let diagnostics = analyzer.analyze(content)
          let new_state = state.set_diagnostics(new_state, uri, diagnostics)
          // Publish diagnostics
          publish_diagnostics(uri, diagnostics)
          #(None, new_state)
        }
        Error(_) -> #(None, state)
      }
    }
    None -> #(None, state)
  }
}

fn extract_text_document(params: Dynamic) -> Result(#(String, Int, String, String), Nil) {
  use td <- result.try(dynamic.field("textDocument", dynamic.dynamic)(params))
  use uri <- result.try(dynamic.field("uri", dynamic.string)(td))
  use version <- result.try(dynamic.field("version", dynamic.int)(td))
  use text <- result.try(dynamic.field("text", dynamic.string)(td))
  use language_id <- result.try(
    result.or(
      dynamic.field("languageId", dynamic.string)(td),
      Ok("betlang"),
    )
  )
  Ok(#(uri, version, text, language_id))
}

/// Handle textDocument/didChange
pub fn handle_did_change(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case extract_change_params(params) {
        Ok(#(uri, version, content)) -> {
          let new_state = state.update_document(state, uri, version, content)
          // Re-run diagnostics
          let diagnostics = analyzer.analyze(content)
          let new_state = state.set_diagnostics(new_state, uri, diagnostics)
          publish_diagnostics(uri, diagnostics)
          #(None, new_state)
        }
        Error(_) -> #(None, state)
      }
    }
    None -> #(None, state)
  }
}

fn extract_change_params(params: Dynamic) -> Result(#(String, Int, String), Nil) {
  use td <- result.try(dynamic.field("textDocument", dynamic.dynamic)(params))
  use uri <- result.try(dynamic.field("uri", dynamic.string)(td))
  use version <- result.try(dynamic.field("version", dynamic.int)(td))
  use changes <- result.try(dynamic.field("contentChanges", dynamic.list(dynamic.dynamic))(params))
  case changes {
    [first, ..] -> {
      case dynamic.field("text", dynamic.string)(first) {
        Ok(text) -> Ok(#(uri, version, text))
        Error(_) -> Error(Nil)
      }
    }
    [] -> Error(Nil)
  }
}

/// Handle textDocument/didClose
pub fn handle_did_close(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case dynamic.field("textDocument", dynamic.dynamic)(params) {
        Ok(td) -> {
          case dynamic.field("uri", dynamic.string)(td) {
            Ok(uri) -> {
              let new_state = state.close_document(state, uri)
              #(None, new_state)
            }
            Error(_) -> #(None, state)
          }
        }
        Error(_) -> #(None, state)
      }
    }
    None -> #(None, state)
  }
}

/// Handle textDocument/didSave
pub fn handle_did_save(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  // Could trigger additional analysis on save
  #(None, state)
}

// ============================================================================
// Language Feature Handlers
// ============================================================================

/// Handle textDocument/hover
pub fn handle_hover(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case extract_position_params(params) {
        Ok(#(uri, line, character)) -> {
          case state.get_document(state, uri) {
            Some(doc) -> {
              let hover_info = analyzer.get_hover(doc.content, line, character)
              case hover_info {
                Some(info) -> {
                  let result = json.object([
                    #("contents", json.object([
                      #("kind", json.string("markdown")),
                      #("value", json.string(info)),
                    ])),
                  ])
                  #(Some(jsonrpc.success(request.id, result)), state)
                }
                None -> #(Some(jsonrpc.success(request.id, json.null())), state)
              }
            }
            None -> #(Some(jsonrpc.success(request.id, json.null())), state)
          }
        }
        Error(_) -> #(Some(jsonrpc.invalid_params(request.id, "Invalid position")), state)
      }
    }
    None -> #(Some(jsonrpc.invalid_params(request.id, "Missing params")), state)
  }
}

fn extract_position_params(params: Dynamic) -> Result(#(String, Int, Int), Nil) {
  use td <- result.try(dynamic.field("textDocument", dynamic.dynamic)(params))
  use uri <- result.try(dynamic.field("uri", dynamic.string)(td))
  use pos <- result.try(dynamic.field("position", dynamic.dynamic)(params))
  use line <- result.try(dynamic.field("line", dynamic.int)(pos))
  use character <- result.try(dynamic.field("character", dynamic.int)(pos))
  Ok(#(uri, line, character))
}

/// Handle textDocument/completion
pub fn handle_completion(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case extract_position_params(params) {
        Ok(#(uri, line, character)) -> {
          case state.get_document(state, uri) {
            Some(doc) -> {
              let completions = analyzer.get_completions(doc.content, line, character)
              let items = list.map(completions, fn(c) {
                json.object([
                  #("label", json.string(c.label)),
                  #("kind", json.int(c.kind)),
                  #("detail", json.string(c.detail)),
                  #("insertText", json.string(c.insert_text)),
                ])
              })
              let result = json.object([
                #("isIncomplete", json.bool(False)),
                #("items", json.array(items, fn(x) { x })),
              ])
              #(Some(jsonrpc.success(request.id, result)), state)
            }
            None -> #(Some(jsonrpc.success(request.id, json.null())), state)
          }
        }
        Error(_) -> #(Some(jsonrpc.invalid_params(request.id, "Invalid position")), state)
      }
    }
    None -> #(Some(jsonrpc.invalid_params(request.id, "Missing params")), state)
  }
}

/// Handle textDocument/definition
pub fn handle_definition(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  // Return null for now - full implementation would track definitions
  #(Some(jsonrpc.success(request.id, json.null())), state)
}

/// Handle textDocument/references
pub fn handle_references(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  // Return empty array for now
  #(Some(jsonrpc.success(request.id, json.array([], fn(x) { x }))), state)
}

/// Handle textDocument/documentSymbol
pub fn handle_document_symbol(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case dynamic.field("textDocument", dynamic.dynamic)(params) {
        Ok(td) -> {
          case dynamic.field("uri", dynamic.string)(td) {
            Ok(uri) -> {
              case state.get_document(state, uri) {
                Some(doc) -> {
                  let symbols = analyzer.get_symbols(doc.content)
                  let items = list.map(symbols, fn(s) {
                    json.object([
                      #("name", json.string(s.name)),
                      #("kind", json.int(s.kind)),
                      #("range", encode_range(s.range)),
                      #("selectionRange", encode_range(s.range)),
                    ])
                  })
                  #(Some(jsonrpc.success(request.id, json.array(items, fn(x) { x }))), state)
                }
                None -> #(Some(jsonrpc.success(request.id, json.array([], fn(x) { x }))), state)
              }
            }
            Error(_) -> #(Some(jsonrpc.invalid_params(request.id, "Missing uri")), state)
          }
        }
        Error(_) -> #(Some(jsonrpc.invalid_params(request.id, "Missing textDocument")), state)
      }
    }
    None -> #(Some(jsonrpc.invalid_params(request.id, "Missing params")), state)
  }
}

fn encode_range(range: state.Range) -> json.Json {
  json.object([
    #("start", json.object([
      #("line", json.int(range.start_line)),
      #("character", json.int(range.start_char)),
    ])),
    #("end", json.object([
      #("line", json.int(range.end_line)),
      #("character", json.int(range.end_char)),
    ])),
  ])
}

/// Handle textDocument/formatting
pub fn handle_formatting(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  // Return empty array (no formatting changes) for now
  #(Some(jsonrpc.success(request.id, json.array([], fn(x) { x }))), state)
}

/// Handle textDocument/diagnostics
pub fn handle_diagnostics(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case dynamic.field("textDocument", dynamic.dynamic)(params) {
        Ok(td) -> {
          case dynamic.field("uri", dynamic.string)(td) {
            Ok(uri) -> {
              let diags = state.get_diagnostics(state, uri)
              let items = list.map(diags, encode_diagnostic)
              let result = json.object([
                #("kind", json.string("full")),
                #("items", json.array(items, fn(x) { x })),
              ])
              #(Some(jsonrpc.success(request.id, result)), state)
            }
            Error(_) -> #(Some(jsonrpc.invalid_params(request.id, "Missing uri")), state)
          }
        }
        Error(_) -> #(Some(jsonrpc.invalid_params(request.id, "Missing textDocument")), state)
      }
    }
    None -> #(Some(jsonrpc.invalid_params(request.id, "Missing params")), state)
  }
}

fn encode_diagnostic(diag: Diagnostic) -> json.Json {
  let severity = case diag.severity {
    state.Error -> 1
    state.Warning -> 2
    state.Information -> 3
    state.Hint -> 4
  }

  json.object([
    #("range", encode_range(diag.range)),
    #("severity", json.int(severity)),
    #("source", json.string(diag.source)),
    #("message", json.string(diag.message)),
  ])
}

// ============================================================================
// REPL Handlers
// ============================================================================

/// Handle betlang/eval
pub fn handle_eval(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case request.params {
    Some(params) -> {
      case dynamic.field("code", dynamic.string)(params) {
        Ok(code) -> {
          let result_text = analyzer.eval(code)
          let new_state = state.add_repl_history(state, code)
          let result = json.object([
            #("result", json.string(result_text)),
          ])
          #(Some(jsonrpc.success(request.id, result)), new_state)
        }
        Error(_) -> #(Some(jsonrpc.invalid_params(request.id, "Missing code")), state)
      }
    }
    None -> #(Some(jsonrpc.invalid_params(request.id, "Missing params")), state)
  }
}

/// Handle betlang/repl/start
pub fn handle_repl_start(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  let session_id = "session-1"  // Could generate UUID
  let new_state = state.start_repl(state, session_id)
  let result = json.object([
    #("sessionId", json.string(session_id)),
  ])
  #(Some(jsonrpc.success(request.id, result)), new_state)
}

/// Handle betlang/repl/stop
pub fn handle_repl_stop(
  request: Request,
  state: ServerState,
) -> #(Option(String), ServerState) {
  let new_state = state.stop_repl(state)
  #(Some(jsonrpc.success(request.id, json.null())), new_state)
}

// ============================================================================
// Helpers
// ============================================================================

/// Publish diagnostics notification
fn publish_diagnostics(uri: String, diagnostics: List(Diagnostic)) -> Nil {
  // This would send a notification to the client
  // For now, diagnostics are sent on request
  Nil
}
