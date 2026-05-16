// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 hyperpolymath

import { assert, assertEquals, assertThrows } from '@std/assert';
import { betConditional, betWeighted, type Branch, expectation, rng } from '../src/probability.ts';
import { type Tri } from '../src/ternary.ts';

Deno.test('rng is deterministic for a fixed seed and stays in [0,1)', () => {
  const a = rng(123);
  const b = rng(123);
  for (let i = 0; i < 50; i++) {
    const x = a();
    assertEquals(x, b());
    assert(x >= 0 && x < 1);
  }
});

Deno.test('betWeighted respects weights within Monte-Carlo tolerance', () => {
  const branches: Branch<Tri>[] = [
    { weight: 0.6, value: () => 'T' },
    { weight: 0.3, value: () => 'U' },
    { weight: 0.1, value: () => 'F' },
  ];
  const draw = rng(42);
  const counts: Record<Tri, number> = { T: 0, U: 0, F: 0 };
  const N = 50_000;
  for (let i = 0; i < N; i++) counts[betWeighted(branches, draw)]++;
  assert(Math.abs(counts.T / N - 0.6) < 0.02);
  assert(Math.abs(counts.U / N - 0.3) < 0.02);
  assert(Math.abs(counts.F / N - 0.1) < 0.02);
});

Deno.test('betWeighted rejects non-positive total weight', () => {
  assertThrows(() => betWeighted([{ weight: 0, value: () => 1 }], rng(1)));
});

Deno.test('expectation converges to the analytic mean', () => {
  const payout: Branch<number>[] = [
    { weight: 1, value: () => 100 },
    { weight: 2, value: () => 10 },
    { weight: 7, value: () => 0 },
  ];
  // analytic EV = (1*100 + 2*10 + 7*0) / 10 = 12
  assert(Math.abs(expectation(payout, 100_000, rng(7)) - 12) < 0.5);
});

Deno.test('betConditional defers to the uncertain branch on Unknown', () => {
  assertEquals(
    betConditional(() => 'U', () => 'yes', () => 'maybe', () => 'no'),
    'maybe',
  );
  assertEquals(
    betConditional(() => 'T', () => 'yes', () => 'maybe', () => 'no'),
    'yes',
  );
});
