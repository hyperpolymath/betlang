-- SPDX-License-Identifier: MPL-2.0
-- SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
--
-- Lake build for the BetLang Lean 4 formalisation.
--
-- The mechanisation is deliberately pure-core Lean 4 (no Mathlib): the
-- only soundness-relevant escape hatch is the single classified axiom
-- `substTop_preserves_typing` (see docs/proof-debt.adoc). Keeping the
-- dependency surface empty makes `lake build` fast and hermetic in CI.
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
