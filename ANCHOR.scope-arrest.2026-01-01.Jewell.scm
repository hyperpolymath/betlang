;; ANCHOR.scope-arrest.2026-01-01.Jewell.scm  (betlang)
;; SPDX-FileCopyrightText: 2026 hyperpolymath
;; SPDX-License-Identifier: PMPL-1.0-or-later
;;
;; Scope arrest anchor for betlang project.
;; Purpose: Lock semantics; stop identity drift; enable future backends without confusion.

(define anchor
  '((schema . "hyperpolymath.anchor/1")
    (repo . "hyperpolymath/betlang")
    (date . "2026-01-01")
    (authority . "repo-superintendent")
    (purpose . ("Lock semantics; stop identity drift; enable future backends without confusion."))
    (identity
      . ((project . "Betlang")
         (kind . "probabilistic language / DSL")
         (domain . "ternary bet operator + weighted/conditional variants")
         (one-sentence . "A probabilistic language centered on a ternary bet primitive and compositional inference primitives.")))

    (semantic-anchor
      . ((policy . "dual")
         (reference-impl . ("Racket interpreter is authoritative"))
         (formal-spec . ("SPEC.core.scm defines bet/bet/weighted/bet/conditional semantics"))
         (probabilistic-testing . ("All stochastic tests must be seedable and reproducible"))))

    (allowed-implementation-languages
      . ("Racket"))
    (quarantined-optional
      . ("Any Rust/Cargo artifacts (tooling only, non-authoritative)"
         "LSP/UI (must not affect semantics)"))
    (forbidden
      . ("Replacing Racket anchor in f0"
         "Adding Julia/other backends in f0"
         "Unseeded randomness in tests"))

    (golden-path
      . ((smoke-test-command . "racket repl/shell.rkt < conformance/smoke.bet")
         (success-criteria . ("smoke program runs"
                              "invalid programs produce deterministic diagnostics"
                              "seeded sampling produces stable summary stats"))))

    (first-pass-directives
      . ("Fix any template placeholders that imply different owner/URLs."
         "Add conformance corpus: deterministic + stochastic (seeded) cases."
         "Explicitly mark any non-Racket components as optional tooling."))

    (rsr
      . ((target-tier . "bronze-now") (upgrade-path . "silver-after-f1")))))

;; End of anchor
