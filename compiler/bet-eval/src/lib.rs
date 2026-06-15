// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Interpreter for Betlang
//!
//! Tree-walking evaluator over the betlang AST. It uses the **shared runtime
//! value type** `bet_rt::value::Value` (re-exported here as `Value`) so the
//! interpreter and the embeddable runtime (`bet-rt`, used by the FFI bindings)
//! speak the same values — one runtime, not two.
//!
//! Probabilistic semantics: `bet { a, b, c }` evaluates to a **distribution**
//! (`Value::Dist`, typed `Dist T`), matching the Rust checker and the mechanised
//! Lean rule `tBet : … → Ty.dist T`. `sample : Dist T -> T` draws from it.
//!
//! Closures: betlang lambdas are AST closures, but `bet_rt::value::Closure`
//! holds a native `Box<dyn Fn(Vec<Value>) -> Value>`. We *closure-convert*: a
//! lambda becomes a native closure that re-invokes `eval` on the captured body
//! and environment. Evaluation errors inside a closure surface as `Value::Error`
//! (the native closure has no `Result` channel) and are re-raised by `apply`.

#![forbid(unsafe_code)]
use bet_syntax::ast::*;
use bet_core::{ValueEnv, CompileError, CompileResult};
use bet_rt::value::{Closure, Distribution, Ternary};
use im::Vector;
use std::sync::Arc;

/// The unified runtime value (shared with `bet-rt` and the FFI bindings).
pub use bet_rt::value::Value;

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
            TernaryValue::True => Ternary::True,
            TernaryValue::False => Ternary::False,
            TernaryValue::Unknown => Ternary::Unknown,
        })),

        // --- Variables ---
        Expr::Var(sym) => {
            let name = sym.as_str();
            env.lookup(&name).ok_or(CompileError::UndefinedVariable {
                name,
                span: None,
            })
        }

        // --- Bet expressions (produce distributions, type `Dist T`) ---
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
            if cond.is_truthy() {
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

        // --- Lambda (closure-converted into a native runtime closure) ---
        Expr::Lambda(lam) => {
            let params: Vec<String> =
                lam.params.iter().map(|p| pattern_name(&p.node)).collect();
            Ok(make_closure(params, lam.body.node.clone(), env.clone()))
        }

        // --- Application ---
        Expr::App(func, args) => {
            let f = eval(&func.node, env)?;
            let mut arg_vals = Vec::with_capacity(args.len());
            for a in args {
                arg_vals.push(eval(&a.node, env)?);
            }
            apply(f, arg_vals)
        }

        // --- Sample: eliminate a distribution (`Dist T -> T`) ---
        Expr::Sample(dist_expr) => {
            let v = eval(&dist_expr.node, env)?;
            v.sample().map_err(|message| CompileError::Runtime {
                message,
                span: None,
            })
        }

        // --- Tuples and Lists ---
        Expr::Tuple(elems) => {
            let vals: Vec<Value> = elems
                .iter()
                .map(|e| eval(&e.node, env))
                .collect::<Result<_, _>>()?;
            Ok(Value::Tuple(Arc::new(vals)))
        }
        Expr::List(elems) => {
            let vals: Vector<Value> = elems
                .iter()
                .map(|e| eval(&e.node, env))
                .collect::<Result<_, _>>()?;
            Ok(Value::List(vals))
        }

        // --- Parallel (evaluate body n times) ---
        Expr::Parallel(n_expr, body) => {
            let n = match eval(&n_expr.node, env)? {
                Value::Int(n) => n as usize,
                _ => {
                    return Err(CompileError::Runtime {
                        message: "parallel count must be an integer".to_string(),
                        span: None,
                    })
                }
            };
            let mut results = Vector::new();
            for _ in 0..n {
                results.push_back(eval(&body.node, env)?);
            }
            Ok(Value::List(results))
        }

        // --- Type annotation (ignored at runtime) ---
        Expr::Annotate(inner, _) => eval(&inner.node, env),

        // Unimplemented forms (Do, Match, Observe, Infer, Record, Field, Index,
        // echo operations, …). Surfaced explicitly rather than silently
        // evaluating to `Unit` (which produced wrong results with no signal).
        other => Err(CompileError::Runtime {
            message: format!(
                "interpreter (bet-eval): expression form not yet implemented: {}",
                expr_form_name(other)
            ),
            span: None,
        }),
    }
}

/// Short, stable name of an expression's syntactic form, for diagnostics.
fn expr_form_name(expr: &Expr) -> &'static str {
    match expr {
        Expr::Do(_) => "do-block",
        Expr::Match(_) => "match",
        Expr::Observe(_, _) => "observe",
        Expr::Infer(_) => "infer",
        Expr::Record(_) => "record",
        Expr::Field(_, _) => "field-access",
        Expr::Index(_, _) => "index",
        _ => "this expression",
    }
}

// ---------------------------------------------------------------------------
// Closures (AST -> native closure conversion)
// ---------------------------------------------------------------------------

/// Build a runtime closure from a betlang lambda by capturing its body AST and
/// defining environment, and re-invoking `eval` on application.
fn make_closure(params: Vec<String>, body: Expr, captured: ValueEnv<Value>) -> Value {
    let call_params = params.clone();
    Value::Closure(Arc::new(Closure {
        params,
        name: None,
        body: Box::new(move |args: Vec<Value>| {
            let mut local = captured.clone();
            for (p, a) in call_params.iter().zip(args.into_iter()) {
                local.bind(p.clone(), a);
            }
            match eval(&body, &mut local) {
                Ok(v) => v,
                // No Result channel in the native closure: encode the failure
                // as an Error value; `apply` re-raises it as a CompileError.
                Err(err) => Value::Error(Arc::new(err.to_string())),
            }
        }),
    }))
}

