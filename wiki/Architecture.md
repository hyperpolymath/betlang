<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# Architecture

BetLang is a **multi-layer system** with clearly separated responsibilities. Each layer
has a single job, and the boundaries are deliberate.

| Layer | Language | Responsibility | Authority |
|-------|----------|----------------|-----------|
| **Frontend / spec** | Racket | `#lang betlang`; `syntax-parse` + nanopass IR; lazy ternary semantics | **Source of truth for semantics** |
| **Compute kernel** | Julia | High-performance numerical/statistical execution (Distributions.jl, StatsBase.jl, Random.jl); the uncertainty number tower migrates here | Performance path |
| **Proof / verification** | Lean 4 | Machine-checks Progress, Preservation, distribution monad laws; `lakefile.lean` + `proofs.yml` | Trust |
| **Compiler tooling** | Rust | `bet-core` (types incl. `Echo`/`EchoR`), `bet-check` (HM checker + unifier), `bet-parse` (LALRPOP), `bet-wasm` (paused) | Tooling only — **not** semantics |

## Why separate them

- **Racket is canonical** because the lazy ternary semantics (`(bet A B C)` evaluates
  only the chosen branch) are subtle and must have one unambiguous definition. The Rust
  checker and Lean proofs *model* that semantics; they do not redefine it.
- **Julia is the kernel** because the numeric/statistical workloads (sampling, inference,
  the number tower) want a mature scientific stack. BetLang drives Julia; it does not
  reimplement it. See [Formal Verification](Formal-Verification) for how this complements
  rather than competes with the scientific-computing incumbents.
- **Lean is firewalled** so the trusted base stays small and auditable
  (`docs/proof-debt.adoc`): zero `sorry`, axiom-free core.
- **Rust is tooling** (type-check, parse, future WASM) — convenient and fast, but
  explicitly *not* authoritative. The workspace `Cargo.toml` says as much at the top.

## Repository layout (orienting)

```
core/            Racket frontend — canonical semantics
lib/             Racket libraries (distributions, sampling, markov, risk, …)
compiler/        Rust tooling — bet-core / bet-check / bet-parse / bet-eval / bet-codegen / bet-wasm
proofs/          Lean 4 — BetLang.lean + theorems/ + papers/
verification/    RSR verification posture (coverage, safety_case, traceability)
docs/            Design docs (echo-types.adoc, number-tower, semantics, comparison)
.machine_readable/  6a2 state/meta + contractiles + self-validating (k9) + svc
```

See [Type System](Type-System) and [Echo Types](Echo-Types) for the `bet-check` layer in
depth, and [Roadmap](Roadmap) for what each layer still owes.
