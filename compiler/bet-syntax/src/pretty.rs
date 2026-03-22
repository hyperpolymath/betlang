// SPDX-License-Identifier: PMPL-1.0-or-later
//! Pretty-printer for BetLang AST.
//!
//! Converts AST nodes into formatted source code strings. Used for the
//! `fmt` command, code generation output, and REPL display.
//!
//! The printer preserves round-trip semantics: `parse(pretty(ast)) == ast`.

use crate::ast::*;
use crate::span::Spanned;
use crate::symbol::Symbol;

/// Configuration for the pretty-printer.
#[derive(Debug, Clone)]
pub struct PrettyConfig {
    /// Number of spaces per indentation level.
    pub indent_width: usize,
    /// Maximum line width before breaking (advisory).
    pub max_width: usize,
}

impl Default for PrettyConfig {
    fn default() -> Self {
        Self {
            indent_width: 2,
            max_width: 100,
        }
    }
}

/// Pretty-printer state.
struct Printer {
    buf: String,
    indent: usize,
    config: PrettyConfig,
}

impl Printer {
    fn new(config: PrettyConfig) -> Self {
        Self {
            buf: String::with_capacity(1024),
            indent: 0,
            config,
        }
    }

    fn emit(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn newline(&mut self) {
        self.buf.push('\n');
        for _ in 0..self.indent * self.config.indent_width {
            self.buf.push(' ');
        }
    }

    fn indented<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.indent += 1;
        f(self);
        self.indent -= 1;
    }

    fn finish(self) -> String {
        self.buf
    }
}

// ============================================================
// Public API
// ============================================================

/// Pretty-print a module to a string with default configuration.
pub fn module_to_string(module: &Module) -> String {
    module_to_string_with_config(module, PrettyConfig::default())
}

/// Pretty-print a module with a custom configuration.
pub fn module_to_string_with_config(module: &Module, config: PrettyConfig) -> String {
    let mut p = Printer::new(config);
    pp_module(&mut p, module);
    p.finish()
}

/// Pretty-print a single expression to a string.
pub fn expr_to_string(expr: &Expr) -> String {
    let mut p = Printer::new(PrettyConfig::default());
    pp_expr(&mut p, expr);
    p.finish()
}

/// Pretty-print a single item to a string.
pub fn item_to_string(item: &Item) -> String {
    let mut p = Printer::new(PrettyConfig::default());
    pp_item(&mut p, item);
    p.finish()
}

// ============================================================
// Module
// ============================================================

fn pp_module(p: &mut Printer, module: &Module) {
    if let Some(ref name) = module.name {
        p.emit("module ");
        p.emit(&name.as_str());
        p.newline();
        p.newline();
    }
    for (i, item) in module.items.iter().enumerate() {
        if i > 0 {
            p.newline();
            p.newline();
        }
        pp_item(p, &item.node);
    }
    p.emit("\n");
}

// ============================================================
// Items
// ============================================================

fn pp_item(p: &mut Printer, item: &Item) {
    match item {
        Item::Let(def) => pp_let_def(p, def),
        Item::TypeDef(td) => pp_type_def(p, td),
        Item::Import(imp) => pp_import(p, imp),
        Item::Expr(e) => pp_expr(p, e),
    }
}

fn pp_let_def(p: &mut Printer, def: &LetDef) {
    p.emit("let ");
    if def.is_rec {
        p.emit("rec ");
    }
    p.emit(&def.name.node.as_str());
    for param in &def.params {
        p.emit(" ");
        pp_pattern(p, &param.node);
    }
    if let Some(ref ty) = def.type_ann {
        p.emit(" : ");
        pp_type(p, &ty.node);
    }
    p.emit(" = ");
    pp_expr(p, &def.body.node);
}

fn pp_type_def(p: &mut Printer, td: &TypeDef) {
    p.emit("type ");
    p.emit(&td.name.node.as_str());
    for param in &td.params {
        p.emit(" ");
        p.emit(&param.node.as_str());
    }
    p.emit(" = ");
    pp_type(p, &td.body.node);
}

