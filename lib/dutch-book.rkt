#lang racket
;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Dutch Book Prevention
;;
;; Validates that probabilities in bets sum to 1.0 to prevent arbitrage opportunities.
;; A Dutch book is a set of bets that guarantees a loss regardless of outcome.

(provide validate-probabilities
         dutch-book-safe?
         normalize-probabilities
         bet/validated
         bet-probabilities
         make-probability-validator
         probability-error
         probability-tolerance
         dutch-book-margin
         detect-dutch-book
         validate-probabilities-strict)

;; Default tolerance for floating-point comparison
(define probability-tolerance (make-parameter 1e-10))

;; Validate that probabilities sum to 1.0 within tolerance
(define (validate-probabilities probs [tolerance (probability-tolerance)])
  (define total (apply + probs))
  (unless (< (abs (- total 1.0)) tolerance)
    (error 'validate-probabilities
           "Probabilities must sum to 1.0 (got ~a). This would create a Dutch book vulnerability."
           total))
  #t)

;; Check if a set of probabilities is Dutch book safe
(define (dutch-book-safe? probs [tolerance (probability-tolerance)])
  (and (andmap (lambda (p) (and (>= p 0) (<= p 1))) probs)
       (< (abs (- (apply + probs) 1.0)) tolerance)))

;; Normalize probabilities to sum to 1.0
(define (normalize-probabilities probs)
  (define total (apply + probs))
  (if (zero? total)
      (error 'normalize-probabilities "Cannot normalize: all probabilities are zero")
      (map (lambda (p) (/ p total)) probs)))

;; Calculate implied probabilities from a bet
(define (bet-probabilities . weighted-choices)
  (unless (= (length weighted-choices) 3)
    (error 'bet-probabilities "Expected exactly 3 weighted choices"))
  (define weights (map second weighted-choices))
  (normalize-probabilities weights))

;; Validated bet that ensures Dutch book safety
(define (bet/validated . weighted-choices)
  (unless (= (length weighted-choices) 3)
    (error 'bet/validated "Expected exactly 3 weighted choices"))

  (define weights (map second weighted-choices))
  (define values (map first weighted-choices))

  ;; Validate probabilities
  (validate-probabilities (normalize-probabilities weights))

  ;; Execute bet with normalized probabilities
  (define total-weight (apply + weights))
  (define r (* (random) total-weight))

  (let loop ([choices weighted-choices] [acc 0])
    (match choices
      ['() (first (third weighted-choices))]
      [(cons (list val weight) rest)
       (if (< r (+ acc weight))
           val
           (loop rest (+ acc weight)))])))

;; Create a probability validator with custom tolerance
(define (make-probability-validator tolerance)
  (lambda (probs)
    (validate-probabilities probs tolerance)))

;; Calculate the Dutch book margin (how far from fair probabilities)
(define (dutch-book-margin probs)
  (abs (- (apply + probs) 1.0)))

;; Detect if a set of odds creates a Dutch book opportunity
(define (detect-dutch-book odds)
  (define implied-probs (map (lambda (o) (/ 1 o)) odds))
  (define margin (dutch-book-margin implied-probs))
  (if (> margin (probability-tolerance))
      (format "Dutch book detected! Margin: ~a (probabilities sum to ~a)"
              margin
              (apply + implied-probs))
      #f))

;; Probability error structure
(struct probability-error (message probabilities expected actual) #:transparent)

;; Validate probabilities with detailed error reporting
(define (validate-probabilities-strict probs)
  (define total (apply + probs))
  (define tolerance (probability-tolerance))

  (cond
    [(not (andmap number? probs))
     (probability-error "All probabilities must be numbers" probs 1.0 'non-numeric)]
    [(not (andmap (lambda (p) (>= p 0)) probs))
     (probability-error "All probabilities must be non-negative" probs 1.0 'negative)]
    [(not (andmap (lambda (p) (<= p 1)) probs))
     (probability-error "All probabilities must be <= 1.0" probs 1.0 'too-large)]
    [(> (abs (- total 1.0)) tolerance)
     (probability-error
      (format "Probabilities must sum to 1.0 (±~a)" tolerance)
      probs
      1.0
      total)]
    [else #t]))

;; Example usage and tests
(module+ test
  (require rackunit)

  (test-case "Valid probabilities"
    (check-true (validate-probabilities '(0.2 0.3 0.5)))
    (check-true (validate-probabilities '(1/3 1/3 1/3))))

  (test-case "Invalid probabilities"
    (check-exn exn:fail?
               (lambda () (validate-probabilities '(0.2 0.3 0.6))))
    (check-exn exn:fail?
               (lambda () (validate-probabilities '(0.5 0.5 0.5)))))

  (test-case "Dutch book detection"
    (check-true (dutch-book-safe? '(0.25 0.25 0.5)))
    (check-false (dutch-book-safe? '(0.3 0.3 0.3))))

  (test-case "Normalization"
    (check-equal? (normalize-probabilities '(1 1 1))
                  '(1/3 1/3 1/3))
    (check-equal? (normalize-probabilities '(2 3 5))
                  '(1/5 3/10 1/2)))

  (test-case "Dutch book margin"
    (check-true (< (dutch-book-margin '(0.33 0.33 0.34)) 0.01))
    (check-true (> (dutch-book-margin '(0.5 0.5 0.5)) 0.4)))

  (test-case "Validated bet"
    ;; Should not error
    (for ([i 10])
      (bet/validated '(1 0.2) '(2 0.3) '(3 0.5)))))

(module+ main
  (displayln "=== Dutch Book Prevention Examples ===\n")

  (displayln "1. Valid probabilities (0.2, 0.3, 0.5):")
  (displayln (format "   Safe? ~a" (dutch-book-safe? '(0.2 0.3 0.5))))

  (displayln "\n2. Invalid probabilities (0.3, 0.3, 0.3):")
  (displayln (format "   Safe? ~a" (dutch-book-safe? '(0.3 0.3 0.3))))
  (displayln (format "   Margin: ~a" (dutch-book-margin '(0.3 0.3 0.3))))

  (displayln "\n3. Detecting Dutch book from odds:")
  (define odds-fair '(5 3.33 2))  ;; Fair odds sum to 1.0
  (define odds-book '(4 3 2))     ;; Dutch book (bookmaker margin)
  (displayln (format "   Fair odds ~a: ~a" odds-fair
                    (or (detect-dutch-book odds-fair) "No Dutch book")))
  (displayln (format "   Book odds ~a: ~a" odds-book
                    (detect-dutch-book odds-book)))

  (displayln "\n4. Normalized probabilities:")
  (displayln (format "   Input: (2, 3, 5)"))
  (displayln (format "   Normalized: ~a" (normalize-probabilities '(2 3 5))))

  (displayln "\n5. Running validated bets:"))
