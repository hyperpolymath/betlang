// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//! Betlang Command-Line Interface
//!
//! Usage:
//!   bet repl          - Start interactive REPL
//!   bet run <file>    - Run a betlang file
//!   bet check <file>  - Type-check a file
//!   bet fmt <file>    - Format a file
//!   bet parse <file>  - Parse and print AST

#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

mod repl;

#[derive(Parser)]
#[command(name = "bet")]
#[command(author = "Betlang Team")]
#[command(version)]
#[command(about = "Betlang - A ternary probabilistic programming language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive REPL
    Repl,

    /// Run a betlang file
    Run {
        /// The file to run
        file: PathBuf,
    },

    /// Parse a file and print AST
    Parse {
        /// The file to parse
        file: PathBuf,

        /// Output format: debug (default), sexpr, json
        #[arg(long, default_value = "debug")]
        output: String,
    },

    /// Dump AST as S-expression (shorthand for parse --output sexpr)
    DumpSexpr {
        /// The file to parse
        file: PathBuf,
    },

    /// Type-check a file
    Check {
        /// The file to check
        file: PathBuf,
    },

    /// Format a file
    Fmt {
        /// The file to format
        file: PathBuf,

        /// Write output to file (instead of stdout)
        #[arg(short, long)]
        write: bool,
    },

    /// Show version information
    Version,
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Repl) | None => {
            repl::run_repl()
        }
        Some(Commands::Run { file }) => {
            run_file(&file)
        }
        Some(Commands::Parse { file, output }) => {
            parse_file(&file, &output)
        }
        Some(Commands::DumpSexpr { file }) => {
            parse_file(&file, "sexpr")
        }
        Some(Commands::Check { file }) => {
            check_file(&file)
        }
        Some(Commands::Fmt { file, write }) => {
            format_file(&file, write)
        }
        Some(Commands::Version) => {
            print_version();
            Ok(())
        }
    }
}

fn run_file(path: &PathBuf) -> Result<()> {
    let source = std::fs::read_to_string(path).into_diagnostic()?;

    let module = bet_parse::parse(&source).map_err(|e| miette::miette!("{}", e))?;

    // Type check first (warnings only — do not abort on type errors during eval)
    match bet_check::check_module(&module) {
        Ok(_env) => {
            tracing::debug!("Type check passed for {}", path.display());
        }
        Err(e) => {
            eprintln!("Warning: type check error: {}", e);
        }
    }

    // Evaluate each top-level item
    let mut env = bet_core::ValueEnv::new();
    for item in &module.items {
        match &item.node {
            bet_syntax::ast::Item::Let(def) => {
                // Evaluate the body and bind in the environment
                let val = bet_eval::eval(&def.body.node, &mut env)
                    .map_err(|e| miette::miette!("{}", e))?;
                env.bind(def.name.node.to_string(), val);
            }
            bet_syntax::ast::Item::Expr(expr) => {
                let val = bet_eval::eval(expr, &mut env)
                    .map_err(|e| miette::miette!("{}", e))?;
                println!("{}", val);
            }
            _ => {} // Type defs and imports handled at compile time
        }
    }

    Ok(())
}

/// Parse a file and display the AST in the requested output format.
///
/// Supported formats:
///   - "debug" — Rust Debug pretty-print (default)
///   - "sexpr" — S-expression representation following JtV reference pattern
///   - "json"  — Pretty-printed JSON via serde
fn parse_file(path: &PathBuf, format: &str) -> Result<()> {
    let source = std::fs::read_to_string(path).into_diagnostic()?;

    let module = bet_parse::parse(&source).map_err(|e| miette::miette!("{}", e))?;

    match format {
        "json" => {
            let json_str = serde_json::to_string_pretty(&module)
                .map_err(|e| miette::miette!("JSON serialization failed: {}", e))?;
            println!("{}", json_str);
        }
        "sexpr" | "s-expr" | "sexp" => {
            println!("{}", module_to_sexpr(&module));
        }
        "debug" | _ => {
            println!("{:#?}", module);
        }
    }

    Ok(())
}

// ============================================================================
// S-expression AST dump
//
// Converts the Betlang AST into a Lisp-like S-expression representation,
// following the Julia-the-Viper (JtV) reference pattern. Each AST node
// becomes a parenthesised list tagged by its variant name.
// ============================================================================

/// Convert an entire Betlang module to S-expression format.
fn module_to_sexpr(module: &bet_syntax::ast::Module) -> String {
    let mut out = String::new();
    out.push_str("(module");
    if let Some(name) = &module.name {
        out.push_str(&format!(" \"{}\"", name));
    }
    for item in &module.items {
        out.push_str("\n  ");
        item_to_sexpr(&item.node, &mut out, 2);
    }
    out.push(')');
    out
}