fn pp_import(p: &mut Printer, imp: &Import) {
    p.emit("import ");
    for (i, seg) in imp.path.iter().enumerate() {
        if i > 0 {
            p.emit(".");
        }
        p.emit(&seg.node.as_str());
    }
    if let Some(ref items) = imp.items {
        p.emit(".{");
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                p.emit(", ");
            }
            p.emit(&item.node.as_str());
        }
        p.emit("}");
    }
}

// ============================================================
// Expressions
// ============================================================

fn pp_expr(p: &mut Printer, expr: &Expr) {
    match expr {
        Expr::Int(n) => p.emit(&n.to_string()),
        Expr::Float(f) => p.emit(&format!("{}", f)),
        Expr::String(s) => {
            p.emit("\"");
            p.emit(&s.replace('\\', "\\\\").replace('"', "\\\""));
            p.emit("\"");
        }
        Expr::Bool(b) => p.emit(if *b { "true" } else { "false" }),
        Expr::Ternary(tv) => pp_ternary_value(p, tv),
        Expr::Unit => p.emit("()"),

        Expr::Bet(bet) => {
            p.emit("bet { ");
            pp_spanned_expr(p, &bet.alternatives[0]);
            p.emit(", ");
            pp_spanned_expr(p, &bet.alternatives[1]);
            p.emit(", ");
            pp_spanned_expr(p, &bet.alternatives[2]);
            p.emit(" }");
        }

        Expr::WeightedBet(wb) => {
            p.emit("bet { ");
            for (i, (expr, weight)) in wb.alternatives.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                pp_spanned_expr(p, expr);
                p.emit(" @ ");
                pp_spanned_expr(p, weight);
            }
            p.emit(" }");
        }

        Expr::ConditionalBet(cb) => {
            p.emit("bet_if ");
            pp_spanned_expr(p, &cb.condition);
            p.emit(" { ");
            pp_spanned_expr(p, &cb.if_true);
            p.emit(" } else { ");
            pp_spanned_expr(p, &cb.if_false[0]);
            p.emit(", ");
            pp_spanned_expr(p, &cb.if_false[1]);
            p.emit(", ");
            pp_spanned_expr(p, &cb.if_false[2]);
            p.emit(" }");
        }

        Expr::Var(sym) => p.emit(&sym.as_str()),

        Expr::App(callee, args) => {
            pp_spanned_expr(p, callee);
            for arg in args {
                p.emit(" ");
                pp_spanned_expr_parens(p, &arg.node);
            }
        }

        Expr::Lambda(lam) => {
            p.emit("fun ");
            for (i, param) in lam.params.iter().enumerate() {
                if i > 0 {
                    p.emit(" ");
                }
                pp_pattern(p, &param.node);
            }
            p.emit(" -> ");
            pp_spanned_expr(p, &lam.body);
        }

        Expr::Let(le) => {
            p.emit("let ");
            if le.is_rec {
                p.emit("rec ");
            }
            pp_pattern(p, &le.pattern.node);
            if let Some(ref ty) = le.type_ann {
                p.emit(" : ");
                pp_type(p, &ty.node);
            }
            p.emit(" = ");
            pp_spanned_expr(p, &le.value);
            p.emit(" in");
            p.indented(|p| {
                p.newline();
                pp_spanned_expr(p, &le.body);
            });
        }

        Expr::Do(do_expr) => {
            p.emit("do {");
            p.indented(|p| {
                for stmt in &do_expr.statements {
                    p.newline();
                    pp_do_statement(p, &stmt.node);
                    p.emit(";");
                }
            });
            p.newline();
            p.emit("}");
        }

        Expr::If(if_expr) => {
            p.emit("if ");
            pp_spanned_expr(p, &if_expr.condition);
            p.emit(" then");
            p.indented(|p| {
                p.newline();
                pp_spanned_expr(p, &if_expr.then_branch);
            });
            p.newline();
            p.emit("else");
            p.indented(|p| {
                p.newline();
                pp_spanned_expr(p, &if_expr.else_branch);
            });
        }

        Expr::Match(me) => {
            p.emit("match ");
            pp_spanned_expr(p, &me.scrutinee);
            p.emit(" {");
            p.indented(|p| {
                for arm in &me.arms {
                    p.newline();
                    pp_pattern(p, &arm.pattern.node);
                    if let Some(ref guard) = arm.guard {
                        p.emit(" when ");
                        pp_spanned_expr(p, guard);
                    }
                    p.emit(" -> ");
                    pp_expr(p, &arm.body.node);
                    p.emit(";");
                }
            });
            p.newline();
            p.emit("}");
        }

        Expr::Tuple(elems) => {
            p.emit("(");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                pp_spanned_expr(p, elem);
            }
            p.emit(")");
        }

        Expr::List(elems) => {
            p.emit("[");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                pp_spanned_expr(p, elem);
            }
            p.emit("]");
        }

        Expr::Record(fields) => {
            p.emit("{ ");
            for (i, (name, val)) in fields.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                p.emit(&name.node.as_str());
                p.emit(" = ");
                pp_spanned_expr(p, val);
            }
            p.emit(" }");
        }

        Expr::Field(target, field) => {
            pp_spanned_expr(p, target);
            p.emit(".");
            p.emit(&field.node.as_str());
        }

        Expr::Index(target, idx) => {
            pp_spanned_expr(p, target);
            p.emit("[");
            pp_spanned_expr(p, idx);
            p.emit("]");
        }

        Expr::BinOp(op, lhs, rhs) => {
            p.emit("(");
            pp_spanned_expr(p, lhs);
            p.emit(" ");
            pp_binop(p, op);
            p.emit(" ");
            pp_spanned_expr(p, rhs);
            p.emit(")");
        }

        Expr::UnOp(op, operand) => {
            pp_unop(p, op);
            pp_spanned_expr(p, operand);
        }

        Expr::Sample(dist) => {
            p.emit("sample ");
            pp_spanned_expr(p, dist);
        }

        Expr::Observe(dist, val) => {
            p.emit("observe ");
            pp_spanned_expr(p, dist);
            p.emit(" ");
            pp_spanned_expr(p, val);
        }

        Expr::Infer(inf) => {
            p.emit("infer ");
            pp_infer_method(p, &inf.method);
            if !inf.params.is_empty() {
                p.emit(" { ");
                for (i, (name, val)) in inf.params.iter().enumerate() {
                    if i > 0 {
                        p.emit(", ");
                    }
                    p.emit(&name.node.as_str());
                    p.emit(" = ");
                    pp_spanned_expr(p, val);
                }
                p.emit(" } ");
            } else {
                p.emit(" ");
            }
            pp_spanned_expr(p, &inf.model);
        }

        Expr::Parallel(n, body) => {
            p.emit("parallel ");
            pp_spanned_expr(p, n);
            p.emit(" { ");
            pp_spanned_expr(p, body);
            p.emit(" }");
        }

        Expr::Annotate(e, ty) => {
            pp_spanned_expr(p, e);
            p.emit(" : ");
            pp_type(p, &ty.node);
        }

        Expr::Hole(name) => {
            if let Some(sym) = name {
                p.emit("?");
                p.emit(&sym.as_str());
            } else {
                p.emit("_");
            }
        }

        Expr::Error => p.emit("(* error *)"),
    }
}

