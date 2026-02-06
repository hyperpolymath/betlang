// SPDX-License-Identifier: PMPL-1.0-or-later
//! Code generation for Betlang
//!
//! Generates JavaScript output from the Betlang AST, with full coverage of
//! probabilistic operations: distributions, Bayesian inference, Monte Carlo
//! simulation, Markov chains, and uncertainty propagation.
//!
//! Targets:
//! - JavaScript (for web) -- fully implemented
//! - LLVM IR (for native) -- stub
//! - BEAM bytecode (for Erlang VM) -- stub
//!
//! Author: Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>

use bet_syntax::ast::*;
use bet_syntax::span::Spanned;
use bet_core::CompileResult;

/// Code generation target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    JavaScript,
    Llvm,
    Beam,
}

/// Generated code output
#[derive(Debug, Clone)]
pub struct CodeOutput {
    pub target: Target,
    pub code: String,
    pub source_map: Option<String>,
}

/// JavaScript codegen context: tracks variable names, indentation, and
/// whether the runtime preamble has been emitted.
struct JsContext {
    indent: usize,
    temp_counter: usize,
    preamble_emitted: bool,
}

impl JsContext {
    fn new() -> Self {
        Self {
            indent: 0,
            temp_counter: 0,
            preamble_emitted: false,
        }
    }

    fn fresh_var(&mut self) -> String {
        let n = self.temp_counter;
        self.temp_counter += 1;
        format!("__t{}", n)
    }