/// Emit a top-level item as an S-expression.
fn item_to_sexpr(item: &bet_syntax::ast::Item, out: &mut String, indent: usize) {
    use bet_syntax::ast::Item;
    match item {
        Item::Let(def) => {
            let rec_tag = if def.is_rec { "let-rec" } else { "let" };
            out.push_str(&format!("({} \"{}\"", rec_tag, def.name.node));
            // Parameters (if any — means this is a function definition)
            if !def.params.is_empty() {
                out.push_str("\n");
                out.push_str(&" ".repeat(indent + 2));
                out.push_str("(params");
                for p in &def.params {
                    out.push(' ');
                    pattern_to_sexpr(&p.node, out);
                }
                out.push(')');
            }
            // Optional type annotation
            if let Some(ty) = &def.type_ann {
                out.push_str("\n");
                out.push_str(&" ".repeat(indent + 2));
                out.push_str("(: ");
                type_to_sexpr(&ty.node, out);
                out.push(')');
            }
            // Body expression
            out.push_str("\n");
            out.push_str(&" ".repeat(indent + 2));
            expr_to_sexpr(&def.body.node, out, indent + 2);
            out.push(')');
        }
        Item::TypeDef(td) => {
            out.push_str(&format!("(type-def \"{}\"", td.name.node));
            if !td.params.is_empty() {
                out.push_str(" (type-params");
                for p in &td.params {
                    out.push_str(&format!(" \"{}\"", p.node));
                }
                out.push(')');
            }
            out.push(' ');
            type_to_sexpr(&td.body.node, out);
            out.push(')');
        }
        Item::Import(imp) => {
            let path_str: Vec<String> = imp.path.iter().map(|s| s.node.to_string()).collect();
            out.push_str(&format!("(import \"{}\"", path_str.join(".")));
            if let Some(items) = &imp.items {
                out.push_str(" (items");
                for i in items {
                    out.push_str(&format!(" \"{}\"", i.node));
                }
                out.push(')');
            }
            out.push(')');
        }
        Item::Expr(expr) => {
            out.push_str("(top-expr ");
            expr_to_sexpr(expr, out, indent + 2);
            out.push(')');
        }
    }
}

