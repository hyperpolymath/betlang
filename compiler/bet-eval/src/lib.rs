// SPDX-License-Identifier: PMPL-1.0-or-later
//! Interpreter for Betlang
//!
//! Evaluates Betlang AST nodes directly, supporting probabilistic operations.
//!
//! Author: Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>

use bet_syntax::ast::*;
use bet_core::{ValueEnv, CompileError, CompileResult};
use rand::prelude::*;
use std::sync::Arc;

/// Runtime value
#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Bool(bool),
    Ternary(TernaryVal),
    Int(i64),
    Float(f64),
    String(Arc<String>),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Closure(Arc<Closure>),
}

/// Ternary logic value for the evaluator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TernaryVal {
    True,
    False,
    Unknown,
}

/// A closure capturing its environment
#[derive(Debug)]
pub struct Closure {
    pub params: Vec<String>,
    pub body: Expr,
    pub env: ValueEnv<Value>,
}

/// Evaluate an expression in the given environment
pub fn eval(expr: &Expr, env: &mut ValueEnv<Value>) -> CompileResult<Value> {
    match expr {
        // --- Literals ---
        Expr::Unit => Ok(Value::Unit),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Int(i) => Ok(Value::Int(*i)),
        Expr::Float(f) => Ok(Value::Float(*f)),
        Expr::String(s) => Ok(Value::String(Arc::new(s.clone()))),
        Expr::Ternary(t) => Ok(Value::Ternary(match t {
            TernaryValue::True => TernaryVal::True,
            TernaryValue::False => TernaryVal::False,
            TernaryValue::Unknown => TernaryVal::Unknown,
        })),

        // --- Variables ---
        Expr::Var(sym) => {
            let name = sym.as_str();
            env.lookup(&name).ok_or_else(|| CompileError::UndefinedVariable {
                name,
                span: None,
            })
        }

        // --- Bet expressions ---
        Expr::Bet(bet) => eval_bet(bet, env),
        Expr::WeightedBet(wb) => eval_weighted_bet(wb, env),
        Expr::ConditionalBet(cb) => eval_conditional_bet(cb, env),

        // --- Let binding ---
        Expr::Let(le) => {
            let value = eval(&le.value.node, env)?;
            bind_pattern(&le.pattern.node, value, env)?;
            eval(&le.body.node, env)
        }

        // --- If ---
        Expr::If(ifexpr) => {
            let cond = eval(&ifexpr.condition.node, env)?;
            if is_truthy(&cond) {
                eval(&ifexpr.then_branch.node, env)
            } else {
                eval(&ifexpr.else_branch.node, env)
            }
        }

        // --- Binary operators ---
        Expr::BinOp(op, lhs, rhs) => {
            let l = eval(&lhs.node, env)?;
            let r = eval(&rhs.node, env)?;
            eval_binop(*op, l, r)
        }

        // --- Unary operators ---
        Expr::UnOp(op, operand) => {
            let v = eval(&operand.node, env)?;
            eval_unop(*op, v)
        }

        // --- Lambda ---
        Expr::Lambda(lam) => {
            let params: Vec<String> = lam.params.iter().map(|p| pattern_name(&p.node)).collect();
            Ok(Value::Closure(Arc::new(Closure {
                params,
                body: lam.body.node.clone(),
                env: env.clone(),
            })))
        }

        // --- Application ---
        Expr::App(func, args) => {
            let f = eval(&func.node, env)?;
            let mut arg_vals = Vec::new();
            for a in args {
                arg_vals.push(eval(&a.node, env)?);
            }
            apply(f, arg_vals)
        }

        // --- Sample ---
        Expr::Sample(dist_expr) => {
            let dist = eval(&dist_expr.node, env)?;
            // For now, just return the value (distributions not fully modelled)
            Ok(dist)
        }

        // --- Tuples and Lists ---
        Expr::Tuple(elems) => {
            let vals: Vec<Value> = elems
                .iter()
                .map(|e| eval(&e.node, env))
                .collect::<Result<_, _>>()?;
            Ok(Value::Tuple(vals))
        }
        Expr::List(elems) => {
            let vals: Vec<Value> = elems
                .iter()
                .map(|e| eval(&e.node, env))
                .collect::<Result<_, _>>()?;
            Ok(Value::List(vals))
        }

        // --- Parallel (evaluate body n times) ---
        Expr::Parallel(n_expr, body) => {
            let n = match eval(&n_expr.node, env)? {
                Value::Int(n) => n as usize,
                _ => return Err(CompileError::Runtime {
                    message: "parallel count must be an integer".to_string(),
                    span: None,
                }),
            };
            let mut results = Vec::with_capacity(n);
            for _ in 0..n {
                results.push(eval(&body.node, env)?);
            }
            Ok(Value::List(results))
        }

        // --- Type annotation (ignored at runtime) ---
        Expr::Annotate(inner, _) => eval(&inner.node, env),

        // Catch-all for unimplemented nodes
        _ => Ok(Value::Unit),
    }
}

