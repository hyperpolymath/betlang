// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 hyperpolymath
//
// BetLang playground — uncertainty-aware number tower (sample).
//
// The full language exposes ~14 uncertainty number systems as its type
// system. This example demonstrates two of them — interval arithmetic and
// Gaussian (mean ± sd) propagation — and how the lazy ternary core lets a
// comparison return Unknown when two uncertain quantities overlap.

import { type Tri } from '../src/ternary.ts';

/** Closed real interval [lo, hi]. */
export interface Interval {
  lo: number;
  hi: number;
}

export const iv = (lo: number, hi: number): Interval => ({
  lo: Math.min(lo, hi),
  hi: Math.max(lo, hi),
});

export function addI(a: Interval, b: Interval): Interval {
  return iv(a.lo + b.lo, a.hi + b.hi);
}

export function mulI(a: Interval, b: Interval): Interval {
  const ps = [a.lo * b.lo, a.lo * b.hi, a.hi * b.lo, a.hi * b.hi];
  return iv(Math.min(...ps), Math.max(...ps));
}

/** Ternary comparison: definite when disjoint, Unknown when they overlap. */
export function ltI(a: Interval, b: Interval): Tri {
  if (a.hi < b.lo) return 'T';
  if (a.lo > b.hi) return 'F';
  return 'U';
}

/** Gaussian number: mean with standard deviation. */
export interface Gaussian {
  mu: number;
  sd: number;
}

export const gauss = (mu: number, sd: number): Gaussian => ({ mu, sd: Math.abs(sd) });

/** First-order (uncorrelated) propagation through sum and product. */
export function addG(a: Gaussian, b: Gaussian): Gaussian {
  return gauss(a.mu + b.mu, Math.hypot(a.sd, b.sd));
}

export function mulG(a: Gaussian, b: Gaussian): Gaussian {
  const mu = a.mu * b.mu;
  const rel = Math.hypot(a.sd / a.mu, b.sd / b.mu);
  return gauss(mu, Math.abs(mu) * rel);
}

function main(): void {
  console.log('=== BetLang Uncertainty Modeling ===\n');

  const a = iv(2, 4);
  const b = iv(3, 5);
  console.log(`Intervals: a=[${a.lo},${a.hi}]  b=[${b.lo},${b.hi}]`);
  console.log(`  a + b = [${addI(a, b).lo}, ${addI(a, b).hi}]`);
  console.log(`  a * b = [${mulI(a, b).lo}, ${mulI(a, b).hi}]`);
  console.log(`  a < b ? -> ${ltI(a, b)}  (overlap => Unknown, not a false certainty)`);
  console.log(`  [0,1] < [5,6] ? -> ${ltI(iv(0, 1), iv(5, 6))}`);

  const g1 = gauss(10, 1);
  const g2 = gauss(20, 2);
  const s = addG(g1, g2);
  const p = mulG(g1, g2);
  console.log(`\nGaussians: g1=${g1.mu}±${g1.sd}  g2=${g2.mu}±${g2.sd}`);
  console.log(`  g1 + g2 = ${s.mu}±${s.sd.toFixed(4)}`);
  console.log(`  g1 * g2 = ${p.mu}±${p.sd.toFixed(4)}`);
}

if (import.meta.main) {
  main();
}