/// Emit an expression as an S-expression.
///
/// This is the core of the dump: every `Expr` variant maps to a named
/// S-expression list, with sub-expressions recursively emitted.
fn expr_to_sexpr(expr: &bet_syntax::ast::Expr, out: &mut String, indent: usize) {
    use bet_syntax::ast::Expr;
    match expr {
        // --- Literals ---
        Expr::Int(v) => out.push_str(&format!("(int {})", v)),
        Expr::Float(v) => out.push_str(&format!("(float {})", v)),
        Expr::String(s) => out.push_str(&format!("(string {:?})", s)),
        Expr::Bool(b) => out.push_str(&format!("(bool {})", b)),
        Expr::Ternary(tv) => out.push_str(&format!("(ternary {:?})", tv)),
        Expr::Unit => out.push_str("(unit)"),

        // --- Core ternary primitive ---
        Expr::Bet(bet) => {
            out.push_str("(bet");
            for alt in &bet.alternatives {
                out.push(' ');
                expr_to_sexpr(&alt.node, out, indent);
            }
            out.push(')');
        }
        Expr::WeightedBet(wb) => {
            out.push_str("(weighted-bet");
            for (alt, weight) in &wb.alternatives {
                out.push_str("\n");
                out.push_str(&" ".repeat(indent + 2));
                out.push_str("(@ ");
                expr_to_sexpr(&alt.node, out, indent + 4);
                out.push(' ');
                expr_to_sexpr(&weight.node, out, indent + 4);
                out.push(')');
            }
            out.push(')');
        }
        Expr::ConditionalBet(cb) => {
            out.push_str("(bet-if ");
            expr_to_sexpr(&cb.condition.node, out, indent + 2);
            out.push_str("\n");
            out.push_str(&" ".repeat(indent + 2));
            out.push_str("(then ");
            expr_to_sexpr(&cb.if_true.node, out, indent + 4);
            out.push(')');
            out.push_str("\n");
            out.push_str(&" ".repeat(indent + 2));
            out.push_str("(else");
            for alt in &cb.if_false {
                out.push(' ');
                expr_to_sexpr(&alt.node, out, indent + 4);
            }
            out.push_str("))");
        }

        // --- Variables and application ---
        Expr::Var(sym) => out.push_str(&format!("(var \"{}\")", sym)),
        Expr::App(func, args) => {
            out.push_str("(app ");
            expr_to_sexpr(&func.node, out, indent + 2);
            for arg in args {
                out.push(' ');
                expr_to_sexpr(&arg.node, out, indent + 2);
            }
            out.push(')');
        }
        Expr::Lambda(lam) => {
            out.push_str("(lambda (params");
            for p in &lam.params {
                out.push(' ');
                pattern_to_sexpr(&p.node, out);
            }
            out.push(')');
            out.push_str("\n");
            out.push_str(&" ".repeat(indent + 2));
            expr_to_sexpr(&lam.body.node, out, indent + 2);
            out.push(')');
        }

        // --- Binding forms ---
        Expr::Let(le) => {
            let rec_tag = if le.is_rec { "let-rec" } else { "let" };
            out.push_str(&format!("({} ", rec_tag));
            pattern_to_sexpr(&le.pattern.node, out);
            if let Some(ty) = &le.type_ann {
                out.push_str(" (: ");
                type_to_sexpr(&ty.node, out);
                out.push(')');
            }
            out.push(' ');
            expr_to_sexpr(&le.value.node, out, indent + 2);
            out.push_str("\n");
            out.push_str(&" ".repeat(indent + 2));
            expr_to_sexpr(&le.body.node, out, indent + 2);
            out.push(')');
        }
        Expr::Do(doex) => {
            out.push_str("(do");
            for stmt in &doex.statements {
                out.push_str("\n");
                out.push_str(&" ".repeat(indent + 2));
                do_stmt_to_sexpr(&stmt.node, out, indent + 2);
            }
            out.push(')');
        }

        // --- Control flow ---
        Expr::If(ifex) => {
            out.push_str("(if ");
            expr_to_sexpr(&ifex.condition.node, out, indent + 2);
            out.push_str("\n");
            out.push_str(&" ".repeat(indent + 2));
            out.push_str("(then ");
            expr_to_sexpr(&ifex.then_branch.node, out, indent + 4);
            out.push(')');
            out.push_str("\n");
            out.push_str(&" ".repeat(indent + 2));
            out.push_str("(else ");
            expr_to_sexpr(&ifex.else_branch.node, out, indent + 4);
            out.push_str("))");
        }
        Expr::Match(m) => {
            out.push_str("(match ");
            expr_to_sexpr(&m.scrutinee.node, out, indent + 2);
            for arm in &m.arms {
                out.push_str("\n");
                out.push_str(&" ".repeat(indent + 2));
                out.push_str("(arm ");
                pattern_to_sexpr(&arm.pattern.node, out);
                if let Some(guard) = &arm.guard {
                    out.push_str(" (guard ");
                    expr_to_sexpr(&guard.node, out, indent + 4);
                    out.push(')');
                }
                out.push(' ');
                expr_to_sexpr(&arm.body.node, out, indent + 4);
                out.push(')');
            }
            out.push(')');
        }

        // --- Data structures ---
        Expr::Tuple(elems) => {
            out.push_str("(tuple");
            for e in elems {
                out.push(' ');
                expr_to_sexpr(&e.node, out, indent);
            }
            out.push(')');
        }
        Expr::List(elems) => {
            out.push_str("(list");
            for e in elems {
                out.push(' ');
                expr_to_sexpr(&e.node, out, indent);
            }
            out.push(')');
        }
        Expr::Record(fields) => {
            out.push_str("(record");
            for (name, val) in fields {
                out.push_str(&format!(" (field \"{}\" ", name.node));
                expr_to_sexpr(&val.node, out, indent + 2);
                out.push(')');
            }
            out.push(')');
        }
        Expr::Field(obj, field) => {
            out.push_str("(field-access ");
            expr_to_sexpr(&obj.node, out, indent + 2);
            out.push_str(&format!(" \"{}\")", field.node));
        }
        Expr::Index(obj, idx) => {
            out.push_str("(index ");
            expr_to_sexpr(&obj.node, out, indent + 2);
            out.push(' ');
            expr_to_sexpr(&idx.node, out, indent + 2);
            out.push(')');
        }

        // --- Operators ---
        Expr::BinOp(op, lhs, rhs) => {
            out.push_str(&format!("({:?} ", op));
            expr_to_sexpr(&lhs.node, out, indent + 2);
            out.push(' ');
            expr_to_sexpr(&rhs.node, out, indent + 2);
            out.push(')');
        }
        Expr::UnOp(op, operand) => {
            out.push_str(&format!("({:?} ", op));
            expr_to_sexpr(&operand.node, out, indent + 2);
            out.push(')');
        }

        // --- Probabilistic operations ---
        Expr::Sample(dist) => {
            out.push_str("(sample ");
            expr_to_sexpr(&dist.node, out, indent + 2);
            out.push(')');
        }
        Expr::Observe(dist, val) => {
            out.push_str("(observe ");
            expr_to_sexpr(&dist.node, out, indent + 2);
            out.push(' ');
            expr_to_sexpr(&val.node, out, indent + 2);
            out.push(')');
        }
        Expr::Infer(inf) => {
            out.push_str(&format!("(infer {:?}", inf.method));
            if !inf.params.is_empty() {
                out.push_str(" (params");
                for (name, val) in &inf.params {
                    out.push_str(&format!(" (\"{}\" ", name.node));
                    expr_to_sexpr(&val.node, out, indent + 4);
                    out.push(')');
                }
                out.push(')');
            }
            out.push(' ');
            expr_to_sexpr(&inf.model.node, out, indent + 2);
            out.push(')');
        }
        Expr::Parallel(count, body) => {
            out.push_str("(parallel ");
            expr_to_sexpr(&count.node, out, indent + 2);
            out.push(' ');
            expr_to_sexpr(&body.node, out, indent + 2);
            out.push(')');
        }

        // --- Type annotation ---
        Expr::Annotate(e, ty) => {
            out.push_str("(: ");
            expr_to_sexpr(&e.node, out, indent + 2);
            out.push(' ');
            type_to_sexpr(&ty.node, out);
            out.push(')');
        }

        // --- Special ---
        Expr::Hole(name) => match name {
            Some(n) => out.push_str(&format!("(hole \"{}\")", n)),
            None => out.push_str("(hole _)"),
        },
        Expr::Error => out.push_str("(error)"),
    }
}

