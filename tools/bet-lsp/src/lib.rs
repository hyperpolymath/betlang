// SPDX-License-Identifier: MIT OR Apache-2.0
//! Betlang Language Server Protocol (LSP) implementation
//!
//! This library provides language server features for Betlang:
//! - Code completion
//! - Hover documentation
//! - Go-to-definition
//! - Diagnostics
//! - Code formatting

pub mod backend;
pub mod document;
pub mod handlers;
pub mod utils;
