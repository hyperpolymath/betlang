// SPDX-License-Identifier: MIT OR Apache-2.0
//! Type checker for Betlang

use bet_syntax::ast::*;
use bet_core::{Type, TypeEnv, CompileError, CompileResult};

/// Type check an expression
pub fn check(expr: &Expr, env: &TypeEnv) -> CompileResult<Type> {
    // TODO: Implement full type checking
    match expr {
        // Literals
        Expr::Unit => Ok(Type::Unit),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Ternary(_) => Ok(Type::Ternary),
        Expr::Int(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),
        Expr::String(_) => Ok(Type::String),

        // Variables
        Expr::Var(name) => env
            .lookup(&name.to_string())
            .cloned()
            .ok_or_else(|| CompileError::UndefinedVariable {
                name: name.to_string(),
                span: None,
            }),

        // Bet expressions
        Expr::Bet(bet) => check_bet(bet, env),

        _ => Ok(Type::Unit), // Placeholder for other expressions
    }
}

fn check_bet(bet: &BetExpr, env: &TypeEnv) -> CompileResult<Type> {
    // All three alternatives must have the same type
    let types: Vec<_> = bet
        .alternatives
        .iter()
        .map(|alt| check(&alt.node, env))
        .collect::<Result<_, _>>()?;

    if types.len() != 3 {
        return Err(CompileError::InvalidBet { span: None });
    }

    // Check all types match
    let first = &types[0];
    for ty in &types[1..] {
        if ty != first {
            return Err(CompileError::TypeMismatch {
                expected: format!("{:?}", first),
                found: format!("{:?}", ty),
                span: None,
            });
        }
    }

    Ok(first.clone())
}