fn pp_spanned_expr(p: &mut Printer, se: &Spanned<Expr>) {
    pp_expr(p, &se.node);
}

/// Print an expression, wrapping in parens if it is compound.
fn pp_spanned_expr_parens(p: &mut Printer, e: &Expr) {
    let needs_parens = matches!(
        e,
        Expr::App(..)
            | Expr::BinOp(..)
            | Expr::Lambda(..)
            | Expr::Let(..)
            | Expr::If(..)
            | Expr::Match(..)
    );
    if needs_parens {
        p.emit("(");
        pp_expr(p, e);
        p.emit(")");
    } else {
        pp_expr(p, e);
    }
}

// ============================================================
// Do-notation
// ============================================================

fn pp_do_statement(p: &mut Printer, stmt: &DoStatement) {
    match stmt {
        DoStatement::Bind(pat, expr) => {
            pp_pattern(p, &pat.node);
            p.emit(" <- ");
            pp_expr(p, &expr.node);
        }
        DoStatement::Expr(expr) => pp_expr(p, &expr.node),
        DoStatement::Let(pat, expr) => {
            p.emit("let ");
            pp_pattern(p, &pat.node);
            p.emit(" = ");
            pp_expr(p, &expr.node);
        }
    }
}

// ============================================================
// Patterns
// ============================================================

