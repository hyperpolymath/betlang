// SPDX-License-Identifier: MIT OR Apache-2.0
//! Error types for Betlang compiler

use thiserror::Error;
use bet_syntax::Span;

/// Compiler error
#[derive(Error, Debug, Clone)]
pub enum CompileError {
    #[error("Parse error: {message}")]
    Parse { message: String, span: Option<Span> },

    #[error("Type error: {message}")]
    Type { message: String, span: Option<Span> },

    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String, span: Option<Span> },

    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        expected: String,
        found: String,
        span: Option<Span>,
    },

    #[error("Cannot unify types: {left} and {right}")]
    UnificationError {
        left: String,
        right: String,
        span: Option<Span>,
    },

    #[error("Invalid ternary bet: must have exactly 3 alternatives")]
    InvalidBet { span: Option<Span> },

    #[error("Runtime error: {message}")]
    Runtime { message: String, span: Option<Span> },

    #[error("IO error: {message}")]
    Io { message: String },
}

impl CompileError {
    pub fn span(&self) -> Option<Span> {
        match self {
            CompileError::Parse { span, .. } => span.clone(),
            CompileError::Type { span, .. } => span.clone(),
            CompileError::UndefinedVariable { span, .. } => span.clone(),
            CompileError::TypeMismatch { span, .. } => span.clone(),
            CompileError::UnificationError { span, .. } => span.clone(),
            CompileError::InvalidBet { span } => span.clone(),
            CompileError::Runtime { span, .. } => span.clone(),
            CompileError::Io { .. } => None,
        }
    }
}

/// Result type for compiler operations
pub type CompileResult<T> = Result<T, CompileError>;
