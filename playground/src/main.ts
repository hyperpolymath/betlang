// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2025 hyperpolymath
//
// BetLang playground entry point (`deno task dev` / `just dev`).
//
// Runs the ternary core demo and the probabilistic layer demo in sequence,
// giving a contributor a single runnable surface for the sandbox.

import { main as ternaryDemo } from './ternary.ts';
import { main as probabilityDemo } from './probability.ts';

function main(): void {
  console.log('BetLang Playground — Symbolic Probabilistic Metalanguage\n');
  ternaryDemo();
  console.log('\n' + '-'.repeat(60) + '\n');
  probabilityDemo();
  console.log('\nDone. See `just --list` for individual demos.');
}

if (import.meta.main) {
  main();
}