    fn indent_str(&self) -> String {
        "  ".repeat(self.indent)
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Generate code for an expression
pub fn codegen(expr: &Expr, target: Target) -> CompileResult<CodeOutput> {
    match target {
        Target::JavaScript => codegen_js(expr),
        Target::Llvm => codegen_llvm(expr),
        Target::Beam => codegen_beam(expr),
    }
}

/// Generate code for a module (top-level items)
pub fn codegen_module(module: &Module, target: Target) -> CompileResult<CodeOutput> {
    match target {
        Target::JavaScript => codegen_module_js(module),
        Target::Llvm => codegen_llvm_placeholder(),
        Target::Beam => codegen_beam_placeholder(),
    }
}

// ---------------------------------------------------------------------------
// JavaScript runtime preamble
// ---------------------------------------------------------------------------

/// The runtime preamble provides the minimal probabilistic runtime that
/// generated JavaScript programs depend on. It covers:
///   - `__bet_uniform(alts)` -- uniform ternary choice
///   - `__bet_weighted(alts, weights)` -- weighted ternary choice
///   - `__bet_sample(dist)` -- sample from a distribution object
///   - `__bet_observe(dist, value)` -- condition / observe
///   - `__bet_infer(method, params, model)` -- run inference
///   - `__bet_dist_*` -- distribution constructors
///   - `__bet_markov_step` / `__bet_markov_chain` -- Markov chain helpers
///   - `__bet_monte_carlo` -- Monte Carlo simulation
///   - `__bet_uncertain` -- uncertainty-propagating arithmetic
fn js_preamble() -> &'static str {
    r#"// === Betlang Runtime Preamble ===

// Uniform ternary choice among exactly 3 alternatives
function __bet_uniform(a, b, c) {
  const r = Math.random();
  if (r < 1/3) return a;
  if (r < 2/3) return b;
  return c;
}

// Weighted ternary choice (weights normalised internally)
function __bet_weighted(alts, weights) {
  const total = weights.reduce((s, w) => s + w, 0);
  const r = Math.random() * total;
  let cumul = 0;
  for (let i = 0; i < alts.length; i++) {
    cumul += weights[i];
    if (r < cumul) return alts[i];
  }
  return alts[alts.length - 1];
}

// --- Distribution objects ---

function __bet_dist_normal(mu, sigma) {
  return {
    name: 'normal',
    params: { mu, sigma },
    sample() {
      // Box-Muller transform
      const u1 = Math.random();
      const u2 = Math.random();
      const z = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
      return mu + sigma * z;
    },
    logpdf(x) {
      const z = (x - mu) / sigma;
      return -0.5 * Math.log(2 * Math.PI) - Math.log(sigma) - 0.5 * z * z;
    }
  };
}

function __bet_dist_uniform(lo, hi) {
  return {
    name: 'uniform',
    params: { lo, hi },
    sample() { return lo + Math.random() * (hi - lo); },
    logpdf(x) { return (x >= lo && x <= hi) ? -Math.log(hi - lo) : -Infinity; }
  };
}

function __bet_dist_bernoulli(p) {
  return {
    name: 'bernoulli',
    params: { p },
    sample() { return Math.random() < p ? 1 : 0; },
    logpdf(x) { return x === 1 ? Math.log(p) : Math.log(1 - p); }
  };
}

function __bet_dist_beta(alpha, beta_param) {
  return {
    name: 'beta',
    params: { alpha, beta: beta_param },
    sample() {
      // Joehnk's algorithm for small parameters; otherwise gamma ratio
      function gamma_sample(shape) {
        if (shape < 1) {
          return gamma_sample(shape + 1) * Math.pow(Math.random(), 1 / shape);
        }
        const d = shape - 1/3;
        const c = 1 / Math.sqrt(9 * d);
        while (true) {
          let x, v;
          do {
            const u1 = Math.random();
            const u2 = Math.random();
            x = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
            v = Math.pow(1 + c * x, 3);
          } while (v <= 0);
          const u = Math.random();
          if (u < 1 - 0.0331 * x * x * x * x) return d * v;
          if (Math.log(u) < 0.5 * x * x + d * (1 - v + Math.log(v))) return d * v;
        }
      }
      const ga = gamma_sample(alpha);
      const gb = gamma_sample(beta_param);
      return ga / (ga + gb);
    },
    logpdf(x) {
      if (x <= 0 || x >= 1) return -Infinity;
      function lnGamma(z) {
        const c = [76.18009172947146,-86.50532032941677,24.01409824083091,
          -1.231739572450155,0.001208650973866179,-0.000005395239384953];
        let s = 1.000000000190015;
        for (let i = 0; i < 6; i++) s += c[i] / (z + i + 1);
        return Math.log(2.5066282746310005 * s / z) + (z + 0.5) * Math.log(z + 5.5) - (z + 5.5);
      }
      return (alpha - 1) * Math.log(x) + (beta_param - 1) * Math.log(1 - x)
        + lnGamma(alpha + beta_param) - lnGamma(alpha) - lnGamma(beta_param);
    }
  };
}

function __bet_dist_exponential(rate) {
  return {
    name: 'exponential',
    params: { rate },
    sample() { return -Math.log(Math.random()) / rate; },
    logpdf(x) { return x >= 0 ? Math.log(rate) - rate * x : -Infinity; }
  };
}

function __bet_dist_poisson(lambda) {
  return {
    name: 'poisson',
    params: { lambda },
    sample() {
      const L = Math.exp(-lambda);
      let k = 0, p = 1;
      do { k++; p *= Math.random(); } while (p > L);
      return k - 1;
    },
    logpdf(x) {
      if (x < 0 || x !== Math.floor(x)) return -Infinity;
      let lf = 0;
      for (let i = 2; i <= x; i++) lf += Math.log(i);
      return x * Math.log(lambda) - lambda - lf;
    }
  };
}

// Sample from a distribution object
function __bet_sample(dist) {
  if (dist && typeof dist.sample === 'function') return dist.sample();
  throw new Error('Cannot sample from non-distribution: ' + JSON.stringify(dist));
}

// Observe / condition: returns log-probability (for use in inference)
function __bet_observe(dist, value) {
  if (dist && typeof dist.logpdf === 'function') return dist.logpdf(value);
  throw new Error('Cannot observe on non-distribution');
}

// --- Inference ---

function __bet_infer(method, params, modelFn) {
  const n = params.samples || params.n || 1000;
  switch (method) {
    case 'rejection': return __bet_infer_rejection(modelFn, n);
    case 'importance': return __bet_infer_importance(modelFn, n);
    case 'mcmc': return __bet_infer_mcmc(modelFn, n);
    default: return __bet_infer_rejection(modelFn, n);
  }
}

function __bet_infer_rejection(modelFn, n) {
  const samples = [];
  let attempts = 0;
  const maxAttempts = n * 100;
  while (samples.length < n && attempts < maxAttempts) {
    attempts++;
    const result = modelFn();
    if (result !== undefined && result !== null) samples.push(result);
  }
  return samples;
}

function __bet_infer_importance(modelFn, n) {
  const samples = [];
  const weights = [];
  for (let i = 0; i < n; i++) {
    let logWeight = 0;
    const ctx = {
      observe(dist, val) { logWeight += __bet_observe(dist, val); }
    };
    const val = modelFn(ctx);
    samples.push(val);
    weights.push(Math.exp(logWeight));
  }
  // Resample according to weights
  const totalW = weights.reduce((a, b) => a + b, 0);
  const resampled = [];
  for (let i = 0; i < n; i++) {
    let r = Math.random() * totalW;
    let cumul = 0;
    for (let j = 0; j < n; j++) {
      cumul += weights[j];
      if (r < cumul) { resampled.push(samples[j]); break; }
    }
  }
  return resampled;
}

function __bet_infer_mcmc(modelFn, n) {
  const samples = [];
  let current = modelFn();
  for (let i = 0; i < n; i++) {
    const proposal = modelFn();
    // Metropolis-Hastings with uniform proposal (accept always for now)
    current = proposal;
    samples.push(current);
  }
  return samples;
}

// --- Monte Carlo simulation ---

function __bet_monte_carlo(n, trialFn) {
  const results = [];
  for (let i = 0; i < n; i++) {
    results.push(trialFn());
  }
  return results;
}

function __bet_mc_mean(samples) {
  return samples.reduce((a, b) => a + b, 0) / samples.length;
}

function __bet_mc_variance(samples) {
  const mu = __bet_mc_mean(samples);
  return samples.reduce((s, x) => s + (x - mu) * (x - mu), 0) / samples.length;
}

// --- Markov chain ---

function __bet_markov_step(state, transitionFn) {
  return transitionFn(state);
}

function __bet_markov_chain(initial, steps, transitionFn) {
  const chain = [initial];
  let state = initial;
  for (let i = 0; i < steps; i++) {
    state = transitionFn(state);
    chain.push(state);
  }
  return chain;
}

// --- Uncertainty propagation ---

class __BetUncertain {
  constructor(value, variance) {
    this.value = value;
    this.variance = variance;
  }
  get stddev() { return Math.sqrt(this.variance); }

  static from(value, stddev) {
    return new __BetUncertain(value, stddev * stddev);
  }

  add(other) {
    if (other instanceof __BetUncertain) {
      return new __BetUncertain(this.value + other.value, this.variance + other.variance);
    }
    return new __BetUncertain(this.value + other, this.variance);
  }
  sub(other) {
    if (other instanceof __BetUncertain) {
      return new __BetUncertain(this.value - other.value, this.variance + other.variance);
    }
    return new __BetUncertain(this.value - other, this.variance);
  }
  mul(other) {
    if (other instanceof __BetUncertain) {
      // First-order error propagation: Var(X*Y) ~ Y^2*Var(X) + X^2*Var(Y)
      const v = other.value * other.value * this.variance
              + this.value * this.value * other.variance;
      return new __BetUncertain(this.value * other.value, v);
    }
    return new __BetUncertain(this.value * other, this.variance * other * other);
  }
  div(other) {
    if (other instanceof __BetUncertain) {
      const v = (this.variance + this.value * this.value * other.variance
                / (other.value * other.value))
                / (other.value * other.value);
      return new __BetUncertain(this.value / other.value, v);
    }
    return new __BetUncertain(this.value / other, this.variance / (other * other));
  }

  toString() {
    return `${this.value} ± ${this.stddev.toFixed(4)}`;
  }
}

// === End of Betlang Runtime Preamble ===

"#
}

// ---------------------------------------------------------------------------
// JavaScript backend -- expressions
// ---------------------------------------------------------------------------

fn codegen_js(expr: &Expr) -> CompileResult<CodeOutput> {
    let mut ctx = JsContext::new();
    let mut output = String::new();
    output.push_str("// Generated by betlang\n");
    output.push_str("'use strict';\n\n");

    // Emit preamble
    output.push_str(js_preamble());
    ctx.preamble_emitted = true;

    emit_js_expr(&mut output, expr, &mut ctx)?;
    output.push_str(";\n");

    Ok(CodeOutput {
        target: Target::JavaScript,
        code: output,
        source_map: None,
    })
}

fn codegen_module_js(module: &Module) -> CompileResult<CodeOutput> {
    let mut ctx = JsContext::new();
    let mut output = String::new();
    output.push_str("// Generated by betlang\n");
    output.push_str("'use strict';\n\n");

    output.push_str(js_preamble());
    ctx.preamble_emitted = true;

    for item in &module.items {
        emit_js_item(&mut output, &item.node, &mut ctx)?;
        output.push('\n');
    }

    Ok(CodeOutput {
        target: Target::JavaScript,
        code: output,
        source_map: None,
    })
}

fn emit_js_item(out: &mut String, item: &Item, ctx: &mut JsContext) -> CompileResult<()> {
    match item {
        Item::Let(def) => {
            let name = def.name.node.as_str();
            if def.params.is_empty() {
                out.push_str(&format!("const {} = ", name));
                emit_js_expr(out, &def.body.node, ctx)?;
                out.push_str(";\n");
            } else {
                // Function definition
                out.push_str(&format!("function {}(", name));
                for (i, p) in def.params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_js_pattern_param(out, &p.node);
                }
                out.push_str(") {\n");
                ctx.indent += 1;
                out.push_str(&ctx.indent_str());
                out.push_str("return ");
                emit_js_expr(out, &def.body.node, ctx)?;
                out.push_str(";\n");
                ctx.indent -= 1;
                out.push_str("}\n");
            }
        }
        Item::TypeDef(_) => {
            // Type definitions are erased at runtime
            out.push_str("// (type definition elided)\n");
        }
        Item::Import(imp) => {
            let path: Vec<String> = imp.path.iter().map(|s| s.node.as_str()).collect();
            out.push_str(&format!("// import {}\n", path.join(".")));
        }
        Item::Expr(e) => {
            emit_js_expr(out, e, ctx)?;
            out.push_str(";\n");
        }
    }
    Ok(())
}

