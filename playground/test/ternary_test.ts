// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 hyperpolymath

import { assertEquals } from '@std/assert';
import { and, bet, implies, not, or, type Tri } from '../src/ternary.ts';

const ALL: Tri[] = ['T', 'U', 'F'];

Deno.test('negation is involutive and fixes Unknown', () => {
  assertEquals(not('T'), 'F');
  assertEquals(not('F'), 'T');
  assertEquals(not('U'), 'U');
  for (const v of ALL) assertEquals(not(not(v)), v);
});

Deno.test('AND matches the documented Justfile truth table (min)', () => {
  assertEquals(and('T', 'T'), 'T');
  assertEquals(and('T', 'U'), 'U');
  assertEquals(and('T', 'F'), 'F');
  assertEquals(and('U', 'U'), 'U');
  assertEquals(and('U', 'F'), 'F');
  assertEquals(and('F', 'F'), 'F');
});

Deno.test('AND and OR are commutative; De Morgan holds', () => {
  for (const a of ALL) {
    for (const b of ALL) {
      assertEquals(and(a, b), and(b, a));
      assertEquals(or(a, b), or(b, a));
      assertEquals(not(and(a, b)), or(not(a), not(b)));
    }
  }
});

Deno.test('implies(a,b) == or(not a, b)', () => {
  for (const a of ALL) {
    for (const b of ALL) assertEquals(implies(a, b), or(not(a), b));
  }
});

Deno.test('bet is lazy: only the selected branch is forced', () => {
  const forced: string[] = [];
  const r = bet(
    () => 'F',
    () => {
      forced.push('T');
      return 1;
    },
    () => {
      forced.push('U');
      return 2;
    },
    () => {
      forced.push('F');
      return 3;
    },
  );
  assertEquals(r, 3);
  assertEquals(forced, ['F']);
});
