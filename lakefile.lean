-- SPDX-License-Identifier: MPL-2.0
-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
--
-- Lake build for the BetLang Lean 4 formalisation.
--
-- The mechanisation is deliberately pure-core Lean 4 (no Mathlib) and
-- AXIOM-FREE: the former classified axiom `substTop_preserves_typing` has
-- been discharged to a proved `theorem` (TP-4; see docs/proof-debt.adoc), so
-- no `axiom` declarations remain. Every theorem — including the echo-comonad
-- metatheory (TP-5) — depends only on the standard Lean kernel axioms.
-- Keeping the dependency surface empty makes `lake build` fast and hermetic
-- in CI.
--
-- The canonical source lives at `proofs/BetLang.lean` (referenced by
-- docs/proof-debt.adoc); we point the library root there rather than
-- moving the file.

import Lake
open Lake DSL

package betlang where
  -- no external dependencies; core Lean 4 only

@[default_target]
lean_lib BetLang where
  srcDir := "proofs"
  roots := #[`BetLang]
