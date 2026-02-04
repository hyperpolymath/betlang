#lang racket
;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Uncertainty-Aware Number Systems
;;
;; 8 number systems for representing and computing with uncertainty:
;; 1. DistnumberNormal - Gaussian distributions
;; 2. AffineNumber - Interval arithmetic
;; 3. FuzzyTriangular - Fuzzy logic with triangular membership
;; 4. BayesianNumber - Bayesian inference
;; 5. RiskNumber - VaR/CVaR calculations
;; 6. SurrealFuzzy - Fuzzy logic with infinitesimal tolerance
;; 7. p-Adic Probability - Hierarchical digit-expansion probabilities
;; 8. LotteryNumber - Discrete outcomes with weighted uncertainty
;; Planned advanced types: Hyperreal, Imprecise, Dempster-Shafer

(provide ;; DistnumberNormal
         make-distnumber-normal
         distnumber-normal?
         distnumber-normal-mean
         distnumber-normal-stddev
         distnumber-add
         distnumber-mul
         distnumber-sample

         ;; AffineNumber (Interval Arithmetic)
         make-affine-number
         affine-number?
         affine-number-lower
         affine-number-upper
         affine-add
         affine-mul
         affine-contains?

         ;; FuzzyTriangular
         make-fuzzy-triangular
         fuzzy-triangular?
         fuzzy-membership
         fuzzy-and
         fuzzy-or
         fuzzy-not

         ;; BayesianNumber
         make-bayesian-number
         bayesian-number?
         bayesian-update
         bayesian-posterior

         ;; RiskNumber
         make-risk-number
         risk-number?
         value-at-risk
         conditional-var
         expected-shortfall

         ;; SurrealFuzzy
         make-surreal-fuzzy
         surreal-fuzzy?
         surreal-fuzzy-membership

         ;; p-Adic Probability
         make-padic-probability
         padic-probability?
         padic-probability->real
         padic-probability-refine

         ;; LotteryNumber
         make-lottery-number
         lottery-number?
         lottery-number-expected-value
         lottery-number-sample)

(require "distributions.rkt")
(require "statistics.rkt")

;; ============================================================================
;; 1. DistnumberNormal - Numbers as Gaussian Distributions
;; ============================================================================

