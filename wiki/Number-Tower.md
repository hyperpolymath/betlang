<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath) -->
# The Uncertainty Number Tower

BetLang's "real moat" is that **uncertainty is not noise to be averaged away — it is a
first-class, typed object.** The language ships (target: 14) uncertainty-aware number
systems. These are not add-on libraries; they are *the type system of the language*.

| System | Captures |
|--------|----------|
| Gaussian | mean ± variance; closed-form propagation |
| Interval / affine arithmetic | guaranteed bounds; correlation tracking (affine) |
| Fuzzy numbers | membership-graded magnitude |
| Bayesian numbers | prior → posterior under evidence |
| Risk-based (VaR / CVaR) | tail risk |
| Surreal / hyperreal | infinitesimals & infinities |
| p-adic probability | non-Archimedean / hierarchical support |
| Imprecise probabilities | credal sets (lower/upper) |
| Dempster–Shafer belief functions | mass over the power set; belief vs plausibility |

(plus further systems converging on the target of 14.)

## Why a *tower*

Each system is an uncertainty *representation* with its own arithmetic; the tower is the
ordered family of these representations, with the type system tracking which one a value
inhabits and refusing nonsensical mixes. This is the numeric counterpart of what
[Echo types](Echo-Types) do for *loss*: both make an aspect of uncertainty that other
languages leave implicit into something the checker can see.

## Migration to Julia

Over time the tower's heavy arithmetic migrates to the Julia [compute kernel](Architecture)
(AbstractAlgebra.jl, IntervalArithmetic.jl, differentiable inference via Zygote/Enzyme),
while the *types* remain canonical in the Racket frontend.

## Contrast

R, Octave, Scilab, Mathematica and Maple can all *compute* with many of these
representations — but they do not give them **types** that the language enforces. A
Gaussian and an interval are just numbers/objects there; in BetLang they are distinct
typed inhabitants of the tower. See [Formal Verification](Formal-Verification) for the
broader comparison.

Reference: `docs/number-tower.md`.