/// Emit a do-notation statement as an S-expression.
fn do_stmt_to_sexpr(stmt: &bet_syntax::ast::DoStatement, out: &mut String, indent: usize) {
    use bet_syntax::ast::DoStatement;
    match stmt {
        DoStatement::Bind(pat, expr) => {
            out.push_str("(bind ");
            pattern_to_sexpr(&pat.node, out);
            out.push(' ');
            expr_to_sexpr(&expr.node, out, indent + 2);
            out.push(')');
        }
        DoStatement::Expr(expr) => {
            expr_to_sexpr(&expr.node, out, indent);
        }
        DoStatement::Let(pat, expr) => {
            out.push_str("(do-let ");
            pattern_to_sexpr(&pat.node, out);
            out.push(' ');
            expr_to_sexpr(&expr.node, out, indent + 2);
            out.push(')');
        }
    }
}

/// Emit a pattern as an S-expression.
fn pattern_to_sexpr(pat: &bet_syntax::ast::Pattern, out: &mut String) {
    use bet_syntax::ast::Pattern;
    match pat {
        Pattern::Wildcard => out.push('_'),
        Pattern::Var(sym) => out.push_str(&format!("\"{}\"", sym)),
        Pattern::Literal(lit) => literal_to_sexpr(lit, out),
        Pattern::Tuple(elems) => {
            out.push_str("(tuple-pat");
            for e in elems {
                out.push(' ');
                pattern_to_sexpr(&e.node, out);
            }
            out.push(')');
        }
        Pattern::List(elems, rest) => {
            out.push_str("(list-pat");
            for e in elems {
                out.push(' ');
                pattern_to_sexpr(&e.node, out);
            }
            if let Some(r) = rest {
                out.push_str(" | ");
                pattern_to_sexpr(&r.node, out);
            }
            out.push(')');
        }
        Pattern::Constructor(name, args) => {
            out.push_str(&format!("(ctor \"{}\"", name));
            for a in args {
                out.push(' ');
                pattern_to_sexpr(&a.node, out);
            }
            out.push(')');
        }
        Pattern::Record(fields) => {
            out.push_str("(record-pat");
            for (name, pat_opt) in fields {
                out.push_str(&format!(" (\"{}\"", name.node));
                if let Some(p) = pat_opt {
                    out.push(' ');
                    pattern_to_sexpr(&p.node, out);
                }
                out.push(')');
            }
            out.push(')');
        }
        Pattern::As(pat, name) => {
            out.push_str("(as ");
            pattern_to_sexpr(&pat.node, out);
            out.push_str(&format!(" \"{}\")", name));
        }
        Pattern::Or(a, b, c) => {
            out.push_str("(or ");
            pattern_to_sexpr(&a.node, out);
            out.push(' ');
            pattern_to_sexpr(&b.node, out);
            out.push(' ');
            pattern_to_sexpr(&c.node, out);
            out.push(')');
        }
        Pattern::Annotate(pat, ty) => {
            out.push_str("(: ");
            pattern_to_sexpr(&pat.node, out);
            out.push(' ');
            type_to_sexpr(&ty.node, out);
            out.push(')');
        }
    }
}