// ---------------------------------------------------------------------------
// Bet evaluation
// ---------------------------------------------------------------------------

fn eval_bet(bet: &BetExpr, env: &mut ValueEnv<Value>) -> CompileResult<Value> {
    let values: Vec<Value> = bet
        .alternatives
        .iter()
        .map(|alt| eval(&alt.node, env))
        .collect::<Result<_, _>>()?;

    let idx = thread_rng().gen_range(0..3);
    Ok(values.into_iter().nth(idx).unwrap())
}

fn eval_weighted_bet(wb: &WeightedBetExpr, env: &mut ValueEnv<Value>) -> CompileResult<Value> {
    let mut values = Vec::new();
    let mut weights = Vec::new();

    for (val_expr, wt_expr) in &wb.alternatives {
        values.push(eval(&val_expr.node, env)?);
        let w = match eval(&wt_expr.node, env)? {
            Value::Float(f) => f,
            Value::Int(i) => i as f64,
            _ => 1.0,
        };
        weights.push(w);
    }

    let total: f64 = weights.iter().sum();
    let r: f64 = thread_rng().gen::<f64>() * total;
    let mut cumul = 0.0;
    for (i, w) in weights.iter().enumerate() {
        cumul += w;
        if r < cumul {
            return Ok(values.into_iter().nth(i).unwrap());
        }
    }
    Ok(values.into_iter().last().unwrap())
}

fn eval_conditional_bet(
    cb: &ConditionalBetExpr,
    env: &mut ValueEnv<Value>,
) -> CompileResult<Value> {
    let cond = eval(&cb.condition.node, env)?;
    if is_truthy(&cond) {
        eval(&cb.if_true.node, env)
    } else {
        let values: Vec<Value> = cb
            .if_false
            .iter()
            .map(|alt| eval(&alt.node, env))
            .collect::<Result<_, _>>()?;
        let idx = thread_rng().gen_range(0..3);
        Ok(values.into_iter().nth(idx).unwrap())
    }
}

// ---------------------------------------------------------------------------
// Operators
// ---------------------------------------------------------------------------

