#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/distributions.rkt")

;; Bet Analysis and Visualization Tools

(provide analyze-bet
         compare-bets
         convergence-analysis
         distribution-fit
         text-histogram
         probability-report
         entropy-analysis
         simulate-and-analyze)

;; Main analysis function
(define (analyze-bet bet-fn n)
  (displayln "=== Bet Analysis ===")
  (define samples (bet-repeat n bet-fn))

  ;; Frequency analysis
  (displayln "\n1. Frequency Distribution:")
  (define freq (frequency-table samples))
  (define sorted-freq (sort freq < #:key car))
  (for ([pair sorted-freq])
    (define proportion (/ (cdr pair) (exact->inexact n)))
    (displayln (format "   ~a: ~a times (~a%)"
                       (car pair) (cdr pair) (* 100 proportion))))

  ;; Statistical measures
  (displayln "\n2. Statistical Measures:")
  (when (andmap number? samples)
    (displayln (format "   Mean: ~a" (mean samples)))
    (displayln (format "   Median: ~a" (median samples)))
    (displayln (format "   Std Dev: ~a" (stddev samples)))
    (displayln (format "   Variance: ~a" (variance samples))))

  ;; Entropy
  (displayln "\n3. Information Theory:")
  (define ent (bet-entropy samples))
  (displayln (format "   Entropy: ~a bits" ent))
  (define max-entropy (log (length (remove-duplicates samples)) 2))
  (displayln (format "   Max possible entropy: ~a bits" max-entropy))
  (displayln (format "   Efficiency: ~a%" (* 100 (/ ent max-entropy))))

  ;; Text histogram
  (displayln "\n4. Visual Distribution:")
  (text-histogram freq 50)

  samples)

;; Compare multiple bet strategies
(define (compare-bets bet-fns names n)
  (displayln "=== Bet Comparison ===\n")
  (for ([bet-fn bet-fns]
        [name names]
        [idx (in-naturals 1)])
    (displayln (format "Strategy ~a: ~a" idx name))
    (define samples (bet-repeat n bet-fn))
    (define freq (frequency-table samples))
    (for ([pair freq])
      (displayln (format "  ~a: ~a%"
                         (car pair)
                         (* 100 (/ (cdr pair) (exact->inexact n))))))
    (displayln "")))

;; Convergence analysis
(define (convergence-analysis bet-fn target-outcome trials)
  (displayln "=== Convergence Analysis ===\n")
  (define checkpoints '(10 50 100 500 1000 5000 10000))

  (displayln (format "Tracking probability of outcome: ~a\n" target-outcome))
  (displayln "Trials | Observed Probability | Error from 0.333")
  (displayln "-------|---------------------|------------------")

  (for ([checkpoint checkpoints]
        #:when (<= checkpoint trials))
    (define samples (bet-repeat checkpoint bet-fn))
    (define count (length (filter (lambda (x) (equal? x target-outcome)) samples)))
    (define prob (/ count (exact->inexact checkpoint)))
    (define error (abs (- prob 0.333)))
    (displayln (format "~a    | ~a                | ~a"
                       (~r checkpoint #:min-width 6)
                       (~r prob #:precision 4 #:min-width 19)
                       (~r error #:precision 4))))

  (displayln "\n(Probability should converge to 0.333 for uniform ternary bet)"))

;; Distribution fitting
(define (distribution-fit samples)
  (displayln "=== Distribution Fit Analysis ===\n")

  (when (andmap number? samples)
    (define sample-mean (mean samples))
    (define sample-std (stddev samples))

    (displayln "Testing against known distributions:")
    (displayln (format "  Sample mean: ~a" sample-mean))
    (displayln (format "  Sample std: ~a" sample-std))
    (displayln "")

    ;; Compare with uniform distribution
    (define min-val (apply min samples))
    (define max-val (apply max samples))
    (define expected-uniform-mean (/ (+ min-val max-val) 2))
    (displayln (format "  Uniform[~a,~a] would have mean: ~a"
                       min-val max-val expected-uniform-mean))

    ;; Kolmogorov-Smirnov-like test (simplified)
    (define sorted (sort samples <))
    (displayln "")))

;; Text-based histogram
(define (text-histogram freq max-width)
  (define max-count (apply max (map cdr freq)))
  (define scale (/ max-width max-count))

  (for ([pair (sort freq < #:key car)])
    (define bar-length (exact->inexact (* (cdr pair) scale)))
    (define bar (make-string (exact->inexact (floor bar-length)) #\█))
    (displayln (format "   ~a: ~a ~a" (car pair) bar (cdr pair)))))

;; Generate probability report
(define (probability-report bet-fn outcomes n)
  (displayln "=== Probability Report ===\n")
  (displayln (format "Running ~a trials...\n" n))

  (for ([outcome outcomes])
    (define prob (bet-probability n
                                  (lambda (x) (equal? x outcome))
                                  (first outcomes)
                                  (second outcomes)
                                  (third outcomes)))
    (displayln (format "P(~a) ≈ ~a" outcome (~r prob #:precision 4)))))

;; Entropy analysis over time
(define (entropy-analysis bet-fn window-size total-trials)
  (displayln "=== Entropy Over Time ===\n")
  (define all-samples (bet-repeat total-trials bet-fn))

  (displayln (format "Window Size: ~a trials\n" window-size))
  (displayln "Window # | Entropy (bits) | Unique Values")
  (displayln "---------|----------------|---------------")

  (for ([i (in-range 0 (- total-trials window-size) window-size)])
    (define window (take (drop all-samples i) window-size))
    (define ent (bet-entropy window))
    (define unique (length (remove-duplicates window)))
    (displayln (format "~a       | ~a           | ~a"
                       (quotient i window-size)
                       (~r ent #:precision 3 #:min-width 14)
                       unique))))

;; Complete simulation and analysis
(define (simulate-and-analyze description bet-fn n)
  (displayln (format "=== ~a ===" description))
  (displayln "")
  (define start-time (current-inexact-milliseconds))
  (define results (analyze-bet bet-fn n))
  (define end-time (current-inexact-milliseconds))
  (displayln (format "\nSimulation completed in ~a ms"
                     (- end-time start-time)))
  results)

;; Example usage
(module+ main
  (displayln "betlang Analysis Tools\n")
  (displayln "=" 60)

  ;; Example 1: Basic bet analysis
  (simulate-and-analyze
   "Basic Ternary Bet"
   (lambda () (bet 'A 'B 'C))
   1000)

  (displayln "\n")
  (displayln "=" 60)

  ;; Example 2: Weighted bet analysis
  (simulate-and-analyze
   "Weighted Bet (70%, 20%, 10%)"
   (lambda () (bet/weighted '(common 7) '(uncommon 2) '(rare 1)))
   1000)

  (displayln "\n")
  (displayln "=" 60)

  ;; Example 3: Numeric bet for statistical analysis
  (simulate-and-analyze
   "Numeric Bet (1, 2, 3)"
   (lambda () (bet 1 2 3))
   10000)

  (displayln "\n")
  (displayln "=" 60)

  ;; Example 4: Convergence
  (convergence-analysis
   (lambda () (bet 'heads 'tails 'edge))
   'heads
   10000)

  (displayln "\n")
  (displayln "=" 60)

  ;; Example 5: Strategy comparison
  (compare-bets
   (list (lambda () (bet 'A 'B 'C))
         (lambda () (bet/weighted '(A 5) '(B 3) '(C 2)))
         (lambda () (bet/weighted '(A 1) '(B 1) '(C 8))))
   '("Uniform" "Weighted (50/30/20)" "Weighted (10/10/80)")
   1000)

  (displayln "\n")
  (displayln "=" 60)

  ;; Example 6: Entropy over time
  (entropy-analysis
   (lambda () (bet 'X 'Y 'Z))
   100
   1000))
