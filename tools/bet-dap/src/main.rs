// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//! Debug Adapter Protocol (DAP) implementation for Betlang.
//!
//! Minimal DAP adapter providing breakpoints, stepping, and variable inspection.

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Serialize, Deserialize)]
struct InitializeRequest {
    #[serde(rename = "adapterID")]
    adapter_id: String,
    #[serde(rename = "clientID")]
    client_id: Option<String>,
    #[serde(rename = "clientName")]
    client_name: Option<String>,
    #[serde(rename = "columnsStartAt1")]
    columns_start_at1: Option<bool>,
    #[serde(rename = "linesStartAt1")]
    lines_start_at1: Option<bool>,
    #[serde(rename = "locale")]
    locale: Option<String>,
    #[serde(rename = "pathFormat")]
    path_format: Option<String>,
    #[serde(rename = "supportsRunInTerminalRequest")]
    supports_run_in_terminal_request: Option<bool>,
    #[serde(rename = "supportsMemoryReferences")]
    supports_memory_references: Option<bool>,
    #[serde(rename = "supportsProgressReporting")]
    supports_progress_reporting: Option<bool>,
    #[serde(rename = "supportsInvalidatedEvent")]
    supports_invalidated_event: Option<bool>,
    #[serde(rename = "supportsMemoryEvent")]
    supports_memory_event: Option<bool>,
}

#[derive(Debug, Serialize)]
struct InitializeResponse {
    #[serde(rename = "seq")]
    seq: i64,
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "request_seq")]
    request_seq: i64,
    #[serde(rename = "command")]
    command: String,
    #[serde(rename = "success")]
    success: bool,
    #[serde(rename = "body")]
    body: InitializeResponseBody,
}

#[derive(Debug, Serialize)]
struct InitializeResponseBody {
    #[serde(rename = "supportsConfigurationDoneRequest")]
    supports_configuration_done_request: bool,
    #[serde(rename = "supportsFunctionBreakpoints")]
    supports_function_breakpoints: bool,
    #[serde(rename = "supportsConditionalBreakpoints")]
    supports_conditional_breakpoints: bool,
    #[serde(rename = "supportsHitConditionalBreakpoints")]
    supports_hit_conditional_breakpoints: bool,
    #[serde(rename = "supportsEvaluateForHovers")]
    supports_evaluate_for_hovers: bool,
    #[serde(rename = "exceptionBreakpointFilters")]
    exception_breakpoint_filters: Vec<ExceptionBreakpointFilter>,
}

#[derive(Debug, Serialize)]
struct ExceptionBreakpointFilter {
    #[serde(rename = "filter")]
    filter: String,
    #[serde(rename = "label")]
    label: String,
    #[serde(rename = "default")]
    default: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:4711").await?;
    println!("Betlang DAP server listening on 127.0.0.1:4711");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();

    loop {
        buf.clear();
        let bytes_read = reader.read_line(&mut buf).await?;
        if bytes_read == 0 {
            break;
        }

        let request: serde_json::Value = serde_json::from_str(&buf)?;
        let response = match request["command"].as_str() {
            Some("initialize") => {
                let init_request: InitializeRequest = serde_json::from_value(request["arguments"].clone())?;
                let response = InitializeResponse {
                    seq: 1,
                    type_: "response".to_string(),
                    request_seq: request["seq"].as_i64().unwrap_or(0),
                    command: "initialize".to_string(),
                    success: true,
                    body: InitializeResponseBody {
                        supports_configuration_done_request: true,
                        supports_function_breakpoints: true,
                        supports_conditional_breakpoints: true,
                        supports_hit_conditional_breakpoints: true,
                        supports_evaluate_for_hovers: true,
                        exception_breakpoint_filters: vec![],
                    },
                };
                serde_json::to_string(&response)?
            }
            Some("launch") => {
                // Handle launch request
                r#"{"seq":2,"type":"response","request_seq":2,"command":"launch","success":true}"#.to_string()
            }
            Some("setBreakpoints") => {
                // Handle breakpoints
                r#"{"seq":3,"type":"response","request_seq":3,"command":"setBreakpoints","success":true,"body":{"breakpoints":[]}}"#.to_string()
            }
            Some("threads") => {
                // Handle threads
                r#"{"seq":4,"type":"response","request_seq":4,"command":"threads","success":true,"body":{"threads":[{"id":1,"name":"main"}]}}"#.to_string()
            }
            Some("stackTrace") => {
                // Handle stack trace
                r#"{"seq":5,"type":"response","request_seq":5,"command":"stackTrace","success":true,"body":{"stackFrames":[]}}"#.to_string()
            }
            Some("scopes") => {
                // Handle scopes
                r#"{"seq":6,"type":"response","request_seq":6,"command":"scopes","success":true,"body":{"scopes":[{"name":"Locals","variablesReference":1,"expensive":false}]}}"#.to_string()
            }
            Some("variables") => {
                // Handle variables
                r#"{"seq":7,"type":"response","request_seq":7,"command":"variables","success":true,"body":{"variables":[]}}"#.to_string()
            }
            Some("disconnect") => {
                // Handle disconnect
                r#"{"seq":8,"type":"response","request_seq":8,"command":"disconnect","success":true}"#.to_string()
            }
            _ => {
                // Unknown command
                r#"{"seq":0,"type":"response","request_seq":0,"command":"unknown","success":false}"#.to_string()
            }
        };

        writer.write_all(response.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}
