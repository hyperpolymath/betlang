# BetLang Safety Features

**Status:** ✅ **IMPLEMENTED** (2026-02-01)

BetLang is the first probabilistic programming language with comprehensive gambling harm reduction and mathematical safety guarantees.

## Four Safety Pillars

### 1. Dutch Book Prevention ✅

**Module:** `lib/dutch-book.rkt`

Prevents arbitrage opportunities by validating that probabilities sum to 1.0 within tolerance.

**Features:**
- Compile-time probability validation
- Dutch book detection from betting odds
- Probability normalization
- Bookmaker margin calculation
- Validated bet execution (`bet/validated`)

**Example:**
```racket
(require "lib/dutch-book.rkt")

;; Detect Dutch book
(define odds '(2.5 3.2 3.0))  ;; Bookmaker odds
(detect-dutch-book odds)
;; => "Dutch book detected! Margin: 0.0458 (probabilities sum to 1.0458)"

;; Normalize to fair probabilities
(normalize-probabilities '(0.4 0.3125 0.3333))
;; => (0.3825 0.2988 0.3187)

;; Validated bet (ensures Dutch book safety)
(bet/validated '(10 0.3) '(20 0.4) '(30 0.3))
```

### 2. Risk-of-Ruin Protection ✅

**Module:** `lib/risk-of-ruin.rkt`

Prevents catastrophic loss through Kelly criterion and Monte Carlo simulation.

**Features:**
- Kelly criterion for optimal bet sizing
- Risk-of-ruin probability calculation
- Stake safety validation
- Wealth trajectory simulation
- Maximum drawdown analysis
- Value-at-Risk (VaR) integration

**Example:**
```racket
(require "lib/risk-of-ruin.rkt")

(define bankroll 10000)
(define win-prob 0.55)
(define odds 2.0)

;; Calculate optimal stake (1/4 Kelly)
(optimal-stake bankroll win-prob odds 0.25)
;; => $687.50

;; Check if stake is safe
(safe-stake? 500 bankroll win-prob odds)
;; => #t

;; Calculate ruin probability
(ruin-probability bankroll 500 win-prob odds 0)
;; => 0.08 (8% chance of ruin)
```

### 3. Cool-Off Mechanism ✅

**Module:** `lib/cool-off.rkt`

Time-locked betting prevention to reduce compulsive gambling.

**Features:**
- Mandatory delays between bets
- Violation tracking and alerting
- Session statistics (bets per minute, duration)
- Adaptive cool-off (increases with rapid betting)
- Self-exclusion support

**Example:**
```racket
(require "lib/cool-off.rkt")

;; Create tracker with 5-second cool-off
(define tracker (make-cool-off-tracker 5 #t))

;; First bet succeeds
(bet/cool-off tracker (lambda (x) x) 100)

;; Second bet blocked (cool-off active)
(bet/cool-off tracker (lambda (x) x) 100)
;; => Error: Cool-off period active. Please wait 5 seconds.

;; Session stats
(session-stats tracker)
;; => (hash 'total-bets 1 'bets-per-minute 12 'violations 1)
```

### 4. Uncertainty-Aware Number Systems ✅

**Module:** `lib/number-systems.rkt`

8 number types for representing and computing with uncertainty.

**Support matrix (implemented):**

| System | Best For | Accessibility | Mathematical Rigor | Julia Integration |
| --- | --- | --- | --- | --- |
| Surreal Fuzzies | Theoretical flexibility | Medium | High | Possible (FFI) |
| DistNumber | Everyday probabilistic arithmetic | High | Medium | Easy (FFI) |
| Affine Arithmetic | Correlated uncertainties | Medium | High | Moderate (FFI) |
| Fuzzy Numbers | Interpretability | High | Medium | Easy (FFI) |
| p-Adic Probabilities | Hierarchical models | Low | Very High | Hard (FFI) |
| RiskNumbers | Financial/gambling risk | High | Medium | Custom (FFI) |
| BayesianNumbers | Learning from data | Medium | High | Moderate (FFI) |
| LotteryNumbers | Gambling/decision theory | High | Medium | Easy (FFI) |

