<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->

[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-pink?logo=github)](https://github.com/sponsors/hyperpolymath)
[![License: MPL-2.0](https://img.shields.io/badge/License-MPL--2.0-blue.svg)](LICENSE)

# BetLang

**A Symbolic Probabilistic Metalanguage / Probabilistic CAS**

BetLang is a minimal ternary DSL hosted in Racket for *symbolic probabilistic computation*.
Its core primitive is a three-way stochastic choice, supported by a Lean 4–mechanised type
system and a Rust compiler front-end.

> Computation is structured choice under uncertainty.

For the full documentation see **[README.adoc](README.adoc)** and **[EXPLAINME.adoc](EXPLAINME.adoc)**.

---

## Core Primitive

```scheme
(bet A B C)            ;; uniform ternary choice
(bet/weighted '(A 7) '(B 2) '(C 1))  ;; non-uniform
(bet/lazy thunk-a thunk-b thunk-c)    ;; only selected thunk runs
(bet-with-seed 42 (lambda () ...))    ;; reproducible
```

The selected branch is the only branch evaluated (lazy semantics).

---

## Architecture

| Layer | Role | Status |
|-------|------|--------|
| **Racket** | Language and canonical semantics | ✅ Active |
| **Lean 4** | Mechanised proofs (Progress + Preservation + monad laws) | ✅ Machine-checked |
| **Rust** | Type checker + compiler (`bet-check`, `bet-core`) | ✅ Active |
| **Julia** | High-performance compute backend | 🟡 Development |

---

## Echo Types

BetLang's type system includes structured-loss formers from
[`hyperpolymath/echo-types`](https://github.com/hyperpolymath/echo-types) (Agda source of truth):

| Type | Meaning |
|------|---------|
| `Echo T` | `T`-value with proof-relevant retained-loss residue. **Distinct from `T`.** |
| `EchoR T` | Strict non-recoverable residue. Reserved; operations deferred. |

`unify(Echo T, T)` fails by design. Both types are ghost-erased at runtime until operations
demand a payload. See [docs/echo-types.adoc](docs/echo-types.adoc).

---

## Proofs

`proofs/BetLang.lean` machine-checks Progress, Preservation, and monad laws with **0 `sorry`**.

- `lakefile.lean` + `lean-toolchain` → buildable Lake project
- `.github/workflows/proofs.yml` → CI-checked on every PR (`lake build` + banned-pattern scan)
- Axiom-free: `substTop_preserves_typing` is fully proved — no `axiom`/`sorry` (see `docs/proof-debt.adoc`)

---

## Status

| Component | Status | Notes |
|-----------|--------|-------|
| Racket frontend | ✅ Authoritative | Canonical semantics |
| Lean 4 proofs | ✅ Machine-checked | Progress + Preservation + monad laws |
| Rust type-checker | ✅ Active | bet-check / bet-core (incl. Echo T) |
| Julia backend | 🟡 Development | Core language features |
| VS Code extension | 🟡 In progress | AffineScript source |
| WASM backend | ⏸️ Paused | Pre-existing build issue |

---

## Quick Start

```bash
# Core Racket DSL
racket tests/basics.rkt

# Proofs
lake build     # requires Lean 4 (see lean-toolchain)

# Rust type-checker
cargo test -p bet-check

# All via just
just proof-check-all
```

---

## Language Policy

- **No TypeScript** outside `playground/` (approved sandbox exemption, see `.claude/CLAUDE.md`)
- **No Python, Go, Java, Kotlin, Swift** — see full policy in `.claude/CLAUDE.md`
- **AffineScript** replaces TypeScript/ReScript for editor tooling
- **Deno** replaces Node/npm
- All files must have SPDX `MPL-2.0` headers

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). For a guided tour, read [EXPLAINME.adoc](EXPLAINME.adoc) first.

---

## License

BetLang is licensed under **MPL-2.0**. SPDX identifier: `MPL-2.0`.

See [LICENSE](LICENSE) and [PALIMPSEST.adoc](PALIMPSEST.adoc).

---

## Links

- [EXPLAINME.adoc](EXPLAINME.adoc) — guided tour for contributors and agents
- [docs/echo-types.adoc](docs/echo-types.adoc) — Echo Types design
- [docs/AFFINESCRIPT-ALIGNMENT.adoc](docs/AFFINESCRIPT-ALIGNMENT.adoc) — alignment plan
- [.machine_readable/6a2/ECOSYSTEM.a2ml](.machine_readable/6a2/ECOSYSTEM.a2ml) — ecosystem position
