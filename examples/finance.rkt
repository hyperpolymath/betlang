#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/distributions.rkt")

;; Financial Modeling with betlang

(displayln "=== Financial Modeling Examples ===\n")

;; 1. Stock Price Simulation (Geometric Brownian Motion)
(displayln "1. Stock Price Simulation (GBM)")
(define (stock-price-gbm S0 mu sigma T steps)
  (define dt (/ T steps))
  (define drift (* (- mu (* 0.5 sigma sigma)) dt))
  (define vol (* sigma (sqrt dt)))

  (let loop ([prices (list S0)] [t 0])
    (if (>= t steps)
        (reverse prices)
        (let* ([prev-price (first prices)]
               [dW (normal 0 1)]
               [return (exp (+ drift (* vol dW)))]
               [new-price (* prev-price return)])
          (loop (cons new-price prices) (+ t 1))))))

(define price-path (stock-price-gbm 100 0.08 0.2 1.0 252))
(displayln (format "   Initial price: $100"))
(displayln (format "   Final price: $~a" (exact->inexact (last price-path))))
(displayln (format "   Min price: $~a" (apply min price-path)))
(displayln (format "   Max price: $~a" (apply max price-path)))
(displayln "")

;; 2. European Option Pricing (Monte Carlo)
(displayln "2. European Call Option Pricing (Monte Carlo)")
(define (european-call S0 K T r sigma n-sims)
  (define payoffs
    (for/list ([i (in-range n-sims)])
      (define final-path (stock-price-gbm S0 r sigma T 252))
      (define ST (last final-path))
      (max 0 (- ST K))))

  (* (exp (* (- r) T)) (mean payoffs)))

(define call-price (european-call 100 105 1.0 0.05 0.2 10000))
(displayln (format "   Stock price: $100"))
(displayln (format "   Strike price: $105"))
(displayln (format "   Time to maturity: 1 year"))
(displayln (format "   Risk-free rate: 5%"))
(displayln (format "   Volatility: 20%"))
(displayln (format "   Call option price: $~a" call-price))
(displayln "")

;; 3. Portfolio Value at Risk (VaR)
(displayln "3. Portfolio Value at Risk (VaR)")
(define (portfolio-var weights returns-mean returns-std initial-value confidence n-sims)
  (define portfolio-returns
    (for/list ([i (in-range n-sims)])
      (apply +
             (for/list ([w weights]
                        [mu returns-mean]
                        [sigma returns-std])
               (* w (normal mu sigma))))))

  (define portfolio-values
    (for/list ([r portfolio-returns])
      (* initial-value (+ 1 r))))

  (define sorted-values (sort portfolio-values <))
  (define var-idx (inexact->exact (floor (* (- 1 confidence) n-sims))))
  (define var (- initial-value (list-ref sorted-values var-idx)))

  var)

(define var-95 (portfolio-var '(0.5 0.3 0.2)
                              '(0.08 0.06 0.10)
                              '(0.15 0.12 0.20)
                              1000000
                              0.95
                              10000))
(displayln (format "   Portfolio value: $1,000,000"))
(displayln (format "   95% VaR (1-day): $~a" (exact->inexact var-95)))
(displayln (format "   There's a 5% chance of losing more than $~a in one day"
                   (exact->inexact var-95)))
(displayln "")

;; 4. Credit Risk Modeling (Merton Model)
(displayln "4. Credit Risk - Probability of Default")
(define (merton-default-prob asset-value debt sigma T r n-sims)
  (define defaults
    (count (lambda (_)
             (define asset-path (stock-price-gbm asset-value r sigma T 12))
             (< (last asset-path) debt))
           (range n-sims)))

  (/ defaults (exact->inexact n-sims)))

(define pd (merton-default-prob 1000 800 0.25 1.0 0.03 10000))
(displayln (format "   Company asset value: $1000M"))
(displayln (format "   Debt due in 1 year: $800M"))
(displayln (format "   Asset volatility: 25%"))
(displayln (format "   Probability of default: ~a%" (* 100 pd)))
(displayln "")

;; 5. Interest Rate Term Structure
(displayln "5. Interest Rate Simulation (Vasicek Model)")
(define (vasicek-rates r0 kappa theta sigma T steps)
  (define dt (/ T steps))

  (let loop ([rates (list r0)] [t 0])
    (if (>= t steps)
        (reverse rates)
        (let* ([r-current (first rates)]
               [dr (+ (* kappa (- theta r-current) dt)
                      (* sigma (sqrt dt) (normal 0 1)))]
               [r-new (+ r-current dr)])
          (loop (cons r-new rates) (+ t 1))))))

(define rates-path (vasicek-rates 0.05 0.3 0.06 0.02 5.0 60))
(displayln (format "   Initial rate: 5%"))
(displayln (format "   Long-term mean: 6%"))
(displayln (format "   Mean reversion speed: 0.3"))
(displayln (format "   Final rate: ~a%" (* 100 (last rates-path))))
(displayln (format "   Average rate: ~a%" (* 100 (mean rates-path))))
(displayln "")

;; 6. Portfolio Optimization (Ternary Choice)
(displayln "6. Portfolio Allocation with Uncertainty")
(define (portfolio-return allocation)
  ;; Three possible market scenarios
  (bet
   ;; Bull market: stocks perform well
   (+ (* (first allocation) 0.15)
      (* (second allocation) 0.08)
      (* (third allocation) 0.03))
   ;; Bear market: bonds perform better
   (+ (* (first allocation) -0.05)
      (* (second allocation) 0.06)
      (* (third allocation) 0.02))
   ;; Sideways: balanced
   (+ (* (first allocation) 0.07)
      (* (second allocation) 0.05)
      (* (third allocation) 0.025))))

