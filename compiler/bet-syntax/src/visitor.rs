// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

//! AST Visitor framework for BetLang.

use crate::ast::*;

/// Trait for visiting BetLang AST nodes.
pub trait Visitor: Sized {
    fn visit_module(&mut self, module: &Module) {
        for item in &module.items {
            self.visit_item(&item.node);
        }
    }

    fn visit_item(&mut self, item: &Item) {
        match item {
            Item::Let(def) => self.visit_expr(&def.body.node),
            Item::TypeDef(_) | Item::Import(_) => {}
            Item::Expr(e) => self.visit_expr(e),
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_pattern(&mut self, _pattern: &Pattern) {}
}

/// Walk an expression, visiting all children.
pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &Expr) {
    match expr {
        // Leaves
        Expr::Int(_) | Expr::Float(_) | Expr::String(_) | Expr::Bool(_)
        | Expr::Ternary(_) | Expr::Unit | Expr::Var(_) | Expr::Hole(_)
        | Expr::Error => {}

        // Bet (ternary probabilistic) — exactly 3 alternatives
        Expr::Bet(bet) => {
            for alt in &bet.alternatives {
                visitor.visit_expr(&alt.node);
            }
        }
        Expr::WeightedBet(wb) => {
            for (expr, _weight) in &wb.alternatives {
                visitor.visit_expr(&expr.node);
            }
        }
        Expr::ConditionalBet(cb) => {
            visitor.visit_expr(&cb.condition.node);
            visitor.visit_expr(&cb.if_true.node);
            for alt in &cb.if_false {
                visitor.visit_expr(&alt.node);
            }
        }
        Expr::Sample(inner) => visitor.visit_expr(&inner.node),
        Expr::Observe(dist, val) => {
            visitor.visit_expr(&dist.node);
            visitor.visit_expr(&val.node);
        }
        Expr::Infer(inf) => visitor.visit_expr(&inf.model.node),
        Expr::Parallel(count, body) => {
            visitor.visit_expr(&count.node);
            visitor.visit_expr(&body.node);
        }

        // Functions
        Expr::App(func, args) => {
            visitor.visit_expr(&func.node);
            for arg in args {
                visitor.visit_expr(&arg.node);
            }
        }
        Expr::Lambda(lam) => {
            for p in &lam.params {
                visitor.visit_pattern(&p.node);
            }
            visitor.visit_expr(&lam.body.node);
        }

        // Bindings
        Expr::Let(l) => {
            visitor.visit_pattern(&l.pattern.node);
            visitor.visit_expr(&l.value.node);
            visitor.visit_expr(&l.body.node);
        }
        Expr::Do(d) => {
            for stmt in &d.statements {
                match &stmt.node {
                    DoStatement::Bind(p, e) => {
                        visitor.visit_pattern(&p.node);
                        visitor.visit_expr(&e.node);
                    }
                    DoStatement::Let(p, e) => {
                        visitor.visit_pattern(&p.node);
                        visitor.visit_expr(&e.node);
                    }
                    DoStatement::Expr(e) => visitor.visit_expr(&e.node),
                }
            }
        }

        // Control
        Expr::If(i) => {
            visitor.visit_expr(&i.condition.node);
            visitor.visit_expr(&i.then_branch.node);
            visitor.visit_expr(&i.else_branch.node);
        }
        Expr::Match(m) => {
            visitor.visit_expr(&m.scrutinee.node);
            for arm in &m.arms {
                visitor.visit_pattern(&arm.pattern.node);
                if let Some(guard) = &arm.guard {
                    visitor.visit_expr(&guard.node);
                }
                visitor.visit_expr(&arm.body.node);
            }
        }

        // Collections
        Expr::Tuple(elems) | Expr::List(elems) => {
            for e in elems {
                visitor.visit_expr(&e.node);
            }
        }
        Expr::Record(fields) => {
            for (_, val) in fields {
                visitor.visit_expr(&val.node);
            }
        }
        Expr::Field(expr, _) => visitor.visit_expr(&expr.node),
        Expr::Index(expr, idx) => {
            visitor.visit_expr(&expr.node);
            visitor.visit_expr(&idx.node);
        }

        // Operators
        Expr::BinOp(_, lhs, rhs) => {
            visitor.visit_expr(&lhs.node);
            visitor.visit_expr(&rhs.node);
        }
        Expr::UnOp(_, operand) => visitor.visit_expr(&operand.node),

        // Annotations
        Expr::Annotate(expr, _) => visitor.visit_expr(&expr.node),
    }
}