(struct distnumber-normal (mean stddev) #:transparent)

(define (make-distnumber-normal mean stddev)
  (unless (>= stddev 0)
    (error 'make-distnumber-normal "Standard deviation must be non-negative"))
  (distnumber-normal mean stddev))

;; Addition: N(μ₁, σ₁²) + N(μ₂, σ₂²) = N(μ₁+μ₂, σ₁²+σ₂²)
(define (distnumber-add n1 n2)
  (make-distnumber-normal
   (+ (distnumber-normal-mean n1) (distnumber-normal-mean n2))
   (sqrt (+ (expt (distnumber-normal-stddev n1) 2)
           (expt (distnumber-normal-stddev n2) 2)))))

;; Multiplication (approximate for independent variables)
(define (distnumber-mul n1 n2)
  (define μ1 (distnumber-normal-mean n1))
  (define μ2 (distnumber-normal-mean n2))
  (define σ1 (distnumber-normal-stddev n1))
  (define σ2 (distnumber-normal-stddev n2))

  ;; For X~N(μ₁,σ₁²) and Y~N(μ₂,σ₂²) independent:
  ;; E[XY] = μ₁μ₂
  ;; Var[XY] ≈ μ₁²σ₂² + μ₂²σ₁² + σ₁²σ₂²
  (define mean (* μ1 μ2))
  (define variance (+ (* (expt μ1 2) (expt σ2 2))
                     (* (expt μ2 2) (expt σ1 2))
                     (* (expt σ1 2) (expt σ2 2))))
  (make-distnumber-normal mean (sqrt variance)))

;; Sample from the distribution
(define (distnumber-sample dn)
  (+ (distnumber-normal-mean dn)
     (* (distnumber-normal-stddev dn) (normal 0 1))))

;; ============================================================================
;; 2. AffineNumber - Interval Arithmetic
;; ============================================================================

(struct affine-number (lower upper) #:transparent)

(define (make-affine-number lower upper)
  (unless (<= lower upper)
    (error 'make-affine-number "Lower bound must be <= upper bound"))
  (affine-number lower upper))

;; Addition: [a,b] + [c,d] = [a+c, b+d]
(define (affine-add a1 a2)
  (make-affine-number
   (+ (affine-number-lower a1) (affine-number-lower a2))
   (+ (affine-number-upper a1) (affine-number-upper a2))))

;; Multiplication: [a,b] × [c,d] = [min, max] of all products
(define (affine-mul a1 a2)
  (define products
    (list (* (affine-number-lower a1) (affine-number-lower a2))
          (* (affine-number-lower a1) (affine-number-upper a2))
          (* (affine-number-upper a1) (affine-number-lower a2))
          (* (affine-number-upper a1) (affine-number-upper a2))))
  (make-affine-number (apply min products) (apply max products)))

;; Check if value is in interval
(define (affine-contains? an value)
  (and (>= value (affine-number-lower an))
       (<= value (affine-number-upper an))))

;; ============================================================================
;; 3. FuzzyTriangular - Fuzzy Logic
;; ============================================================================

(struct fuzzy-triangular (left center right) #:transparent)

(define (make-fuzzy-triangular left center right)
  (unless (and (<= left center) (<= center right))
    (error 'make-fuzzy-triangular "Must have left <= center <= right"))
  (fuzzy-triangular left center right))

;; Triangular membership function
(define (fuzzy-membership ft x)
  (define a (fuzzy-triangular-left ft))
  (define b (fuzzy-triangular-center ft))
  (define c (fuzzy-triangular-right ft))

  (cond
    [(<= x a) 0]
    [(and (> x a) (<= x b))
     (/ (- x a) (- b a))]
    [(and (> x b) (< x c))
     (/ (- c x) (- c b))]
    [else 0]))

;; Fuzzy AND (minimum)
(define (fuzzy-and m1 m2)
  (min m1 m2))

;; Fuzzy OR (maximum)
(define (fuzzy-or m1 m2)
  (max m1 m2))

;; Fuzzy NOT (complement)
(define (fuzzy-not m)
  (- 1 m))

;; ============================================================================
;; 4. BayesianNumber - Bayesian Inference
;; ============================================================================

(struct bayesian-number (prior likelihood evidence posterior) #:transparent)

(define (make-bayesian-number prior-prob)
  (bayesian-number prior-prob 1.0 1.0 prior-prob))

;; Bayesian update: P(H|E) = P(E|H) × P(H) / P(E)
(define (bayesian-update bn likelihood-given-h evidence-prob)
  (define prior (bayesian-number-prior bn))
  (define posterior (/ (* likelihood-given-h prior) evidence-prob))
  (bayesian-number prior likelihood-given-h evidence-prob posterior))

(define (bayesian-posterior bn)
  (bayesian-number-posterior bn))

;; ============================================================================
;; 5. RiskNumber - VaR/CVaR Calculations
;; ============================================================================

(struct risk-number (distribution confidence-level) #:transparent)

(define (make-risk-number samples [confidence 0.95])
  (risk-number samples confidence))

;; Value at Risk (VaR): α-quantile of loss distribution
(define (value-at-risk rn)
  (define samples (risk-number-distribution rn))
  (define α (risk-number-confidence-level rn))
  (define sorted (sort samples <))
  (define index (inexact->exact (floor (* (- 1 α) (length sorted)))))
  (list-ref sorted (max 0 index)))

;; Conditional VaR (Expected Shortfall): average of losses beyond VaR
(define (conditional-var rn)
  (define samples (risk-number-distribution rn))
  (define var (value-at-risk rn))
  (define tail-losses (filter (lambda (x) (<= x var)) samples))
  (if (empty? tail-losses)
      var
      (/ (apply + tail-losses) (length tail-losses))))

(define expected-shortfall conditional-var)  ;; Alias

;; ============================================================================
;; 6. SurrealFuzzy - Fuzzy Logic with Infinitesimal Tolerance
;; ============================================================================

(struct surreal-fuzzy (left center right epsilon) #:transparent)

(define (make-surreal-fuzzy left center right [epsilon 1e-9])
  (unless (and (<= left center) (<= center right))
    (error 'make-surreal-fuzzy "Must have left <= center <= right"))
  (unless (>= epsilon 0)
    (error 'make-surreal-fuzzy "Epsilon must be non-negative"))
  (surreal-fuzzy left center right epsilon))

(define (surreal-fuzzy-membership sf x)
  (define a (- (surreal-fuzzy-left sf) (surreal-fuzzy-epsilon sf)))
  (define b (surreal-fuzzy-center sf))
  (define c (+ (surreal-fuzzy-right sf) (surreal-fuzzy-epsilon sf)))
  (cond
    [(<= x a) 0]
    [(and (> x a) (<= x b))
     (/ (- x a) (- b a))]
    [(and (> x b) (< x c))
     (/ (- c x) (- c b))]
    [else 0]))

;; ============================================================================
;; 7. p-Adic Probability - Hierarchical Digit Expansion
;; ============================================================================

(struct padic-probability (base digits) #:transparent)

(define (make-padic-probability base digits)
  (unless (>= base 2)
    (error 'make-padic-probability "Base must be >= 2"))
  (unless (and (list? digits) (andmap integer? digits))
    (error 'make-padic-probability "Digits must be a list of integers"))
  (unless (andmap (lambda (d) (and (>= d 0) (< d base))) digits)
    (error 'make-padic-probability "Digits must be in [0, base)"))
  (padic-probability base digits))

(define (padic-probability->real pp)
  (define base (padic-probability-base pp))
  (define digits (padic-probability-digits pp))
  (for/sum ([d digits] [i (in-naturals 1)])
    (* d (expt base (- i)))))

(define (padic-probability-refine pp digit)
  (define base (padic-probability-base pp))
  (unless (and (integer? digit) (>= digit 0) (< digit base))
    (error 'padic-probability-refine "Digit must be in [0, base)"))
  (padic-probability base (append (padic-probability-digits pp) (list digit))))

;; ============================================================================
;; 8. LotteryNumber - Weighted Discrete Outcomes
;; ============================================================================

(struct lottery-number (outcomes weights) #:transparent)

(define (make-lottery-number outcomes weights)
  (unless (and (list? outcomes) (list? weights))
    (error 'make-lottery-number "Outcomes and weights must be lists"))
  (unless (= (length outcomes) (length weights))
    (error 'make-lottery-number "Outcomes and weights must be same length"))
  (unless (andmap (lambda (w) (>= w 0)) weights)
    (error 'make-lottery-number "Weights must be non-negative"))
  (define total (apply + weights))
  (unless (> total 0)
    (error 'make-lottery-number "Weights must sum to > 0"))
  (lottery-number outcomes weights))

(define (lottery-number-expected-value ln)
  (define outcomes (lottery-number-outcomes ln))
  (define weights (lottery-number-weights ln))
  (unless (andmap number? outcomes)
    (error 'lottery-number-expected-value "Outcomes must be numeric"))
  (define total (apply + weights))
  (/ (for/sum ([o outcomes] [w weights]) (* o w)) total))

(define (lottery-number-sample ln)
  (define outcomes (lottery-number-outcomes ln))
  (define weights (lottery-number-weights ln))
  (define total (apply + weights))
  (define probs (map (lambda (w) (/ w total)) weights))
  (define idx (categorical probs))
  (list-ref outcomes idx))

;; ============================================================================
;; Examples and Tests
;; ============================================================================

(module+ test
  (require rackunit)

  (test-case "DistnumberNormal arithmetic"
    (define n1 (make-distnumber-normal 10 2))
    (define n2 (make-distnumber-normal 5 1))
    (define sum (distnumber-add n1 n2))

    (check-true (< (abs (- (distnumber-normal-mean sum) 15)) 0.01))
    (check-true (< (abs (- (distnumber-normal-stddev sum) (sqrt 5))) 0.01)))

  (test-case "AffineNumber interval arithmetic"
    (define a1 (make-affine-number 1 3))
    (define a2 (make-affine-number 2 4))
    (define sum (affine-add a1 a2))

    (check-equal? (affine-number-lower sum) 3)
    (check-equal? (affine-number-upper sum) 7)
    (check-true (affine-contains? a1 2)))

  (test-case "FuzzyTriangular membership"
    (define ft (make-fuzzy-triangular 0 5 10))

    (check-true (< (abs (- (fuzzy-membership ft 5) 1.0)) 0.01))
    (check-true (< (abs (- (fuzzy-membership ft 2.5) 0.5)) 0.01))
    (check-true (< (abs (- (fuzzy-membership ft 0) 0.0)) 0.01)))

  (test-case "BayesianNumber update"
    (define prior-prob 0.01)  ;; 1% base rate
    (define bn (make-bayesian-number prior-prob))

    ;; Positive test with 90% true positive rate
    (define likelihood 0.9)
    ;; Evidence: P(+) = P(+|D)P(D) + P(+|¬D)P(¬D)
    (define false-positive-rate 0.05)
    (define evidence (+ (* likelihood prior-prob)
                       (* false-positive-rate (- 1 prior-prob))))

    (define updated (bayesian-update bn likelihood evidence))
    (check-true (< (bayesian-posterior updated) 0.2)))  ;; Base rate fallacy demo

  (test-case "RiskNumber VaR calculation"
    (define returns '(-10 -5 -2 0 1 2 3 5 8 10))
    (define rn (make-risk-number returns 0.95))

    (define var (value-at-risk rn))
    (check-true (<= var -5))  ;; 95% VaR should be in worst 5%

    (define cvar (conditional-var rn))
    (check-true (<= cvar var)))  ;; CVaR should be worse than VaR

  (test-case "SurrealFuzzy membership with epsilon"
    (define sf (make-surreal-fuzzy 0 5 10 0.5))
    (check-true (> (surreal-fuzzy-membership sf 0.2) 0.0))
    (check-true (< (surreal-fuzzy-membership sf -1) 0.01)))

  (test-case "p-Adic probability conversion"
    (define pp (make-padic-probability 5 '(2 0 1)))
    (check-true (< (abs (- (padic-probability->real pp) 0.408)) 0.01)))

(test-case "LotteryNumber expected value"
    (define ln (make-lottery-number '(0 10 20) '(1 1 2)))
    (check-true (< (abs (- (lottery-number-expected-value ln) 12.5)) 0.01)))

)

(module+ main
  (displayln "=== Uncertainty-Aware Number Systems ===\n")

  (displayln "1. DistnumberNormal (Gaussian Distributions):")
  (define height (make-distnumber-normal 170 10))  ;; 170cm ± 10cm
  (define weight (make-distnumber-normal 70 8))    ;; 70kg ± 8kg
  (displayln (format "   Height: N(~a, ~a)"
                    (distnumber-normal-mean height)
                    (distnumber-normal-stddev height)))
  (displayln (format "   Sample heights: ~a"
                    (for/list ([i 5]) (distnumber-sample height))))

  (displayln "\n2. AffineNumber (Interval Arithmetic):")
  (define temp-range (make-affine-number 18 22))  ;; 18-22°C
  (define humidity-range (make-affine-number 40 60))  ;; 40-60%
  (displayln (format "   Temperature: [~a, ~a]°C"
                    (affine-number-lower temp-range)
                    (affine-number-upper temp-range)))
  (displayln (format "   Contains 20°C? ~a"
                    (affine-contains? temp-range 20)))

  (displayln "\n3. FuzzyTriangular (Fuzzy Logic):")
  (define warm (make-fuzzy-triangular 15 25 35))
  (displayln (format "   'Warm' membership at 20°C: ~a"
                    (fuzzy-membership warm 20)))
  (displayln (format "   'Warm' membership at 30°C: ~a"
                    (fuzzy-membership warm 30)))

  (displayln "\n4. BayesianNumber (Medical Test Example):")
  (define disease-rate 0.01)
  (define test-sensitivity 0.99)
  (define test-specificity 0.95)
  (define bn (make-bayesian-number disease-rate))
  (define evidence (+ (* test-sensitivity disease-rate)
                     (* (- 1 test-specificity) (- 1 disease-rate))))
  (define updated-bn (bayesian-update bn test-sensitivity evidence))
  (displayln (format "   Prior probability: ~a%" (* disease-rate 100)))
  (displayln (format "   Posterior (after positive test): ~a%"
                    (* (bayesian-posterior updated-bn) 100)))

  (displayln "\n5. RiskNumber (Portfolio Risk):")
  (define portfolio-returns
    '(-15 -10 -5 -2 0 1 2 3 5 8 10 12 15 18 20))
  (define portfolio-risk (make-risk-number portfolio-returns 0.95))
  (displayln (format "   95% VaR: ~a%" (value-at-risk portfolio-risk)))
  (displayln (format "   95% CVaR (Expected Shortfall): ~a%"
                    (conditional-var portfolio-risk)))

  (displayln "\n6. SurrealFuzzy (Infinitesimal Tolerance):")
  (define sf (make-surreal-fuzzy 0 5 10 0.25))
  (displayln (format "   Membership at 0.1: ~a"
                    (surreal-fuzzy-membership sf 0.1)))

  (displayln "\n7. p-Adic Probability (Base 5 digits):")
  (define pp (make-padic-probability 5 '(2 0 1)))
  (displayln (format "   Value: ~a"
                    (padic-probability->real pp)))

  (displayln "\n8. LotteryNumber (Weighted Outcomes):")
  (define ln (make-lottery-number '(0 10 20) '(1 1 2)))
  (displayln (format "   Expected value: ~a"
                    (lottery-number-expected-value ln)))
  (displayln (format "   Sample: ~a"
                    (lottery-number-sample ln))))
