;; SPDX-License-Identifier: PMPL-1.0-or-later
;; SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell
;; ECOSYSTEM.scm - Project relationship mapping

(ecosystem
  (version "1.0")
  (name "betlang")
  (type "programming-language")
  (purpose "Probabilistic programming language with compile-time Dutch book prevention, risk-of-ruin protection, and 11 uncertainty-aware number systems for safe betting and decision-making under uncertainty")

  (position-in-ecosystem
    (category "Programming Languages")
    (subcategory "Probabilistic Programming / Safety-Critical")
    (unique-value
      ("First language with compile-time Dutch book prevention"
       "11 uncertainty-aware number systems (DistnumberNormal, AffineNumber, FuzzyTriangular, BayesianNumber, RiskNumber, etc.)"
       "Cool-off mechanism for gambling harm reduction"
       "Risk-of-ruin protection with runtime Monte Carlo validation"
       "Formal safety proofs via Idris2 dependent types")))

  (related-projects
    ((aggregate-library
      (relationship "stdlib-strategy")
      (description "Long-term plan to share stdlib with Julia via aggregate-library methodology"))
     (affinescript
      (relationship "sibling-language")
      (description "Both experimental type-safe systems languages from hyperpolymath"))
     (ephapax
      (relationship "future-compiler-language")
      (description "Eventually rewrite BetLang compiler in Ephapax linear form"))
     (januskey
      (relationship "potential-integration")
      (description "Identity verification for betting platform compliance"))))

  (what-this-is
    ("Probabilistic programming language with safety guarantees"
     "Compiler with Dutch book prevention and risk analysis"
     "Research platform for uncertainty-aware computation"
     "Tool for responsible decision-making under uncertainty"))

  (what-this-is-not
    ("NOT a gambling promotion tool"
     "NOT exempt from responsible gambling requirements"
     "NOT a general-purpose PPL (focused on betting/auctions/uncertainty)"
     "NOT related to robot simulation or choreography")))
