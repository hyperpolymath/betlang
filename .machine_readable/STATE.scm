;; SPDX-License-Identifier: PMPL-1.0-or-later
;; SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell
;; STATE.scm - Project state for betlang
;; Media-Type: application/vnd.state+scm

(state
  (metadata
    (version "0.6.0")
    (schema-version "1.0")
    (created "2026-01-03")
    (updated "2026-02-07")
    (project "betlang")
    (repo "github.com/hyperpolymath/betlang"))

  (project-context
    (name "BetLang")
    (tagline "Safe probabilistic programming with Dutch book prevention and gambling harm reduction")
    (tech-stack (Racket Rust LALRPOP Julia)))

  (current-position
    (phase "v0.6-number-systems-complete")
    (overall-completion 80)
    (components
      ((racket-implementation (status "complete") (completion 100))
       (safety-features (status "complete") (completion 100))
       (dutch-book-prevention (status "complete") (completion 100))
       (risk-of-ruin-protection (status "complete") (completion 100))
       (cool-off-mechanism (status "complete") (completion 100))
       (number-systems (status "complete") (completion 100))
       (rust-compiler (status "broken") (completion 30))
       (julia-backend (status "planned") (completion 0))))
    (working-features
      ("Ternary bets with equal/weighted probabilities"
       "Dutch book detection and prevention"
       "Kelly criterion optimal stake calculation"
       "Risk-of-ruin Monte Carlo simulation"
       "Cool-off mechanism with violation tracking"
       "All 14 uncertainty-aware number systems:"
       "  1. DistnumberNormal (Gaussian distributions)"
       "  2. AffineNumber (interval arithmetic)"
       "  3. FuzzyTriangular (fuzzy logic)"
       "  4. BayesianNumber (Bayesian inference)"
       "  5. RiskNumber (VaR/CVaR calculations)"
       "  6. SurrealFuzzy (infinitesimal tolerance)"
       "  7. p-Adic Probability (hierarchical digits)"
       "  8. LotteryNumber (weighted discrete outcomes)"
       "  9. DistnumberBeta (Beta distributions)"
       " 10. Hyperreal (non-standard analysis)"
       " 11. SurrealAdvanced (full surreal arithmetic)"
       " 12. PAdicAdvanced (complete p-adic system)"
       " 13. ImpreciseProbability (interval bounds)"
       " 14. DempsterShafer (belief functions)"
       "Comprehensive safety demo (examples/safety-features.rkt)")))

  (route-to-mvp
    (milestones
      ((v0.5-safety-features
        (status "complete")
        (completion 100)
        (items
          (("Dutch book prevention" "complete")
           ("Risk-of-ruin protection" "complete")
           ("Cool-off mechanism" "complete")
           ("5 core number systems" "complete")
           ("Safety features demo" "complete")
           ("Documentation (SAFETY-FEATURES.md)" "complete"))))

       (v0.6-remaining-number-systems
        (status "complete")
        (completion 100)
        (items
          (("DistnumberBeta" "complete")
           ("Hyperreal numbers" "complete")
           ("SurrealAdvanced numbers" "complete")
           ("PAdicAdvanced numbers" "complete")
           ("Imprecise probabilities" "complete")
           ("Dempster-Shafer theory" "complete"))))

       (v0.7-rust-compiler-fix
        (status "planned")
        (completion 0)
        (items
          (("Fix serde serialization errors" "planned")
           ("Complete type checker" "planned")
           ("Complete code generator" "planned")
           ("Complete interpreter" "planned"))))

       (v0.8-julia-backend
        (status "planned")
        (completion 0)
        (items
          (("BetLang → Julia compiler" "planned")
           ("Distributions.jl integration" "planned")
           ("Aggregate-library preparation" "planned"))))

       (v1.0-production
        (status "planned")
        (completion 0)
        (items
          (("Idris2 ABI formal proofs" "planned")
           ("Academic paper submission" "planned")
           ("Performance optimization" "planned")
           ("Full documentation" "planned")
           ("Release preparation" "planned")))))))

  (blockers-and-issues
    (critical ())
    (high ())
    (medium
      (("Rust compiler: LALRPOP parser has 3 shift/reduce conflicts" "documented")
       ("Rust compiler: Parser generation blocks entire build" "documented")
       ("Rust compiler: Optional tooling, non-authoritative" "accepted")
       ("Julia backend doesn't exist yet (planned for v0.8)" "accepted")))
    (low
      (("Rust type checker is stub" "documented")
       ("Missing some GitHub workflows" "accepted"))))

  (critical-next-actions
    (immediate ())
    (optional-future-work
      (("Fix LALRPOP parser conflicts (Rust compiler)" "medium")
       ("Complete Rust type checker and interpreter" "medium")
       ("Create Julia backend (v0.8)" "medium")
       ("Academic paper draft" "low")
       ("Performance optimization" "low"))))

  (session-history
    ((date "2026-02-01")
     (session "safety-features-implementation")
     (accomplishments
       ("Implemented Dutch book prevention (lib/dutch-book.rkt)"
        "Implemented risk-of-ruin protection (lib/risk-of-ruin.rkt)"
        "Implemented cool-off mechanism (lib/cool-off.rkt)"
        "Implemented 5 uncertainty-aware number systems (lib/number-systems.rkt)"
        "Created comprehensive safety demo (examples/safety-features.rkt)"
        "Created safety documentation (SAFETY-FEATURES.md)"
        "All safety features tested and working"
        "Updated ECOSYSTEM.scm with correct project description")))))
