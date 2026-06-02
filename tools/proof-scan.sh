#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
#
# proof-scan.sh — scan formal-proof sources for banned soundness escape
# hatches, mirroring the AffineScript estate's `panic-attack assail
# --proofs-only` gate. Shared by `just proof-scan` and CI (proofs.yml).
#
# Banned tokens (per AffineScript proofs README):
#   sorry, admit       — Lean 4
#   Admitted           — Coq
#   postulate          — Idris2 / Agda
#   believe_me         — Idris2
#   assert_total       — Idris2
#   unsafeCoerce       — Haskell / Lean
#
# NOTE: `axiom` is intentionally NOT banned. BetLang has exactly one
# classified necessary axiom (`substTop_preserves_typing`), triaged in
# docs/proof-debt.adoc under standards#203. AffineScript's banned list
# likewise permits explicit axioms.
#
# Comment-awareness: Lean line comments (`-- ...`) are stripped before
# matching so that prose like "all theorems are fully proved — no sorry"
# does not trip the gate. Only *code* occurrences fail.

set -uo pipefail

ROOT="${1:-.}"
BANNED='\b(sorry|admit|Admitted|postulate|believe_me|assert_total|unsafeCoerce)\b'

# Proof source extensions to scan.
mapfile -t FILES < <(
  find "$ROOT" \
    -path '*/.git' -prune -o \
    -path '*/.lake' -prune -o \
    -type f \( -name '*.lean' -o -name '*.idr' -o -name '*.agda' \
            -o -name '*.v' -o -name '*.tla' \) -print
)

if [ "${#FILES[@]}" -eq 0 ]; then
  echo "proof-scan: no proof sources found under '$ROOT' (nothing to check)"
  exit 0
fi

violations=0
for f in "${FILES[@]}"; do
  # Strip Lean/Idris/Agda line comments (-- ...) and Coq line comments
  # (after the file is read line-by-line) before matching.
  hits=$(sed -E 's/--.*$//' "$f" | grep -nE "$BANNED" || true)
  if [ -n "$hits" ]; then
    echo "✗ banned pattern in $f:"
    echo "$hits" | sed 's/^/    /'
    violations=$((violations + 1))
  fi
done

echo "----"
echo "proof-scan: scanned ${#FILES[@]} proof file(s)."
if [ "$violations" -ne 0 ]; then
  echo "proof-scan: FAILED — $violations file(s) contain banned patterns."
  exit 1
fi
echo "proof-scan: OK — no banned soundness escape hatches found."
