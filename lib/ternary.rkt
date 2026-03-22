#lang racket
(require "../core/betlang.rkt")

;; Ternary Logic and Utilities for betlang

(provide ternary-eval
         ternary-and
         ternary-or
         ternary-not
         ternary-xor
         ternary-implies
         ternary-equiv
         ternary-majority
         ternary-consensus
         ternary-select
         ternary-compare
         ternary-min
         ternary-max
         ternary-median
         ternary-fold
         ternary-map
         ternary-filter
         ternary-reduce
         make-ternary-table
         ternary-truth-value
         TRUE
         FALSE
         UNKNOWN)

;; Ternary truth values
(define TRUE 1)
(define FALSE 0)
(define UNKNOWN 0.5)

;; Abstract ternary conditional: cond → truth ∣ fallback
(define (ternary-eval cond truth fallback)
  (if cond truth fallback))

;; Ternary Logic Operations (Kleene's three-valued logic)

(define (ternary-and a b c)
  ;; Returns minimum of three values
  (min a b c))

(define (ternary-or a b c)
  ;; Returns maximum of three values
  (max a b c))

(define (ternary-not value)
  ;; Inverts truth value
  (cond
    [(= value TRUE) FALSE]
    [(= value FALSE) TRUE]
    [else UNKNOWN]))

(define (ternary-xor a b c)
  ;; XOR for three values
  (define sum (+ a b c))
  (cond
    [(= (modulo (inexact->exact (floor sum)) 2) 1) TRUE]
    [else FALSE]))

(define (ternary-implies a b)
  ;; Implication: a → b
  (cond
    [(and (= a TRUE) (= b FALSE)) FALSE]
    [(= a FALSE) TRUE]
    [(= b TRUE) TRUE]
    [else UNKNOWN]))

(define (ternary-equiv a b)
  ;; Equivalence: a ↔ b
  (if (= a b) TRUE FALSE))

;; Ternary Selection and Aggregation

(define (ternary-majority a b c)
  ;; Returns the value that appears most
  (cond
    [(and (= a b) (= a c)) a]
    [(= a b) a]
    [(= b c) b]
    [(= a c) a]
    [else UNKNOWN]))

(define (ternary-consensus . values)
  ;; Consensus among multiple ternary values
  (define counts (make-hash))
  (for ([v values])
    (hash-set! counts v (+ 1 (hash-ref counts v 0))))

  (define max-count (apply max (hash-values counts)))
  (define consensus-values
    (for/list ([(k v) (in-hash counts)]
               #:when (= v max-count))
      k))

  (if (= (length consensus-values) 1)
      (first consensus-values)
      UNKNOWN))

(define (ternary-select pred a b c)
  ;; Select based on predicate
  (cond
    [(pred a) a]
    [(pred b) b]
    [(pred c) c]
    [else UNKNOWN]))

;; Ternary Comparison

(define (ternary-compare a b c)
  ;; Returns -1, 0, or 1 based on relationship
  (bet
   (cond [(< a b c) -1]
         [(= a b c) 0]
         [else 1])
   (cond [(< b c a) -1]
         [(= a b c) 0]
         [else 1])
   (cond [(< c a b) -1]
         [(= a b c) 0]
         [else 1])))

(define (ternary-min a b c)
  (min a b c))

(define (ternary-max a b c)
  (max a b c))

(define (ternary-median a b c)
  (define sorted (sort (list a b c) <))
  (second sorted))

;; Ternary Higher-Order Functions

(define (ternary-fold f init a b c)
  ;; Fold over three values
  (f c (f b (f a init))))

(define (ternary-map f a b c)
  ;; Map function over three values
  (values (f a) (f b) (f c)))

(define (ternary-filter pred a b c)
  ;; Filter three values by predicate
  (filter pred (list a b c)))

(define (ternary-reduce f a b c)
  ;; Reduce three values with binary function
  (f a (f b c)))

;; Ternary Truth Table

(define (make-ternary-table op)
  ;; Generate truth table for ternary operation
  (define values (list TRUE UNKNOWN FALSE))
  (displayln "A\tB\tC\tResult")
  (displayln "---\t---\t---\t------")
  (for* ([a values]
         [b values]
         [c values])
    (displayln (format "~a\t~a\t~a\t~a"
                       a b c (op a b c)))))

;; Ternary Truth Value Interpretation

(define (ternary-truth-value numeric-value threshold-high threshold-low)
  ;; Convert numeric value to ternary truth value
  (cond
    [(>= numeric-value threshold-high) TRUE]
    [(<= numeric-value threshold-low) FALSE]
    [else UNKNOWN]))

;; Additional Utilities

(define (ternary-threshold a b c threshold)
  ;; Apply threshold to three values
  (values
   (if (> a threshold) TRUE FALSE)
   (if (> b threshold) TRUE FALSE)
   (if (> c threshold) TRUE FALSE)))

(define (ternary-normalize a b c)
  ;; Normalize three values to probabilities
  (define total (+ a b c))
  (if (= total 0)
      (values (/ 1 3) (/ 1 3) (/ 1 3))
      (values (/ a total) (/ b total) (/ c total))))

(define (ternary-distance a b c)
  ;; Calculate distance metric for ternary values
  (sqrt (+ (* a a) (* b b) (* c c))))

(define (ternary-variance a b c)
  ;; Variance of three values
  (define mean (/ (+ a b c) 3))
  (/ (+ (expt (- a mean) 2)
        (expt (- b mean) 2)
        (expt (- c mean) 2))
     3))

;; Example usage
(module+ main
  (displayln "=== Ternary Logic Examples ===\n")

  (displayln "1. Basic Ternary Logic")
  (displayln (format "   TRUE AND FALSE AND UNKNOWN = ~a"
                     (ternary-and TRUE FALSE UNKNOWN)))
  (displayln (format "   TRUE OR FALSE OR UNKNOWN = ~a"
                     (ternary-or TRUE FALSE UNKNOWN)))
  (displayln (format "   NOT UNKNOWN = ~a"
                     (ternary-not UNKNOWN)))
  (displayln "")

  (displayln "2. Ternary Majority Vote")
  (displayln (format "   Majority(1, 1, 0) = ~a"
                     (ternary-majority 1 1 0)))
  (displayln (format "   Majority(1, 0, 0.5) = ~a"
                     (ternary-majority 1 0 0.5)))
  (displayln "")

  (displayln "3. Ternary Median")
  (displayln (format "   Median(10, 5, 8) = ~a"
                     (ternary-median 10 5 8)))
  (displayln "")

  (displayln "4. Ternary Fold")
  (displayln (format "   Fold(+, 0, 1, 2, 3) = ~a"
                     (ternary-fold + 0 1 2 3)))
  (displayln "")

  (displayln "5. Ternary Normalization")
  (define-values (n1 n2 n3) (ternary-normalize 10 20 30))
  (displayln (format "   Normalize(10, 20, 30) = (~a, ~a, ~a)"
                     n1 n2 n3))
  (displayln "")

  (displayln "6. Truth Table for AND")
  (make-ternary-table ternary-and)

  (displayln "\n=== Ternary Logic Complete ==="))
