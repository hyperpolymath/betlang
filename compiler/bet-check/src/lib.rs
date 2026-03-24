// SPDX-License-Identifier: PMPL-1.0-or-later
//! Type checker for Betlang
//!
//! Implements bidirectional type checking for Betlang's probabilistic type
//! system. Key rules:
//!
//! - `bet { A B C }` — all three branches must unify to the same type
//! - `sample(dist)` — takes `Dist<T>`, returns `T`
//! - `observe(dist, value)` — takes `Dist<T>` and `T`
//! - `infer(method, model)` — model returns `Dist<T>`, result is `Dist<T>`
//! - Ternary values have type `Ternary`
//! - Let-polymorphism via generalization at let-boundaries

#![forbid(unsafe_code)]
use bet_core::error::{CompileError, CompileResult};
use bet_core::types::Type;
use bet_syntax::ast::*;
use bet_syntax::span::{Span, Spanned};
use std::collections::HashMap;

// ============================================
// Type Environment
// ============================================

/// Type environment with scoping and type variable generation.
#[derive(Debug, Clone)]
pub struct CheckEnv {
    /// Variable-to-type bindings in current scope.
    bindings: HashMap<String, Type>,
    /// Parent scope (for lexical scoping).
    parent: Option<Box<CheckEnv>>,
    /// Counter for generating fresh type variables.
    next_var: u32,
    /// Substitution map for type variables (union-find style).
    substitutions: HashMap<u32, Type>,
}

impl Default for CheckEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckEnv {
    /// Create a new empty type environment.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
            next_var: 0,
            substitutions: HashMap::new(),
        }
    }

    /// Create a child scope that inherits from this environment.
    pub fn extend(&self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(self.clone())),
            next_var: self.next_var,
            substitutions: self.substitutions.clone(),
        }
    }

    /// Bind a name to a type in the current scope.
    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }

    /// Look up a name, searching parent scopes.
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup(name)))
    }

    /// Generate a fresh type variable.
    pub fn fresh_var(&mut self) -> Type {
        let id = self.next_var;
        self.next_var += 1;
        Type::Var(id)
    }

    /// Resolve a type variable through the substitution chain.
    pub fn resolve(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(id) => {
                if let Some(resolved) = self.substitutions.get(id) {
                    self.resolve(resolved)
                } else {
                    ty.clone()
                }
            }
            Type::Fun(a, b) => {
                Type::Fun(Box::new(self.resolve(a)), Box::new(self.resolve(b)))
            }
            Type::Dist(inner) => Type::Dist(Box::new(self.resolve(inner))),
            Type::List(inner) => Type::List(Box::new(self.resolve(inner))),
            Type::Set(inner) => Type::Set(Box::new(self.resolve(inner))),
            Type::Option(inner) => Type::Option(Box::new(self.resolve(inner))),
            Type::Map(k, v) => {
                Type::Map(Box::new(self.resolve(k)), Box::new(self.resolve(v)))
            }
            Type::Result(ok, err) => {
                Type::Result(Box::new(self.resolve(ok)), Box::new(self.resolve(err)))
            }
            Type::Tuple(elems) => {
                Type::Tuple(elems.iter().map(|e| self.resolve(e)).collect())
            }
            _ => ty.clone(),
        }
    }

    /// Unify two types, updating substitutions. Returns an error on mismatch.
    pub fn unify(&mut self, a: &Type, b: &Type, span: Option<Span>) -> CompileResult<()> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        match (&a, &b) {
            // Identical types always unify.
            _ if a == b => Ok(()),

            // A type variable unifies with anything (occurs check omitted for simplicity).
            (Type::Var(id), _) => {
                self.substitutions.insert(*id, b);
                Ok(())
            }
            (_, Type::Var(id)) => {
                self.substitutions.insert(*id, a);
                Ok(())
            }

            // Structural unification for compound types.
            (Type::Fun(a1, a2), Type::Fun(b1, b2)) => {
                self.unify(a1, b1, span)?;
                self.unify(a2, b2, span)
            }
            (Type::Dist(a_inner), Type::Dist(b_inner)) => {
                self.unify(a_inner, b_inner, span)
            }
            (Type::List(a_inner), Type::List(b_inner)) => {
                self.unify(a_inner, b_inner, span)
            }
            (Type::Set(a_inner), Type::Set(b_inner)) => {
                self.unify(a_inner, b_inner, span)
            }
            (Type::Option(a_inner), Type::Option(b_inner)) => {
                self.unify(a_inner, b_inner, span)
            }
            (Type::Map(ak, av), Type::Map(bk, bv)) => {
                self.unify(ak, bk, span)?;
                self.unify(av, bv, span)
            }
            (Type::Result(aok, aerr), Type::Result(bok, berr)) => {
                self.unify(aok, bok, span)?;
                self.unify(aerr, berr, span)
            }
            (Type::Tuple(a_elems), Type::Tuple(b_elems)) if a_elems.len() == b_elems.len() => {
                for (ae, be) in a_elems.iter().zip(b_elems.iter()) {
                    self.unify(ae, be, span)?;
                }
                Ok(())
            }

            _ => Err(CompileError::TypeMismatch {
                expected: format!("{:?}", a),
                found: format!("{:?}", b),
                span,
            }),
        }
    }
}

