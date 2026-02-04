#lang racket
;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Risk of Ruin Protection
;;
;; Prevents catastrophic loss scenarios through Monte Carlo simulation
;; and mathematical risk analysis. Protects against Kelly criterion violations
;; and implements stake limits.

(provide risk-of-ruin
         kelly-criterion
         optimal-stake
         bet/risk-protected
         stake-limit
         max-drawdown
         ruin-probability
         safe-stake?
         wealth-trajectory
         make-risk-validator)

(require "statistics.rkt")
(require "distributions.rkt")

;; Calculate risk of ruin probability
;; Uses gambler's ruin formula for repeated bets
(define (risk-of-ruin initial-wealth target-wealth stake win-prob [max-trials 10000])
  (cond
    [(= win-prob 0.5)
     ;; Fair game: RoR = (target - initial) / target
     (/ (- target-wealth initial-wealth) target-wealth)]
    [else
     ;; Unfair game: use simulation
     (define lose-prob (- 1 win-prob))
     (define q/p (/ lose-prob win-prob))
     (cond
       [(>= initial-wealth target-wealth) 0.0]
       [else
        ;; Monte Carlo simulation
        (define simulations
          (for/list ([i max-trials])
            (let loop ([wealth initial-wealth])
              (cond
                [(>= wealth target-wealth) 'survive]
                [(<= wealth 0) 'ruin]
                [else
                 (loop (if (< (random) win-prob)
                           (+ wealth stake)
                           (- wealth stake)))]))))
        (/ (count (lambda (x) (eq? x 'ruin)) simulations)
           max-trials)])]))

;; Kelly Criterion: optimal bet size to maximize long-term growth
;; f* = (bp - q) / b
;; where: b = odds, p = win probability, q = lose probability
(define (kelly-criterion win-prob odds)
  (define lose-prob (- 1 win-prob))
  (define edge (- (* odds win-prob) lose-prob))
  (max 0 (/ edge odds)))

;; Calculate optimal stake using Kelly criterion with safety margin
(define (optimal-stake bankroll win-prob odds [kelly-fraction 0.25])
  ;; Use fractional Kelly for safety (typically 1/4 or 1/2 Kelly)
  (define full-kelly (kelly-criterion win-prob odds))
  (* bankroll full-kelly kelly-fraction))

;; Simulate wealth trajectory over multiple bets
(define (wealth-trajectory initial-wealth stake win-prob odds num-bets)
  (let loop ([wealth initial-wealth] [history (list initial-wealth)] [bets-left num-bets])
    (if (or (= bets-left 0) (<= wealth 0))
        (reverse history)
        (let ([new-wealth (if (< (random) win-prob)
                             (+ wealth (* stake odds))
                             (- wealth stake))])
          (loop new-wealth (cons new-wealth history) (- bets-left 1))))))

;; Calculate maximum drawdown from wealth trajectory
(define (max-drawdown trajectory)
  (define (running-max lst)
    (let loop ([lst lst] [current-max (first lst)] [result '()])
      (if (null? lst)
          (reverse result)
          (let ([new-max (max current-max (first lst))])
            (loop (rest lst) new-max (cons new-max result))))))

  (if (empty? trajectory)
      0
      (let* ([running-maxes (running-max trajectory)]
             [drawdowns (map (lambda (wealth peak) (- peak wealth))
                           trajectory
                           running-maxes)])
        (apply max drawdowns))))

;; Check if a stake is safe given bankroll and risk parameters
(define (safe-stake? stake bankroll win-prob odds [max-risk 0.05])
  (define kelly-optimal (kelly-criterion win-prob odds))
  (define stake-fraction (/ stake bankroll))

  ;; Stake should be <= 1/4 Kelly and not exceed max-risk of bankroll
  (and (<= stake-fraction (* kelly-optimal 0.25))
       (<= stake-fraction max-risk)))

;; Calculate ruin probability for a given strategy
(define (ruin-probability initial-wealth stake win-prob odds ruin-threshold [num-sims 10000])
  (define simulations
    (for/list ([i num-sims])
      (let loop ([wealth initial-wealth] [bets 0])
        (cond
          [(>= bets 1000) 'survive]  ;; Survived 1000 bets
          [(<= wealth ruin-threshold) 'ruin]
          [else
           (loop (if (< (random) win-prob)
                     (+ wealth (* stake odds))
                     (- wealth stake))
                 (+ bets 1))]))))

  (/ (count (lambda (x) (eq? x 'ruin)) simulations)
     num-sims))

;; Stake limit validator
(define (stake-limit bankroll max-stake-pct)
  (lambda (stake)
    (define max-allowed (* bankroll max-stake-pct))
    (when (> stake max-allowed)
      (error 'stake-limit
             "Stake $~a exceeds limit of ~a% of bankroll ($~a)"
             stake
             (* max-stake-pct 100)
             max-allowed))
    stake))

;; Risk-protected bet execution
(define (bet/risk-protected value stake bankroll win-prob odds
                           #:max-risk [max-risk 0.05]
                           #:kelly-fraction [kelly-fraction 0.25])
  ;; Validate stake is safe
  (unless (safe-stake? stake bankroll win-prob odds max-risk)
    (error 'bet/risk-protected
           "Stake $~a is too risky for bankroll $~a (win-prob: ~a, odds: ~a)"
           stake bankroll win-prob odds))

  ;; Check Kelly criterion
  (define kelly-stake (optimal-stake bankroll win-prob odds kelly-fraction))
  (when (> stake kelly-stake)
    (error 'bet/risk-protected
           "Stake $~a exceeds Kelly-optimal stake of $~a"
           stake kelly-stake))

  ;; Execute bet
  (if (< (random) win-prob)
      (+ value (* stake odds))
      (- value stake)))

;; Create a custom risk validator
(define (make-risk-validator max-risk kelly-fraction ruin-threshold)
  (lambda (stake bankroll win-prob odds)
    (and (safe-stake? stake bankroll win-prob odds max-risk)
         (let ([rp (ruin-probability bankroll stake win-prob odds ruin-threshold 1000)])
           (< rp 0.01)))))  ;; Less than 1% ruin probability

;; Example usage
(module+ test
  (require rackunit)

  (test-case "Kelly criterion"
    ;; Fair coin flip at 2:1 odds
    (check-true (< (abs (- (kelly-criterion 0.5 2) 0.25)) 0.01))
    ;; Edge bet: 60% win at 2:1 odds
    (check-true (< (abs (- (kelly-criterion 0.6 2) 0.4)) 0.01)))

  (test-case "Safe stake validation"
    (check-true (safe-stake? 25 1000 0.55 2))   ;; 2.5% of bankroll
    (check-false (safe-stake? 200 1000 0.55 2))) ;; 20% of bankroll - too risky

  (test-case "Risk of ruin"
    ;; Small stake, good odds - low RoR
    (define ror-low (risk-of-ruin 1000 2000 10 0.55 1000))
    (check-true (< ror-low 0.2))

    ;; Large stake, bad odds - high RoR
    (define ror-high (risk-of-ruin 1000 2000 500 0.45 1000))
    (check-true (> ror-high 0.5))))

(module+ main
  (displayln "=== Risk of Ruin Protection Examples ===\n")

  (displayln "1. Kelly Criterion (optimal bet sizing):")
  (define bankroll 10000)
  (define win-prob 0.55)
  (define odds 2.0)
  (displayln (format "   Bankroll: $~a" bankroll))
  (displayln (format "   Win probability: ~a%" (* win-prob 100)))
  (displayln (format "   Odds: ~a:1" odds))
  (displayln (format "   Full Kelly fraction: ~a%"
                    (* (kelly-criterion win-prob odds) 100)))
  (displayln (format "   Recommended stake (1/4 Kelly): $~a"
                    (optimal-stake bankroll win-prob odds 0.25)))

  (displayln "\n2. Risk of Ruin Probability:")
  (define stake-safe 100)
  (define stake-risky 1000)
  (displayln (format "   Safe stake ($~a): ~a% ruin probability"
                    stake-safe
                    (* (ruin-probability bankroll stake-safe win-prob odds 0) 100)))
  (displayln (format "   Risky stake ($~a): ~a% ruin probability"
                    stake-risky
                    (* (ruin-probability bankroll stake-risky win-prob odds 0) 100)))

  (displayln "\n3. Wealth Trajectory Simulation:")
  (define trajectory (wealth-trajectory bankroll 200 0.55 2 100))
  (displayln (format "   Initial wealth: $~a" (first trajectory)))
  (displayln (format "   Final wealth: $~a" (last trajectory)))
  (displayln (format "   Maximum drawdown: $~a" (max-drawdown trajectory)))
  (displayln (format "   Return: ~a%"
                    (* (/ (- (last trajectory) bankroll) bankroll) 100))))
