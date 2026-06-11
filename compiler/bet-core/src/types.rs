// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Core type definitions for Betlang

/// Betlang types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Unit type
    Unit,
    /// Boolean
    Bool,
    /// Three-valued logic
    Ternary,
    /// Integer
    Int,
    /// Floating point
    Float,
    /// String
    String,
    /// Bytes
    Bytes,
    /// Function type
    Fun(Box<Type>, Box<Type>),
    /// Distribution type
    Dist(Box<Type>),
    /// List type
    List(Box<Type>),
    /// Map type
    Map(Box<Type>, Box<Type>),
    /// Set type
    Set(Box<Type>),
    /// Tuple type
    Tuple(Vec<Type>),
    /// Option type
    Option(Box<Type>),
    /// Result type
    Result(Box<Type>, Box<Type>),
    /// Echo type: `Echo T` — a proof-relevant *structured-loss* residue over
    /// values of type `T` (after `hyperpolymath/echo-types`). Distinct from
    /// `T`: no implicit forgetting `Echo T -> T`. Domain-agnostic in core; its
    /// canonical betlang introduction site is probabilistic support retention
    /// (a draw/branch is marginalised into `T`, while the echo keeps that
    /// residue statically). The residue is ghost/proof-relevant — erased at
    /// runtime for now.
    Echo(Box<Type>),
    /// Echo residue: `EchoR T` — the strict, non-recoverable weakening of
    /// `Echo T` (no recovery operation is promised). Reserved former; rich
    /// operations are deferred until the residue semantics are settled.
    EchoR(Box<Type>),
    /// Type variable (for inference)
    Var(u32),
    /// Named type
    Named(String),
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }

    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Unit | Type::Bool | Type::Ternary | Type::Int | Type::Float | Type::String
        )
    }
}
