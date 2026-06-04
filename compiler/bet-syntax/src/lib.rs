// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Betlang Abstract Syntax Tree
//!
//! This module defines the core syntax structures for betlang,
//! a ternary probabilistic programming language.

#![forbid(unsafe_code)]
pub mod ast;
pub mod pretty;
pub mod span;
pub mod symbol;
pub mod visitor;

pub use ast::*;
pub use span::Span;
pub use symbol::Symbol;