/// Emit a literal as an S-expression.
fn literal_to_sexpr(lit: &bet_syntax::ast::Literal, out: &mut String) {
    use bet_syntax::ast::Literal;
    match lit {
        Literal::Int(v) => out.push_str(&format!("{}", v)),
        Literal::Float(v) => out.push_str(&format!("{}", v)),
        Literal::String(s) => out.push_str(&format!("{:?}", s)),
        Literal::Bool(b) => out.push_str(&format!("{}", b)),
        Literal::Ternary(tv) => out.push_str(&format!("{:?}", tv)),
        Literal::Unit => out.push_str("()"),
    }
}

/// Emit a type as an S-expression.
fn type_to_sexpr(ty: &bet_syntax::ast::Type, out: &mut String) {
    use bet_syntax::ast::Type;
    match ty {
        Type::Named(sym) => out.push_str(&format!("(type \"{}\")", sym)),
        Type::Var(sym) => out.push_str(&format!("(type-var \"{}\")", sym)),
        Type::App(base, args) => {
            out.push_str("(type-app ");
            type_to_sexpr(&base.node, out);
            for a in args {
                out.push(' ');
                type_to_sexpr(&a.node, out);
            }
            out.push(')');
        }
        Type::Arrow(from, to) => {
            out.push_str("(-> ");
            type_to_sexpr(&from.node, out);
            out.push(' ');
            type_to_sexpr(&to.node, out);
            out.push(')');
        }
        Type::Tuple(elems) => {
            out.push_str("(tuple-type");
            for e in elems {
                out.push(' ');
                type_to_sexpr(&e.node, out);
            }
            out.push(')');
        }
        Type::Record(fields) => {
            out.push_str("(record-type");
            for (name, ty) in fields {
                out.push_str(&format!(" (\"{}\" ", name));
                type_to_sexpr(&ty.node, out);
                out.push(')');
            }
            out.push(')');
        }
        Type::Dist(inner) => {
            out.push_str("(Dist ");
            type_to_sexpr(&inner.node, out);
            out.push(')');
        }
        Type::Prob(prob, inner) => {
            out.push_str("(Prob ");
            // prob is an Expr representing the probability
            expr_to_sexpr(&prob.node, out, 0);
            out.push(' ');
            type_to_sexpr(&inner.node, out);
            out.push(')');
        }
        Type::Ternary => out.push_str("Ternary"),
        Type::Hole => out.push_str("_"),
        Type::Error => out.push_str("(type-error)"),
    }
}

fn check_file(path: &PathBuf) -> Result<()> {
    let source = std::fs::read_to_string(path).into_diagnostic()?;

    let module = bet_parse::parse(&source).map_err(|e| miette::miette!("{}", e))?;

    match bet_check::check_module(&module) {
        Ok(_env) => {
            println!("OK: {} type-checks successfully ({} items)", path.display(), module.items.len());
        }
        Err(e) => {
            eprintln!("Type error in {}: {}", path.display(), e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn format_file(path: &PathBuf, write: bool) -> Result<()> {
    let source = std::fs::read_to_string(path).into_diagnostic()?;

    let _module = bet_parse::parse(&source).map_err(|e| miette::miette!("{}", e))?;

    // Re-emit from parsed AST via S-expression round-trip as a basic formatter.
    // A full pretty-printer would preserve comments and layout; for now we
    // normalise whitespace through parse-then-print.
    let formatted = module_to_sexpr(&_module);

    if write {
        std::fs::write(path, &formatted).into_diagnostic()?;
        println!("Formatted {}", path.display());
    } else {
        print!("{}", formatted);
    }

    Ok(())
}

fn print_version() {
    println!("bet {}", env!("CARGO_PKG_VERSION"));
    println!("Betlang - A ternary probabilistic programming language");
    println!();
    println!("Features:");
    println!("  - Ternary bet primitive: bet {{ a, b, c }}");
    println!("  - First-class distributions: Dist τ");
    println!("  - Monadic do-notation for probabilistic programming");
    println!("  - Ternary logic: true, false, unknown");
}
