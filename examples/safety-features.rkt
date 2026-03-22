#lang racket
;; SPDX-License-Identifier: PMPL-1.0-or-later
;; BetLang Safety Features Demonstration
;;
;; This example demonstrates all four safety pillars:
;; 1. Dutch book prevention
;; 2. Risk-of-ruin protection
;; 3. Cool-off mechanism
;; 4. Uncertainty-aware number systems

(require "../core/betlang.rkt")
(require "../lib/dutch-book.rkt")
(require "../lib/risk-of-ruin.rkt")
(require "../lib/cool-off.rkt")
(require "../lib/number-systems.rkt")
(require "../lib/statistics.rkt")

(displayln "╔════════════════════════════════════════════════════════════════╗")
(displayln "║         BetLang: Safe Probabilistic Programming                ║")
(displayln "║         Demonstrating All Safety Features                      ║")
(displayln "╚════════════════════════════════════════════════════════════════╝\n")

;; ============================================================================
;; Scenario: Sports Betting Platform
;; ============================================================================

(displayln "🏈 SCENARIO: Football Match Betting\n")
(displayln "Match: Team A vs Team B")
(displayln "Bookmaker odds: Team A wins (2.5), Draw (3.2), Team B wins (3.0)\n")

;; ============================================================================
;; SAFETY FEATURE 1: Dutch Book Detection
;; ============================================================================

(displayln "═══ 1. DUTCH BOOK PREVENTION ═══\n")