// ============================================
// Type Checker — Main Entry Points
// ============================================

/// Type-check a module (top-level compilation unit).
pub fn check_module(module: &Module) -> CompileResult<CheckEnv> {
    let mut env = CheckEnv::new();
    // Seed the environment with built-in types for common names.
    seed_builtins(&mut env);

    for item in &module.items {
        check_item(&item.node, &mut env)?;
    }
    Ok(env)
}

/// Seed built-in bindings (e.g. print, to_string).
fn seed_builtins(env: &mut CheckEnv) {
    // print : String -> Unit
    env.bind(
        "print".to_string(),
        Type::Fun(Box::new(Type::String), Box::new(Type::Unit)),
    );
    // to_string : 'a -> String  (polymorphic, approximated with a var)
    let a = env.fresh_var();
    env.bind(
        "to_string".to_string(),
        Type::Fun(Box::new(a), Box::new(Type::String)),
    );
}

/// Type-check a top-level item.
fn check_item(item: &Item, env: &mut CheckEnv) -> CompileResult<()> {
    match item {
        Item::Let(def) => {
            let ty = check_let_def(def, env)?;
            let name = def.name.node.to_string();
            env.bind(name, ty);
            Ok(())
        }
        Item::TypeDef(_) => {
            // Type definitions are handled at the kind level — skip for now.
            Ok(())
        }
        Item::Import(_) => {
            // Imports are resolved before type checking.
            Ok(())
        }
        Item::Expr(expr) => {
            // Top-level expression: infer its type but discard it.
            let spanned = Spanned::dummy(expr.clone());
            check_expr(&spanned, env)?;
            Ok(())
        }
    }
}

/// Type-check a let definition, returning the inferred type.
fn check_let_def(def: &LetDef, env: &mut CheckEnv) -> CompileResult<Type> {
    let mut inner_env = if def.is_rec {
        // For recursive definitions, bind the name to a fresh variable first.
        let mut e = env.extend();
        let placeholder = e.fresh_var();
        e.bind(def.name.node.to_string(), placeholder.clone());
        e
    } else {
        env.extend()
    };

    // Bind parameters.
    let mut param_types = Vec::new();
    for param in &def.params {
        let pt = inner_env.fresh_var();
        bind_pattern(&param.node, &pt, &mut inner_env)?;
        param_types.push(pt);
    }

    // Infer the body type.
    let body_ty = check_expr(&def.body, &mut inner_env)?;

    // If there is a type annotation, unify against it.
    if let Some(ann) = &def.type_ann {
        let ann_ty = ast_type_to_core(&ann.node);
        inner_env.unify(&body_ty, &ann_ty, Some(ann.span))?;
    }

    // Build the full function type (curried).
    let mut result = inner_env.resolve(&body_ty);
    for pt in param_types.into_iter().rev() {
        let resolved_pt = inner_env.resolve(&pt);
        result = Type::Fun(Box::new(resolved_pt), Box::new(result));
    }

    // For recursive defs, unify the placeholder with the result.
    if def.is_rec {
        if let Some(placeholder) = inner_env.lookup(&def.name.node.to_string()) {
            let placeholder = placeholder.clone();
            inner_env.unify(&placeholder, &result, Some(def.name.span))?;
        }
    }

    // Propagate substitutions back to the parent env.
    env.substitutions = inner_env.substitutions;
    env.next_var = inner_env.next_var;

    Ok(result)
}

// ============================================
// Expression Type Checking
// ============================================

/// Public entry point for inferring the type of a single expression.
///
/// Useful for REPL `:type` queries where a full module is not available.
pub fn check_expr_public(expr: &Spanned<Expr>, env: &mut CheckEnv) -> CompileResult<Type> {
    check_expr(expr, env)
}

