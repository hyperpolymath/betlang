;; SPDX-FileCopyrightText: 2026 hyperpolymath
;; SPDX-License-Identifier: PMPL-1.0-or-later
;;
;; STATE.scm - Project State Tracking for betlang
;;
;; This file tracks the current state of the betlang project.
;; Updated as milestones are reached and state changes occur.

(define project-state
  '((schema . "hyperpolymath.state/1")
    (project . "betlang")
    (last-updated . "2026-01-01")

    ;; =========================================================================
    ;; SEMANTIC ANCHOR STATUS
    ;; =========================================================================
    (anchor-status
      . ((anchor-file . "ANCHOR.scope-arrest.2026-01-01.Jewell.scm")
         (policy . "dual")
         (reference-impl . "core/betlang.rkt (Racket)")
         (formal-spec . "SPEC.core.scm")
         (phase . "f0")
         (tier . "bronze-now")))

    ;; =========================================================================
    ;; IMPLEMENTATION STATUS
    ;; =========================================================================
    (implementation
      . ((core-primitives
           . ((bet . "complete")
              (bet/weighted . "complete")
              (bet/conditional . "complete")
              (bet/lazy . "complete")
              (bet-with-seed . "complete")))
         (standard-library
           . ((statistics . "complete")
              (distributions . "complete")
              (bayesian . "complete")
              (markov . "complete")
              (optimization . "complete")
              (sampling . "complete")
              (combinators . "complete")
              (ternary . "complete")))
         (tooling
           . ((repl . "complete")
              (test-suite . "complete")
              (conformance-corpus . "complete")
              (benchmarks . "complete")))))

    ;; =========================================================================
    ;; CONFORMANCE STATUS
    ;; =========================================================================
    (conformance
      . ((smoke-test . "defined")
         (deterministic-tests . "defined")
         (stochastic-seeded-tests . "defined")
         (coverage . "core primitives covered")))

    ;; =========================================================================
    ;; OPTIONAL TOOLING STATUS
    ;; =========================================================================
    (optional-tooling
      . ((rust-compiler . "scaffolded (non-authoritative)")
         (language-bindings
           . ((julia . "scaffolded (non-authoritative)")
              (chapel . "scaffolded (non-authoritative)")))
         (containers . "defined")))

    ;; =========================================================================
    ;; OPERATIONAL AUTHORITY
    ;; =========================================================================
    (operational
      . ((task-runner . "just")
         (deployment . "must (pending)")
         (config . "nickel (pending)")
         (containers . "podman-first")))

    ;; =========================================================================
    ;; MILESTONES
    ;; =========================================================================
    (milestones
      . ((completed
           . ("Initial DSL implementation"
              "Standard library modules"
              "Interactive REPL"
              "Test suite"
              "Scope arrest anchor"
              "Formal specification (SPEC.core.scm)"
              "Conformance corpus"
              "Authority stack implementation"))
         (in-progress
           . ("Documentation refinement"))
         (planned
           . ("Silver tier upgrade (after f1)"
              "Alternative backend exploration (post-f0)"))))

    ;; =========================================================================
    ;; NEXT ACTIONS
    ;; =========================================================================
    (next-actions
      . ("Run: just test (verify all tests pass)"
         "Run: just smoke (verify golden path)"
         "Review conformance coverage"
         "Consider nickel manifest for config"))))

;; End of STATE.scm
