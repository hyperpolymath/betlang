<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# Roadmap

Tracked in `.machine_readable/6a2/STATE.a2ml`; proof obligations in `PROOF-NEEDS.md` /
`PROOF-STATUS.md`.

## Milestones

| ID | Milestone | State |
|----|-----------|-------|
| M1 | Proof foundation — `lakefile.lean` + `proofs.yml` CI + banned-pattern gate | ✅ done (#53) |
| M2 | Governance clean — licence, language policy, Racket tests, timeouts | ✅ done (#54) |
| M3 | Echo types core — `Type::Echo`/`EchoR`, unify, lower, Lean `Ty.echo`/`echoR` | ✅ done (#55) |
| M4 | Discharge `substTop_preserves_typing` axiom → **axiom-free core** | ✅ done (#40) |
| M5 | Echo operations — `echo`/`echo_output`/`echo_map`/`echo_duplicate`/`echo_to_residue` | ✅ done, type-level (#56) |
| M6 | `sample_echo` probabilistic bridge (type-level) / `bet_echo` runtime retention | 🟡 `sample_echo` done (#56); `bet_echo` runtime deferred |
| M7 | Julia backend Phase 2 — core language features | ⏳ active |

## Open proof obligations

- **TP-5** — Echo-operation typing rules + metatheory in Lean (mirror `bet-check`'s
  functor/comonad surface; re-establish Progress/Preservation over the echo terms).
- SEM-1 — continuous measure-theoretic denotation.
- STAT-1 / STAT-2 — max-entropy of uniform ternary = log₂3; SLLN for bet sample means.
- ABI-1…5 — FFI boundary safety (Idris2).
- CONC-1 — parallel bet-execution model (TLA+).

## Next engineering advances

1. **Harden `bet-check`** — add an occurs check to `unify`; replace the shared-variable
   pseudo-polymorphism with real type schemes (generalise/instantiate at let-boundaries).
   *(This is the immediate next work item.)*
2. **`bet_echo` runtime** — give the branch-collapse residue a runtime representation so
   Echo moves from ghost/type-level to operational.
3. **TP-5 in Lean** — mechanise the echo operations' metatheory.
4. **Julia backend Phase 2** — migrate the [number tower](Number-Tower) compute path.
5. **`bet-wasm`** — fix the pre-existing `E0308` match-arm mismatch; unpause the WASM backend.

## Known issues

- `bet-wasm` E0308: pre-existing WASM backend breakage (not caused by the echo work).
- `estate-standardization-20260607` branch is fully merged into `main` and should be
  deleted (owner UI action — git-proxy blocks branch deletion from CI environments).