/// Infer the type of an expression.
fn check_expr(expr: &Spanned<Expr>, env: &mut CheckEnv) -> CompileResult<Type> {
    let span = expr.span;
    match &expr.node {
        // --- Literals ---
        Expr::Int(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),
        Expr::String(_) => Ok(Type::String),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Ternary(_) => Ok(Type::Ternary),
        Expr::Unit => Ok(Type::Unit),

        // --- Variable ---
        Expr::Var(name) => env
            .lookup(&name.to_string())
            .cloned()
            .ok_or_else(|| CompileError::UndefinedVariable {
                name: name.to_string(),
                span: Some(span),
            }),

        // --- Bet (ternary choice) ---
        Expr::Bet(bet) => check_bet(bet, env, span),
        Expr::WeightedBet(wbet) => check_weighted_bet(wbet, env, span),
        Expr::ConditionalBet(cbet) => check_conditional_bet(cbet, env, span),

        // --- Function application ---
        Expr::App(func, args) => {
            let func_ty = check_expr(func, env)?;
            let mut current = func_ty;
            for arg in args {
                let arg_ty = check_expr(arg, env)?;
                let ret = env.fresh_var();
                let expected_fun = Type::Fun(Box::new(arg_ty.clone()), Box::new(ret.clone()));
                env.unify(&current, &expected_fun, Some(arg.span))?;
                current = env.resolve(&ret);
            }
            Ok(current)
        }

        // --- Lambda ---
        Expr::Lambda(lam) => {
            let mut inner = env.extend();
            let mut param_types = Vec::new();
            for param in &lam.params {
                let pt = inner.fresh_var();
                bind_pattern(&param.node, &pt, &mut inner)?;
                param_types.push(pt);
            }
            let body_ty = check_expr(&lam.body, &mut inner)?;
            env.substitutions = inner.substitutions;
            env.next_var = inner.next_var;

            let mut result = body_ty;
            for pt in param_types.into_iter().rev() {
                let resolved_pt = env.resolve(&pt);
                result = Type::Fun(Box::new(resolved_pt), Box::new(result));
            }
            Ok(result)
        }

        // --- Let expression ---
        Expr::Let(let_expr) => {
            let mut inner = env.extend();
            let val_ty = check_expr(&let_expr.value, &mut inner)?;
            if let Some(ann) = &let_expr.type_ann {
                let ann_ty = ast_type_to_core(&ann.node);
                inner.unify(&val_ty, &ann_ty, Some(ann.span))?;
            }
            bind_pattern(&let_expr.pattern.node, &val_ty, &mut inner)?;
            let body_ty = check_expr(&let_expr.body, &mut inner)?;
            env.substitutions = inner.substitutions;
            env.next_var = inner.next_var;
            Ok(body_ty)
        }

        // --- If expression ---
        Expr::If(if_expr) => {
            let cond_ty = check_expr(&if_expr.condition, env)?;
            // Condition must be Bool or Ternary.
            let is_bool_or_ternary =
                matches!(env.resolve(&cond_ty), Type::Bool | Type::Ternary | Type::Var(_));
            if !is_bool_or_ternary {
                return Err(CompileError::TypeMismatch {
                    expected: "Bool or Ternary".to_string(),
                    found: format!("{:?}", env.resolve(&cond_ty)),
                    span: Some(if_expr.condition.span),
                });
            }
            let then_ty = check_expr(&if_expr.then_branch, env)?;
            let else_ty = check_expr(&if_expr.else_branch, env)?;
            env.unify(&then_ty, &else_ty, Some(span))?;
            Ok(env.resolve(&then_ty))
        }

        // --- Match expression ---
        Expr::Match(match_expr) => {
            let scrutinee_ty = check_expr(&match_expr.scrutinee, env)?;
            let result_ty = env.fresh_var();
            for arm in &match_expr.arms {
                let mut arm_env = env.extend();
                bind_pattern(&arm.pattern.node, &scrutinee_ty, &mut arm_env)?;
                if let Some(guard) = &arm.guard {
                    let guard_ty = check_expr(guard, &mut arm_env)?;
                    arm_env.unify(&guard_ty, &Type::Bool, Some(guard.span))?;
                }
                let body_ty = check_expr(&arm.body, &mut arm_env)?;
                arm_env.unify(&result_ty, &body_ty, Some(arm.body.span))?;
                env.substitutions = arm_env.substitutions;
                env.next_var = arm_env.next_var;
            }
            Ok(env.resolve(&result_ty))
        }

        // --- Probabilistic operations ---
        Expr::Sample(dist_expr) => {
            let dist_ty = check_expr(dist_expr, env)?;
            let inner = env.fresh_var();
            let expected = Type::Dist(Box::new(inner.clone()));
            env.unify(&dist_ty, &expected, Some(dist_expr.span))?;
            Ok(env.resolve(&inner))
        }

        Expr::Observe(dist_expr, val_expr) => {
            let dist_ty = check_expr(dist_expr, env)?;
            let val_ty = check_expr(val_expr, env)?;
            let inner = env.fresh_var();
            let expected_dist = Type::Dist(Box::new(inner.clone()));
            env.unify(&dist_ty, &expected_dist, Some(dist_expr.span))?;
            env.unify(&val_ty, &env.resolve(&inner), Some(val_expr.span))?;
            Ok(Type::Unit)
        }

        Expr::Infer(infer_expr) => {
            // The model should produce a Dist<T>. The result is Dist<T>.
            let model_ty = check_expr(&infer_expr.model, env)?;
            let inner = env.fresh_var();
            let expected = Type::Dist(Box::new(inner.clone()));
            env.unify(&model_ty, &expected, Some(infer_expr.model.span))?;
            Ok(Type::Dist(Box::new(env.resolve(&inner))))
        }

        Expr::Parallel(count_expr, body_expr) => {
            let count_ty = check_expr(count_expr, env)?;
            env.unify(&count_ty, &Type::Int, Some(count_expr.span))?;
            let body_ty = check_expr(body_expr, env)?;
            Ok(Type::List(Box::new(body_ty)))
        }

        // --- Binary operations ---
        Expr::BinOp(op, lhs, rhs) => check_binop(*op, lhs, rhs, env, span),

        // --- Unary operations ---
        Expr::UnOp(op, operand) => check_unop(*op, operand, env, span),

        // --- Data structures ---
        Expr::Tuple(elems) => {
            let types: Vec<Type> = elems
                .iter()
                .map(|e| check_expr(e, env))
                .collect::<Result<_, _>>()?;
            Ok(Type::Tuple(types))
        }

        Expr::List(elems) => {
            let elem_ty = env.fresh_var();
            for elem in elems {
                let t = check_expr(elem, env)?;
                env.unify(&t, &elem_ty, Some(elem.span))?;
            }
            Ok(Type::List(Box::new(env.resolve(&elem_ty))))
        }

        Expr::Record(fields) => {
            // Records are structural — we do not track field types as a named
            // record type in this initial implementation. Return Unit.
            for (_, val) in fields {
                check_expr(val, env)?;
            }
            Ok(Type::Unit) // Placeholder: proper record types require row polymorphism.
        }

        Expr::Field(base, _field) => {
            check_expr(base, env)?;
            // Field access requires row-polymorphic types. Return a fresh var.
            Ok(env.fresh_var())
        }

        Expr::Index(base, idx) => {
            let base_ty = check_expr(base, env)?;
            let idx_ty = check_expr(idx, env)?;
            env.unify(&idx_ty, &Type::Int, Some(idx.span))?;
            let elem = env.fresh_var();
            let expected = Type::List(Box::new(elem.clone()));
            env.unify(&base_ty, &expected, Some(base.span))?;
            Ok(env.resolve(&elem))
        }

        // --- Type annotation ---
        Expr::Annotate(inner, ann) => {
            let inferred = check_expr(inner, env)?;
            let ann_ty = ast_type_to_core(&ann.node);
            env.unify(&inferred, &ann_ty, Some(ann.span))?;
            Ok(env.resolve(&inferred))
        }

        // --- Do-notation ---
        Expr::Do(do_expr) => {
            let mut inner = env.extend();
            let mut last_ty = Type::Unit;
            for stmt in &do_expr.statements {
                match &stmt.node {
                    DoStatement::Bind(pat, expr) => {
                        let expr_ty = check_expr(expr, &mut inner)?;
                        // Monadic bind: expr_ty should be Dist<T>, bind T.
                        let elem = inner.fresh_var();
                        let expected = Type::Dist(Box::new(elem.clone()));
                        inner.unify(&expr_ty, &expected, Some(expr.span))?;
                        bind_pattern(&pat.node, &inner.resolve(&elem), &mut inner)?;
                        last_ty = inner.resolve(&elem);
                    }
                    DoStatement::Expr(expr) => {
                        last_ty = check_expr(expr, &mut inner)?;
                    }
                    DoStatement::Let(pat, expr) => {
                        let ty = check_expr(expr, &mut inner)?;
                        bind_pattern(&pat.node, &ty, &mut inner)?;
                        last_ty = ty;
                    }
                }
            }
            env.substitutions = inner.substitutions;
            env.next_var = inner.next_var;
            Ok(Type::Dist(Box::new(last_ty)))
        }

        // --- Holes and errors ---
        Expr::Hole(_) => Ok(env.fresh_var()),
        Expr::Error => Ok(env.fresh_var()),
    }
}