fn pp_pattern(p: &mut Printer, pat: &Pattern) {
    match pat {
        Pattern::Wildcard => p.emit("_"),
        Pattern::Var(sym) => p.emit(&sym.as_str()),
        Pattern::Literal(lit) => pp_literal(p, lit),
        Pattern::Tuple(elems) => {
            p.emit("(");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                pp_pattern(p, &elem.node);
            }
            p.emit(")");
        }
        Pattern::List(elems, rest) => {
            p.emit("[");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                pp_pattern(p, &elem.node);
            }
            if let Some(ref tail) = rest {
                if !elems.is_empty() {
                    p.emit(" | ");
                }
                pp_pattern(p, &tail.node);
            }
            p.emit("]");
        }
        Pattern::Constructor(name, args) => {
            p.emit(&name.as_str());
            for arg in args {
                p.emit(" ");
                pp_pattern(p, &arg.node);
            }
        }
        Pattern::Record(fields) => {
            p.emit("{ ");
            for (i, (name, pat)) in fields.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                p.emit(&name.node.as_str());
                if let Some(ref p_inner) = pat {
                    p.emit(" = ");
                    pp_pattern(p, &p_inner.node);
                }
            }
            p.emit(" }");
        }
        Pattern::As(inner, name) => {
            pp_pattern(p, &inner.node);
            p.emit(" as ");
            p.emit(&name.as_str());
        }
        Pattern::Or(a, b, c) => {
            pp_pattern(p, &a.node);
            p.emit(" | ");
            pp_pattern(p, &b.node);
            p.emit(" | ");
            pp_pattern(p, &c.node);
        }
        Pattern::Annotate(inner, ty) => {
            pp_pattern(p, &inner.node);
            p.emit(" : ");
            pp_type(p, &ty.node);
        }
    }
}

fn pp_literal(p: &mut Printer, lit: &Literal) {
    match lit {
        Literal::Int(n) => p.emit(&n.to_string()),
        Literal::Float(f) => p.emit(&format!("{}", f)),
        Literal::String(s) => {
            p.emit("\"");
            p.emit(&s.replace('\\', "\\\\").replace('"', "\\\""));
            p.emit("\"");
        }
        Literal::Bool(b) => p.emit(if *b { "true" } else { "false" }),
        Literal::Ternary(tv) => pp_ternary_value(p, tv),
        Literal::Unit => p.emit("()"),
    }
}

fn pp_ternary_value(p: &mut Printer, tv: &TernaryValue) {
    match tv {
        TernaryValue::True => p.emit("true"),
        TernaryValue::False => p.emit("false"),
        TernaryValue::Unknown => p.emit("unknown"),
    }
}

// ============================================================
// Operators
// ============================================================

fn pp_binop(p: &mut Printer, op: &BinOp) {
    p.emit(match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Mod => "%",
        BinOp::Pow => "**",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::Lt => "<",
        BinOp::Le => "<=",
        BinOp::Gt => ">",
        BinOp::Ge => ">=",
        BinOp::And => "and",
        BinOp::Or => "or",
        BinOp::Xor => "xor",
        BinOp::Concat => "++",
        BinOp::Cons => "::",
        BinOp::Append => "@",
        BinOp::Compose => ">>",
    });
}

fn pp_unop(p: &mut Printer, op: &UnOp) {
    match op {
        UnOp::Neg => p.emit("-"),
        UnOp::Not => p.emit("not "),
        UnOp::Sample => p.emit("sample "),
    }
}

