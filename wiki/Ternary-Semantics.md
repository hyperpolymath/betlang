<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Owner: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk> -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# The `bet` Primitive & Ternary Semantics

## The forms

| Form | Meaning |
|------|---------|
| `(bet A B C)` | primitive stochastic choice — uniform over three branches |
| `(bet/weighted '(A w1) '(B w2) '(C w3))` | non-uniform probabilities |
| `(bet/conditional pred A B C)` | predicate-driven selection |
| `(bet/lazy thunk-a thunk-b thunk-c)` | only the selected thunk is invoked |
| `(bet-with-seed seed thunk)` | deterministic seed for reproducibility |

## Laziness is load-bearing

Only the **chosen** branch is evaluated. This is not an optimisation detail — it is what
lets `bet` range over **symbolic and infinite structures** without forcing them, and it
is the property the Rust checker and the Lean calculus must both respect. In `bet-check`,
all three branches must *type-unify* (they share a result type) even though only one is
*evaluated*; in Lean, the `bet e₁ e₂ e₃` step relation reduces to one branch while typing
demands all three share a type.

## Two distinct notions of "three-valued"

BetLang is careful to separate:

1. **Ternary logic** — Kleene-style truth values `{true, unknown, false}` (the `Ternary`
   type). This is *logical* three-valuedness: `unknown` is a genuine truth value with its
   own conjunction/disjunction/negation tables.
2. **Ternary probabilistic belief** — a *distribution* over three outcomes. This is
   *epistemic* three-valuedness: a measure, not a truth value.

Conflating these is a category error. `Ternary` (Kleene) lives in the type system as a
primitive type; probabilistic belief lives in [the number tower](Number-Tower) and the
`Dist T` former. The distinction is formalised in `docs/ternary-semantics.md`.

## Why three (not two)

Per ADR-001: real-world decisions are rarely yes/no; the musical **ternary form (A–B–A)**
gives compositional structure; and **Dutch-book** semantics need at least three outcomes
to express the coherence constraints BetLang exists to enforce.

## Composition

Bets compose like algebraic objects — chaining, mapping, folding, higher-order
composition. A `bet` is a first-class value, so `(bet (bet a b c) x y)` (nested/entangled
choice) is ordinary. This compositionality is what makes BetLang a *metalanguage* for
uncertainty rather than a fixed simulation script. The collapse a `bet` performs is also
the canonical [Echo](Echo-Types) introduction site.