// ============================================
// Bet Expression Checking
// ============================================

/// Check a uniform bet: all three branches must have the same type.
fn check_bet(bet: &BetExpr, env: &mut CheckEnv, span: Span) -> CompileResult<Type> {
    let t0 = check_expr(&bet.alternatives[0], env)?;
    let t1 = check_expr(&bet.alternatives[1], env)?;
    let t2 = check_expr(&bet.alternatives[2], env)?;

    env.unify(&t0, &t1, Some(span))?;
    env.unify(&t0, &t2, Some(span))?;
    Ok(env.resolve(&t0))
}

/// Check a weighted bet: branches unify, weights must be numeric.
fn check_weighted_bet(
    wbet: &WeightedBetExpr,
    env: &mut CheckEnv,
    span: Span,
) -> CompileResult<Type> {
    let mut branch_types = Vec::new();
    for (expr, weight) in &wbet.alternatives {
        let ty = check_expr(expr, env)?;
        branch_types.push(ty);
        let w_ty = check_expr(weight, env)?;
        if !matches!(env.resolve(&w_ty), Type::Int | Type::Float | Type::Var(_)) {
            return Err(CompileError::TypeMismatch {
                expected: "numeric weight (Int or Float)".to_string(),
                found: format!("{:?}", env.resolve(&w_ty)),
                span: Some(weight.span),
            });
        }
    }
    // All branch types must unify.
    for i in 1..branch_types.len() {
        env.unify(&branch_types[0], &branch_types[i], Some(span))?;
    }
    Ok(env.resolve(&branch_types[0]))
}