fn pp_infer_method(p: &mut Printer, method: &InferMethod) {
    p.emit(match method {
        InferMethod::MCMC => "mcmc",
        InferMethod::HMC => "hmc",
        InferMethod::SMC => "smc",
        InferMethod::VI => "vi",
        InferMethod::Rejection => "rejection",
        InferMethod::Importance => "importance",
    });
}

// ============================================================
// Types
// ============================================================

fn pp_type(p: &mut Printer, ty: &Type) {
    match ty {
        Type::Named(sym) => p.emit(&sym.as_str()),
        Type::Var(sym) => {
            p.emit("'");
            p.emit(&sym.as_str());
        }
        Type::App(base, args) => {
            pp_spanned_type(p, base);
            for arg in args {
                p.emit(" ");
                pp_spanned_type(p, arg);
            }
        }
        Type::Arrow(param, result) => {
            pp_spanned_type(p, param);
            p.emit(" -> ");
            pp_spanned_type(p, result);
        }
        Type::Tuple(elems) => {
            p.emit("(");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                pp_spanned_type(p, elem);
            }
            p.emit(")");
        }
        Type::Record(fields) => {
            p.emit("{ ");
            for (i, (name, ty)) in fields.iter().enumerate() {
                if i > 0 {
                    p.emit(", ");
                }
                p.emit(&name.as_str());
                p.emit(" : ");
                pp_spanned_type(p, ty);
            }
            p.emit(" }");
        }
        Type::Dist(inner) => {
            p.emit("Dist ");
            pp_spanned_type(p, inner);
        }
        Type::Prob(prob, inner) => {
            p.emit("Prob ");
            pp_spanned_expr(p, prob);
            p.emit(" ");
            pp_spanned_type(p, inner);
        }
        Type::Ternary => p.emit("Ternary"),
        Type::Hole => p.emit("_"),
        Type::Error => p.emit("(* type error *)"),
    }
}