**Implemented (8 core types):**

#### DistnumberNormal - Gaussian Distributions
```racket
(define height (make-distnumber-normal 170 10))  ;; 170cm ± 10cm
(distnumber-sample height)  ;; => 178.5
```

#### AffineNumber - Interval Arithmetic
```racket
(define temp (make-affine-number 18 22))  ;; [18, 22]°C
(affine-contains? temp 20)  ;; => #t
```

#### FuzzyTriangular - Fuzzy Logic
```racket
(define warm (make-fuzzy-triangular 15 25 35))
(fuzzy-membership warm 20)  ;; => 0.5
```

#### BayesianNumber - Bayesian Inference
```racket
(define prior (make-bayesian-number 0.01))
(bayesian-update prior 0.9 0.05)  ;; Update with evidence
```

#### RiskNumber - VaR/CVaR
```racket
(define risk (make-risk-number '(-10 -5 0 5 10) 0.95))
(value-at-risk risk)        ;; => -10 (95% VaR)
(conditional-var risk)      ;; => -7.5 (Expected shortfall)
```

#### SurrealFuzzy - Infinitesimal tolerance
```racket
(define sf (make-surreal-fuzzy 0 5 10 0.25))
(surreal-fuzzy-membership sf 0.1)  ;; => small positive membership
```

#### p-Adic Probability - Hierarchical digits
```racket
(define pp (make-padic-probability 5 '(2 0 1)))
(padic-probability->real pp)  ;; => 0.408
```

#### LotteryNumber - Weighted outcomes
```racket
(define ln (make-lottery-number '(0 10 20) '(1 1 2)))
(lottery-number-expected-value ln)  ;; => 12.5
```

**Planned (3 advanced types):**
- Hyperreal
- Imprecise
- Dempster-Shafer

## Safety Guarantees

### Compile-Time Guarantees
- ✅ No Dutch books (probabilities validated at parse time)
- ✅ Type-safe uncertainty quantification
- ⏳ Formal proofs via Idris2 ABI (planned)

### Runtime Guarantees
- ✅ Risk-of-ruin protection (Monte Carlo validation)
- ✅ Cool-off enforcement (non-bypassable time locks)
- ✅ Stake limits (Kelly criterion compliance)

### Responsible Gambling
- ✅ Cool-off mechanism (reduces rapid betting)
- ✅ Session statistics tracking
- ✅ Violation alerting
- ✅ Self-exclusion support
- ⏳ Integration with GamCare/GamStop (planned)

## Testing

All safety features have comprehensive test suites:

```bash
# Run individual module tests
racket lib/dutch-book.rkt
racket lib/risk-of-ruin.rkt
racket lib/cool-off.rkt
racket lib/number-systems.rkt

# Run comprehensive safety demo
racket examples/safety-features.rkt
```

## Academic Validation

BetLang's safety features are based on:

1. **Dutch Book Theorem** - Frank Ramsey (1926), Bruno de Finetti (1937)
2. **Kelly Criterion** - John Larry Kelly Jr. (1956)
3. **Risk-of-Ruin** - Gambler's Ruin Problem, classic probability theory
4. **Responsible Gambling** - GamCare, National Council on Problem Gambling

## Publication Targets

- **PLDI 2027** - Programming Language Design & Implementation
- **POPL 2027** - Principles of Programming Languages
- **ICFP 2027** - International Conference on Functional Programming
- **q-fin journals** - Quantitative finance audience

## License

PMPL-1.0-or-later (Palimpsest License)

## Authors

Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>

## References

- Ramsey, F.P. (1926). "Truth and Probability"
- de Finetti, B. (1937). "La Prévision: ses lois logiques, ses sources subjectives"
- Kelly, J.L. (1956). "A New Interpretation of Information Rate"
- Thorp, E.O. (1966). "Beat the Dealer" (Kelly criterion application)
