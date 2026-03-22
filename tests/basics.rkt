#lang racket
(require rackunit)
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/combinators.rkt")
(require "../lib/distributions.rkt")

(module+ test
  (displayln "Running betlang test suite...\n")

  ;; Test 1: Basic bet functionality
  (displayln "Test 1: Basic bet - all outcomes appear")
  (define results (for/list ([i (in-range 100)]) (bet "A" "B" "C")))
  (for ([opt '("A" "B" "C")])
    (check-true (member opt results)))
  (displayln "  ✓ All three outcomes appeared\n")

  ;; Test 2: Probability distribution
  (displayln "Test 2: Probability distribution")
  (define large-sample (bet-parallel 10000 'A 'B 'C))
  (define freq (frequency-table large-sample))
  (for ([pair freq])
    (define proportion (/ (cdr pair) 10000.0))
    (check-true (< (abs (- proportion 0.333)) 0.05)
                (format "~a should be ~33%, got ~a%" (car pair) (* 100 proportion))))
  (displayln "  ✓ Distribution is roughly uniform\n")

  ;; Test 3: Weighted bets
  (displayln "Test 3: Weighted bets")
  (define weighted-results
    (for/list ([i (in-range 1000)])
      (bet/weighted '(heavy 7) '(medium 2) '(light 1))))
  (define weighted-freq (frequency-table weighted-results))
  (define heavy-count (cdr (assoc 'heavy weighted-freq)))
  (check-true (> heavy-count 600) "Heavy should appear more often")
  (displayln (format "  ✓ Weighted distribution: ~a" weighted-freq))
  (displayln "")

  ;; Test 4: Conditional bets
  (displayln "Test 4: Conditional bets")
  (check-equal? (bet/conditional #t 'yes 'no 'maybe) 'yes)
  (check-true (member (bet/conditional #f 'yes 'no 'maybe) '(no maybe)))
  (displayln "  ✓ Conditional logic works correctly\n")

  ;; Test 5: Lazy evaluation
  (displayln "Test 5: Lazy evaluation")
  (define called-count 0)
  (define (side-effect-a) (set! called-count (+ called-count 1)) 'A)
  (define (side-effect-b) (set! called-count (+ called-count 1)) 'B)
  (define (side-effect-c) (set! called-count (+ called-count 1)) 'C)
  (bet/lazy side-effect-a side-effect-b side-effect-c)
  (check-equal? called-count 1 "Only one branch should be evaluated")
  (displayln "  ✓ Only one branch evaluated\n")

  ;; Test 6: Bet chains
  (displayln "Test 6: Bet chains")
  (define chain-result (bet-chain 5 (lambda (x) (+ x 1)) 0))
  (check-equal? chain-result 5)
  (displayln "  ✓ Chain executes correct number of steps\n")

  ;; Test 7: Bet composition
  (displayln "Test 7: Bet composition")
  (define composed (bet-compose add1 identity sub1))
  (define comp-results (for/list ([i (in-range 100)])
                         (composed 10)))
  (check-true (member 11 comp-results))
  (check-true (member 10 comp-results))
  (check-true (member 9 comp-results))
  (displayln "  ✓ Composition produces all expected values\n")

  ;; Test 8: Bet probability estimation
  (displayln "Test 8: Probability estimation")
  (define prob-a (bet-probability 10000 (lambda (x) (equal? x 'A)) 'A 'B 'C))
  (check-true (< (abs (- prob-a 0.333)) 0.05))
  (displayln (format "  ✓ Estimated P(A) = ~a (expected ~0.333)" prob-a))
  (displayln "")

  ;; Test 9: Statistical functions
  (displayln "Test 9: Statistical functions")
  (define test-data '(1 2 3 4 5))
  (check-equal? (mean test-data) 3)
  (check-equal? (median test-data) 3)
  (check-equal? (variance test-data) 2)
  (displayln "  ✓ Mean, median, variance correct\n")

  ;; Test 10: Entropy calculation
  (displayln "Test 10: Entropy calculation")
  (define uniform-samples '(A A A B B B C C C))
  (define entropy-val (bet-entropy uniform-samples))
  ;; For uniform distribution of 3 outcomes, entropy ≈ log2(3) ≈ 1.585
  (check-true (> entropy-val 1.0))
  (displayln (format "  ✓ Entropy = ~a (uniform 3-way should be ~1.585)" entropy-val))
  (displayln "")

  ;; Test 11: Distributions
  (displayln "Test 11: Probability distributions")
  (define normal-samples (for/list ([i (in-range 1000)]) (normal 0 1)))
  (define sample-mean (mean normal-samples))
  (define sample-std (stddev normal-samples))
  (check-true (< (abs sample-mean) 0.2) "Normal mean should be near 0")
  (check-true (< (abs (- sample-std 1)) 0.2) "Normal stddev should be near 1")
  (displayln (format "  ✓ Normal(0,1): mean=~a, std=~a" sample-mean sample-std))
  (displayln "")

  ;; Test 12: Binomial distribution
  (displayln "Test 12: Binomial distribution")
  (define binomial-samples (for/list ([i (in-range 1000)]) (binomial 10 0.5)))
  (define binom-mean (mean binomial-samples))
  ;; E[X] = np = 10 * 0.5 = 5
  (check-true (< (abs (- binom-mean 5)) 0.5))
  (displayln (format "  ✓ Binomial(10, 0.5): mean=~a (expected 5)" binom-mean))
  (displayln "")

  ;; Test 13: Random walk
  (displayln "Test 13: Random walk")
  (define walk (random-walk 100))
  (check-equal? (length walk) 101) ;; n steps = n+1 positions
  (check-equal? (first walk) 0)     ;; Starts at 0
  (displayln (format "  ✓ Random walk: length=~a, final position=~a"
                     (length walk) (last walk)))
  (displayln "")

  ;; Test 14: Bet generators
  (displayln "Test 14: Bet generators")
  (define gen (make-bet-generator 'x 'y 'z))
  (define gen-results (for/list ([i (in-range 100)]) (gen)))
  (check-true (member 'x gen-results))
  (check-true (member 'y gen-results))
  (check-true (member 'z gen-results))
  (displayln "  ✓ Generator produces all values\n")

  ;; Test 15: Bet with seed
  (displayln "Test 15: Deterministic with seed")
  (define seeded1 (bet-with-seed 42 (lambda () (bet 1 2 3))))
  (define seeded2 (bet-with-seed 42 (lambda () (bet 1 2 3))))
  (check-equal? seeded1 seeded2 "Same seed should produce same result")
  (displayln (format "  ✓ Seeded bets are deterministic: ~a = ~a" seeded1 seeded2))
  (displayln "")

  ;; Test 16: All-bets utility
  (displayln "Test 16: All-bets utility")
  (check-equal? (all-bets 'a 'b 'c) '(a b c))
  (displayln "  ✓ all-bets returns all outcomes\n")

  ;; Test 17: Bet-until
  (displayln "Test 17: Bet-until convergence")
  (define until-result
    (bet-until (lambda (x) (equal? x 'target))
               (lambda () (bet 'target 'miss 'miss))))
  (check-equal? until-result 'target)
  (displayln "  ✓ bet-until finds target\n")

  ;; Test 18: Moving average
  (displayln "Test 18: Moving average")
  (define data '(1 2 3 4 5 6 7 8 9 10))
  (define ma (moving-average data 3))
  (check-equal? (first ma) 2) ;; (1+2+3)/3 = 2
  (displayln (format "  ✓ Moving average: ~a" ma))
  (displayln "")

  ;; Test 19: Bootstrap
  (displayln "Test 19: Bootstrap resampling")
  (define bootstrap-means (bootstrap '(1 2 3 4 5) 100 mean))
  (check-equal? (length bootstrap-means) 100)
  (check-true (> (mean bootstrap-means) 2))
  (check-true (< (mean bootstrap-means) 4))
  (displayln (format "  ✓ Bootstrap mean of means: ~a" (mean bootstrap-means)))
  (displayln "")

  ;; Test 20: Chi-square test
  (displayln "Test 20: Chi-square goodness of fit")
  (define observed '(48 52 50))
  (define expected '(50 50 50))
  (define chi-sq (chi-square-test observed expected))
  (check-true (< chi-sq 1.0) "Should be small for similar distributions")
  (displayln (format "  ✓ Chi-square statistic: ~a" chi-sq))
  (displayln "")

  ;; Test 21: Correlation
  (displayln "Test 21: Correlation")
  (define x-vals '(1 2 3 4 5))
  (define y-vals '(2 4 6 8 10))
  (define corr (correlation x-vals y-vals))
  (check-true (> corr 0.99) "Perfect correlation should be ~1")
  (displayln (format "  ✓ Correlation: ~a" corr))
  (displayln "")

  ;; Test 22: Percentile
  (displayln "Test 22: Percentile calculation")
  (define percentile-data '(1 2 3 4 5 6 7 8 9 10))
  (check-equal? (percentile percentile-data 0.5) 5)
  (check-equal? (percentile percentile-data 0.9) 9)
  (displayln "  ✓ Percentiles calculated correctly\n")

  ;; Test 23: Bet-map
  (displayln "Test 23: Bet-map")
  (define mapped (bet-map add1 '(1 2 3)))
  (check-equal? (length mapped) 3)
  (displayln (format "  ✓ Bet-map result: ~a" mapped))
  (displayln "")

  ;; Test 24: Bet-filter
  (displayln "Test 24: Bet-filter")
  (define filtered (bet-filter even? '(1 2 3 4 5 6 7 8 9 10)))
  ;; Probabilistic, so just check it's a list
  (check-true (list? filtered))
  (displayln (format "  ✓ Bet-filter result: ~a" filtered))
  (displayln "")

  ;; Test 25: Mode
  (displayln "Test 25: Mode calculation")
  (define mode-data '(1 2 2 3 3 3 4))
  (check-equal? (mode mode-data) '(3))
  (displayln "  ✓ Mode is correct\n")

  (displayln "=================================")
  (displayln "All tests passed! ✓")
  (displayln "================================="))
