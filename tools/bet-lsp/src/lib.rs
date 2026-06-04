// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
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
