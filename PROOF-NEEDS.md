<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
<!-- Defines proof obligations. Completion tracked in PROOF-STATUS.md. -->
<!-- Aligned to the AffineScript estate PROOF-NEEDS format. -->

# PROOF-NEEDS.md — BetLang

This file defines *what* must be proven. Completion is tracked in
`PROOF-STATUS.md`; the phased plan is in
`docs/AFFINESCRIPT-ALIGNMENT.adoc`; the trusted-base ledger is in
`docs/proof-debt.adoc`.

## Obligation Categories

| Code | Category | Default prover | Rationale |
|------|----------|----------------|-----------|
| TP   | Typing / metatheory | Lean4 | Core calculus soundness (progress/preservation, monad laws) |
| SEM  | Semantics | Lean4 | Operational ↔ denotational adequacy; continuous measure semantics |
| STAT | Statistics | Lean4 | Limit theorems, entropy bounds, MC convergence |
| ABI  | ABI / FFI | Idris2 | Rust/Julia FFI boundary safety (mandatory, mirrors AffineScript) |
| CONC | Concurrency | TLA+ | Parallel bet-execution model |

## Required Proofs

| ID | Obligation | Category | Prover | Priority | Status |
|----|------------|----------|--------|----------|--------|
| TP-1   | Progress (well-typed ⇒ value or steps)             | TP   | Lean4  | P1 | ✅ done |
| TP-2   | Preservation (typing preserved under step)          | TP   | Lean4  | P1 | ✅ done |
| TP-3   | Distribution monad laws (×3)                        | TP   | Lean4  | P1 | ✅ done |
| TP-4   | Discharge `substTop_preserves_typing` axiom         | TP   | Lean4  | P1 | ✅ done |
| TP-5   | Echo intro/elim typing rules + metatheory (Progress/Preservation re-established) | TP | Lean4 | P2 | ✅ done |
| TP-5b  | Richer echo surface ops (echo_map/echo_duplicate/echo_to_residue/sample_echo) + comonad laws | TP | Lean4 | P3 | remaining |
| SEM-1  | Continuous measure-theoretic denotation             | SEM  | Lean4  | P2 | remaining |
| STAT-1 | Maximum entropy of uniform ternary = log₂3          | STAT | Lean4  | P2 | remaining |
| STAT-2 | SLLN for bet sample means                           | STAT | Lean4  | P2 | remaining |
| ABI-1  | FFI non-null pointer safety                         | ABI  | Idris2 | P1 | remaining |
| ABI-2  | FFI memory-layout correctness                       | ABI  | Idris2 | P1 | remaining |
| ABI-3  | Platform type-size proofs                           | ABI  | Idris2 | P1 | remaining |
| ABI-4  | Foreign return-type proofs                          | ABI  | Idris2 | P1 | remaining |
| ABI-5  | C-ABI compliance                                    | ABI  | Idris2 | P1 | remaining |
| CONC-1 | Parallel bet-execution model                        | CONC | TLA+   | P3 | remaining |

## Banned Patterns

No `sorry` / `admit` (Lean), `Admitted` (Coq), `postulate` /
`believe_me` / `assert_total` (Idris2/Agda), `unsafeCoerce`. A single
**classified** `axiom` is permitted under the standards#203
trusted-base-reduction policy and must be registered in
`docs/proof-debt.adoc`. Enforced by `tools/proof-scan.sh`.

## How to Add a Proof

1. Choose the prover (see categories above).
2. Place the file in the correct home (Lean: `proofs/`; others:
   `proofs/<prover>/`, exposed via `verification/proofs/`).
3. Lean: rely on `lake build`; Idris2: `%default total`.
4. Run `just proof-check-all`.
5. Update `PROOF-STATUS.md` and, for any new escape hatch,
   `docs/proof-debt.adoc`.

---

## Historical Note — Template ABI Cleanup (2026-03-29)

Template ABI removed — was creating a false impression of formal
verification. The removed files (`Types.idr`, `Layout.idr`,
`Foreign.idr`) contained only RSR template scaffolding with unresolved
`{{PROJECT}}`/`{{AUTHOR}}` placeholders and no domain-specific proofs.

When this project needs formal ABI verification (obligations **ABI-1..5**
above, Phase 3), create domain-specific Idris2 proofs following the
pattern in repos like `typed-wasm`, `proven`, `echidna`, or
`boj-server`, and in the AffineScript estate
(`affinescript-vite/verification/proofs/idris2/ABI/`).
