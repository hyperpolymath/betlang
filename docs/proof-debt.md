<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Proof Debt — betlang

**Schema**: [`hyperpolymath/standards/docs/TRUSTED-BASE-REDUCTION-POLICY.adoc`](https://github.com/hyperpolymath/standards/blob/main/docs/TRUSTED-BASE-REDUCTION-POLICY.adoc)
— the canonical estate-wide policy (standards#203). Companion to the
strategic narrative in [`../PROOF-NEEDS.md`](../PROOF-NEEDS.md). Detection
script: [`scripts/check-trusted-base.sh`](https://github.com/hyperpolymath/standards/blob/main/scripts/check-trusted-base.sh)
(standards#211).

This file is the schema-conformant per-repo proof-debt index for
`hyperpolymath/betlang`. It enumerates every soundness-relevant escape
hatch in `proofs/BetLang.lean`.

## Marker count (2026-05-27)

`check-trusted-base.sh` reports **4 syntactic hits** under its Lean regex
(`\bsorry\b|^[[:space:]]*axiom[[:space:]]`); **1 is a real declaration**
and **3 are comment-line false positives** that the script's
`is_comment_line` heuristic strips. After comment-skip: **1 real marker**
— the single named axiom `substTop_preserves_typing`.

| File:line | Raw token | Real? | Disposition |
|---|---|---|---|
| `proofs/BetLang.lean:19`  | `sorry` (inside `-- All theorems are fully proved — no \`sorry\`.`) | false positive (comment) | n/a |
| `proofs/BetLang.lean:387` | `axiom` / `sorry` inside `-- IMPORTANT: This is NOT sorry — it is an axiom.` | false positive (comment) | n/a |
| `proofs/BetLang.lean:388` | `axioms` / `sorry` inside `-- axioms are explicit assumptions ... whereas sorry` | false positive (comment) | n/a |
| `proofs/BetLang.lean:392` | `axiom substTop_preserves_typing :` | **real** | §(d) DEBT |

## (a) DISCHARGED in this repo

*(None yet — entries move here when a marker is removed or replaced by a
total/proven counterpart.)*

## (b) BUDGETED — tested with a refutation budget

*(None. BetLang's proof file is pure Lean 4 metatheory; there is no
extraction-boundary surface needing a property-test budget.)*

## (c) NECESSARY AXIOM

*(None. The only real axiom in this repo (`substTop_preserves_typing`)
is **not** a metatheoretic assumption — it is a standard TAPL Ch.9
substitution-preservation lemma with a known constructive discharge.
It is therefore classified as §(d) DEBT, not §(c) NECESSARY.)*

## (d) DEBT — actively to be closed

### `proofs/BetLang.lean:392` — `axiom substTop_preserves_typing`

- **Statement**: `∀ (Γ : Ctx) (S T : Ty) (body v : Expr), HasType (S :: Γ) body T → HasType Γ v S → HasType Γ (substTop v body) T`.
- **Why it is §(d) and not §(c)**: this is the standard top-level
  substitution-preservation lemma (Pierce, TAPL Ch.9). It is constructively
  provable in Lean 4 by induction on the typing derivation, factored
  through a generalised `substAt_preserves_typing` and three
  context-insertion lookup lemmas. It does **not** encode a
  metatheoretic assumption (no funExt, no choice, no UIP). It only
  exists in the source because a full de Bruijn substitution calculus
  would, per the inline doc-comment at L383-385, "triple the file size"
  — a *cost* judgement, not a *necessity* judgement.
- **Owner**: @hyperpolymath
- **Plan** (verbatim from the discharge recipe in PR
  [#27](https://github.com/hyperpolymath/betlang/pull/27) body, lightly
  reformatted):
  1. **`Ctx.insertAt`** — define a context-insertion operator. (`Ctx`
     is a `List Ty` `abbrev`, so dot-notation `Γ.insertAt` does not
     resolve to anything useful; use a top-level
     `def Ctx.insertAt (Γ : Ctx) (k : Nat) (U : Ty) : Ctx :=
     Γ.take k ++ U :: Γ.drop k`.)
  2. **Three lookup lemmas**, all needing the `k ≤ Γ.length`
     hypothesis (without which they are false at empty Γ with k > 0):
     - `lookup_ctxInsertAt_lt : n < k → Ctx.lookup (ctxInsertAt Γ k U) n = Ctx.lookup Γ n`
     - `lookup_ctxInsertAt_eq : Ctx.lookup (ctxInsertAt Γ k U) k = some U`
     - `lookup_ctxInsertAt_gt : n > k → Ctx.lookup (ctxInsertAt Γ k U) n = Ctx.lookup Γ (n - 1)`
  3. **`shift_preserves_typing`** (weakening at `amount = 1`):
     `HasType Γ e T → ∀ k U, k ≤ Γ.length → HasType (ctxInsertAt Γ k U) (shift 1 k e) T`.
     Induction on `HasType`; binder cases (`tLam`, `tLet`) bump `k` by 1
     and recurse. Variable case needs `Int.toNat (↑n + 1) = n + 1`.
  4. **`subst_preserves_typing`** (generalised):
     `HasType (ctxInsertAt Γ k S) e T → HasType Γ v S → HasType (ctxInsertAt Γ k S) (subst k (shift (k+1 : Int) 0 v) e) T`.
     Induction on `e`. Binder cases recurse with `k+1` after pulling in
     the extra `shift 1 0`.
  5. **"No surviving var k" tracking lemma**: after `subst k v e`, define
     a predicate `freeVarNotIn k e` and prove it holds. Then prove
     `shift_down` preserves typing when this predicate holds.
  6. **Headline**:
     `substTop_preserves_typing Γ S T body v hb hv := ...` combining the
     chain. Then delete `axiom substTop_preserves_typing` at L392.
     Verify `preservation` still typechecks.
- **Estimated size**: ~300-400 LoC of standard TAPL Ch.9 mechanisation.
  The `Int.toNat` clamping in BetLang's `shift` (signed `amount`) adds
  non-trivial arithmetic friction at every `var` case.
- **Status**: a partial implementation lives on the local working
  branch `proofs/discharge-substTop-axiom-23` (commit `8fa128d`
  "proofs: discharge substTop_preserves_typing axiom (closes #23)"),
  not yet opened as a PR. Issue
  [#23](https://github.com/hyperpolymath/betlang/issues/23) was closed
  optimistically when PR #27 merged the build-fix half of the work;
  the axiom itself remained in source.
- **Deadline**: INDEFINITE — single-maintainer repo; discharge is bounded
  in scope but not currently scheduled. Re-open issue #23 (or file a
  successor) when work resumes.

## Cross-language survey (Idris2)

The repo also contains Idris2 test scaffolding under `tests/idris2/**`
and `playground/tests/idris2/**`. `check-trusted-base.sh` finds **zero**
`believe_me` / `really_believe_me` / `assert_total` / top-level
`partial` markers in those files. No §(b)/(c)/(d) entries needed.

## How to update this file

When markers in `proofs/BetLang.lean` (or any future proof-bearing file)
change:

1. Re-run `bash scripts/check-trusted-base.sh .` (the script is
   delivered from `hyperpolymath/standards` and called from the
   governance bundle).
2. If the count drops, move the resolved entries from §(d) → §(a).
3. If new markers appear, add them to §(d) with an owner + (possibly
   INDEFINITE) deadline.
4. The `trusted-base` CI job (`governance-reusable.yml`, landed in
   standards#211) will fail on un-annotated AND un-enumerated markers
   — so every change in source MUST be reflected here.

## Companion documents

- [`../PROOF-NEEDS.md`](../PROOF-NEEDS.md) — the strategic proof-debt
  narrative for betlang.
- [hyperpolymath/standards#203](https://github.com/hyperpolymath/standards/pull/203)
  — the policy this file conforms to.
- [hyperpolymath/standards#211](https://github.com/hyperpolymath/standards/pull/211)
  — `scripts/check-trusted-base.sh` and the `trusted-base` job.
- [hyperpolymath/standards#222](https://github.com/hyperpolymath/standards/pull/222)
  — the fill-in PR for the policy doc.
- [hyperpolymath/betlang#23](https://github.com/hyperpolymath/betlang/issues/23)
  — the (closed) tracking issue for discharging
  `substTop_preserves_typing`.
- [hyperpolymath/betlang#27](https://github.com/hyperpolymath/betlang/pull/27)
  — the merged PR whose body contains the canonical discharge recipe.

---

Initial seed by Claude Code, 2026-05-27, applying the standards#203
schema to betlang per the standards#222 per-repo triage pass.
