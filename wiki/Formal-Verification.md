<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# Formal Verification — and How BetLang Compares

BetLang's claim is not "a faster numeric kernel" — Julia, R, Octave, Scilab, Mathematica
and Maple are all formidable there. Its claim is **typed, proof-anchored uncertainty**:
the language's loss/uncertainty objects are *checked by a type system* and *grounded in
machine-checked metatheory*. This page covers both halves.

## The Lean 4 layer

The core calculus is mechanised in `proofs/BetLang.lean` as a first-class Lake project
(CI-verified by `.github/workflows/proofs.yml`):

| ID | Theorem | Status |
|----|---------|--------|
| TP-1 | **Progress** — a well-typed closed term is a value or steps | ✅ proved |
| TP-2 | **Preservation** — typing is preserved under reduction | ✅ proved |
| TP-3 | **Distribution monad laws** (left id, right id, assoc) | ✅ proved |
| TP-4 | Discharge the `substTop_preserves_typing` axiom | ✅ proved — **axiom-free core** |
| TP-5 | Echo-operation typing rules + metatheory (mirror `bet-check`) | ⏳ open |

**Zero `sorry`.** After TP-4 the only axiom dependencies are Lean 4 core
(`propext`, `Classical.choice`, `Quot.sound`). The trusted-base ledger lives in
`docs/proof-debt.adoc`; obligations are enumerated in `PROOF-NEEDS.md` and tracked in
`PROOF-STATUS.md`. A banned-pattern gate (`tools/proof-scan.sh`) forbids
`sorry`/`admit`/`postulate`/`believe_me`/`unsafeCoerce` in CI.

The Echo formers currently appear in Lean as *type formers only* (`Ty.echo`, `Ty.echoR`),
so Progress/Preservation are unaffected; mirroring the operational typing rules from
`bet-check` is the open obligation **TP-5**.

## How BetLang compares to the scientific/CAS languages

BetLang complements rather than competes with these tools — and the differentiator is the
type/proof layer, not raw numerics.

| System | Uncertainty model | Typed loss/uncertainty? | Machine-checked metatheory? | Relationship to BetLang |
|--------|-------------------|--------------------------|------------------------------|--------------------------|
| **Julia** | Distributions.jl, rich numerics | No (dynamic types) | No (the *companion* `EchoTypes.jl` is the executable echo shadow) | **Compute kernel** — BetLang's high-performance backend; `StatistEase` cross-checks |
| **R** | distributions, sampling | No | No | Statistics interchange; BetLang adds typed support-retention |
| **Octave / Scilab** | numeric matrices, Monte Carlo | No | No | Numeric kernels; no notion of typed information-loss |
| **Mathematica** | symbolic + probability | No (symbolic, untyped) | No | Symbolic CAS peer; BetLang is symbolic-*and*-typed-*and*-proved |
| **Maple** | symbolic + statistics | No | No | As Mathematica |

The recurring column is the point: **none of R/Octave/Scilab/Mathematica/Maple gives
uncertainty or information-loss a *type*, and none ships a machine-checked metatheory.**
BetLang does both:

- **Typed uncertainty** — the [Number Tower](Number-Tower) (14 systems: Gaussian, interval,
  fuzzy, Bayesian, VaR/CVaR, surreal, p-adic, imprecise, Dempster–Shafer, …) *is* the type
  system, and [Echo types](Echo-Types) type the *loss* itself.
- **Proof-anchored** — the Lean 4 metatheory above, plus the Echo formers' meaning being
  inherited from the machine-checked Agda [`echo-types`](https://github.com/hyperpolymath/echo-types).

So the honest framing: **Julia is where BetLang computes; the Agda `echo-types` spine is
where BetLang's loss types get their meaning; Lean is where BetLang's calculus is proved
sound.** R/Octave/Scilab/Mathematica/Maple are the numeric/symbolic incumbents BetLang
positions against by adding the typed, proof-backed layer they lack.

## See also

- [Echo Types](Echo-Types) — the structured-loss formers and operations
- [Type System](Type-System) — the Hindley–Milner core that hosts them
- `docs/proof-debt.adoc`, `PROOF-STATUS.md` — the trusted-base ledger
