;; SPDX-FileCopyrightText: 2025 hyperpolymath
;; SPDX-License-Identifier: PMPL-1.0-or-later
;;
;; SPEC.core.scm - Formal Semantics Specification for betlang
;;
;; This file defines the authoritative semantics for the core betlang primitives.
;; The Racket interpreter in core/betlang.rkt is the reference implementation.
;; All implementations and backends MUST conform to these semantics.

(define betlang-spec
  '((version . "1.0.0")
    (schema . "hyperpolymath.betlang.spec/1")
    (status . "authoritative")
    (reference-implementation . "core/betlang.rkt")

    ;;=========================================================================
    ;; CORE PRIMITIVE: bet
    ;;=========================================================================
    (bet
     . ((signature . "(bet A B C) -> A | B | C")
        (description . "Randomly select one of three values with uniform probability")

        (formal-semantics
         . ((denotation . "[[bet A B C]] : Value x Value x Value -> Dist(Value)")
            (probability-model
             . ((P-A . "1/3")
                (P-B . "1/3")
                (P-C . "1/3")))
            (entropy . "log2(3) = 1.585 bits")))

        (evaluation-order . "strict")
        (note . "All three arguments are evaluated before selection occurs")

        (algebraic-properties
         . ((symmetry . "bet A B C ~ bet B C A ~ bet C A B  (distributionally)")
            (idempotence . "bet X X X = X  (deterministic)")
            (independence . "Successive bet calls are independent")))

        (edge-cases
         . ((identical-values . "bet X X X always returns X")
            (thunk-arguments . "Thunks are NOT called; use bet/lazy for lazy evaluation")))))

    ;;=========================================================================
    ;; WEIGHTED VARIANT: bet/weighted
    ;;=========================================================================
    (bet/weighted
     . ((signature . "(bet/weighted '(A w1) '(B w2) '(C w3)) -> A | B | C")
        (description . "Randomly select one of three values with weights proportional to probabilities")

        (formal-semantics
         . ((denotation . "[[bet/weighted (A w1) (B w2) (C w3)]] : (Value x Weight)^3 -> Dist(Value)")
            (probability-model
             . ((total-weight . "W = w1 + w2 + w3")
                (P-A . "w1 / W")
                (P-B . "w2 / W")
                (P-C . "w3 / W")))
            (entropy . "H = -sum(pi * log2(pi)) where pi = wi/W")))

        (constraints
         . ((arity . "Exactly 3 weighted pairs required")
            (weights . "Must be non-negative numbers; total > 0")
            (format . "Each argument is a quoted pair '(value weight)")))

        (error-conditions
         . ((wrong-arity . "exn:fail if not exactly 3 arguments")
            (zero-total . "undefined behavior if w1+w2+w3 = 0")
            (negative-weight . "undefined behavior for negative weights")))

        (examples
         . (("(bet/weighted '(A 1) '(B 1) '(C 1))" . "Equivalent to (bet A B C)")
            ("(bet/weighted '(rare 1) '(uncommon 3) '(common 6))" . "P(rare)=0.1, P(uncommon)=0.3, P(common)=0.6")))))

    ;;=========================================================================
    ;; CONDITIONAL VARIANT: bet/conditional
    ;;=========================================================================
    (bet/conditional
     . ((signature . "(bet/conditional pred A B C) -> A | B | C | A")
        (description . "Conditional bet: if pred is true return A, else bet between B C A")

        (formal-semantics
         . ((denotation . "[[bet/conditional pred A B C]] : Bool x Value^3 -> Dist(Value)")
            (semantics
             . ((when-true . "pred = #t => return A deterministically")
                (when-false . "pred = #f => return [[bet B C A]]")))
            (probability-model-when-false
             . ((P-B . "1/3")
                (P-C . "1/3")
                (P-A . "1/3")))))

        (evaluation-order . "pred evaluated first; then A B C if needed")

        (note . "The false-branch includes A in the bet, giving A a 'second chance'")))

    ;;=========================================================================
    ;; LAZY VARIANT: bet/lazy
    ;;=========================================================================
    (bet/lazy
     . ((signature . "(bet/lazy thunk-a thunk-b thunk-c) -> (thunk-a) | (thunk-b) | (thunk-c)")
        (description . "Lazy bet: only the selected thunk is evaluated")

        (formal-semantics
         . ((denotation . "[[bet/lazy ta tb tc]] : Thunk^3 -> Dist(Value)")
            (semantics . "Select index i in {0,1,2} uniformly; return (ti)")
            (probability-model
             . ((P-a . "1/3")
                (P-b . "1/3")
                (P-c . "1/3")))))

        (evaluation-order . "lazy - only selected thunk invoked")
        (constraints . "Arguments must be zero-arity procedures (thunks)")))

    ;;=========================================================================
    ;; SEEDED EXECUTION: bet-with-seed
    ;;=========================================================================
    (bet-with-seed
     . ((signature . "(bet-with-seed seed thunk) -> Value")
        (description . "Execute bet with deterministic random seed for reproducibility")

        (formal-semantics
         . ((denotation . "[[bet-with-seed s t]] : Int x Thunk -> Value")
            (semantics . "Parameterize PRNG with seed s, then invoke thunk")
            (determinism . "Same seed always produces same sequence of random choices")))

        (constraints
         . ((seed-type . "Non-negative integer")
            (scope . "Seed affects only bets within the thunk's dynamic extent")))

        (use-cases
         . ("Reproducible tests"
            "Debugging stochastic behavior"
            "Conformance testing"))))

    ;;=========================================================================
    ;; TESTING REQUIREMENTS
    ;;=========================================================================
    (testing-requirements
     . ((seedability . "All stochastic tests MUST use bet-with-seed for reproducibility")
        (statistical-tolerance . "Probability estimates should use sufficient sample size (n >= 1000)")
        (deterministic-tests . "Include tests for deterministic edge cases (bet X X X)")
        (conformance-corpus . "See conformance/ directory for canonical test cases")))

    ;;=========================================================================
    ;; IMPLEMENTATION NOTES
    ;;=========================================================================
    (implementation-notes
     . ((prng . "Uses Racket's (random) with (random 3) for uniform ternary selection")
        (thread-safety . "Not thread-safe by default; use Racket synchronization")
        (memory-model . "Eager evaluation except for bet/lazy")
        (error-propagation . "Racket exceptions propagate through bet forms")))))

;; End of SPEC.core.scm