/// Check a conditional bet: condition is Bool/Ternary, all branches unify.
fn check_conditional_bet(
    cbet: &ConditionalBetExpr,
    env: &mut CheckEnv,
    span: Span,
) -> CompileResult<Type> {
    let cond_ty = check_expr(&cbet.condition, env)?;
    match env.resolve(&cond_ty) {
        Type::Bool | Type::Ternary | Type::Var(_) => {}
        other => {
            return Err(CompileError::TypeMismatch {
                expected: "Bool or Ternary".to_string(),
                found: format!("{:?}", other),
                span: Some(cbet.condition.span),
            });
        }
    }

    let true_ty = check_expr(&cbet.if_true, env)?;
    let f0 = check_expr(&cbet.if_false[0], env)?;
    let f1 = check_expr(&cbet.if_false[1], env)?;
    let f2 = check_expr(&cbet.if_false[2], env)?;

    env.unify(&true_ty, &f0, Some(span))?;
    env.unify(&true_ty, &f1, Some(span))?;
    env.unify(&true_ty, &f2, Some(span))?;
    Ok(env.resolve(&true_ty))
}

// ============================================
// Operator Checking
// ============================================

/// Check a binary operation.
fn check_binop(
    op: BinOp,
    lhs: &Spanned<Expr>,
    rhs: &Spanned<Expr>,
    env: &mut CheckEnv,
    span: Span,
) -> CompileResult<Type> {
    let lt = check_expr(lhs, env)?;
    let rt = check_expr(rhs, env)?;

    match op {
        // Arithmetic: both operands numeric, result is the wider type.
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::Pow => {
            let lt_r = env.resolve(&lt);
            let rt_r = env.resolve(&rt);
            match (&lt_r, &rt_r) {
                (Type::Int, Type::Int) => Ok(Type::Int),
                (Type::Float, Type::Float) | (Type::Int, Type::Float) | (Type::Float, Type::Int) => {
                    Ok(Type::Float)
                }
                _ => {
                    env.unify(&lt, &rt, Some(span))?;
                    Ok(env.resolve(&lt))
                }
            }
        }

        // Comparison: operands must be the same type, result is Bool.
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
            env.unify(&lt, &rt, Some(span))?;
            Ok(Type::Bool)
        }

        // Logical: operands are Bool or Ternary, result is Ternary.
        BinOp::And | BinOp::Or | BinOp::Xor => {
            // Accept Bool or Ternary on either side.
            check_bool_or_ternary(&lt, env, lhs.span)?;
            check_bool_or_ternary(&rt, env, rhs.span)?;
            Ok(Type::Ternary)
        }

        // String concatenation.
        BinOp::Concat => {
            env.unify(&lt, &Type::String, Some(lhs.span))?;
            env.unify(&rt, &Type::String, Some(rhs.span))?;
            Ok(Type::String)
        }

        // List cons: a :: [a] -> [a]
        BinOp::Cons => {
            let elem = env.fresh_var();
            env.unify(&lt, &elem, Some(lhs.span))?;
            let list_ty = Type::List(Box::new(elem.clone()));
            env.unify(&rt, &list_ty, Some(rhs.span))?;
            Ok(env.resolve(&Type::List(Box::new(elem))))
        }

        // List append: [a] ++ [a] -> [a]
        BinOp::Append => {
            let elem = env.fresh_var();
            let list_ty = Type::List(Box::new(elem.clone()));
            env.unify(&lt, &list_ty, Some(lhs.span))?;
            env.unify(&rt, &list_ty, Some(rhs.span))?;
            Ok(env.resolve(&list_ty))
        }

        // Kleisli composition: (a -> Dist b) >> (b -> Dist c) -> (a -> Dist c)
        BinOp::Compose => {
            let a = env.fresh_var();
            let b = env.fresh_var();
            let c = env.fresh_var();
            let f_ty = Type::Fun(
                Box::new(a.clone()),
                Box::new(Type::Dist(Box::new(b.clone()))),
            );
            let g_ty = Type::Fun(
                Box::new(b.clone()),
                Box::new(Type::Dist(Box::new(c.clone()))),
            );
            env.unify(&lt, &f_ty, Some(lhs.span))?;
            env.unify(&rt, &g_ty, Some(rhs.span))?;
            Ok(Type::Fun(
                Box::new(env.resolve(&a)),
                Box::new(Type::Dist(Box::new(env.resolve(&c)))),
            ))
        }
    }
}

/// Check a unary operation.
fn check_unop(
    op: UnOp,
    operand: &Spanned<Expr>,
    env: &mut CheckEnv,
    span: Span,
) -> CompileResult<Type> {
    let t = check_expr(operand, env)?;
    match op {
        UnOp::Neg => {
            let resolved = env.resolve(&t);
            if matches!(resolved, Type::Int | Type::Float | Type::Var(_)) {
                Ok(resolved)
            } else {
                Err(CompileError::TypeMismatch {
                    expected: "numeric (Int or Float)".to_string(),
                    found: format!("{:?}", resolved),
                    span: Some(span),
                })
            }
        }
        UnOp::Not => {
            check_bool_or_ternary(&t, env, operand.span)?;
            Ok(Type::Ternary)
        }
        UnOp::Sample => {
            let inner = env.fresh_var();
            let expected = Type::Dist(Box::new(inner.clone()));
            env.unify(&t, &expected, Some(operand.span))?;
            Ok(env.resolve(&inner))
        }
    }
}

