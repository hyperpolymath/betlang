<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# Type System

BetLang's type checker lives in the Rust crate `compiler/bet-check` (with core type
definitions in `bet-core`). It is a **Hindley–Milner-style** bidirectional checker with
unification, extended with the domain-specific formers that make BetLang what it is.

## Core types

`Unit`, `Bool`, `Ternary` (Kleene), `Int`, `Float`, `String`, `Bytes`, plus the compound
formers `Fun`, `Dist`, `List`, `Map`, `Set`, `Tuple`, `Option`, `Result`, and the
structured-loss formers **`Echo`** and **`EchoR`** (see [Echo Types](Echo-Types)).

## Key typing rules

- `bet { A B C }` — all three branches must unify to one type (only one is evaluated; see
  [Ternary Semantics](Ternary-Semantics)).
- `sample(dist)` — `Dist T → T`.
- `observe(dist, value)` — `Dist T` and `T`.
- `infer(method, model)` — model returns `Dist T`, result is `Dist T`.
- The **echo operations** — `echo`, `echo_output`, `echo_map`, `echo_duplicate`,
  `echo_to_residue`, `sample_echo` — are polymorphic builtins (see [Echo Types](Echo-Types)).

## Unification and the distinctness invariant

`unify` is structural and recurses through every former. The one rule that carries the
language's identity: **`Echo T` unifies only with `Echo T'` (iff `T` ~ `T'`), never with
the bare carrier `T`, and never with `EchoR T`.** `unify(Echo T, T)` *fails by design* —
that failure is the entire point of "retained loss". Distinctness is enforced both at the
former level and *through the operations* (e.g. `echo_output` applied to a bare `T` is
rejected).

## Polymorphism

Echo operations are instantiated with a **fresh** type variable at every use site, so a
single operation can be applied at many carrier types in one scope. (This is genuine
generalisation; the legacy `to_string` builtin uses a weaker shared-variable
approximation — replacing that with proper type schemes, and adding an occurs check to
`unify`, is tracked next; see [Roadmap](Roadmap).)

## Relationship to the other layers

The Rust checker is **tooling, not semantics** (see [Architecture](Architecture)). The
authoritative semantics are Racket's; the soundness guarantees are Lean's
([Formal Verification](Formal-Verification)). The checker's job is fast, helpful static
feedback consistent with both.

## Source

- `compiler/bet-core/src/types.rs` — the `Type` enum
- `compiler/bet-check/src/lib.rs` — `CheckEnv`, `unify`, `resolve`, `check_expr`, `echo_builtin_type`, tests