(define odds '(2.5 3.2 3.0))
(define implied-probs (map (lambda (o) (/ 1 o)) odds))

(displayln (format "Bookmaker odds: ~a" odds))
(displayln (format "Implied probabilities: ~a" implied-probs))
(displayln (format "Sum of probabilities: ~a" (apply + implied-probs)))

(define margin (dutch-book-margin implied-probs))
(displayln (format "Bookmaker margin (overround): ~a%" (* margin 100)))

(if (> margin 0.01)
    (displayln "⚠️  WARNING: Dutch book detected! Probabilities sum to > 1.0")
    (displayln "✓ Fair odds - no Dutch book"))

(displayln "\nNormalizing to fair probabilities:")
(define fair-probs (normalize-probabilities implied-probs))
(displayln (format "Fair probabilities: ~a" fair-probs))
(displayln (format "Validation: ~a" (dutch-book-safe? fair-probs)))

;; ============================================================================
;; SAFETY FEATURE 2: Risk-of-Ruin Protection
;; ============================================================================

(displayln "\n═══ 2. RISK-OF-RUIN PROTECTION ═══\n")

(define bankroll 10000)
(define win-prob (first fair-probs))  ;; Team A win probability
(define payout-odds 2.5)

(displayln (format "Your bankroll: $~a" bankroll))
(displayln (format "Win probability (Team A): ~a%" (* win-prob 100)))
(displayln (format "Payout odds: ~a:1" payout-odds))

;; Kelly Criterion
(define kelly-fraction (kelly-criterion win-prob payout-odds))
(displayln (format "\nKelly Criterion: ~a% of bankroll" (* kelly-fraction 100)))

(define recommended-stake (optimal-stake bankroll win-prob payout-odds 0.25))
(displayln (format "Recommended stake (1/4 Kelly): $~a" recommended-stake))

;; Test different stake sizes
(displayln "\nRisk analysis for different stake sizes:")
(for ([stake '(100 500 1000 2000)])
  (define rp (ruin-probability bankroll stake win-prob (- payout-odds 1) 0 1000))
  (define safe (safe-stake? stake bankroll win-prob payout-odds 0.05))
  (displayln (format "  $~a: Ruin prob = ~a%, Safe? ~a"
                    stake
                    (* rp 100)
                    safe)))

;; ============================================================================
;; SAFETY FEATURE 3: Cool-Off Mechanism
;; ============================================================================

(displayln "\n═══ 3. COOL-OFF MECHANISM ═══\n")

(define cool-off-tracker (make-cool-off-tracker 3 #t))  ;; 3 second cool-off
(displayln (format "Cool-off period: ~a seconds" (get-cool-off-period cool-off-tracker)))

(displayln "\nPlacing first bet...")
(with-handlers ([exn:fail? (lambda (e)
                            (displayln (format "  Error: ~a" (exn-message e))))])
  (bet/cool-off cool-off-tracker
                (lambda (amount result)
                  (displayln (format "  ✓ Bet placed: $~a on ~a" amount result)))
                recommended-stake
                "Team A"))

(displayln "\nAttempting immediate second bet...")
(with-handlers ([exn:fail? (lambda (e)
                            (displayln (format "  ✗ BLOCKED: ~a"
                                              (exn-message e))))])
  (bet/cool-off cool-off-tracker
                (lambda (a r) (void))
                100
                "Team B"))

(define remaining (ceiling (time-until-next-bet cool-off-tracker)))
(displayln (format "\nTime until next bet allowed: ~a seconds" remaining))
(displayln (format "Violation count: ~a" (cool-off-violation-count cool-off-tracker)))

;; ============================================================================
;; SAFETY FEATURE 4: Uncertainty-Aware Number Systems
;; ============================================================================

(displayln "\n═══ 4. UNCERTAINTY-AWARE NUMBER SYSTEMS ═══\n")

;; 4a. DistnumberNormal - Model player performance as distribution
(displayln "4a. DistnumberNormal (Player Performance)")
(define team-a-goals (make-distnumber-normal 1.8 0.6))  ;; avg 1.8 goals ± 0.6
(define team-b-goals (make-distnumber-normal 1.2 0.5))  ;; avg 1.2 goals ± 0.5

(displayln (format "  Team A goals: N(~a, ~a)"
                  (distnumber-normal-mean team-a-goals)
                  (distnumber-normal-stddev team-a-goals)))
(displayln (format "  Team B goals: N(~a, ~a)"
                  (distnumber-normal-mean team-b-goals)
                  (distnumber-normal-stddev team-b-goals)))

(displayln "  Simulated match outcomes:")
(for ([i 5])
  (define goals-a (max 0 (floor (distnumber-sample team-a-goals))))
  (define goals-b (max 0 (floor (distnumber-sample team-b-goals))))
  (displayln (format "    Match ~a: Team A ~a - ~a Team B" (+ i 1) goals-a goals-b)))

;; 4b. AffineNumber - Temperature affects player performance
(displayln "\n4b. AffineNumber (Environmental Factors)")
(define temp-range (make-affine-number 18 28))  ;; 18-28°C
(displayln (format "  Match temperature range: [~a, ~a]°C"
                  (affine-number-lower temp-range)
                  (affine-number-upper temp-range)))
(displayln (format "  Optimal temp (22°C) in range? ~a"
                  (affine-contains? temp-range 22)))

;; 4c. FuzzyTriangular - Player form
(displayln "\n4c. FuzzyTriangular (Player Form Assessment)")
(define good-form (make-fuzzy-triangular 6 8 10))  ;; Rating 6-10
(displayln "  'Good Form' membership function:")
(for ([rating '(5 6 7 8 9 10)])
  (displayln (format "    Rating ~a: ~a"
                    rating
                    (fuzzy-membership good-form rating))))

;; 4d. BayesianNumber - Update beliefs with match stats
(displayln "\n4d. BayesianNumber (Belief Update)")
(define prior-win-prob 0.45)
(displayln (format "  Prior: Team A win probability = ~a%" (* prior-win-prob 100)))

(define bn (make-bayesian-number prior-win-prob))
;; Observe: Team A scored first (90% of first-scorers win)
(define likelihood-first-scorer 0.9)
(define evidence-first-score 0.5)  ;; 50% chance of any team scoring first
(define updated-bn (bayesian-update bn likelihood-first-scorer evidence-first-score))

(displayln "  Evidence: Team A scored first goal")
(displayln (format "  Posterior: ~a%" (* (bayesian-posterior updated-bn) 100)))

;; 4e. RiskNumber - Portfolio of bets
(displayln "\n4e. RiskNumber (Portfolio Risk)")
(define historical-returns '(-20 -15 -10 -5 0 5 10 15 20 25 30))
(define portfolio-risk (make-risk-number historical-returns 0.95))

(displayln (format "  Historical bet returns: ~a" historical-returns))
(displayln (format "  95% VaR: ~a%" (value-at-risk portfolio-risk)))
(displayln (format "  95% CVaR: ~a%" (conditional-var portfolio-risk)))

;; ============================================================================
;; COMBINED SAFETY CHECK
;; ============================================================================

(displayln "\n═══ COMBINED SAFETY VALIDATION ═══\n")

(define (safe-bet-check stake odds-list)
  (displayln (format "Validating bet: $~a" stake))

  ;; Check 1: Dutch book
  (define probs (normalize-probabilities (map (lambda (o) (/ 1 o)) odds-list)))
  (if (dutch-book-safe? probs)
      (displayln "  ✓ Dutch book check: PASS")
      (displayln "  ✗ Dutch book check: FAIL"))

  ;; Check 2: Risk of ruin
  (if (safe-stake? stake bankroll win-prob payout-odds)
      (displayln "  ✓ Risk check: PASS (within limits)")
      (displayln "  ✗ Risk check: FAIL (stake too high)"))

  ;; Check 3: Cool-off
  (if (cool-off-active? cool-off-tracker)
      (displayln "  ✗ Cool-off check: FAIL (still in cool-off)")
      (displayln "  ✓ Cool-off check: PASS"))

  (displayln ""))

(safe-bet-check 500 odds)
(safe-bet-check 5000 odds)  ;; Too risky

(displayln "\n╔════════════════════════════════════════════════════════════════╗")
(displayln "║  All safety features successfully demonstrated!               ║")
(displayln "║                                                                ║")
(displayln "║  BetLang ensures:                                              ║")
(displayln "║  ✓ No Dutch books (fair probabilities)                        ║")
(displayln "║  ✓ Risk-of-ruin protection (Kelly criterion)                  ║")
(displayln "║  ✓ Cool-off enforcement (responsible gambling)                ║")
(displayln "║  ✓ Uncertainty quantification (8 number systems)              ║")
(displayln "╚════════════════════════════════════════════════════════════════╝")
