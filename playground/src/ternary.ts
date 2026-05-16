// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 hyperpolymath
//
// BetLang playground — ternary core.
//
// BetLang's minimal core is a Kleene strong three-valued logic over
// { True, False, Unknown } plus the lazy ternary choice primitive
//   (bet A B C)
// where only the selected branch is ever evaluated.
//
// The AND truth table here is exactly the one documented in the playground
// Justfile (`just ternary-demo`): conjunction is `min`, disjunction is `max`
// under the order  F < U < T.

/** The three logical values of the BetLang core. */
export type Tri = 'T' | 'F' | 'U';

const ORDER: Record<Tri, number> = { F: 0, U: 1, T: 2 };
const BY_RANK: Tri[] = ['F', 'U', 'T'];

/** Kleene negation: swaps T/F, fixes U. */
export function not(a: Tri): Tri {
  if (a === 'T') return 'F';
  if (a === 'F') return 'T';
  return 'U';
}

/** Kleene conjunction = min under F < U < T. */
export function and(a: Tri, b: Tri): Tri {
  return BY_RANK[Math.min(ORDER[a], ORDER[b])];
}

/** Kleene disjunction = max under F < U < T. */
export function or(a: Tri, b: Tri): Tri {
  return BY_RANK[Math.max(ORDER[a], ORDER[b])];
}

/** Material implication, defined as `or(not(a), b)`. */
export function implies(a: Tri, b: Tri): Tri {
  return or(not(a), b);
}

/**
 * The lazy ternary choice primitive `(bet A B C)`.
 *
 * Branches are passed as thunks; exactly one is forced. `selector` decides
 * which branch wins — when it returns 'U' the middle branch is taken, which
 * is what makes the choice *total* even under uncertainty.
 */
export function bet<A>(
  selector: () => Tri,
  onTrue: () => A,
  onUnknown: () => A,
  onFalse: () => A,
): A {
  switch (selector()) {
    case 'T':
      return onTrue();
    case 'F':
      return onFalse();
    default:
      return onUnknown();
  }
}

/** Render a full binary truth table for a Tri operator. */
function table(name: string, op: (a: Tri, b: Tri) => Tri): string {
  const vals: Tri[] = ['T', 'U', 'F'];
  const rows = vals.flatMap((a) => vals.map((b) => `  ${a} ${name} ${b} = ${op(a, b)}`));
  return [`${name} truth table:`, ...rows].join('\n');
}

export function main(): void {
  console.log('=== BetLang Ternary Core ===');
  console.log('Values: True (T), False (F), Unknown (U)\n');
  console.log(table('AND', and));
  console.log();
  console.log(table('OR', or));
  console.log();
  console.log('NOT: NOT T = ' + not('T') + ', NOT U = ' + not('U') + ', NOT F = ' + not('F'));
  console.log();

  // Laziness demonstration: only the selected thunk runs.
  let evaluated = '';
  const result = bet(
    () => 'U',
    () => {
      evaluated = 'true-branch';
      return 1;
    },
    () => {
      evaluated = 'unknown-branch';
      return 0;
    },
    () => {
      evaluated = 'false-branch';
      return -1;
    },
  );
  console.log(`Lazy (bet ? : :) on Unknown -> result=${result}, evaluated=${evaluated}`);
  console.log('(only the Unknown branch ran; True/False thunks were never forced)');
}

if (import.meta.main) {
  main();
}