fn eval_binop(op: BinOp, lhs: Value, rhs: Value) -> CompileResult<Value> {
    match (op, &lhs, &rhs) {
        // Integer arithmetic
        (BinOp::Add, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
        (BinOp::Sub, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
        (BinOp::Mul, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
        (BinOp::Div, Value::Int(a), Value::Int(b)) => {
            if *b == 0 {
                Err(CompileError::Runtime {
                    message: "Division by zero".to_string(),
                    span: None,
                })
            } else {
                Ok(Value::Int(a / b))
            }
        }
        (BinOp::Mod, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),

        // Float arithmetic
        (BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        (BinOp::Div, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),

        // Mixed numeric
        (BinOp::Add, Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
        (BinOp::Add, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
        (BinOp::Mul, Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
        (BinOp::Mul, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),

        // Comparisons
        (BinOp::Eq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Ne, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
        (BinOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Le, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Gt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Ge, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),

        (BinOp::Eq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Lt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Le, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Gt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Ge, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),

        // Boolean logic
        (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
        (BinOp::Or, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),

        // String concatenation
        (BinOp::Concat, Value::String(a), Value::String(b)) => {
            Ok(Value::String(Arc::new(format!("{}{}", a, b))))
        }

        // List operations
        (BinOp::Cons, val, Value::List(list)) => {
            let mut new_list = vec![val.clone()];
            new_list.extend(list.iter().cloned());
            Ok(Value::List(new_list))
        }
        (BinOp::Append, Value::List(a), Value::List(b)) => {
            let mut new_list = a.clone();
            new_list.extend(b.iter().cloned());
            Ok(Value::List(new_list))
        }

        _ => Err(CompileError::Runtime {
            message: format!("Unsupported operation {:?} on {:?} and {:?}", op, lhs, rhs),
            span: None,
        }),
    }
}

fn eval_unop(op: UnOp, val: Value) -> CompileResult<Value> {
    match (op, &val) {
        (UnOp::Neg, Value::Int(i)) => Ok(Value::Int(-i)),
        (UnOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
        (UnOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (UnOp::Not, Value::Ternary(t)) => Ok(Value::Ternary(match t {
            TernaryVal::True => TernaryVal::False,
            TernaryVal::False => TernaryVal::True,
            TernaryVal::Unknown => TernaryVal::Unknown,
        })),
        _ => Err(CompileError::Runtime {
            message: format!("Unsupported unary operation {:?} on {:?}", op, val),
            span: None,
        }),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Bool(b) => *b,
        Value::Int(i) => *i != 0,
        Value::Float(f) => *f != 0.0,
        Value::Ternary(TernaryVal::True) => true,
        Value::Unit => false,
        _ => true,
    }
}

fn bind_pattern(pat: &Pattern, val: Value, env: &mut ValueEnv<Value>) -> CompileResult<()> {
    match pat {
        Pattern::Var(sym) => {
            env.bind(sym.as_str(), val);
            Ok(())
        }
        Pattern::Wildcard => Ok(()),
        Pattern::Tuple(pats) => {
            if let Value::Tuple(vals) = val {
                if pats.len() != vals.len() {
                    return Err(CompileError::Runtime {
                        message: "Tuple pattern length mismatch".to_string(),
                        span: None,
                    });
                }
                for (p, v) in pats.iter().zip(vals) {
                    bind_pattern(&p.node, v, env)?;
                }
                Ok(())
            } else {
                Err(CompileError::Runtime {
                    message: "Expected tuple value".to_string(),
                    span: None,
                })
            }
        }
        _ => Ok(()), // Other patterns: just ignore for now
    }
}

fn pattern_name(pat: &Pattern) -> String {
    match pat {
        Pattern::Var(sym) => sym.as_str(),
        Pattern::Wildcard => "_".to_string(),
        _ => "_".to_string(),
    }
}

fn apply(func: Value, args: Vec<Value>) -> CompileResult<Value> {
    match func {
        Value::Closure(closure) => {
            let mut new_env = closure.env.clone();
            for (param, arg) in closure.params.iter().zip(args) {
                new_env.bind(param.clone(), arg);
            }
            eval(&closure.body, &mut new_env)
        }
        _ => Err(CompileError::Runtime {
            message: format!("Cannot apply non-function: {:?}", func),
            span: None,
        }),
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Ternary(t) => write!(f, "{:?}", t),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(x) => write!(f, "{}", x),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Tuple(t) => {
                write!(f, "(")?;
                for (i, v) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            Value::Closure(_) => write!(f, "<closure>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bet_syntax::span::Spanned;
    use bet_syntax::Symbol;

    fn dummy<T>(node: T) -> Spanned<T> {
        Spanned::dummy(node)
    }

    #[test]
    fn test_eval_int() {
        let mut env = ValueEnv::new();
        let val = eval(&Expr::Int(42), &mut env).unwrap();
        assert!(matches!(val, Value::Int(42)));
    }

    #[test]
    fn test_eval_addition() {
        let mut env = ValueEnv::new();
        let expr = Expr::BinOp(
            BinOp::Add,
            Box::new(dummy(Expr::Int(3))),
            Box::new(dummy(Expr::Int(7))),
        );
        let val = eval(&expr, &mut env).unwrap();
        assert!(matches!(val, Value::Int(10)));
    }

    #[test]
    fn test_eval_let_binding() {
        let mut env = ValueEnv::new();
        let expr = Expr::Let(LetExpr {
            pattern: dummy(Pattern::Var(Symbol::intern("x"))),
            type_ann: None,
            value: Box::new(dummy(Expr::Int(5))),
            body: Box::new(dummy(Expr::BinOp(
                BinOp::Mul,
                Box::new(dummy(Expr::Var(Symbol::intern("x")))),
                Box::new(dummy(Expr::Int(2))),
            ))),
            is_rec: false,
        });
        let val = eval(&expr, &mut env).unwrap();
        assert!(matches!(val, Value::Int(10)));
    }

    #[test]
    fn test_eval_bet() {
        let mut env = ValueEnv::new();
        let expr = Expr::Bet(BetExpr {
            alternatives: [
                Box::new(dummy(Expr::Int(1))),
                Box::new(dummy(Expr::Int(2))),
                Box::new(dummy(Expr::Int(3))),
            ],
        });
        // Should return one of 1, 2, or 3
        for _ in 0..100 {
            let val = eval(&expr, &mut env).unwrap();
            match val {
                Value::Int(n) => assert!(n >= 1 && n <= 3),
                _ => panic!("Expected Int"),
            }
        }
    }

    #[test]
    fn test_eval_if() {
        let mut env = ValueEnv::new();
        let expr = Expr::If(IfExpr {
            condition: Box::new(dummy(Expr::Bool(true))),
            then_branch: Box::new(dummy(Expr::Int(1))),
            else_branch: Box::new(dummy(Expr::Int(0))),
        });
        let val = eval(&expr, &mut env).unwrap();
        assert!(matches!(val, Value::Int(1)));
    }

    #[test]
    fn test_eval_lambda_apply() {
        let mut env = ValueEnv::new();
        let expr = Expr::App(
            Box::new(dummy(Expr::Lambda(LambdaExpr {
                params: vec![dummy(Pattern::Var(Symbol::intern("x")))],
                body: Box::new(dummy(Expr::BinOp(
                    BinOp::Add,
                    Box::new(dummy(Expr::Var(Symbol::intern("x")))),
                    Box::new(dummy(Expr::Int(1))),
                ))),
            }))),
            vec![dummy(Expr::Int(41))],
        );
        let val = eval(&expr, &mut env).unwrap();
        assert!(matches!(val, Value::Int(42)));
    }
}
