<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# Echo Types — Structured Loss as a Typed Object

When a computation collapses information — a `(bet A B C)` picks one branch, a `sample`
marginalises away which draw fired — the *fact of what was lost* normally vanishes.
**Echo types** make that residue a first-class, typed object.

## The idea (and where it comes from)

Upstream, in the Agda repo [`echo-types`](https://github.com/hyperpolymath/echo-types)
(the **source of truth**), the echo at `y : B` for a map `f : A → B` is the fibre

```
Echo f y  :=  Σ (x : A), (f x ≡ y)
```

— a *proof-relevant* record of which inputs collapsed onto `y`. BetLang is
**non-dependent**, so it does not represent the literal fibre. Instead it adds two
**unary type formers**:

| Former | Meaning |
|--------|---------|
| `Echo T`  | A `T`-value carrying a proof-relevant residue of retained loss. **Distinct from `T`.** |
| `EchoR T` | The strict, non-recoverable residue of `Echo T`. |

The load-bearing invariant: **`Echo T` is not `T`.** `unify(Echo T, T)` fails *by design*
(and so does `unify(Echo T, EchoR T)`). There is **no implicit forgetting** `Echo T → T`;
you must ask for the value back explicitly. If `Echo T` unified with `T`, the checker
would lose the entire point of retained loss.

## Operations (the functor + comonad surface)

`bet-check` makes the formers *operational* via polymorphic builtins (each instantiated
with a fresh carrier per use site, so they are genuinely generic). Surface syntax is
ordinary application — `echo(x)` — so no grammar change was needed.

| Operation | Typing rule | Role |
|-----------|-------------|------|
| `echo`            | `'a → Echo 'a`              | introduction (`echo-intro`) |
| `echo_output`     | `Echo 'a → 'a`              | **explicit** projection — the comonad **counit** |
| `echo_map`        | `('a → 'b) → Echo 'a → Echo 'b` | functor action (`map-over`) |
| `echo_duplicate`  | `Echo 'a → Echo (Echo 'a)`  | comonad comultiplication |
| `echo_to_residue` | `Echo 'a → EchoR 'a`        | lower a full echo to its strict residue |
| `sample_echo`     | `Dist 'a → Echo 'a`         | probabilistic-support bridge — retains what `sample` discards |

Together `echo_map` / `echo_output` / `echo_duplicate` give `Echo` the **functor +
comonad** structure. This is the *ungraded, ghost shadow* of the **graded comonad of
structured loss** proved upstream (`EchoGradedComonad.agda`: `gextract` / `gduplicate` /
coassoc). BetLang carries the *typing*; the *laws* live in the Agda (and are tracked for
Lean as obligation **TP-5**).

## Ghost / erased

`Echo T` and `EchoR T` **erase to `T` at runtime** — no residue payload is materialised
yet. The value today is the *typed discipline* (you cannot accidentally forget; forgetting
is explicit and visible in types), plus the cross-repo anchoring to machine-checked
semantics. A future pass may give the residue a runtime representation (branch tags,
support traces); until then BetLang deliberately avoids premature commitment.

## The `bet` bridge

The core primitive bridges to `Echo` *by composition*, with no new machinery:
`echo(bet a b c) : Echo T` already type-checks — viewing the branch-collapse as a
retained-loss site. A dedicated `bet_echo` is only needed for **runtime** branch-tag
retention (deferred).

## Why this matters

`Echo` turns "irreversible computation" into "reversible *representation*": the collapse
still happens, but the type remembers that it happened and refuses to silently pretend it
didn't. That is the seed of provenance, auditability, and differential/▢-style reasoning
in a probabilistic setting — see [Formal Verification](Formal-Verification) for how this
slots into BetLang's proof story and how it sets BetLang apart from untyped numeric tools.

## Source

- `compiler/bet-core/src/types.rs` — `Type::Echo`, `Type::EchoR`
- `compiler/bet-check/src/lib.rs` — `echo_builtin_type`, unification distinctness, tests
- `docs/echo-types.adoc` — full design rationale
- `proofs/BetLang.lean` — `Ty.echo` / `Ty.echoR` (formers; operations' metatheory is TP-5)
