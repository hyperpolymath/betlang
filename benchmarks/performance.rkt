#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/distributions.rkt")
(require "../lib/markov.rkt")

;; Performance Benchmarks for betlang

(define (benchmark name thunk iterations)
  (displayln (format "\n=== ~a ===" name))
  (collect-garbage)
  (collect-garbage)
  (collect-garbage)

  (define start-time (current-inexact-milliseconds))
  (for ([i (in-range iterations)])
    (thunk))
  (define end-time (current-inexact-milliseconds))

  (define total-time (- end-time start-time))
  (define per-op (/ total-time iterations))

  (displayln (format "Total time: ~a ms" total-time))
  (displayln (format "Iterations: ~a" iterations))
  (displayln (format "Time per operation: ~a ms" per-op))
  (displayln (format "Operations per second: ~a"
                     (exact->inexact (/ 1000 per-op)))))

(displayln "╔══════════════════════════════════════════╗")
(displayln "║   betlang Performance Benchmarks        ║")
(displayln "╚══════════════════════════════════════════╝")
(displayln "")

;; Benchmark 1: Basic bet performance
(benchmark "Basic Bet"
           (lambda () (bet 1 2 3))
           100000)

;; Benchmark 2: Weighted bet performance
(benchmark "Weighted Bet"
           (lambda () (bet/weighted '(1 5) '(2 3) '(3 2)))
           100000)

;; Benchmark 3: Bet parallel (small)
(benchmark "Bet Parallel (n=100)"
           (lambda () (bet-parallel 100 'A 'B 'C))
           1000)

;; Benchmark 4: Bet parallel (large)
(benchmark "Bet Parallel (n=10000)"
           (lambda () (bet-parallel 10000 'A 'B 'C))
           10)

;; Benchmark 5: Statistical analysis
(benchmark "Mean calculation (n=10000)"
           (lambda ()
             (define data (for/list ([i (in-range 10000)]) (random)))
             (mean data))
           100)

;; Benchmark 6: Normal distribution sampling
(benchmark "Normal Distribution Sampling"
           (lambda () (normal 0 1))
           10000)

;; Benchmark 7: Binomial distribution
(benchmark "Binomial Distribution (n=100, p=0.5)"
           (lambda () (binomial 100 0.5))
           1000)

;; Benchmark 8: Random walk
(benchmark "Random Walk (100 steps)"
           (lambda () (random-walk 100))
           1000)

;; Benchmark 9: Markov simulation
(define test-chain
  (make-markov-chain
    '(A B C)
    (hash 'A '((A 0.5) (B 0.3) (C 0.2))
          'B '((A 0.2) (B 0.5) (C 0.3))
          'C '((A 0.3) (B 0.2) (C 0.5)))
    'A))

(benchmark "Markov Chain Simulation (100 steps)"
           (lambda () (markov-simulate test-chain 100))
           1000)

;; Benchmark 10: Entropy calculation
(benchmark "Entropy Calculation (n=1000)"
           (lambda ()
             (define samples (for/list ([i (in-range 1000)])
                              (bet 'A 'B 'C)))
             (bet-entropy samples))
           100)

;; Benchmark 11: Bootstrap
(benchmark "Bootstrap Resampling (n=100, samples=100)"
           (lambda ()
             (define data (for/list ([i (in-range 100)]) (random)))
             (bootstrap data 100 mean))
           10)

;; Benchmark 12: Bet chain
(benchmark "Bet Chain (length=100)"
           (lambda ()
             (bet-chain 100 (lambda (x) (+ x 1)) 0))
           1000)

;; Benchmark 13: Frequency table
(benchmark "Frequency Table (n=10000)"
           (lambda ()
             (define data (for/list ([i (in-range 10000)])
                           (bet 'A 'B 'C)))
             (frequency-table data))
           10)

;; Benchmark 14: Correlation calculation
(benchmark "Correlation (n=1000)"
           (lambda ()
             (define x (for/list ([i (in-range 1000)]) (random)))
             (define y (for/list ([i (in-range 1000)]) (random)))
             (correlation x y))
           100)

;; Benchmark 15: Bet probability estimation
(benchmark "Bet Probability Estimation (n=10000)"
           (lambda ()
             (bet-probability 10000
                            (lambda (x) (equal? x 'A))
                            'A 'B 'C))
           10)

(displayln "\n╔══════════════════════════════════════════╗")
(displayln "║   Benchmarks Complete                    ║")
(displayln "╚══════════════════════════════════════════╝")
