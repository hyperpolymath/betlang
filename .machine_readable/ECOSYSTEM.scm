;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ECOSYSTEM.scm - Ecosystem position for betlang
;; Media-Type: application/vnd.ecosystem+scm

(ecosystem
  (version "1.0")
  (name "betlang")
  (type "programming-language")
  (purpose "Ternary probabilistic programming with Dutch book prevention and gambling harm reduction")

  (position-in-ecosystem
    (category "probabilistic-programming")
    (subcategory "safety-focused-dsl")
    (unique-value
      ("Ternary bet primitive (3-way choices)"
       "Built-in Dutch book detection and prevention"
       "Risk-of-ruin Monte Carlo protection"
       "Cool-off mechanism for gambling harm reduction"
       "14 uncertainty-aware number systems"
       "Safety features integrated at language level")))

  (related-projects
    ((name "Distributions.jl")
     (relationship "planned-integration")
     (description "Julia probability distributions library, target for v0.8 backend"))
    ((name "Racket")
     (relationship "foundation")
     (description "Implementation language, provides S-expression syntax"))
    ((name "Stan")
     (relationship "inspiration")
     (description "Bayesian inference DSL"))
    ((name "Church/Anglican")
     (relationship "inspiration")
     (description "Probabilistic programming languages")))

  (what-this-is
    ("Domain-specific language for probabilistic programming"
     "Racket-based DSL with ternary bet primitive"
     "Statistical modeling toolkit with 180+ functions"
     "Monte Carlo simulation framework"
     "Safe gambling/betting language with harm reduction"
     "Uncertainty quantification system with 14 number systems"))

  (what-this-is-not
    ("Not a general-purpose programming language"
     "Not a replacement for statistical packages (R, Python SciPy)"
     "Not a production Rust compiler (Racket is authoritative)"
     "Not a machine learning framework"
     "Not a gambling promotion tool (focuses on harm reduction)")))
