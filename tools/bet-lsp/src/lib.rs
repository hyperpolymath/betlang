// SPDX-License-Identifier: MIT OR Apache-2.0
//! Betlang Language Server Protocol (LSP) implementation
//!
//! This library provides language server features for Betlang:
//! - Code completion (keywords, builtins, stdlib modules)
//! - Hover documentation (bet forms, keywords)
//! - Go-to-definition (let, define, type declarations)
//! - Document symbols (functions, variables, types, imports)
//! - Diagnostics (delimiter balance, ternary arity, unclosed strings)
//! - Code formatting (basic Racket/S-expression indentation)

#![forbid(unsafe_code)]
pub mod backend;
pub mod document;
pub mod handlers;
pub mod utils;