fn apply(func: Value, args: Vec<Value>) -> CompileResult<Value> {
    match func {
        Value::Closure(closure) => match (closure.body)(args) {
            Value::Error(e) => Err(CompileError::Runtime {
                message: e.as_ref().clone(),
                span: None,
            }),
            v => Ok(v),
        },
        Value::Native(native) => (native.func)(args).map_err(|message| CompileError::Runtime {
            message,
            span: None,
        }),
        other => Err(CompileError::Runtime {
            message: format!("Cannot apply non-function: {}", other.type_name()),
            span: None,
        }),
    }
}

// ---------------------------------------------------------------------------
// Bet evaluation — each form builds a `Value::Dist`
// ---------------------------------------------------------------------------

fn eval_bet(bet: &BetExpr, env: &mut ValueEnv<Value>) -> CompileResult<Value> {
    let v0 = eval(&bet.alternatives[0].node, env)?;
    let v1 = eval(&bet.alternatives[1].node, env)?;
    let v2 = eval(&bet.alternatives[2].node, env)?;
    // Uniform ternary distribution (sampled via bet-rt's RNG-backed sampler).
    Ok(Value::bet(v0, v1, v2))
}

fn eval_weighted_bet(wb: &WeightedBetExpr, env: &mut ValueEnv<Value>) -> CompileResult<Value> {
    let mut values: Vec<Value> = Vec::new();
    let mut weights: Vec<f64> = Vec::new();

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
    Ok(categorical_dist(values, weights, total))
}

fn eval_conditional_bet(
    cb: &ConditionalBetExpr,
    env: &mut ValueEnv<Value>,
) -> CompileResult<Value> {
    let cond = eval(&cb.condition.node, env)?;
    if cond.is_truthy() {
        // Point-mass distribution on the deterministic branch value.
        let v = eval(&cb.if_true.node, env)?;
        Ok(point_mass(v))
    } else {
        let v0 = eval(&cb.if_false[0].node, env)?;
        let v1 = eval(&cb.if_false[1].node, env)?;
        let v2 = eval(&cb.if_false[2].node, env)?;
        Ok(Value::bet(v0, v1, v2))
    }
}

/// A categorical distribution over `values` with the given (unnormalised)
/// `weights`. Supports any number of alternatives.
fn categorical_dist(values: Vec<Value>, weights: Vec<f64>, total: f64) -> Value {
    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || {
            let r = bet_rand::uniform(0.0, total);
            let mut cumul = 0.0;
            for (i, w) in weights.iter().enumerate() {
                cumul += w;
                if r < cumul {
                    return values[i].clone();
                }
            }
            values.last().cloned().unwrap_or(Value::Unit)
        }),
        name: "weighted_bet".to_string(),
    }))
}

/// A point-mass distribution: sampling always yields `v`.
fn point_mass(v: Value) -> Value {
    Value::Dist(Arc::new(Distribution {
        sampler: Box::new(move || v.clone()),
        name: "pure".to_string(),
    }))
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

        // List operations (`bet_rt::Value::List` is an `im::Vector`)
        (BinOp::Cons, val, Value::List(list)) => {
            let mut new_list = list.clone();
            new_list.push_front(val.clone());
            Ok(Value::List(new_list))
        }
        (BinOp::Append, Value::List(a), Value::List(b)) => {
            let mut new_list = a.clone();
            for v in b.iter() {
                new_list.push_back(v.clone());
            }
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
            Ternary::True => Ternary::False,
            Ternary::False => Ternary::True,
            Ternary::Unknown => Ternary::Unknown,
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
                for (p, v) in pats.iter().zip(vals.iter()) {
                    bind_pattern(&p.node, v.clone(), env)?;
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
        let val = eval(&Expr::Int(42), &mut env).expect("eval int");
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
        let val = eval(&expr, &mut env).expect("eval add");
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
        let val = eval(&expr, &mut env).expect("eval let");
        assert!(matches!(val, Value::Int(10)));
    }

    #[test]
    fn test_eval_bet_is_distribution() {
        let mut env = ValueEnv::new();
        let expr = Expr::Bet(BetExpr {
            alternatives: [
                Box::new(dummy(Expr::Int(1))),
                Box::new(dummy(Expr::Int(2))),
                Box::new(dummy(Expr::Int(3))),
            ],
        });
        // `bet` now evaluates to a distribution (Dist Int), not a drawn value.
        let val = eval(&expr, &mut env).expect("eval bet");
        assert!(matches!(val, Value::Dist(_)));
    }

    #[test]
    fn test_eval_sample_of_bet() {
        let mut env = ValueEnv::new();
        // sample(bet { 1, 2, 3 }) draws one of 1, 2, 3.
        let expr = Expr::Sample(Box::new(dummy(Expr::Bet(BetExpr {
            alternatives: [
                Box::new(dummy(Expr::Int(1))),
                Box::new(dummy(Expr::Int(2))),
                Box::new(dummy(Expr::Int(3))),
            ],
        }))));
        for _ in 0..100 {
            match eval(&expr, &mut env).expect("eval sample") {
                Value::Int(n) => assert!((1..=3).contains(&n)),
                other => panic!("Expected Int, got {other:?}"),
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
        let val = eval(&expr, &mut env).expect("eval if");
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
        let val = eval(&expr, &mut env).expect("eval lambda apply");
        assert!(matches!(val, Value::Int(42)));
    }
}
