// SPDX-License-Identifier: MIT OR Apache-2.0
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
