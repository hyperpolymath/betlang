// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 hyperpolymath
//
// BetLang playground — probabilistic layer.
//
// Builds the non-uniform / predicate-driven choice forms on top of the
// lazy ternary core:
//   (bet/weighted ...)     — non-uniform probabilities
//   (bet_conditional ...)  — predicate-driven selection
//
// Evaluation stays lazy: a Bet is a *description* of a choice; only the
// branch that wins a draw is forced.

import { bet, type Tri } from './ternary.ts';

/** A lazy, weighted branch: a relative weight and a thunk producing a value. */
export interface Branch<A> {
  weight: number;
  value: () => A;
}

/** Deterministic, seedable PRNG (mulberry32) so demos/tests are reproducible. */
export function rng(seed: number): () => number {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** Force exactly one branch, chosen with probability proportional to weight. */
export function betWeighted<A>(branches: Branch<A>[], draw: () => number): A {
  const total = branches.reduce((acc, b) => acc + b.weight, 0);
  if (total <= 0) throw new Error('betWeighted: weights must sum to a positive number');
  let r = draw() * total;
  for (const b of branches) {
    r -= b.weight;
    if (r <= 0) return b.value();
  }
  return branches[branches.length - 1].value(); // float-rounding fallback
}

/**
 * Predicate-driven ternary selection. The predicate yields a Tri; a definite
 * answer takes the matching branch, Unknown defers to the `uncertain` branch.
 */
export function betConditional<A>(
  predicate: () => Tri,
  ifTrue: () => A,
  uncertain: () => A,
  ifFalse: () => A,
): A {
  return bet(predicate, ifTrue, uncertain, ifFalse);
}

/** Monte-Carlo expectation of a numeric weighted bet over `n` samples. */
export function expectation(branches: Branch<number>[], n: number, draw: () => number): number {
  let sum = 0;
  for (let i = 0; i < n; i++) sum += betWeighted(branches, draw);
  return sum / n;
}

export function main(): void {
  console.log('=== BetLang Probabilistic Layer ===\n');

  // A loaded three-sided "coin": 60% True, 30% Unknown, 10% False.
  const loaded: Branch<Tri>[] = [
    { weight: 0.6, value: () => 'T' as Tri },
    { weight: 0.3, value: () => 'U' as Tri },
    { weight: 0.1, value: () => 'F' as Tri },
  ];
  const draw = rng(42);
  const counts: Record<Tri, number> = { T: 0, U: 0, F: 0 };
  const N = 100_000;
  for (let i = 0; i < N; i++) counts[betWeighted(loaded, draw)]++;
  console.log(`Empirical distribution over ${N.toLocaleString()} draws (target 0.60/0.30/0.10):`);
  console.log(
    `  T=${(counts.T / N).toFixed(3)}  U=${(counts.U / N).toFixed(3)}  ` +
      `F=${(counts.F / N).toFixed(3)}`,
  );

  // Expected payout of a weighted numeric bet.
  const payout: Branch<number>[] = [
    { weight: 1, value: () => 100 },
    { weight: 2, value: () => 10 },
    { weight: 7, value: () => 0 },
  ];
  const ev = expectation(payout, 200_000, rng(7));
  console.log(`\nExpected payout (analytic = 12.0): ${ev.toFixed(2)}`);

  // Conditional choice that stays total under Unknown.
  const choice = betConditional(
    () => 'U' as Tri,
    () => 'committed',
    () => 'hedged (predicate was Unknown)',
    () => 'declined',
  );
  console.log(`\nbet_conditional under Unknown -> ${choice}`);
}

if (import.meta.main) {
  main();
}