fn pp_spanned_type(p: &mut Printer, st: &Spanned<Type>) {
    pp_type(p, &st.node);
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{Span, Spanned};

    fn dummy<T>(node: T) -> Spanned<T> {
        Spanned::dummy(node)
    }

    fn boxed_dummy(expr: Expr) -> Box<Spanned<Expr>> {
        Box::new(dummy(expr))
    }

    #[test]
    fn test_int_literal() {
        assert_eq!(expr_to_string(&Expr::Int(42)), "42");
    }

    #[test]
    fn test_float_literal() {
        assert_eq!(expr_to_string(&Expr::Float(3.14)), "3.14");
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(expr_to_string(&Expr::String("hello".into())), "\"hello\"");
    }

    #[test]
    fn test_bool_literals() {
        assert_eq!(expr_to_string(&Expr::Bool(true)), "true");
        assert_eq!(expr_to_string(&Expr::Bool(false)), "false");
    }

    #[test]
    fn test_ternary_values() {
        assert_eq!(
            expr_to_string(&Expr::Ternary(TernaryValue::Unknown)),
            "unknown"
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(expr_to_string(&Expr::Unit), "()");
    }

    #[test]
    fn test_bet_expression() {
        let bet = Expr::Bet(BetExpr {
            alternatives: [
                boxed_dummy(Expr::Int(1)),
                boxed_dummy(Expr::Int(2)),
                boxed_dummy(Expr::Int(3)),
            ],
        });
        assert_eq!(expr_to_string(&bet), "bet { 1, 2, 3 }");
    }

    #[test]
    fn test_binop() {
        let e = Expr::BinOp(
            BinOp::Add,
            boxed_dummy(Expr::Var(Symbol::intern("x"))),
            boxed_dummy(Expr::Int(1)),
        );
        assert_eq!(expr_to_string(&e), "(x + 1)");
    }

    #[test]
    fn test_lambda() {
        let lam = Expr::Lambda(LambdaExpr {
            params: vec![dummy(Pattern::Var(Symbol::intern("x")))],
            body: boxed_dummy(Expr::Var(Symbol::intern("x"))),
        });
        assert_eq!(expr_to_string(&lam), "fun x -> x");
    }

    #[test]
    fn test_let_in() {
        let le = Expr::Let(LetExpr {
            pattern: dummy(Pattern::Var(Symbol::intern("x"))),
            type_ann: None,
            value: boxed_dummy(Expr::Int(42)),
            body: boxed_dummy(Expr::Var(Symbol::intern("x"))),
            is_rec: false,
        });
        assert_eq!(expr_to_string(&le), "let x = 42 in\n  x");
    }

    #[test]
    fn test_match() {
        let me = Expr::Match(MatchExpr {
            scrutinee: boxed_dummy(Expr::Var(Symbol::intern("x"))),
            arms: vec![
                MatchArm {
                    pattern: dummy(Pattern::Literal(Literal::Int(1))),
                    guard: None,
                    body: dummy(Expr::String("one".into())),
                },
                MatchArm {
                    pattern: dummy(Pattern::Wildcard),
                    guard: None,
                    body: dummy(Expr::String("other".into())),
                },
            ],
        });
        let result = expr_to_string(&me);
        assert!(result.contains("match x {"));
        assert!(result.contains("1 -> \"one\""));
        assert!(result.contains("_ -> \"other\""));
    }

    #[test]
    fn test_do_notation() {
        let d = Expr::Do(DoExpr {
            statements: vec![
                dummy(DoStatement::Bind(
                    dummy(Pattern::Var(Symbol::intern("x"))),
                    dummy(Expr::App(
                        boxed_dummy(Expr::Var(Symbol::intern("sample"))),
                        vec![dummy(Expr::Var(Symbol::intern("normal")))],
                    )),
                )),
                dummy(DoStatement::Expr(dummy(Expr::Var(Symbol::intern("x"))))),
            ],
        });
        let result = expr_to_string(&d);
        assert!(result.contains("do {"));
        assert!(result.contains("x <- sample normal"));
    }

    #[test]
    fn test_list_tuple() {
        let list = Expr::List(vec![
            dummy(Expr::Int(1)),
            dummy(Expr::Int(2)),
            dummy(Expr::Int(3)),
        ]);
        assert_eq!(expr_to_string(&list), "[1, 2, 3]");

        let tuple = Expr::Tuple(vec![
            dummy(Expr::Bool(true)),
            dummy(Expr::Int(42)),
        ]);
        assert_eq!(expr_to_string(&tuple), "(true, 42)");
    }

    #[test]
    fn test_if_expr() {
        let ie = Expr::If(IfExpr {
            condition: boxed_dummy(Expr::Bool(true)),
            then_branch: boxed_dummy(Expr::Int(1)),
            else_branch: boxed_dummy(Expr::Int(0)),
        });
        let result = expr_to_string(&ie);
        assert!(result.contains("if true then"));
        assert!(result.contains("else"));
    }

    #[test]
    fn test_sample_observe() {
        let s = Expr::Sample(boxed_dummy(Expr::Var(Symbol::intern("normal"))));
        assert_eq!(expr_to_string(&s), "sample normal");
    }

    #[test]
    fn test_record() {
        let r = Expr::Record(vec![
            (dummy(Symbol::intern("x")), dummy(Expr::Int(1))),
            (dummy(Symbol::intern("y")), dummy(Expr::Int(2))),
        ]);
        assert_eq!(expr_to_string(&r), "{ x = 1, y = 2 }");
    }

    #[test]
    fn test_let_def_item() {
        let item = Item::Let(LetDef {
            name: dummy(Symbol::intern("add")),
            params: vec![
                dummy(Pattern::Var(Symbol::intern("x"))),
                dummy(Pattern::Var(Symbol::intern("y"))),
            ],
            type_ann: None,
            body: dummy(Expr::BinOp(
                BinOp::Add,
                boxed_dummy(Expr::Var(Symbol::intern("x"))),
                boxed_dummy(Expr::Var(Symbol::intern("y"))),
            )),
            is_rec: false,
        });
        assert_eq!(item_to_string(&item), "let add x y = (x + y)");
    }
}
