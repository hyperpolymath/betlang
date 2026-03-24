// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//! Debug Adapter Protocol (DAP) server for Betlang.
//!
//! Provides breakpoint, step, and variable inspection support for Betlang
//! programs via the DAP specification (used by VS Code, Neovim, etc.).
//!
//! Status: scaffolding only — protocol messages are defined but the debug
//! session loop is not yet wired to the Betlang evaluator.

/// DAP protocol message types.
#[derive(Debug, Clone)]
pub enum DapMessage {
    /// Client requests the debugger to initialise.
    Initialize,
    /// Client requests the debugger to launch a program.
    Launch { program: String },
    /// Client requests the debugger to set breakpoints.
    SetBreakpoints { path: String, lines: Vec<u32> },
    /// Client requests the debugger to continue execution.
    Continue,
    /// Client requests the debugger to step over the next statement.
    StepOver,
    /// Client requests the debugger to step into the next call.
    StepIn,
    /// Client requests the debugger to disconnect.
    Disconnect,
}

/// DAP protocol response types.
#[derive(Debug, Clone)]
pub enum DapResponse {
    /// Capabilities of this debug adapter.
    Initialized {
        supports_stepping: bool,
        supports_breakpoints: bool,
    },
    /// Acknowledgement of a successful operation.
    Ack,
    /// An error response.
    Error { message: String },
}

/// A minimal DAP session placeholder.
///
/// Future work: wire this to `bet_eval::eval` with breakpoint hooks
/// and a JSON-RPC transport layer.
pub struct DapSession {
    /// Whether the session is currently running.
    pub running: bool,
}

impl DapSession {
    /// Create a new DAP session.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Handle an incoming DAP message and produce a response.
    pub fn handle(&mut self, msg: DapMessage) -> DapResponse {
        match msg {
            DapMessage::Initialize => DapResponse::Initialized {
                supports_stepping: false,
                supports_breakpoints: false,
            },
            DapMessage::Launch { .. } => {
                self.running = true;
                DapResponse::Ack
            }
            DapMessage::Disconnect => {
                self.running = false;
                DapResponse::Ack
            }
            _ => DapResponse::Error {
                message: "Not yet implemented".to_string(),
            },
        }
    }
}

impl Default for DapSession {
    fn default() -> Self {
        Self::new()
    }
}
