<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# BetLang Wiki

**BetLang** is a *Symbolic Probabilistic Metalanguage* — a **probabilistic computer
algebra system (CAS)**, not a betting language. Its one idea:

> Computation is structured choice under uncertainty.

The fundamental primitive is the ternary form `(bet A B C)` — a probabilistic, **lazy**
choice between three branches (only the selected branch is evaluated). On top of that
sit two things that distinguish BetLang from every mainstream numeric/CAS tool: a
**type system that treats uncertainty and information-loss as first-class typed objects**
(the uncertainty number tower + **Echo types**), and a **machine-checked metatheory**
(Lean 4).

## Map of this wiki

| Page | What it covers |
|------|----------------|
| [Architecture](Architecture) | The four-layer system: Racket (authoritative semantics) · Julia (compute kernel) · Lean 4 (proofs) · Rust (compiler tooling) |
| [The `bet` Primitive & Ternary Semantics](Ternary-Semantics) | `bet` / `bet/weighted` / `bet/conditional` / `bet/lazy`; Kleene ternary logic vs ternary probabilistic belief |
| [Type System](Type-System) | Hindley–Milner core in `bet-check`; the Echo/`EchoR` structured-loss formers and their operations |
| [Echo Types](Echo-Types) | Structured loss as a typed object; the functor/comonad surface; the cross-repo Agda proof anchor |
| [Formal Verification](Formal-Verification) | Lean 4 Progress / Preservation / monad laws; the proof-debt ledger; **how BetLang compares to Julia, R, Octave, Mathematica, Maple, Scilab** |
| [Uncertainty Number Tower](Number-Tower) | The 14 uncertainty-aware number systems (Gaussian, interval, fuzzy, Bayesian, p-adic, Dempster–Shafer, …) |
| [Roadmap](Roadmap) | Milestones M1–M7, open proof obligations, next advances |

## Status (2026-06-13)

| Layer | State |
|-------|-------|
| Racket frontend | ✅ Authoritative semantics |
| Lean 4 proofs | ✅ Progress + Preservation + monad laws; **axiom-free core** (TP-4 discharged) |
| Rust type-checker (`bet-check`) | ✅ HM + Echo formers **+ typed echo operations** (functor/comonad surface) |
| Julia compute backend | 🟡 Active development |
| `bet-wasm` backend | ⏸️ Paused (pre-existing build issue) |

## Ecosystem position

BetLang is the **applied probabilistic consumer** in a three-repo Echo-types spine:
- [`echo-types`](https://github.com/hyperpolymath/echo-types) — Agda, the **source of truth** (constructive, `--safe --without-K`).
- [`EchoTypes.jl`](https://github.com/hyperpolymath/EchoTypes.jl) — Julia, the **executable finite-domain shadow**.
- **BetLang** — borrows the unary `Echo T` / `EchoR T` formers as a *typed discipline* over probabilistic loss.

License: **Palimpsest License (SPDX: MPL-2.0)**.