(define allocations
  '((0.7 0.2 0.1)   ;; Aggressive
    (0.5 0.3 0.2)   ;; Balanced
    (0.3 0.4 0.3))) ;; Conservative

(displayln "   Simulating 1000 scenarios for each allocation:")
(for ([alloc allocations]
      [name '("Aggressive" "Balanced" "Conservative")])
  (define returns
    (for/list ([i (in-range 1000)])
      (portfolio-return alloc)))
  (displayln (format "   ~a (~a stocks, ~a bonds, ~a cash):"
                     name
                     (first alloc) (second alloc) (third alloc)))
  (displayln (format "     Mean return: ~a%"
                     (* 100 (mean returns))))
  (displayln (format "     Std dev: ~a%"
                     (* 100 (stddev returns)))))
(displayln "")

;; 7. Dividend Discount Model with Uncertainty
(displayln "7. Stock Valuation with Uncertain Growth")
(define (ddm-valuation current-div r scenarios)
  (define growth-rate (bet (first scenarios) (second scenarios) (third scenarios)))
  (if (>= growth-rate r)
      +inf.0  ;; Undefined when g >= r
      (/ (* current-div (+ 1 growth-rate))
         (- r growth-rate))))

(define valuations
  (for/list ([i (in-range 1000)])
    (ddm-valuation 3.00 0.10 '(0.05 0.07 0.09))))

(displayln (format "   Current dividend: $3.00"))
(displayln (format "   Required return: 10%"))
(displayln (format "   Growth scenarios: 5%, 7%, 9% (equal probability)"))
(displayln (format "   Mean valuation: $~a"
                   (mean (filter (lambda (x) (not (infinite? x))) valuations))))
(displayln (format "   Median valuation: $~a"
                   (median (filter (lambda (x) (not (infinite? x))) valuations))))
(displayln "")

;; 8. Currency Exchange Rate Random Walk
(displayln "8. Currency Exchange Rate Simulation")
(define (currency-walk initial steps volatility)
  (let loop ([rates (list initial)] [t 0])
    (if (>= t steps)
        (reverse rates)
        (let* ([current (first rates)]
               [change (bet (* -1 volatility)
                           0
                           volatility)]
               [new-rate (* current (+ 1 change))])
          (loop (cons new-rate rates) (+ t 1))))))

(define usd-eur (currency-walk 1.10 30 0.005))
(displayln (format "   Initial USD/EUR rate: 1.10"))
(displayln (format "   After 30 days: ~a" (last usd-eur)))
(displayln (format "   Maximum: ~a" (apply max usd-eur)))
(displayln (format "   Minimum: ~a" (apply min usd-eur)))
(displayln "")

;; 9. Bond Pricing with Default Risk
(displayln "9. Risky Bond Pricing")
(define (risky-bond-price face-value coupon-rate maturity risk-free-rate default-prob recovery-rate)
  (define default-scenario
    (bet/weighted
     ;; No default: full payments
     (list (for/sum ([t (in-range 1 (+ maturity 1))])
             (/ (* face-value coupon-rate)
                (expt (+ 1 risk-free-rate) t)))
           (- 1 default-prob))
     ;; Default in year 1: recovery only
     (list (* face-value recovery-rate)
           (/ default-prob 3))
     ;; Default in year 2: some coupons + recovery
     (list (+ (/ (* face-value coupon-rate)
                 (+ 1 risk-free-rate))
              (/ (* face-value recovery-rate)
                 (expt (+ 1 risk-free-rate) 2)))
           (/ default-prob 3))))

  (define final-pv
    (+ (/ face-value (expt (+ 1 risk-free-rate) maturity))
       default-scenario))

  final-pv)

(define bond-prices
  (for/list ([i (in-range 1000)])
    (risky-bond-price 1000 0.06 5 0.03 0.10 0.40)))

(displayln (format "   Face value: $1000"))
(displayln (format "   Coupon rate: 6%"))
(displayln (format "   Maturity: 5 years"))
(displayln (format "   Risk-free rate: 3%"))
(displayln (format "   Default probability: 10%"))
(displayln (format "   Recovery rate: 40%"))
(displayln (format "   Average bond price: $~a" (mean bond-prices)))
(displayln (format "   Price std dev: $~a" (stddev bond-prices)))
(displayln "")

;; 10. Market Crash Simulation
(displayln "10. Tail Risk - Market Crash Simulation")
(define (market-crash-sim initial years crash-prob crash-magnitude normal-return normal-vol)
  (for/fold ([value initial])
            ([year (in-range years)])
    (define crash? (< (random) crash-prob))
    (define return
      (if crash?
          crash-magnitude
          (normal normal-return normal-vol)))
    (* value (+ 1 return))))

(define simulations
  (for/list ([i (in-range 10000)])
    (market-crash-sim 100 10 0.10 -0.30 0.08 0.15)))

(displayln (format "   Initial investment: $100"))
(displayln (format "   Time horizon: 10 years"))
(displayln (format "   Annual crash probability: 10%"))
(displayln (format "   Crash magnitude: -30%"))
(displayln (format "   Normal return: 8% ± 15%"))
(displayln (format "   Mean final value: $~a" (mean simulations)))
(displayln (format "   Median final value: $~a" (median simulations)))
(displayln (format "   5th percentile: $~a" (percentile simulations 0.05)))
(displayln (format "   95th percentile: $~a" (percentile simulations 0.95)))

(displayln "\n=== Financial Modeling Complete ===")
