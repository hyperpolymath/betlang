# SPDX-License-Identifier: PMPL-1.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

# BetLang System Specifications

BetLang is a probabilistic programming language designed for Bayesian inference
and stochastic modelling. Implementation stack: Rust compiler (LALRPOP parser),
Racket runtime for symbolic computation and inference.

---

## Memory Model

BetLang's memory model spans two runtime layers: the Rust compiler and the
Racket execution environment.

### Rust Compiler Layer

- The LALRPOP-generated parser produces an owned AST (`Box<Expr>`, `Vec<Stmt>`).
- All compiler passes (desugaring, type inference, IR generation) operate on
  owned data with standard Rust move semantics.
- No reference counting or garbage collection in the compiler pipeline.
- Intermediate representations are allocated on the heap and freed when each
  pass completes.

### Racket Runtime Layer

- Racket's precise generational garbage collector manages all runtime values.
- Probabilistic samples are boxed Racket values subject to GC.
- Distribution objects (Normal, Beta, Bernoulli, etc.) are Racket structs
  allocated on the managed heap.
- Trace storage for inference algorithms (particle histories, MCMC chains)
  uses Racket vectors, collected when inference completes.

### Probability Monad Storage

- Each probabilistic program executes within an inference monad that maintains:
  - **Log-weight accumulator**: running log-probability of the current trace.
  - **Sample store**: mapping from sample site names to drawn values.
  - **Observation log**: list of observed values with their likelihoods.
- The sample store is ephemeral per inference iteration and GC-eligible after.

---

## Concurrency Model

BetLang exploits the statistical independence of probabilistic samples to
enable safe parallelism.

### Probabilistic Parallelism

- **Particle-level parallelism**: In Sequential Monte Carlo (SMC) inference,
  each particle runs an independent copy of the program. Particles share no
  mutable state and can execute on separate Racket places.
- **Chain-level parallelism**: Multiple MCMC chains run independently. Each
  chain maintains its own sample store and log-weight.

### Racket Places

- Racket's `place` construct provides OS-thread-level parallelism with
  message-passing communication (no shared memory).
- Each inference particle or chain runs in its own place.
- Results are collected via place channels after inference completes.

### Synchronisation Points

- **Resampling barriers**: In SMC, particles synchronise at resampling steps.
  Weights are collected from all places, resampling occurs centrally, and
  new particle assignments are distributed.
- No locks or shared mutable state — all coordination is message-based.

---

## Effect System

BetLang's effect system tracks probabilistic operations as first-class effects
within an inference monad.

### Probabilistic Effects

| Effect        | Operation   | Description                                    |
|---------------|-------------|------------------------------------------------|
| `Sample`      | `sample d`  | Draw a value from distribution `d`             |
| `Observe`     | `observe d v` | Condition on value `v` under distribution `d`|
| `Condition`   | `condition b` | Hard constraint — reject trace if `b` is false|
| `Score`       | `score w`   | Manually adjust trace log-weight by `w`        |

### Inference Monad

- All probabilistic effects are interpreted by an inference backend.
- The monad signature: `Infer a = TraceState -> (a, TraceState, LogWeight)`.
- Different inference algorithms provide different effect handlers:
  - **Importance sampling**: `sample` draws from prior, `observe` updates weight.
  - **MH (Metropolis-Hastings)**: `sample` proposes from kernel, accept/reject.
  - **SMC**: `sample` draws from prior, `observe` triggers resampling.

### Deterministic Subset

- Programs using no probabilistic effects are pure and execute deterministically.
- The type system distinguishes `Pure a` from `Prob a` at the top level.
- Pure functions can be called from probabilistic contexts but not vice versa.

### Effect Composition

- Probabilistic effects compose with standard effects (IO, State) via monad
  transformers in the Racket runtime.
- IO effects are restricted to the outermost layer — inference internals are
  pure with respect to IO.

---

## Module System

BetLang uses a Racket-style module system for code organisation.

### Module Declaration

- Each file is a module: `#lang betlang` at the top.
- Exports are explicit: `(provide func1 func2 DistributionType)`.
- Imports use `(require "path/to/module.bet")` or `(require betlang/stdlib)`.

### Standard Library Modules

| Module                | Contents                                      |
|-----------------------|-----------------------------------------------|
| `betlang/distributions` | Normal, Beta, Bernoulli, Poisson, etc.      |
| `betlang/inference`   | SMC, MH, importance sampling, enumeration    |
| `betlang/combinators`  | Probabilistic combinators (mixture, product) |
| `betlang/plotting`     | Posterior visualisation utilities             |
| `betlang/data`         | Data loading and observation helpers         |

### Racket Interoperability

- BetLang modules can import standard Racket libraries via `(require racket/*)`.
- Racket modules can consume BetLang exports as standard Racket values.
- Distribution objects implement Racket's `gen:custom-write` for REPL display.

### Compiler Integration

- The Rust compiler (`betlang-compiler` crate) parses `.bet` files and emits
  Racket source (`.rkt`) for execution.
- Module resolution happens at the Racket level after compilation.
- Circular imports are detected and rejected by Racket's module system.
