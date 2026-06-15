<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
<!-- Tracks proof completion. Obligations defined in PROOF-NEEDS.md. -->
<!-- Aligned to the AffineScript estate PROOF-STATUS format. -->

# Proof Status — BetLang

See `docs/AFFINESCRIPT-ALIGNMENT.adoc` for the phased plan and
`docs/proof-debt.adoc` for the trusted-base reduction ledger.

## Summary

| Category | Total | Done | In Progress | Blocked | Remaining |
|----------|-------|------|-------------|---------|-----------|
| Typing / metatheory (TP)   | 6 | 5 | 0 | 0 | 1 |
| Semantics (SEM)            | 1 | 0 | 0 | 0 | 1 |
| Statistics (STAT)          | 2 | 0 | 0 | 0 | 2 |
| ABI / FFI (ABI)            | 5 | 0 | 0 | 0 | 5 |
| Concurrency (CONC)         | 1 | 0 | 0 | 0 | 1 |
| **Total**                  | **15** | **5** | **0** | **0** | **10** |

**Overall**: 33% proven (5 / 15). Lean core metatheory mechanised and
(as of Phase 1) machine-checked in CI.

## Proofs Done

| ID | Proof | Prover | File | Verified By |
|----|-------|--------|------|-------------|
| TP-1 | Progress — well-typed closed term is a value or steps | Lean4 | `proofs/BetLang.lean` | `lake build` (CI: `proofs.yml`) |
| TP-2 | Preservation — typing preserved under reduction | Lean4 | `proofs/BetLang.lean` | `lake build` (CI: `proofs.yml`) |
| TP-3 | Distribution monad laws (left id, right id, assoc) | Lean4 | `proofs/BetLang.lean` | `lake build` (CI: `proofs.yml`) |
| TP-4 | Discharge `substTop_preserves_typing` (de Bruijn subst lemma) — now a proved theorem, axiom-free | Lean4 | `proofs/BetLang.lean` | `lake build` (CI: `proofs.yml`) |
| TP-5 | Echo intro/elim metatheory — `echoIntro`/`echoElim` (echo_output) typing + β-rule + congruences + `canonical_echo`; Progress/Preservation re-established over the extended calculus | Lean4 | `proofs/BetLang.lean` | `lake build` (CI: `proofs.yml`) |

> Note: TP-2 is axiom-free. `substTop_preserves_typing`
> (`proofs/BetLang.lean:918`) is a fully proved `theorem` — the former
> classified axiom (TP-4) has been discharged (see `docs/proof-debt.adoc`
> §(a)). No `axiom` declarations remain.

## Proofs In Progress

| ID | Proof | Prover | Notes |
|----|-------|--------|-------|
| — | — | — | — |

## Proofs Blocked

| ID | Proof | Blocked By | Notes |
|----|-------|------------|-------|
| — | — | — | — |

## Proofs Remaining

| ID | Proof | Category | Prover | Phase | Priority |
|----|-------|----------|--------|-------|----------|
| TP-5b  | Richer echo surface ops in Lean — `echo_map`/`echo_duplicate`/`echo_to_residue`/`sample_echo` (compose from `echoIntro`/`echoElim`) + the comonad laws from `EchoGradedComonad.agda` | TP | Lean4 | 2 | P3 |
| SEM-1  | Continuous measure-theoretic denotational semantics           | SEM  | Lean4  | 2 | P2 |
| STAT-1 | Maximum entropy of uniform ternary = log₂3 bits               | STAT | Lean4  | 2 | P2 |
| STAT-2 | SLLN for bet sample means (a.s. convergence to expectation)   | STAT | Lean4  | 2 | P2 |
| ABI-1  | FFI non-null pointer safety                                   | ABI  | Idris2 | 3 | P1 |
| ABI-2  | FFI memory-layout correctness                                 | ABI  | Idris2 | 3 | P1 |
| ABI-3  | Platform type-size proofs                                     | ABI  | Idris2 | 3 | P1 |
| ABI-4  | Foreign return-type proofs                                    | ABI  | Idris2 | 3 | P1 |
| ABI-5  | C-ABI compliance                                             | ABI  | Idris2 | 3 | P1 |
| CONC-1 | Parallel bet-execution model                                 | CONC | TLA+   | 3 | P3 |

## Verification Commands

```bash
just proof-check-all      # banned-pattern scan + lake build
just proof-check-lean4    # machine-check the Lean 4 formalisation
just proof-scan           # banned-pattern gate only
```

## Banned Patterns

`sorry` / `admit` (Lean), `Admitted` (Coq), `postulate` / `believe_me` /
`assert_total` (Idris2/Agda), `unsafeCoerce`. Enforced by
`tools/proof-scan.sh` in CI (`.github/workflows/proofs.yml`). No `axiom`
declarations remain; any future classified `axiom` would be permitted by
policy (standards#203) and registered in `docs/proof-debt.adoc`.

## Changelog

| Date | Change | By |
|------|--------|-----|
| 2026-06-02 | Phase 1: Lean proofs made CI-machine-checked; status table created. | alignment branch |
| 2026-06-03 | Echo operations typed in `bet-check` (`echo`/`echo_output`/`echo_to_residue`/`sample_echo`); registered TP-5 for the Lean metatheory mirror. | echo-types pass |
| 2026-06-15 | TP-5 discharged: `echoIntro`/`echoElim` modelled in Lean (typing + β-rule + congruences + `canonical_echo`); Progress/Preservation re-established; zero `sorry`. Richer surface ops split to TP-5b. | TP-5 pass |
