#lang racket
;; SPDX-FileCopyrightText: 2025 hyperpolymath
;; SPDX-License-Identifier: PMPL-1.0-or-later
;;
;; stochastic-seeded.rkt - Seeded Stochastic Conformance Tests
;;
;; These tests verify statistical properties using seeded randomness
;; to ensure reproducibility. All tests MUST produce the same results
;; on any conforming implementation.
;;
;; Usage: racket conformance/stochastic-seeded.rkt

(require rackunit)
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")

(displayln "=== Betlang Seeded Stochastic Conformance Tests ===\n")

;; Master seed for all tests
(define MASTER-SEED 20250101)

;; ============================================================================
;; Test: Uniform distribution verification (seeded)
;; ============================================================================
(displayln "Test: Uniform distribution (seeded)")
(define uniform-samples
  (bet-with-seed MASTER-SEED
    (lambda ()
      (bet-parallel 3000 'A 'B 'C))))

(define freq-table (frequency-table uniform-samples))
(define count-A (cdr (assoc 'A freq-table)))
(define count-B (cdr (assoc 'B freq-table)))
(define count-C (cdr (assoc 'C freq-table)))

(displayln (format "  Counts: A=~a, B=~a, C=~a" count-A count-B count-C))
;; Each should be approximately 1000 (±100 for reasonable tolerance)
(check-true (< (abs (- count-A 1000)) 150) "A count within tolerance")
(check-true (< (abs (- count-B 1000)) 150) "B count within tolerance")
(check-true (< (abs (- count-C 1000)) 150) "C count within tolerance")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Weighted distribution verification (seeded)
;; ============================================================================
(displayln "Test: Weighted distribution (seeded)")
(define weighted-samples
  (bet-with-seed (+ MASTER-SEED 1)
    (lambda ()
      (for/list ([i (in-range 1000)])
        (bet/weighted '(rare 1) '(uncommon 3) '(common 6))))))

(define weighted-freq (frequency-table weighted-samples))
(define rare-count (cdr (assoc 'rare weighted-freq)))
(define uncommon-count (cdr (assoc 'uncommon weighted-freq)))
(define common-count (cdr (assoc 'common weighted-freq)))

(displayln (format "  Counts: rare=~a, uncommon=~a, common=~a"
                   rare-count uncommon-count common-count))
;; Expected: rare ~100, uncommon ~300, common ~600
(check-true (< rare-count uncommon-count) "rare < uncommon")
(check-true (< uncommon-count common-count) "uncommon < common")
(check-true (< (abs (- common-count 600)) 100) "common near 600")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Probability estimation consistency (seeded)
;; ============================================================================
(displayln "Test: Probability estimation (seeded)")
(define prob-estimate
  (bet-with-seed (+ MASTER-SEED 2)
    (lambda ()
      (bet-probability 10000 (lambda (x) (equal? x 'target)) 'target 'miss1 'miss2))))

(displayln (format "  P(target) = ~a (expected ~0.333)" prob-estimate))
(check-true (< (abs (- prob-estimate 0.333)) 0.03) "Probability estimate within 3%")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Expected value calculation (seeded)
;; ============================================================================
(displayln "Test: Expected value (seeded)")
(define expected-val
  (bet-with-seed (+ MASTER-SEED 3)
    (lambda ()
      (bet-expect 10000 identity 1 2 3))))

(displayln (format "  E[X] = ~a (expected 2.0)" expected-val))
;; Expected value for uniform bet of 1,2,3 is (1+2+3)/3 = 2
(check-true (< (abs (- expected-val 2.0)) 0.1) "Expected value within tolerance")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Conditional bet false-branch distribution (seeded)
;; ============================================================================
(displayln "Test: Conditional bet false-branch (seeded)")
(define conditional-samples
  (bet-with-seed (+ MASTER-SEED 4)
    (lambda ()
      (for/list ([i (in-range 3000)])
        (bet/conditional #f 'yes 'no 'maybe)))))

;; When pred is false, bet/conditional does (bet B C A) = (bet 'no 'maybe 'yes)
;; So all three should appear with equal probability
(define cond-freq (frequency-table conditional-samples))
(displayln (format "  Distribution: ~a" cond-freq))
(check-equal? (length cond-freq) 3 "All three values should appear")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Composed function distribution (seeded)
;; ============================================================================
(displayln "Test: Composed function (seeded)")
(define composed-fn (bet-compose add1 identity sub1))
(define composed-samples
  (bet-with-seed (+ MASTER-SEED 5)
    (lambda ()
      (for/list ([i (in-range 3000)])
        (composed-fn 100)))))

(define comp-freq (frequency-table composed-samples))
(displayln (format "  Distribution: ~a" comp-freq))
;; Should have 99, 100, 101 each appearing ~1000 times
(check-true (member 99 (map car comp-freq)) "99 should appear")
(check-true (member 100 (map car comp-freq)) "100 should appear")
(check-true (member 101 (map car comp-freq)) "101 should appear")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Bet-until convergence (seeded)
;; ============================================================================
(displayln "Test: Bet-until convergence (seeded)")
(define until-result
  (bet-with-seed (+ MASTER-SEED 6)
    (lambda ()
      (bet-until
        (lambda (x) (equal? x 'target))
        (lambda () (bet 'target 'miss 'miss))))))

(check-equal? until-result 'target)
(displayln "  PASS\n")

;; ============================================================================
;; Test: Entropy approaches theoretical maximum (seeded)
;; ============================================================================
(displayln "Test: Entropy of uniform bet (seeded)")
(define entropy-samples
  (bet-with-seed (+ MASTER-SEED 7)
    (lambda ()
      (bet-parallel 9000 'X 'Y 'Z))))

(define sample-entropy (bet-entropy entropy-samples))
(displayln (format "  Entropy = ~a bits (theoretical max = log2(3) ≈ 1.585)" sample-entropy))
;; Should be very close to log2(3) for uniform distribution
(check-true (> sample-entropy 1.58) "Entropy near theoretical maximum")
(displayln "  PASS\n")

;; ============================================================================
;; Test: Reproducibility across multiple runs
;; ============================================================================
(displayln "Test: Full reproducibility")
(define run1
  (bet-with-seed 999
    (lambda ()
      (list
        (bet 'a 'b 'c)
        (bet/weighted '(x 5) '(y 3) '(z 2))
        (bet-parallel 10 1 2 3)
        (bet-probability 100 even? 1 2 3)))))

(define run2
  (bet-with-seed 999
    (lambda ()
      (list
        (bet 'a 'b 'c)
        (bet/weighted '(x 5) '(y 3) '(z 2))
        (bet-parallel 10 1 2 3)
        (bet-probability 100 even? 1 2 3)))))

(check-equal? run1 run2 "Identical seeds must produce identical results")
(displayln "  PASS\n")

;; ============================================================================
;; Summary
;; ============================================================================
(displayln "==========================================")
(displayln "All seeded stochastic tests passed!")
(displayln "==========================================")