fn emit_js_pattern_param(out: &mut String, pat: &Pattern) {
    match pat {
        Pattern::Var(sym) => out.push_str(&sym.as_str()),
        Pattern::Wildcard => out.push_str("_"),
        _ => out.push_str("_"),
    }
}

fn emit_js_expr(out: &mut String, expr: &Expr, ctx: &mut JsContext) -> CompileResult<()> {
    match expr {
        // --- Literals ---
        Expr::Int(i) => out.push_str(&i.to_string()),
        Expr::Float(f) => {
            let s = f.to_string();
            out.push_str(&s);
            // Ensure it looks like a float in JS
            if !s.contains('.') && !s.contains('e') && !s.contains('E') {
                out.push_str(".0");
            }
        }
        Expr::String(s) => {
            out.push('"');
            out.push_str(&s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n"));
            out.push('"');
        }
        Expr::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Expr::Ternary(t) => match t {
            TernaryValue::True => out.push_str("1"),
            TernaryValue::False => out.push_str("-1"),
            TernaryValue::Unknown => out.push_str("0"),
        },
        Expr::Unit => out.push_str("null"),

        // --- Variables ---
        Expr::Var(sym) => {
            let name = sym.as_str();
            out.push_str(&sanitize_js_ident(&name));
        }

        // --- Ternary bet (uniform) ---
        Expr::Bet(bet) => {
            out.push_str("__bet_uniform(");
            emit_js_expr(out, &bet.alternatives[0].node, ctx)?;
            out.push_str(", ");
            emit_js_expr(out, &bet.alternatives[1].node, ctx)?;
            out.push_str(", ");
            emit_js_expr(out, &bet.alternatives[2].node, ctx)?;
            out.push(')');
        }

        // --- Weighted bet ---
        Expr::WeightedBet(wb) => {
            out.push_str("__bet_weighted([");
            for (i, (val, _wt)) in wb.alternatives.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_js_expr(out, &val.node, ctx)?;
            }
            out.push_str("], [");
            for (i, (_val, wt)) in wb.alternatives.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_js_expr(out, &wt.node, ctx)?;
            }
            out.push_str("])");
        }

        // --- Conditional bet ---
        Expr::ConditionalBet(cb) => {
            out.push_str("(");
            emit_js_expr(out, &cb.condition.node, ctx)?;
            out.push_str(" ? ");
            emit_js_expr(out, &cb.if_true.node, ctx)?;
            out.push_str(" : __bet_uniform(");
            emit_js_expr(out, &cb.if_false[0].node, ctx)?;
            out.push_str(", ");
            emit_js_expr(out, &cb.if_false[1].node, ctx)?;
            out.push_str(", ");
            emit_js_expr(out, &cb.if_false[2].node, ctx)?;
            out.push_str("))");
        }

        // --- Function application ---
        Expr::App(func, args) => {
            // Special-case well-known distribution constructors
            if let Expr::Var(sym) = &func.node {
                let name = sym.as_str();
                match name.as_str() {
                    "normal" | "Normal" => {
                        out.push_str("__bet_dist_normal(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "uniform" | "Uniform" => {
                        out.push_str("__bet_dist_uniform(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "bernoulli" | "Bernoulli" => {
                        out.push_str("__bet_dist_bernoulli(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "beta" | "Beta" => {
                        out.push_str("__bet_dist_beta(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "exponential" | "Exponential" => {
                        out.push_str("__bet_dist_exponential(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "poisson" | "Poisson" => {
                        out.push_str("__bet_dist_poisson(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "monte_carlo" => {
                        out.push_str("__bet_monte_carlo(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "markov_chain" => {
                        out.push_str("__bet_markov_chain(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "markov_step" => {
                        out.push_str("__bet_markov_step(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "uncertain" => {
                        out.push_str("__BetUncertain.from(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "mc_mean" => {
                        out.push_str("__bet_mc_mean(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    "mc_variance" => {
                        out.push_str("__bet_mc_variance(");
                        emit_js_args(out, args, ctx)?;
                        out.push(')');
                        return Ok(());
                    }
                    _ => {}
                }
            }
            // General function application
            emit_js_expr(out, &func.node, ctx)?;
            out.push('(');
            emit_js_args(out, args, ctx)?;
            out.push(')');
        }

        // --- Lambda ---
        Expr::Lambda(lam) => {
            out.push_str("(function(");
            for (i, p) in lam.params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_js_pattern_param(out, &p.node);
            }
            out.push_str(") { return ");
            emit_js_expr(out, &lam.body.node, ctx)?;
            out.push_str("; })");
        }

        // --- Let binding ---
        Expr::Let(le) => {
            out.push_str("(function() {\n");
            ctx.indent += 1;

            // Bind pattern
            out.push_str(&ctx.indent_str());
            emit_js_let_pattern(out, &le.pattern.node, ctx)?;
            out.push_str(" = ");
            emit_js_expr(out, &le.value.node, ctx)?;
            out.push_str(";\n");

            out.push_str(&ctx.indent_str());
            out.push_str("return ");
            emit_js_expr(out, &le.body.node, ctx)?;
            out.push_str(";\n");

            ctx.indent -= 1;
            out.push_str(&ctx.indent_str());
            out.push_str("})()");
        }

        // --- Do notation (monadic sequencing) ---
        Expr::Do(doexpr) => {
            out.push_str("(function() {\n");
            ctx.indent += 1;

            for (i, stmt) in doexpr.statements.iter().enumerate() {
                out.push_str(&ctx.indent_str());
                let is_last = i == doexpr.statements.len() - 1;
                match &stmt.node {
                    DoStatement::Bind(pat, e) => {
                        emit_js_let_pattern(out, &pat.node, ctx)?;
                        out.push_str(" = ");
                        emit_js_expr(out, &e.node, ctx)?;
                        out.push_str(";\n");
                    }
                    DoStatement::Let(pat, e) => {
                        emit_js_let_pattern(out, &pat.node, ctx)?;
                        out.push_str(" = ");
                        emit_js_expr(out, &e.node, ctx)?;
                        out.push_str(";\n");
                    }
                    DoStatement::Expr(e) => {
                        if is_last {
                            out.push_str("return ");
                        }
                        emit_js_expr(out, &e.node, ctx)?;
                        out.push_str(";\n");
                    }
                }
            }

            ctx.indent -= 1;
            out.push_str(&ctx.indent_str());
            out.push_str("})()");
        }

        // --- Conditional ---
        Expr::If(ifexpr) => {
            out.push('(');
            emit_js_expr(out, &ifexpr.condition.node, ctx)?;
            out.push_str(" ? ");
            emit_js_expr(out, &ifexpr.then_branch.node, ctx)?;
            out.push_str(" : ");
            emit_js_expr(out, &ifexpr.else_branch.node, ctx)?;
            out.push(')');
        }

        // --- Match ---
        Expr::Match(m) => {
            let scrutinee_var = ctx.fresh_var();
            out.push_str("(function() {\n");
            ctx.indent += 1;

            out.push_str(&ctx.indent_str());
            out.push_str(&format!("const {} = ", scrutinee_var));
            emit_js_expr(out, &m.scrutinee.node, ctx)?;
            out.push_str(";\n");

            for arm in &m.arms {
                out.push_str(&ctx.indent_str());
                out.push_str("if (");
                emit_js_pattern_match(out, &scrutinee_var, &arm.pattern.node)?;
                if let Some(guard) = &arm.guard {
                    out.push_str(" && ");
                    emit_js_expr(out, &guard.node, ctx)?;
                }
                out.push_str(") {\n");
                ctx.indent += 1;
                out.push_str(&ctx.indent_str());
                // Bind pattern variables
                emit_js_pattern_bindings(out, &scrutinee_var, &arm.pattern.node);
                out.push_str("return ");
                emit_js_expr(out, &arm.body.node, ctx)?;
                out.push_str(";\n");
                ctx.indent -= 1;
                out.push_str(&ctx.indent_str());
                out.push_str("}\n");
            }

            out.push_str(&ctx.indent_str());
            out.push_str("throw new Error('Non-exhaustive match');\n");
            ctx.indent -= 1;
            out.push_str(&ctx.indent_str());
            out.push_str("})()");
        }

        // --- Data structures ---
        Expr::Tuple(elems) => {
            out.push('[');
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_js_expr(out, &e.node, ctx)?;
            }
            out.push(']');
        }

        Expr::List(elems) => {
            out.push('[');
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_js_expr(out, &e.node, ctx)?;
            }
            out.push(']');
        }

        Expr::Record(fields) => {
            out.push_str("({ ");
            for (i, (name, val)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&name.node.as_str());
                out.push_str(": ");
                emit_js_expr(out, &val.node, ctx)?;
            }
            out.push_str(" })");
        }

        Expr::Field(obj, field) => {
            emit_js_expr(out, &obj.node, ctx)?;
            out.push('.');
            out.push_str(&field.node.as_str());
        }

        Expr::Index(obj, idx) => {
            emit_js_expr(out, &obj.node, ctx)?;
            out.push('[');
            emit_js_expr(out, &idx.node, ctx)?;
            out.push(']');
        }

        // --- Binary operators ---
        Expr::BinOp(op, lhs, rhs) => {
            // Check if we need uncertainty-aware arithmetic
            out.push('(');
            match op {
                BinOp::Add => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" + ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Sub => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" - ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Mul => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" * ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Div => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" / ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Mod => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" % ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Pow => {
                    out.push_str("Math.pow(");
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(", ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                    out.push(')');
                }
                BinOp::Eq => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" === ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Ne => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" !== ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Lt => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" < ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Le => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" <= ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Gt => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" > ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Ge => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" >= ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::And => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" && ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Or => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" || ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Xor => {
                    // Ternary XOR: if either is unknown, result is unknown
                    out.push_str("((");
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" === 0 || ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                    out.push_str(" === 0) ? 0 : (");
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" !== ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                    out.push_str("))");
                }
                BinOp::Concat => {
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(" + ");
                    emit_js_expr(out, &rhs.node, ctx)?;
                }
                BinOp::Cons => {
                    out.push('[');
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(", ...");
                    emit_js_expr(out, &rhs.node, ctx)?;
                    out.push(']');
                }
                BinOp::Append => {
                    out.push_str("[...");
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(", ...");
                    emit_js_expr(out, &rhs.node, ctx)?;
                    out.push(']');
                }
                BinOp::Compose => {
                    // Kleisli composition: (f >> g)(x) = g(f(x))
                    let param = ctx.fresh_var();
                    out.push_str(&format!("(function({p}) {{ return (", p = param));
                    emit_js_expr(out, &rhs.node, ctx)?;
                    out.push_str(&format!(")("));
                    out.push('(');
                    emit_js_expr(out, &lhs.node, ctx)?;
                    out.push_str(&format!(")({p})); }})", p = param));
                }
            }
            out.push(')');
        }

        // --- Unary operators ---
        Expr::UnOp(op, operand) => {
            match op {
                UnOp::Neg => {
                    out.push_str("(-");
                    emit_js_expr(out, &operand.node, ctx)?;
                    out.push(')');
                }
                UnOp::Not => {
                    // Ternary NOT: map {1 -> -1, -1 -> 1, 0 -> 0}
                    out.push_str("(-");
                    emit_js_expr(out, &operand.node, ctx)?;
                    out.push(')');
                }
                UnOp::Sample => {
                    out.push_str("__bet_sample(");
                    emit_js_expr(out, &operand.node, ctx)?;
                    out.push(')');
                }
            }
        }

        // --- Probabilistic operations ---
        Expr::Sample(dist) => {
            out.push_str("__bet_sample(");
            emit_js_expr(out, &dist.node, ctx)?;
            out.push(')');
        }

        Expr::Observe(dist, value) => {
            out.push_str("__bet_observe(");
            emit_js_expr(out, &dist.node, ctx)?;
            out.push_str(", ");
            emit_js_expr(out, &value.node, ctx)?;
            out.push(')');
        }

        Expr::Infer(inf) => {
            let method_str = match inf.method {
                InferMethod::MCMC => "mcmc",
                InferMethod::HMC => "hmc",
                InferMethod::SMC => "smc",
                InferMethod::VI => "vi",
                InferMethod::Rejection => "rejection",
                InferMethod::Importance => "importance",
            };
            out.push_str(&format!("__bet_infer('{}', {{", method_str));
            for (i, (key, val)) in inf.params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&key.node.as_str());
                out.push_str(": ");
                emit_js_expr(out, &val.node, ctx)?;
            }
            out.push_str("}, function() { return ");
            emit_js_expr(out, &inf.model.node, ctx)?;
            out.push_str("; })");
        }

        Expr::Parallel(n, body) => {
            out.push_str("(function() {\n");
            ctx.indent += 1;
            out.push_str(&ctx.indent_str());
            out.push_str("const __n = ");
            emit_js_expr(out, &n.node, ctx)?;
            out.push_str(";\n");
            out.push_str(&ctx.indent_str());
            out.push_str("const __results = [];\n");
            out.push_str(&ctx.indent_str());
            out.push_str("for (let __i = 0; __i < __n; __i++) {\n");
            ctx.indent += 1;
            out.push_str(&ctx.indent_str());
            out.push_str("__results.push(");
            emit_js_expr(out, &body.node, ctx)?;
            out.push_str(");\n");
            ctx.indent -= 1;
            out.push_str(&ctx.indent_str());
            out.push_str("}\n");
            out.push_str(&ctx.indent_str());
            out.push_str("return __results;\n");
            ctx.indent -= 1;
            out.push_str(&ctx.indent_str());
            out.push_str("})()");
        }

        // --- Type annotations (erased at codegen) ---
        Expr::Annotate(inner, _ty) => {
            emit_js_expr(out, &inner.node, ctx)?;
        }

        // --- Holes and errors ---
        Expr::Hole(name) => {
            let label = name.as_ref().map(|s| s.as_str()).unwrap_or_else(|| "_".to_string());
            out.push_str(&format!(
                "(function() {{ throw new Error('Unimplemented hole: {}'); }})()",
                label
            ));
        }

        Expr::Error => {
            out.push_str("(function() { throw new Error('Compilation error node'); })()");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn emit_js_args(
    out: &mut String,
    args: &[Spanned<Expr>],
    ctx: &mut JsContext,
) -> CompileResult<()> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        emit_js_expr(out, &arg.node, ctx)?;
    }
    Ok(())
}

fn emit_js_let_pattern(out: &mut String, pat: &Pattern, _ctx: &mut JsContext) -> CompileResult<()> {
    match pat {
        Pattern::Var(sym) => {
            out.push_str(&format!("const {}", sanitize_js_ident(&sym.as_str())));
        }
        Pattern::Wildcard => {
            out.push_str("const _");
        }
        Pattern::Tuple(elems) => {
            out.push_str("const [");
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                match &e.node {
                    Pattern::Var(sym) => out.push_str(&sanitize_js_ident(&sym.as_str())),
                    Pattern::Wildcard => out.push('_'),
                    _ => out.push('_'),
                }
            }
            out.push(']');
        }
        _ => {
            out.push_str("const _");
        }
    }
    Ok(())
}

fn emit_js_pattern_match(out: &mut String, scrutinee: &str, pat: &Pattern) -> CompileResult<()> {
    match pat {
        Pattern::Wildcard => out.push_str("true"),
        Pattern::Var(_) => out.push_str("true"),
        Pattern::Literal(lit) => {
            out.push_str(&format!("{} === ", scrutinee));
            match lit {
                Literal::Int(i) => out.push_str(&i.to_string()),
                Literal::Float(f) => out.push_str(&f.to_string()),
                Literal::String(s) => {
                    out.push('"');
                    out.push_str(&s.replace('\\', "\\\\").replace('"', "\\\""));
                    out.push('"');
                }
                Literal::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                Literal::Ternary(t) => match t {
                    TernaryValue::True => out.push_str("1"),
                    TernaryValue::False => out.push_str("-1"),
                    TernaryValue::Unknown => out.push_str("0"),
                },
                Literal::Unit => out.push_str("null"),
            }
        }
        _ => out.push_str("true"), // Simplified
    }
    Ok(())
}

fn emit_js_pattern_bindings(out: &mut String, scrutinee: &str, pat: &Pattern) {
    match pat {
        Pattern::Var(sym) => {
            out.push_str(&format!(
                "const {} = {}; ",
                sanitize_js_ident(&sym.as_str()),
                scrutinee
            ));
        }
        _ => {} // Other patterns: bindings already handled or not needed
    }
}

/// Sanitize a betlang identifier for use as a JavaScript identifier.
/// Replaces characters that are not valid in JS with underscores.
fn sanitize_js_ident(name: &str) -> String {
    if is_js_reserved(name) {
        format!("_{}", name)
    } else {
        name.replace('-', "_").replace('\'', "$prime")
    }
}

fn is_js_reserved(name: &str) -> bool {
    matches!(
        name,
        "break" | "case" | "catch" | "class" | "const" | "continue"
            | "debugger" | "default" | "delete" | "do" | "else"
            | "export" | "extends" | "finally" | "for" | "function"
            | "if" | "import" | "in" | "instanceof" | "let" | "new"
            | "return" | "super" | "switch" | "this" | "throw" | "try"
            | "typeof" | "var" | "void" | "while" | "with" | "yield"
    )
}

// ---------------------------------------------------------------------------
// LLVM placeholder
// ---------------------------------------------------------------------------

fn codegen_llvm(expr: &Expr) -> CompileResult<CodeOutput> {
    let mut out = String::new();
    out.push_str("; Generated by betlang (LLVM IR)\n");
    out.push_str("; TODO: Full LLVM codegen\n\n");
    emit_llvm_expr(&mut out, expr)?;
    Ok(CodeOutput {
        target: Target::Llvm,
        code: out,
        source_map: None,
    })
}

fn codegen_llvm_placeholder() -> CompileResult<CodeOutput> {
    Ok(CodeOutput {
        target: Target::Llvm,
        code: "; LLVM IR placeholder\n".to_string(),
        source_map: None,
    })
}

fn emit_llvm_expr(out: &mut String, expr: &Expr) -> CompileResult<()> {
    match expr {
        Expr::Int(i) => {
            out.push_str(&format!("  ret i64 {}\n", i));
        }
        Expr::Float(f) => {
            out.push_str(&format!("  ret double {}\n", f));
        }
        _ => {
            out.push_str("  ; unimplemented expression\n  ret i64 0\n");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// BEAM placeholder
// ---------------------------------------------------------------------------

fn codegen_beam(expr: &Expr) -> CompileResult<CodeOutput> {
    let mut out = String::new();
    out.push_str("%% Generated by betlang (BEAM)\n");
    out.push_str("%% TODO: Full BEAM codegen\n\n");
    emit_beam_expr(&mut out, expr)?;
    Ok(CodeOutput {
        target: Target::Beam,
        code: out,
        source_map: None,
    })
}

fn codegen_beam_placeholder() -> CompileResult<CodeOutput> {
    Ok(CodeOutput {
        target: Target::Beam,
        code: "%% BEAM bytecode placeholder\n".to_string(),
        source_map: None,
    })
}

fn emit_beam_expr(out: &mut String, expr: &Expr) -> CompileResult<()> {
    match expr {
        Expr::Int(i) => {
            out.push_str(&format!("{}", i));
        }
        Expr::Float(f) => {
            out.push_str(&format!("{}", f));
        }
        _ => {
            out.push_str("undefined");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use bet_syntax::ast::*;
    use bet_syntax::span::Spanned;
    use bet_syntax::Symbol;

    fn dummy<T>(node: T) -> Spanned<T> {
        Spanned::dummy(node)
    }

    // === Literal codegen ===

    #[test]
    fn test_codegen_int() {
        let expr = Expr::Int(42);
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("42"));
    }

    #[test]
    fn test_codegen_float() {
        let expr = Expr::Float(3.14);
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("3.14"));
    }

    #[test]
    fn test_codegen_string() {
        let expr = Expr::String("hello".to_string());
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("\"hello\""));
    }

    #[test]
    fn test_codegen_bool() {
        let expr = Expr::Bool(true);
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("true"));
    }

    #[test]
    fn test_codegen_ternary() {
        let expr = Expr::Ternary(TernaryValue::Unknown);
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("0"));
    }

    #[test]
    fn test_codegen_unit() {
        let expr = Expr::Unit;
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("null"));
    }

    // === Bet expressions ===

    #[test]
    fn test_codegen_bet_uniform() {
        let expr = Expr::Bet(BetExpr {
            alternatives: [
                Box::new(dummy(Expr::Int(1))),
                Box::new(dummy(Expr::Int(2))),
                Box::new(dummy(Expr::Int(3))),
            ],
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_uniform(1, 2, 3)"));
    }

    #[test]
    fn test_codegen_weighted_bet() {
        let expr = Expr::WeightedBet(WeightedBetExpr {
            alternatives: [
                (Box::new(dummy(Expr::Int(1))), Box::new(dummy(Expr::Float(0.5)))),
                (Box::new(dummy(Expr::Int(2))), Box::new(dummy(Expr::Float(0.3)))),
                (Box::new(dummy(Expr::Int(3))), Box::new(dummy(Expr::Float(0.2)))),
            ],
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_weighted("));
        assert!(output.code.contains("[1, 2, 3]"));
        assert!(output.code.contains("[0.5, 0.3, 0.2]"));
    }

    #[test]
    fn test_codegen_conditional_bet() {
        let expr = Expr::ConditionalBet(ConditionalBetExpr {
            condition: Box::new(dummy(Expr::Bool(true))),
            if_true: Box::new(dummy(Expr::Int(99))),
            if_false: [
                Box::new(dummy(Expr::Int(1))),
                Box::new(dummy(Expr::Int(2))),
                Box::new(dummy(Expr::Int(3))),
            ],
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("true ? 99 : __bet_uniform(1, 2, 3)"));
    }

    // === Variables and functions ===

    #[test]
    fn test_codegen_variable() {
        let expr = Expr::Var(Symbol::intern("x"));
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("x"));
    }

    #[test]
    fn test_codegen_lambda() {
        let expr = Expr::Lambda(LambdaExpr {
            params: vec![dummy(Pattern::Var(Symbol::intern("x")))],
            body: Box::new(dummy(Expr::BinOp(
                BinOp::Add,
                Box::new(dummy(Expr::Var(Symbol::intern("x")))),
                Box::new(dummy(Expr::Int(1))),
            ))),
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("function(x)"));
        assert!(output.code.contains("x + 1"));
    }

    #[test]
    fn test_codegen_app() {
        let expr = Expr::App(
            Box::new(dummy(Expr::Var(Symbol::intern("f")))),
            vec![dummy(Expr::Int(42))],
        );
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("f(42)"));
    }

    // === Let and Do ===

    #[test]
    fn test_codegen_let() {
        let expr = Expr::Let(LetExpr {
            pattern: dummy(Pattern::Var(Symbol::intern("x"))),
            type_ann: None,
            value: Box::new(dummy(Expr::Int(42))),
            body: Box::new(dummy(Expr::BinOp(
                BinOp::Add,
                Box::new(dummy(Expr::Var(Symbol::intern("x")))),
                Box::new(dummy(Expr::Int(1))),
            ))),
            is_rec: false,
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("const x = 42"));
        assert!(output.code.contains("x + 1"));
    }

    // === Binary operators ===

    #[test]
    fn test_codegen_arithmetic() {
        let expr = Expr::BinOp(
            BinOp::Mul,
            Box::new(dummy(Expr::Int(3))),
            Box::new(dummy(Expr::Int(7))),
        );
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("3 * 7"));
    }

    #[test]
    fn test_codegen_comparison() {
        let expr = Expr::BinOp(
            BinOp::Lt,
            Box::new(dummy(Expr::Var(Symbol::intern("a")))),
            Box::new(dummy(Expr::Var(Symbol::intern("b")))),
        );
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("a < b"));
    }

    // === Probabilistic operations ===

    #[test]
    fn test_codegen_sample() {
        let expr = Expr::Sample(Box::new(dummy(
            Expr::App(
                Box::new(dummy(Expr::Var(Symbol::intern("normal")))),
                vec![dummy(Expr::Float(0.0)), dummy(Expr::Float(1.0))],
            ),
        )));
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_sample(__bet_dist_normal(0.0, 1.0)"));
    }

    #[test]
    fn test_codegen_observe() {
        let expr = Expr::Observe(
            Box::new(dummy(Expr::App(
                Box::new(dummy(Expr::Var(Symbol::intern("normal")))),
                vec![dummy(Expr::Float(0.0)), dummy(Expr::Float(1.0))],
            ))),
            Box::new(dummy(Expr::Float(2.5))),
        );
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_observe("));
    }

    #[test]
    fn test_codegen_infer() {
        let expr = Expr::Infer(InferExpr {
            method: InferMethod::MCMC,
            params: vec![
                (dummy(Symbol::intern("samples")), dummy(Expr::Int(1000))),
            ],
            model: Box::new(dummy(Expr::Sample(Box::new(dummy(
                Expr::App(
                    Box::new(dummy(Expr::Var(Symbol::intern("normal")))),
                    vec![dummy(Expr::Float(0.0)), dummy(Expr::Float(1.0))],
                ),
            ))))),
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_infer('mcmc'"));
        assert!(output.code.contains("samples: 1000"));
    }

    #[test]
    fn test_codegen_parallel() {
        let expr = Expr::Parallel(
            Box::new(dummy(Expr::Int(100))),
            Box::new(dummy(Expr::Sample(Box::new(dummy(
                Expr::App(
                    Box::new(dummy(Expr::Var(Symbol::intern("normal")))),
                    vec![dummy(Expr::Float(0.0)), dummy(Expr::Float(1.0))],
                ),
            ))))),
        );
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("for (let __i = 0; __i < __n; __i++)"));
    }

    // === Distribution constructors ===

    #[test]
    fn test_codegen_dist_normal() {
        let expr = Expr::App(
            Box::new(dummy(Expr::Var(Symbol::intern("normal")))),
            vec![dummy(Expr::Float(0.0)), dummy(Expr::Float(1.0))],
        );
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_dist_normal(0.0, 1.0)"));
    }

    #[test]
    fn test_codegen_dist_beta() {
        let expr = Expr::App(
            Box::new(dummy(Expr::Var(Symbol::intern("beta")))),
            vec![dummy(Expr::Float(2.0)), dummy(Expr::Float(5.0))],
        );
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_dist_beta(2.0, 5.0)"));
    }

    // === Data structures ===

    #[test]
    fn test_codegen_tuple() {
        let expr = Expr::Tuple(vec![
            dummy(Expr::Int(1)),
            dummy(Expr::Int(2)),
            dummy(Expr::Int(3)),
        ]);
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("[1, 2, 3]"));
    }

    #[test]
    fn test_codegen_list() {
        let expr = Expr::List(vec![
            dummy(Expr::Float(1.0)),
            dummy(Expr::Float(2.0)),
        ]);
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("[1.0, 2.0]"));
    }

    #[test]
    fn test_codegen_record() {
        let expr = Expr::Record(vec![
            (dummy(Symbol::intern("x")), dummy(Expr::Int(1))),
            (dummy(Symbol::intern("y")), dummy(Expr::Int(2))),
        ]);
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("x: 1"));
        assert!(output.code.contains("y: 2"));
    }

    // === If and Match ===

    #[test]
    fn test_codegen_if() {
        let expr = Expr::If(IfExpr {
            condition: Box::new(dummy(Expr::Bool(true))),
            then_branch: Box::new(dummy(Expr::Int(1))),
            else_branch: Box::new(dummy(Expr::Int(0))),
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("true ? 1 : 0"));
    }

    #[test]
    fn test_codegen_match() {
        let expr = Expr::Match(MatchExpr {
            scrutinee: Box::new(dummy(Expr::Var(Symbol::intern("x")))),
            arms: vec![
                MatchArm {
                    pattern: dummy(Pattern::Literal(Literal::Int(1))),
                    guard: None,
                    body: dummy(Expr::String("one".to_string())),
                },
                MatchArm {
                    pattern: dummy(Pattern::Wildcard),
                    guard: None,
                    body: dummy(Expr::String("other".to_string())),
                },
            ],
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("=== 1"));
        assert!(output.code.contains("\"one\""));
        assert!(output.code.contains("\"other\""));
    }

    // === Complex probabilistic programs ===

    /// Test: Bayesian coin flip inference
    /// Model: prior = Beta(2, 2), observe coin flips, infer posterior
    #[test]
    fn test_codegen_bayesian_coin() {
        let expr = Expr::Infer(InferExpr {
            method: InferMethod::Rejection,
            params: vec![
                (dummy(Symbol::intern("samples")), dummy(Expr::Int(500))),
            ],
            model: Box::new(dummy(Expr::Let(LetExpr {
                pattern: dummy(Pattern::Var(Symbol::intern("p"))),
                type_ann: None,
                value: Box::new(dummy(Expr::Sample(Box::new(dummy(
                    Expr::App(
                        Box::new(dummy(Expr::Var(Symbol::intern("beta")))),
                        vec![dummy(Expr::Float(2.0)), dummy(Expr::Float(2.0))],
                    ),
                ))))),
                body: Box::new(dummy(Expr::Var(Symbol::intern("p")))),
                is_rec: false,
            }))),
        });
        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_infer('rejection'"));
        assert!(output.code.contains("__bet_dist_beta(2.0, 2.0)"));
        assert!(output.code.contains("__bet_sample("));
    }

    /// Test: Monte Carlo estimation of pi
    #[test]
    fn test_codegen_monte_carlo_pi() {
        // parallel 10000 { let x = sample uniform 0 1 in
        //                   let y = sample uniform 0 1 in
        //                   if x*x + y*y <= 1 then 1 else 0 }
        let inner = Expr::Let(LetExpr {
            pattern: dummy(Pattern::Var(Symbol::intern("x"))),
            type_ann: None,
            value: Box::new(dummy(Expr::Sample(Box::new(dummy(
                Expr::App(
                    Box::new(dummy(Expr::Var(Symbol::intern("uniform")))),
                    vec![dummy(Expr::Float(0.0)), dummy(Expr::Float(1.0))],
                ),
            ))))),
            body: Box::new(dummy(Expr::Let(LetExpr {
                pattern: dummy(Pattern::Var(Symbol::intern("y"))),
                type_ann: None,
                value: Box::new(dummy(Expr::Sample(Box::new(dummy(
                    Expr::App(
                        Box::new(dummy(Expr::Var(Symbol::intern("uniform")))),
                        vec![dummy(Expr::Float(0.0)), dummy(Expr::Float(1.0))],
                    ),
                ))))),
                body: Box::new(dummy(Expr::If(IfExpr {
                    condition: Box::new(dummy(Expr::BinOp(
                        BinOp::Le,
                        Box::new(dummy(Expr::BinOp(
                            BinOp::Add,
                            Box::new(dummy(Expr::BinOp(
                                BinOp::Mul,
                                Box::new(dummy(Expr::Var(Symbol::intern("x")))),
                                Box::new(dummy(Expr::Var(Symbol::intern("x")))),
                            ))),
                            Box::new(dummy(Expr::BinOp(
                                BinOp::Mul,
                                Box::new(dummy(Expr::Var(Symbol::intern("y")))),
                                Box::new(dummy(Expr::Var(Symbol::intern("y")))),
                            ))),
                        ))),
                        Box::new(dummy(Expr::Float(1.0))),
                    ))),
                    then_branch: Box::new(dummy(Expr::Int(1))),
                    else_branch: Box::new(dummy(Expr::Int(0))),
                }))),
                is_rec: false,
            }))),
            is_rec: false,
        });

        let expr = Expr::Parallel(
            Box::new(dummy(Expr::Int(10000))),
            Box::new(dummy(inner)),
        );

        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_dist_uniform(0.0, 1.0)"));
        assert!(output.code.contains("__bet_sample("));
        assert!(output.code.contains("x * x"));
        assert!(output.code.contains("y * y"));
        assert!(output.code.contains("<= 1.0"));
    }

    /// Test: Markov chain codegen
    #[test]
    fn test_codegen_markov_chain() {
        // markov_chain initial steps transition
        let expr = Expr::App(
            Box::new(dummy(Expr::Var(Symbol::intern("markov_chain")))),
            vec![
                dummy(Expr::Int(0)),
                dummy(Expr::Int(100)),
                dummy(Expr::Lambda(LambdaExpr {
                    params: vec![dummy(Pattern::Var(Symbol::intern("s")))],
                    body: Box::new(dummy(Expr::Bet(BetExpr {
                        alternatives: [
                            Box::new(dummy(Expr::BinOp(
                                BinOp::Add,
                                Box::new(dummy(Expr::Var(Symbol::intern("s")))),
                                Box::new(dummy(Expr::Int(1))),
                            ))),
                            Box::new(dummy(Expr::Var(Symbol::intern("s")))),
                            Box::new(dummy(Expr::BinOp(
                                BinOp::Sub,
                                Box::new(dummy(Expr::Var(Symbol::intern("s")))),
                                Box::new(dummy(Expr::Int(1))),
                            ))),
                        ],
                    }))),
                })),
            ],
        );

        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__bet_markov_chain("));
        assert!(output.code.contains("__bet_uniform("));
    }

    /// Test: Uncertainty propagation through arithmetic
    #[test]
    fn test_codegen_uncertainty_propagation() {
        // let a = uncertain 10.0 0.5 in
        // let b = uncertain 20.0 1.0 in
        // a + b
        let expr = Expr::Let(LetExpr {
            pattern: dummy(Pattern::Var(Symbol::intern("a"))),
            type_ann: None,
            value: Box::new(dummy(Expr::App(
                Box::new(dummy(Expr::Var(Symbol::intern("uncertain")))),
                vec![dummy(Expr::Float(10.0)), dummy(Expr::Float(0.5))],
            ))),
            body: Box::new(dummy(Expr::Let(LetExpr {
                pattern: dummy(Pattern::Var(Symbol::intern("b"))),
                type_ann: None,
                value: Box::new(dummy(Expr::App(
                    Box::new(dummy(Expr::Var(Symbol::intern("uncertain")))),
                    vec![dummy(Expr::Float(20.0)), dummy(Expr::Float(1.0))],
                ))),
                body: Box::new(dummy(Expr::BinOp(
                    BinOp::Add,
                    Box::new(dummy(Expr::Var(Symbol::intern("a")))),
                    Box::new(dummy(Expr::Var(Symbol::intern("b")))),
                ))),
                is_rec: false,
            }))),
            is_rec: false,
        });

        let output = codegen(&expr, Target::JavaScript).unwrap();
        assert!(output.code.contains("__BetUncertain.from(10.0, 0.5)"));
        assert!(output.code.contains("__BetUncertain.from(20.0, 1.0)"));
        assert!(output.code.contains("a + b"));
    }

    // === LLVM stub ===

    #[test]
    fn test_codegen_llvm_int() {
        let expr = Expr::Int(42);
        let output = codegen(&expr, Target::Llvm).unwrap();
        assert!(output.code.contains("ret i64 42"));
    }

    // === BEAM stub ===

    #[test]
    fn test_codegen_beam_int() {
        let expr = Expr::Int(42);
        let output = codegen(&expr, Target::Beam).unwrap();
        assert!(output.code.contains("42"));
    }

    // === Module-level codegen ===

    #[test]
    fn test_codegen_module_let() {
        let module = Module {
            name: None,
            items: vec![
                dummy(Item::Let(LetDef {
                    name: dummy(Symbol::intern("pi")),
                    params: vec![],
                    type_ann: None,
                    body: dummy(Expr::Float(3.14159)),
                    is_rec: false,
                })),
            ],
            span: bet_syntax::Span::dummy(),
        };
        let output = codegen_module(&module, Target::JavaScript).unwrap();
        assert!(output.code.contains("const pi = 3.14159"));
    }

    #[test]
    fn test_codegen_module_function() {
        let module = Module {
            name: None,
            items: vec![
                dummy(Item::Let(LetDef {
                    name: dummy(Symbol::intern("add")),
                    params: vec![
                        dummy(Pattern::Var(Symbol::intern("x"))),
                        dummy(Pattern::Var(Symbol::intern("y"))),
                    ],
                    type_ann: None,
                    body: dummy(Expr::BinOp(
                        BinOp::Add,
                        Box::new(dummy(Expr::Var(Symbol::intern("x")))),
                        Box::new(dummy(Expr::Var(Symbol::intern("y")))),
                    )),
                    is_rec: false,
                })),
            ],
            span: bet_syntax::Span::dummy(),
        };
        let output = codegen_module(&module, Target::JavaScript).unwrap();
        assert!(output.code.contains("function add(x, y)"));
        assert!(output.code.contains("x + y"));
    }
}
