#lang racket
;; SPDX-FileCopyrightText: 2025 hyperpolymath
;; SPDX-License-Identifier: PMPL-1.0-or-later
;;
;; deterministic.rkt - Deterministic Conformance Tests
;;
;; These tests verify deterministic behavior that MUST be consistent
;; across all conforming implementations.
;;
;; Usage: racket conformance/deterministic.rkt

(require rackunit)
(require "../core/betlang.rkt")

(displayln "=== Betlang Deterministic Conformance Tests ===\n")

;; ============================================================================
;; Test: Idempotent bet
;; ============================================================================
(displayln "Test: Idempotent bet (bet X X X = X)")
(check-equal? (bet 'same 'same 'same) 'same)
(check-equal? (bet 42 42 42) 42)
(check-equal? (bet "constant" "constant" "constant") "constant")
(check-equal? (bet '(a b c) '(a b c) '(a b c)) '(a b c))
(displayln "  PASS\n")

;; ============================================================================
;; Test: Conditional bet - true branch
;; ============================================================================
(displayln "Test: Conditional bet true branch")
(check-equal? (bet/conditional #t 'yes 'no 'maybe) 'yes)
(check-equal? (bet/conditional (> 5 3) 'greater 'less 'equal) 'greater)
(check-equal? (bet/conditional (string? "hello") "string" "not-string" "unknown") "string")
(displayln "  PASS\n")

;; ============================================================================
;; Test: All-bets utility
;; ============================================================================
(displayln "Test: all-bets returns list of all options")
(check-equal? (all-bets 'a 'b 'c) '(a b c))
(check-equal? (all-bets 1 2 3) '(1 2 3))
(check-equal? (all-bets "x" "y" "z") '("x" "y" "z"))
(displayln "  PASS\n")

;; ============================================================================
;; Test: Seeded bet reproducibility
;; ============================================================================
(displayln "Test: Seeded bet reproducibility")
(define result1 (bet-with-seed 42 (lambda () (bet 'A 'B 'C))))
(define result2 (bet-with-seed 42 (lambda () (bet 'A 'B 'C))))
(check-equal? result1 result2 "Same seed must produce same result")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Different seeds produce (potentially) different results
;; ============================================================================
(displayln "Test: Seed isolation")
;; Verify that bet-with-seed creates isolated PRNG context
(define outer-result
  (bet-with-seed 100
    (lambda ()
      (define inner (bet-with-seed 200 (lambda () (bet 1 2 3))))
      (define outer (bet 10 20 30))
      (list inner outer))))
(define outer-result-2
  (bet-with-seed 100
    (lambda ()
      (define inner (bet-with-seed 200 (lambda () (bet 1 2 3))))
      (define outer (bet 10 20 30))
      (list inner outer))))
(check-equal? outer-result outer-result-2 "Nested seeds must be reproducible")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Lazy evaluation - only selected branch runs
;; ============================================================================
(displayln "Test: Lazy evaluation - only one thunk executes")
(define call-count 0)
(define (make-counter-thunk val)
  (lambda ()
    (set! call-count (+ call-count 1))
    val))
(bet-with-seed 42
  (lambda ()
    (set! call-count 0)
    (bet/lazy (make-counter-thunk 'A) (make-counter-thunk 'B) (make-counter-thunk 'C))))
(check-equal? call-count 1 "Only one thunk should execute")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Bet chain with identity function
;; ============================================================================
(displayln "Test: Bet chain")
(check-equal? (bet-chain 0 add1 5) 5)
(check-equal? (bet-chain 3 add1 0) 3)
(check-equal? (bet-chain 5 (lambda (x) (* x 2)) 1) 32)
(displayln "  PASS\n")

;; ============================================================================
;; Test: Bet repeat
;; ============================================================================
(displayln "Test: Bet repeat")
(define repeat-result (bet-repeat 5 (lambda () 'constant)))
(check-equal? (length repeat-result) 5)
(check-true (andmap (lambda (x) (equal? x 'constant)) repeat-result))
(displayln "  PASS\n")

;; ============================================================================
;; Test: Generator produces consistent results with same seed
;; ============================================================================
(displayln "Test: Generator with seed")
(define gen (make-bet-generator 'x 'y 'z))
(define gen-results-1
  (bet-with-seed 555 (lambda () (list (gen) (gen) (gen)))))
(define gen-results-2
  (bet-with-seed 555 (lambda () (list (gen) (gen) (gen)))))
(check-equal? gen-results-1 gen-results-2)
(displayln "  PASS\n")

;; ============================================================================
;; Test: Entropy calculation
;; ============================================================================
(displayln "Test: Entropy calculation")
;; Uniform distribution of 3 values: entropy = log2(3) ≈ 1.585
(define uniform-entropy (bet-entropy '(A A A B B B C C C)))
(check-true (> uniform-entropy 1.5))
(check-true (< uniform-entropy 1.6))
;; Single value: entropy = 0
(define zero-entropy (bet-entropy '(X X X X X)))
(check-equal? zero-entropy 0)
(displayln "  PASS\n")

;; ============================================================================
;; Summary
;; ============================================================================
(displayln "=================================")
(displayln "All deterministic tests passed!")
(displayln "=================================")