/// Verify a type is Bool or Ternary (or an unresolved variable).
fn check_bool_or_ternary(ty: &Type, env: &CheckEnv, span: Span) -> CompileResult<()> {
    match env.resolve(ty) {
        Type::Bool | Type::Ternary | Type::Var(_) => Ok(()),
        other => Err(CompileError::TypeMismatch {
            expected: "Bool or Ternary".to_string(),
            found: format!("{:?}", other),
            span: Some(span),
        }),
    }
}

// ============================================
// Pattern Binding
// ============================================

/// Bind names introduced by a pattern to the inferred type.
fn bind_pattern(pattern: &Pattern, ty: &Type, env: &mut CheckEnv) -> CompileResult<()> {
    match pattern {
        Pattern::Wildcard => Ok(()),
        Pattern::Var(name) => {
            env.bind(name.to_string(), ty.clone());
            Ok(())
        }
        Pattern::Literal(lit) => {
            let lit_ty = literal_type(lit);
            env.unify(&lit_ty, ty, None)?;
            Ok(())
        }
        Pattern::Tuple(pats) => {
            let elem_types: Vec<Type> = pats.iter().map(|_| env.fresh_var()).collect();
            let tuple_ty = Type::Tuple(elem_types.clone());
            env.unify(ty, &tuple_ty, None)?;
            for (pat, et) in pats.iter().zip(elem_types.iter()) {
                bind_pattern(&pat.node, et, env)?;
            }
            Ok(())
        }
        Pattern::List(pats, rest) => {
            let elem = env.fresh_var();
            let list_ty = Type::List(Box::new(elem.clone()));
            env.unify(ty, &list_ty, None)?;
            for pat in pats {
                bind_pattern(&pat.node, &elem, env)?;
            }
            if let Some(rest_pat) = rest {
                bind_pattern(&rest_pat.node, &list_ty, env)?;
            }
            Ok(())
        }
        Pattern::Constructor(_name, args) => {
            // Constructor patterns require a type database; bind args as fresh vars.
            for arg in args {
                let v = env.fresh_var();
                bind_pattern(&arg.node, &v, env)?;
            }
            Ok(())
        }
        Pattern::Record(fields) => {
            for (_, pat) in fields {
                if let Some(p) = pat {
                    let v = env.fresh_var();
                    bind_pattern(&p.node, &v, env)?;
                }
            }
            Ok(())
        }
        Pattern::As(inner, name) => {
            env.bind(name.to_string(), ty.clone());
            bind_pattern(&inner.node, ty, env)?;
            Ok(())
        }
        Pattern::Or(a, b, c) => {
            // All branches must bind the same names to the same types.
            bind_pattern(&a.node, ty, env)?;
            bind_pattern(&b.node, ty, env)?;
            bind_pattern(&c.node, ty, env)?;
            Ok(())
        }
        Pattern::Annotate(inner, ann) => {
            let ann_ty = ast_type_to_core(&ann.node);
            env.unify(ty, &ann_ty, Some(ann.span))?;
            bind_pattern(&inner.node, ty, env)?;
            Ok(())
        }
    }
}

/// Get the type of a literal pattern.
fn literal_type(lit: &Literal) -> Type {
    match lit {
        Literal::Int(_) => Type::Int,
        Literal::Float(_) => Type::Float,
        Literal::String(_) => Type::String,
        Literal::Bool(_) => Type::Bool,
        Literal::Ternary(_) => Type::Ternary,
        Literal::Unit => Type::Unit,
    }
}

// ============================================
// AST Type → Core Type Conversion
// ============================================

