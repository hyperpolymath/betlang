#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/distributions.rkt")

;; Monte Carlo Examples for betlang

(displayln "=== Monte Carlo Simulations ===\n")

;; Example 1: Estimate Pi using Monte Carlo
(displayln "1. Estimating Pi:")
(define pi-estimate (monte-carlo-pi 100000))
(displayln (format "   Estimated π ≈ ~a (actual: ~a)" pi-estimate pi))
(displayln (format "   Error: ~a%" (* 100 (/ (abs (- pi-estimate pi)) pi))))

;; Example 2: Buffon's Needle using bets
(displayln "\n2. Buffon's Needle Problem:")
(define (buffon-needle n)
  (define hits
    (count (lambda (_)
             (define angle (* (random) pi))
             (define center (* (random) 2))
             (define half-len 1)
             (<= (abs (- center 1)) (* half-len (sin angle))))
           (range n)))
  (if (> hits 0)
      (/ (* 2 n) hits)
      0))
(define buffon-pi (buffon-needle 10000))
(displayln (format "   Buffon's π ≈ ~a" buffon-pi))

;; Example 3: Monte Carlo Integration
(displayln "\n3. Monte Carlo Integration of x² from 0 to 1:")
(define (integrate-x-squared n)
  (define samples
    (for/list ([i (in-range n)])
      (define x (random))
      (* x x)))
  (mean samples))
(define integral-estimate (integrate-x-squared 100000))
(displayln (format "   Estimated integral: ~a (actual: 0.333...)" integral-estimate))

;; Example 4: European Option Pricing (simplified)
(displayln "\n4. European Call Option Pricing:")
(define (european-call S K T r sigma n)
  ;; S: current price, K: strike, T: time, r: rate, sigma: volatility
  (define dt (/ T 252)) ;; daily steps
  (define drift (* (- r (* 0.5 sigma sigma)) dt))
  (define vol (* sigma (sqrt dt)))

  (define payoffs
    (for/list ([i (in-range n)])
      (define final-price
        (for/fold ([price S]) ([day (in-range 252)])
          (* price (exp (+ drift (* vol (normal 0 1)))))))
      (max 0 (- final-price K))))

  (* (exp (* (- r) T)) (mean payoffs)))

(define option-price (european-call 100 105 1.0 0.05 0.2 10000))
(displayln (format "   Call option price: $~a" (real->decimal-string option-price 2)))

;; Example 5: Random Walk hitting time
(displayln "\n5. Random Walk Hitting Time:")
(define (hitting-time target)
  (let loop ([pos 0] [steps 0])
    (if (= pos target)
        steps
        (loop (+ pos (bet -1 0 1)) (+ steps 1)))))

(define hitting-times
  (for/list ([i (in-range 1000)])
    (hitting-time 10)))
(displayln (format "   Average steps to reach +10: ~a"
                   (exact->inexact (mean hitting-times))))
(displayln (format "   Std deviation: ~a"
                   (exact->inexact (stddev hitting-times))))

;; Example 6: Percolation simulation
(displayln "\n6. Percolation Simulation (simplified 1D):")
(define (percolates? p n)
  (define path
    (for/list ([i (in-range n)])
      (< (random) p)))
  (andmap identity path))

(define (find-critical-p n-trials)
  (for/first ([p (in-range 0 1 0.01)])
    (define percolation-count
      (count identity
             (for/list ([i (in-range n-trials)])
               (percolates? p 100))))
    (>= (/ percolation-count n-trials) 0.5)))

(define critical-p (find-critical-p 100))
(displayln (format "   Critical percolation probability: ~a" critical-p))

;; Example 7: Monte Carlo for variance reduction
(displayln "\n7. Variance Reduction with Antithetic Variates:")
(define (estimate-exp-mean n use-antithetic?)
  (if use-antithetic?
      (let ([samples1 (for/list ([i (in-range (quotient n 2))])
                        (normal 0 1))])
        (define samples2 (map - samples1))
        (mean (append (map exp samples1) (map exp samples2))))
      (mean (for/list ([i (in-range n)])
              (exp (normal 0 1))))))

(define standard-estimate (estimate-exp-mean 10000 #f))
(define antithetic-estimate (estimate-exp-mean 10000 #t))
(displayln (format "   Standard estimate: ~a" standard-estimate))
(displayln (format "   Antithetic estimate: ~a" antithetic-estimate))
(displayln (format "   Theoretical E[e^Z] for Z~N(0,1): ~a" (exp 0.5)))

(displayln "\n=== Simulations Complete ===")
