#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/distributions.rkt")

;; Basic Tutorial for betlang

(displayln "=== betlang Basic Tutorial ===\n")

;; Section 1: The Basic Bet
(displayln "1. The Basic Bet Form")
(displayln "   (bet A B C) randomly selects one of three values\n")

(displayln "   Example: (bet 'red 'green 'blue)")
(displayln (format "   Result: ~a" (bet 'red 'green 'blue)))

(displayln "\n   Example: (bet 1 2 3)")
(displayln (format "   Result: ~a" (bet 1 2 3)))

(displayln "\n   Example: (bet \"Win\" \"Draw\" \"Lose\")")
(displayln (format "   Result: ~a\n" (bet "Win" "Draw" "Lose")))

;; Section 2: Understanding Probability
(displayln "2. Understanding Ternary Probability")
(displayln "   Each outcome has equal probability: 1/3 ≈ 33.33%\n")

(define trials (bet-parallel 1000 'A 'B 'C))
(define freq (frequency-table trials))
(displayln "   Running 1000 trials of (bet 'A 'B 'C):")
(for ([pair freq])
  (displayln (format "     ~a appeared ~a times (~a%)"
                     (car pair) (cdr pair)
                     (exact->inexact (* 100 (/ (cdr pair) 1000))))))

;; Section 3: Weighted Bets
(displayln "\n3. Weighted Bets")
(displayln "   Use bet/weighted for non-uniform probabilities\n")

(displayln "   Example: (bet/weighted '(\"Common\" 5) '(\"Uncommon\" 3) '(\"Rare\" 2))")
(displayln "   Probabilities: Common=50%, Uncommon=30%, Rare=20%\n")

(define weighted-trials
  (for/list ([i (in-range 1000)])
    (bet/weighted '("Common" 5) '("Uncommon" 3) '("Rare" 2))))
(define weighted-freq (frequency-table weighted-trials))
(displayln "   Results from 1000 trials:")
(for ([pair weighted-freq])
  (displayln (format "     ~a: ~a times (~a%)"
                     (car pair) (cdr pair)
                     (exact->inexact (* 100 (/ (cdr pair) 1000))))))

;; Section 4: Conditional Bets
(displayln "\n4. Conditional Bets")
(displayln "   Use bet/conditional to make decisions based on conditions\n")

(define (weather-decision temperature)
  (bet/conditional (> temperature 70)
                   "Go swimming"
                   "Stay inside"
                   "Go for a walk"))

(displayln "   Example: Weather decisions based on temperature")
(displayln (format "     60°F → ~a" (weather-decision 60)))
(displayln (format "     75°F → ~a" (weather-decision 75)))
(displayln (format "     50°F → ~a" (weather-decision 50)))

;; Section 5: Chaining Bets
(displayln "\n5. Chaining Bets Together")
(displayln "   Use bet-chain to create sequences of probabilistic decisions\n")

(define (random-walk-step x)
  (+ x (bet -1 0 1)))

(define walk-result (bet-chain 10 random-walk-step 0))
(displayln (format "   Random walk of 10 steps starting at 0: ~a" walk-result))

;; Section 6: Parallel Bets
(displayln "\n6. Running Parallel Simulations")
(displayln "   Use bet-parallel to run many trials at once\n")

(define coin-flips (bet-parallel 20 'heads 'tails 'edge))
(displayln (format "   20 coin flips: ~a" coin-flips))
(displayln (format "   Heads: ~a, Tails: ~a, Edge: ~a"
                   (count (lambda (x) (equal? x 'heads)) coin-flips)
                   (count (lambda (x) (equal? x 'tails)) coin-flips)
                   (count (lambda (x) (equal? x 'edge)) coin-flips)))

;; Section 7: Statistical Analysis
(displayln "\n7. Statistical Analysis of Bets")
(displayln "   Calculate probabilities and expected values\n")

(define prob-estimate
  (bet-probability 10000 (lambda (x) (equal? x 'jackpot))
                   'jackpot 'miss 'miss))
(displayln (format "   Probability of 'jackpot: ~a (expected: 0.333...)"
                   (exact->inexact prob-estimate)))

(define expected-value
  (bet-expect 10000 (lambda (x) x) 1 2 3))
(displayln (format "   Expected value of (bet 1 2 3): ~a (expected: 2)"
                   (exact->inexact expected-value)))

;; Section 8: Lazy Evaluation
(displayln "\n8. Lazy Evaluation")
(displayln "   Use bet/lazy to only compute the selected branch\n")

(define (expensive-a) (displayln "     Computing A...") 'A)
(define (expensive-b) (displayln "     Computing B...") 'B)
(define (expensive-c) (displayln "     Computing C...") 'C)

(displayln "   Only one branch is evaluated:")
(define lazy-result (bet/lazy expensive-a expensive-b expensive-c))
(displayln (format "   Result: ~a" lazy-result))

;; Section 9: Composed Bets
(displayln "\n9. Function Composition with Bets")
(displayln "   Create functions that randomly choose between strategies\n")

(define process-number (bet-compose add1 identity sub1))
(displayln (format "   Applying random operation to 10: ~a" (process-number 10)))
(displayln (format "   Applying random operation to 10: ~a" (process-number 10)))
(displayln (format "   Applying random operation to 10: ~a" (process-number 10)))

;; Section 10: Real-World Example
(displayln "\n10. Real-World Example: Customer Service Simulator")

(define (customer-arrival-time)
  ;; Time between customers in minutes
  (bet 2 5 10))

(define (service-outcome)
  (bet/weighted '("satisfied" 7) '("neutral" 2) '("complaint" 1)))

(define (simulate-day hours)
  (let loop ([time 0] [customers 0] [outcomes '()])
    (if (>= time (* hours 60))
        (list customers outcomes)
        (let ([wait (customer-arrival-time)]
              [outcome (service-outcome)])
          (loop (+ time wait)
                (+ customers 1)
                (cons outcome outcomes))))))

(define day-results (simulate-day 8))
(define total-customers (first day-results))
(define all-outcomes (second day-results))
(define outcome-freq (frequency-table all-outcomes))

(displayln "\n   8-hour day simulation:")
(displayln (format "     Total customers: ~a" total-customers))
(displayln "     Outcomes:")
(for ([pair outcome-freq])
  (displayln (format "       ~a: ~a (~a%)"
                     (car pair) (cdr pair)
                     (exact->inexact (* 100 (/ (cdr pair) total-customers))))))

(displayln "\n=== Tutorial Complete ===")
(displayln "Try these examples in the REPL!")
(displayln "Run: racket repl/shell.rkt")