/// Convert an AST-level `Type` to a core `bet_core::Type`.
fn ast_type_to_core(ty: &bet_syntax::ast::Type) -> Type {
    match ty {
        bet_syntax::ast::Type::Named(sym) => {
            let name = sym.to_string();
            match name.as_str() {
                "Int" => Type::Int,
                "Float" => Type::Float,
                "Bool" => Type::Bool,
                "String" => Type::String,
                "Unit" => Type::Unit,
                "Ternary" => Type::Ternary,
                "Bytes" => Type::Bytes,
                _ => Type::Named(name),
            }
        }
        bet_syntax::ast::Type::Var(_sym) => Type::Var(0), // Simplified
        bet_syntax::ast::Type::Arrow(a, b) => Type::Fun(
            Box::new(ast_type_to_core(&a.node)),
            Box::new(ast_type_to_core(&b.node)),
        ),
        bet_syntax::ast::Type::Dist(inner) => {
            Type::Dist(Box::new(ast_type_to_core(&inner.node)))
        }
        bet_syntax::ast::Type::Tuple(elems) => {
            Type::Tuple(elems.iter().map(|e| ast_type_to_core(&e.node)).collect())
        }
        bet_syntax::ast::Type::App(base, args) => {
            let base_name = match &base.node {
                bet_syntax::ast::Type::Named(sym) => sym.to_string(),
                _ => return Type::Named("?".to_string()),
            };
            match base_name.as_str() {
                "List" if args.len() == 1 => {
                    Type::List(Box::new(ast_type_to_core(&args[0].node)))
                }
                "Set" if args.len() == 1 => {
                    Type::Set(Box::new(ast_type_to_core(&args[0].node)))
                }
                "Option" if args.len() == 1 => {
                    Type::Option(Box::new(ast_type_to_core(&args[0].node)))
                }
                "Map" if args.len() == 2 => Type::Map(
                    Box::new(ast_type_to_core(&args[0].node)),
                    Box::new(ast_type_to_core(&args[1].node)),
                ),
                "Result" if args.len() == 2 => Type::Result(
                    Box::new(ast_type_to_core(&args[0].node)),
                    Box::new(ast_type_to_core(&args[1].node)),
                ),
                "Dist" if args.len() == 1 => {
                    Type::Dist(Box::new(ast_type_to_core(&args[0].node)))
                }
                _ => Type::Named(base_name),
            }
        }
        bet_syntax::ast::Type::Ternary => Type::Ternary,
        bet_syntax::ast::Type::Hole | bet_syntax::ast::Type::Error => Type::Var(0),
        bet_syntax::ast::Type::Record(_) => Type::Unit, // Placeholder
        bet_syntax::ast::Type::Prob(_, inner) => {
            // Probability-indexed types are experimental. Treat as the inner type.
            ast_type_to_core(&inner.node)
        }
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a dummy-spanned expression.
    fn dummy(expr: Expr) -> Spanned<Expr> {
        Spanned::dummy(expr)
    }

    #[test]
    fn test_literal_types() {
        let mut env = CheckEnv::new();
        assert_eq!(check_expr(&dummy(Expr::Int(42)), &mut env).unwrap(), Type::Int);
        assert_eq!(check_expr(&dummy(Expr::Float(3.14)), &mut env).unwrap(), Type::Float);
        assert_eq!(check_expr(&dummy(Expr::Bool(true)), &mut env).unwrap(), Type::Bool);
        assert_eq!(
            check_expr(&dummy(Expr::Ternary(TernaryValue::Unknown)), &mut env).unwrap(),
            Type::Ternary
        );
        assert_eq!(check_expr(&dummy(Expr::String("hi".into())), &mut env).unwrap(), Type::String);
        assert_eq!(check_expr(&dummy(Expr::Unit), &mut env).unwrap(), Type::Unit);
    }

    #[test]
    fn test_undefined_variable() {
        let mut env = CheckEnv::new();
        let result = check_expr(&dummy(Expr::Var(Symbol::intern("x"))), &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_variable_lookup() {
        let mut env = CheckEnv::new();
        env.bind("x".to_string(), Type::Int);
        let result = check_expr(&dummy(Expr::Var(Symbol::intern("x"))), &mut env);
        assert_eq!(result.unwrap(), Type::Int);
    }

    #[test]
    fn test_bet_uniform_same_type() {
        let mut env = CheckEnv::new();
        let bet = BetExpr {
            alternatives: [
                Box::new(dummy(Expr::Int(1))),
                Box::new(dummy(Expr::Int(2))),
                Box::new(dummy(Expr::Int(3))),
            ],
        };
        let result = check_expr(&dummy(Expr::Bet(bet)), &mut env);
        assert_eq!(result.unwrap(), Type::Int);
    }

    #[test]
    fn test_bet_type_mismatch() {
        let mut env = CheckEnv::new();
        let bet = BetExpr {
            alternatives: [
                Box::new(dummy(Expr::Int(1))),
                Box::new(dummy(Expr::String("x".into()))),
                Box::new(dummy(Expr::Int(3))),
            ],
        };
        let result = check_expr(&dummy(Expr::Bet(bet)), &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_if_condition_must_be_bool_or_ternary() {
        let mut env = CheckEnv::new();
        let if_expr = IfExpr {
            condition: Box::new(dummy(Expr::Int(1))),
            then_branch: Box::new(dummy(Expr::Int(2))),
            else_branch: Box::new(dummy(Expr::Int(3))),
        };
        let result = check_expr(&dummy(Expr::If(if_expr)), &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_if_branches_must_unify() {
        let mut env = CheckEnv::new();
        let if_expr = IfExpr {
            condition: Box::new(dummy(Expr::Bool(true))),
            then_branch: Box::new(dummy(Expr::Int(1))),
            else_branch: Box::new(dummy(Expr::String("x".into()))),
        };
        let result = check_expr(&dummy(Expr::If(if_expr)), &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_if_valid() {
        let mut env = CheckEnv::new();
        let if_expr = IfExpr {
            condition: Box::new(dummy(Expr::Ternary(TernaryValue::Unknown))),
            then_branch: Box::new(dummy(Expr::Int(1))),
            else_branch: Box::new(dummy(Expr::Int(2))),
        };
        let result = check_expr(&dummy(Expr::If(if_expr)), &mut env);
        assert_eq!(result.unwrap(), Type::Int);
    }

    #[test]
    fn test_sample_from_dist() {
        let mut env = CheckEnv::new();
        env.bind("d".to_string(), Type::Dist(Box::new(Type::Float)));
        let sample = Expr::Sample(Box::new(dummy(Expr::Var(Symbol::intern("d")))));
        let result = check_expr(&dummy(sample), &mut env);
        assert_eq!(result.unwrap(), Type::Float);
    }

    #[test]
    fn test_sample_non_dist_fails() {
        let mut env = CheckEnv::new();
        env.bind("x".to_string(), Type::Int);
        let sample = Expr::Sample(Box::new(dummy(Expr::Var(Symbol::intern("x")))));
        let result = check_expr(&dummy(sample), &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_observe() {
        let mut env = CheckEnv::new();
        env.bind("d".to_string(), Type::Dist(Box::new(Type::Int)));
        let observe = Expr::Observe(
            Box::new(dummy(Expr::Var(Symbol::intern("d")))),
            Box::new(dummy(Expr::Int(5))),
        );
        let result = check_expr(&dummy(observe), &mut env);
        assert_eq!(result.unwrap(), Type::Unit);
    }

    #[test]
    fn test_observe_type_mismatch() {
        let mut env = CheckEnv::new();
        env.bind("d".to_string(), Type::Dist(Box::new(Type::Int)));
        let observe = Expr::Observe(
            Box::new(dummy(Expr::Var(Symbol::intern("d")))),
            Box::new(dummy(Expr::String("not an int".into()))),
        );
        let result = check_expr(&dummy(observe), &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_arithmetic_operators() {
        let mut env = CheckEnv::new();
        // Int + Int = Int
        let add = Expr::BinOp(
            BinOp::Add,
            Box::new(dummy(Expr::Int(1))),
            Box::new(dummy(Expr::Int(2))),
        );
        assert_eq!(check_expr(&dummy(add), &mut env).unwrap(), Type::Int);

        // Float * Int = Float
        let mul = Expr::BinOp(
            BinOp::Mul,
            Box::new(dummy(Expr::Float(1.0))),
            Box::new(dummy(Expr::Int(2))),
        );
        assert_eq!(check_expr(&dummy(mul), &mut env).unwrap(), Type::Float);
    }

    #[test]
    fn test_comparison_returns_bool() {
        let mut env = CheckEnv::new();
        let cmp = Expr::BinOp(
            BinOp::Lt,
            Box::new(dummy(Expr::Int(1))),
            Box::new(dummy(Expr::Int(2))),
        );
        assert_eq!(check_expr(&dummy(cmp), &mut env).unwrap(), Type::Bool);
    }

    #[test]
    fn test_logical_returns_ternary() {
        let mut env = CheckEnv::new();
        let and = Expr::BinOp(
            BinOp::And,
            Box::new(dummy(Expr::Bool(true))),
            Box::new(dummy(Expr::Ternary(TernaryValue::Unknown))),
        );
        assert_eq!(check_expr(&dummy(and), &mut env).unwrap(), Type::Ternary);
    }

    #[test]
    fn test_unify_basic() {
        let mut env = CheckEnv::new();
        let a = env.fresh_var();
        env.unify(&a, &Type::Int, None).unwrap();
        assert_eq!(env.resolve(&a), Type::Int);
    }

    #[test]
    fn test_unify_mismatch() {
        let mut env = CheckEnv::new();
        let result = env.unify(&Type::Int, &Type::String, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_literal() {
        let mut env = CheckEnv::new();
        let list = Expr::List(vec![
            dummy(Expr::Int(1)),
            dummy(Expr::Int(2)),
            dummy(Expr::Int(3)),
        ]);
        let result = check_expr(&dummy(list), &mut env).unwrap();
        assert_eq!(result, Type::List(Box::new(Type::Int)));
    }

    #[test]
    fn test_tuple_literal() {
        let mut env = CheckEnv::new();
        let tuple = Expr::Tuple(vec![
            dummy(Expr::Int(1)),
            dummy(Expr::String("hi".into())),
        ]);
        let result = check_expr(&dummy(tuple), &mut env).unwrap();
        assert_eq!(result, Type::Tuple(vec![Type::Int, Type::String]));
    }

    #[test]
    fn test_negation_numeric() {
        let mut env = CheckEnv::new();
        let neg = Expr::UnOp(UnOp::Neg, Box::new(dummy(Expr::Int(5))));
        assert_eq!(check_expr(&dummy(neg), &mut env).unwrap(), Type::Int);
    }

    #[test]
    fn test_not_on_bool() {
        let mut env = CheckEnv::new();
        let not = Expr::UnOp(UnOp::Not, Box::new(dummy(Expr::Bool(true))));
        assert_eq!(check_expr(&dummy(not), &mut env).unwrap(), Type::Ternary);
    }

    #[test]
    fn test_parallel() {
        let mut env = CheckEnv::new();
        let par = Expr::Parallel(
            Box::new(dummy(Expr::Int(100))),
            Box::new(dummy(Expr::Float(3.14))),
        );
        let result = check_expr(&dummy(par), &mut env).unwrap();
        assert_eq!(result, Type::List(Box::new(Type::Float)));
    }
}
