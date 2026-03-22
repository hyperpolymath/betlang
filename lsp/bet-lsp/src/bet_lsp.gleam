// SPDX-License-Identifier: MIT OR Apache-2.0
//// Betlang Language Server Protocol Implementation
////
//// This module provides the main entry point and LSP server logic.
//// Communicates over stdio using JSON-RPC 2.0.

import gleam/erlang/process
import gleam/io
import gleam/option.{type Option, None, Some}
import gleam/result
import gleam/string
import bet_lsp/jsonrpc
import bet_lsp/protocol
import bet_lsp/handlers
import bet_lsp/state.{type ServerState}

/// Main entry point - starts the LSP server
pub fn main() {
  io.println_error("Betlang LSP starting...")

  // Initialize server state
  let initial_state = state.new()

  // Start the main loop
  run_server(initial_state)
}

/// Main server loop - reads from stdin, processes, writes to stdout
fn run_server(state: ServerState) -> Nil {
  case read_message() {
    Ok(message) -> {
      let #(response, new_state) = handle_message(message, state)

      case response {
        Some(resp) -> write_message(resp)
        None -> Nil
      }

      // Check if we should exit
      case state.should_exit(new_state) {
        True -> {
          io.println_error("Betlang LSP shutting down...")
          Nil
        }
        False -> run_server(new_state)
      }
    }
    Error(err) -> {
      io.println_error("Error reading message: " <> err)
      run_server(state)
    }
  }
}

/// Read a JSON-RPC message from stdin
fn read_message() -> Result(String, String) {
  // LSP uses Content-Length header followed by \r\n\r\n then JSON body
  case read_headers() {
    Ok(content_length) -> read_body(content_length)
    Error(e) -> Error(e)
  }
}

/// Read LSP headers and extract Content-Length
fn read_headers() -> Result(Int, String) {
  do_read_headers(0)
}

fn do_read_headers(content_length: Int) -> Result(Int, String) {
  case erlang_read_line() {
    Ok(line) -> {
      let trimmed = string.trim(line)
      case trimmed {
        "" -> Ok(content_length)  // Empty line = end of headers
        header -> {
          case string.split(header, ": ") {
            ["Content-Length", len_str] -> {
              case int_parse(string.trim(len_str)) {
                Ok(len) -> do_read_headers(len)
                Error(_) -> Error("Invalid Content-Length")
              }
            }
            _ -> do_read_headers(content_length)  // Skip other headers
          }
        }
      }
    }
    Error(e) -> Error(e)
  }
}

/// Read the JSON body
fn read_body(length: Int) -> Result(String, String) {
  erlang_read_bytes(length)
}

/// Write a JSON-RPC response to stdout
fn write_message(json: String) -> Nil {
  let content_length = string.byte_size(json)
  let header = "Content-Length: " <> int_to_string(content_length) <> "\r\n\r\n"

  erlang_write(header <> json)
}

/// Handle an incoming JSON-RPC message
fn handle_message(
  message: String,
  state: ServerState,
) -> #(Option(String), ServerState) {
  case jsonrpc.parse(message) {
    Ok(request) -> {
      case request.method {
        "initialize" -> handlers.handle_initialize(request, state)
        "initialized" -> handlers.handle_initialized(request, state)
        "shutdown" -> handlers.handle_shutdown(request, state)
        "exit" -> handlers.handle_exit(request, state)

        // Text document methods
        "textDocument/didOpen" -> handlers.handle_did_open(request, state)
        "textDocument/didChange" -> handlers.handle_did_change(request, state)
        "textDocument/didClose" -> handlers.handle_did_close(request, state)
        "textDocument/didSave" -> handlers.handle_did_save(request, state)

        // Language features
        "textDocument/hover" -> handlers.handle_hover(request, state)
        "textDocument/completion" -> handlers.handle_completion(request, state)
        "textDocument/definition" -> handlers.handle_definition(request, state)
        "textDocument/references" -> handlers.handle_references(request, state)
        "textDocument/documentSymbol" -> handlers.handle_document_symbol(request, state)
        "textDocument/formatting" -> handlers.handle_formatting(request, state)
        "textDocument/diagnostics" -> handlers.handle_diagnostics(request, state)

        // REPL integration
        "betlang/eval" -> handlers.handle_eval(request, state)
        "betlang/repl/start" -> handlers.handle_repl_start(request, state)
        "betlang/repl/stop" -> handlers.handle_repl_stop(request, state)

        // Unknown method
        _ -> {
          let response = jsonrpc.method_not_found(request.id)
          #(Some(response), state)
        }
      }
    }
    Error(err) -> {
      let response = jsonrpc.parse_error(err)
      #(Some(response), state)
    }
  }
}

// External Erlang FFI functions
@external(erlang, "bet_lsp_io", "read_line")
fn erlang_read_line() -> Result(String, String)

@external(erlang, "bet_lsp_io", "read_bytes")
fn erlang_read_bytes(length: Int) -> Result(String, String)

@external(erlang, "bet_lsp_io", "write")
fn erlang_write(data: String) -> Nil

@external(erlang, "erlang", "integer_to_binary")
fn int_to_string(n: Int) -> String

@external(erlang, "bet_lsp_io", "parse_int")
fn int_parse(s: String) -> Result(Int, Nil)
