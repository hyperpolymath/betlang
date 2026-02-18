#lang racket
;; SPDX-License-Identifier: PMPL-1.0-or-later
;; Uncertainty-Aware Number Systems
;;
;; 14 number systems for representing and computing with uncertainty:
;; 1. DistnumberNormal - Gaussian distributions
;; 2. AffineNumber - Interval arithmetic
;; 3. FuzzyTriangular - Fuzzy logic with triangular membership
;; 4. BayesianNumber - Bayesian inference
;; 5. RiskNumber - VaR/CVaR calculations
;; 6. SurrealFuzzy - Fuzzy logic with infinitesimal tolerance
;; 7. p-Adic Probability - Hierarchical digit-expansion probabilities
;; 8. LotteryNumber - Discrete outcomes with weighted uncertainty
;; 9. DistnumberBeta - Beta distribution for bounded probabilities
;; 10. Hyperreal - Non-standard analysis with infinitesimals
;; 11. SurrealAdvanced - Full surreal number arithmetic
;; 12. PAdicAdvanced - Complete p-adic number system
;; 13. ImpreciseProbability - Interval-valued probability bounds
;; 14. DempsterShafer - Belief functions and evidence theory

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
         lottery-number-sample

         ;; DistnumberBeta
         make-distnumber-beta
         distnumber-beta?
         distnumber-beta-mean
         distnumber-beta-variance
         distnumber-beta-sample

         ;; Hyperreal
         make-hyperreal
         hyperreal?
         hyperreal-add
         hyperreal-mul
         hyperreal-standard-part
         hyperreal-infinitesimal?

         ;; SurrealAdvanced
         make-surreal-advanced
         surreal-advanced?
         surreal-zero
         surreal-one
         surreal-minus-one
         surreal-leq?
         surreal-add
         surreal-to-real

         ;; PAdicAdvanced
         make-padic-advanced
         padic-advanced?
         padic-advanced-normalize
         padic-advanced-add
         padic-advanced->real

         ;; ImpreciseProbability
         make-imprecise-probability
         imprecise-probability?
         imprecise-prob-precise?
         imprecise-prob-complement
         imprecise-prob-and
         imprecise-prob-or
         imprecise-prob-update

         ;; DempsterShafer
         make-dempster-shafer
         dempster-shafer?
         ds-belief
         ds-plausibility
         ds-combine)

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
;; 9. DistnumberBeta - Beta Distribution Numbers
;; ============================================================================

(struct distnumber-beta (alpha beta-param) #:transparent)

(define (make-distnumber-beta alpha beta-param)
  (unless (and (> alpha 0) (> beta-param 0))
    (error 'make-distnumber-beta "Alpha and beta must be positive"))
  (distnumber-beta alpha beta-param))

(define (distnumber-beta-mean db)
  (define α (distnumber-beta-alpha db))
  (define β (distnumber-beta-beta-param db))
  (/ α (+ α β)))

(define (distnumber-beta-variance db)
  (define α (distnumber-beta-alpha db))
  (define β (distnumber-beta-beta-param db))
  (/ (* α β) (* (expt (+ α β) 2) (+ α β 1))))

(define (distnumber-beta-sample db)
  ;; Sample using rejection sampling (simple method)
  (beta (distnumber-beta-alpha db) (distnumber-beta-beta-param db)))

;; ============================================================================
;; 10. Hyperreal - Non-Standard Analysis with Infinitesimals
;; ============================================================================

(struct hyperreal (finite infinitesimal) #:transparent)

(define (make-hyperreal finite [infinitesimal 0])
  (hyperreal finite infinitesimal))

(define (hyperreal-add h1 h2)
  (make-hyperreal
   (+ (hyperreal-finite h1) (hyperreal-finite h2))
   (+ (hyperreal-infinitesimal h1) (hyperreal-infinitesimal h2))))

(define (hyperreal-mul h1 h2)
  (define f1 (hyperreal-finite h1))
  (define f2 (hyperreal-finite h2))
  (define i1 (hyperreal-infinitesimal h1))
  (define i2 (hyperreal-infinitesimal h2))
  ;; (f1 + i1ε)(f2 + i2ε) = f1*f2 + (f1*i2 + f2*i1)ε + i1*i2*ε²
  ;; We drop ε² terms (infinitely small)
  (make-hyperreal
   (* f1 f2)
   (+ (* f1 i2) (* f2 i1))))

(define (hyperreal-standard-part h)
  ;; Return the finite part (standard part)
  (hyperreal-finite h))

(define (hyperreal-infinitesimal? h [tolerance 1e-100])
  ;; Check if hyperreal is infinitesimal
  (< (abs (hyperreal-finite h)) tolerance))

;; ============================================================================
;; 11. SurrealAdvanced - Full Surreal Number Arithmetic
;; ============================================================================

(struct surreal-advanced (left right) #:transparent)

(define (make-surreal-advanced left right)
  ;; left and right are lists of surreal numbers
  ;; Condition: all elements of left < all elements of right
  (surreal-advanced left right))

(define surreal-zero (make-surreal-advanced '() '()))
(define surreal-one (make-surreal-advanced (list surreal-zero) '()))
(define surreal-minus-one (make-surreal-advanced '() (list surreal-zero)))

(define (surreal-leq? s1 s2)
  ;; s1 ≤ s2 iff no element of s1.right ≤ s2 and no element of s2.left ≤ s1
  (and (not (ormap (lambda (r) (surreal-leq? s2 r)) (surreal-advanced-right s1)))
       (not (ormap (lambda (l) (surreal-leq? l s1)) (surreal-advanced-left s2)))))

(define (surreal-add s1 s2)
  ;; Addition: {L1 + s2, s1 + L2 | R1 + s2, s1 + R2}
  (define left-sum
    (append (map (lambda (l) (surreal-add l s2)) (surreal-advanced-left s1))
            (map (lambda (l) (surreal-add s1 l)) (surreal-advanced-left s2))))
  (define right-sum
    (append (map (lambda (r) (surreal-add r s2)) (surreal-advanced-right s1))
            (map (lambda (r) (surreal-add s1 r)) (surreal-advanced-right s2))))
  (make-surreal-advanced left-sum right-sum))

(define (surreal-to-real s [depth 10])
  ;; Approximate conversion to real number (limited depth)
  (cond
    [(<= depth 0) 0.0]
    [(and (empty? (surreal-advanced-left s)) (empty? (surreal-advanced-right s))) 0.0]
    [(empty? (surreal-advanced-right s))
     (if (empty? (surreal-advanced-left s))
         0.0
         (+ 1 (surreal-to-real (car (surreal-advanced-left s)) (- depth 1))))]
    [(empty? (surreal-advanced-left s))
     (- (surreal-to-real (car (surreal-advanced-right s)) (- depth 1)) 1)]
    [else
     (/ (+ (surreal-to-real (car (surreal-advanced-left s)) (- depth 1))
           (surreal-to-real (car (surreal-advanced-right s)) (- depth 1)))
        2)]))

;; ============================================================================
;; 12. PAdicAdvanced - Full p-Adic Number Arithmetic
;; ============================================================================

(struct padic-advanced (prime digits valuation) #:transparent)

(define (make-padic-advanced prime digits [valuation 0])
  (unless (prime? prime)
    (error 'make-padic-advanced "Base must be prime"))
  (padic-advanced prime digits valuation))

(define (prime? n)
  (and (> n 1)
       (for/and ([i (in-range 2 (+ 1 (integer-sqrt n)))])
         (not (zero? (modulo n i))))))

(define (padic-advanced-normalize pa)
  ;; Remove leading zeros and adjust valuation
  (define p (padic-advanced-prime pa))
  (define digits (padic-advanced-digits pa))
  (define val (padic-advanced-valuation pa))
  (define non-zero-digits (dropf digits zero?))
  (if (empty? non-zero-digits)
      (make-padic-advanced p '(0) 0)
      (make-padic-advanced p non-zero-digits
                          (+ val (- (length digits) (length non-zero-digits))))))

(define (padic-advanced-add pa1 pa2)
  (unless (= (padic-advanced-prime pa1) (padic-advanced-prime pa2))
    (error 'padic-advanced-add "p-adic numbers must have same prime base"))
  (define p (padic-advanced-prime pa1))
  (define val1 (padic-advanced-valuation pa1))
  (define val2 (padic-advanced-valuation pa2))
  (define min-val (min val1 val2))

  ;; Align digits and add
  (define d1 (append (make-list (- val1 min-val) 0) (padic-advanced-digits pa1)))
  (define d2 (append (make-list (- val2 min-val) 0) (padic-advanced-digits pa2)))
  (define max-len (max (length d1) (length d2)))
  (define d1-padded (append d1 (make-list (- max-len (length d1)) 0)))
  (define d2-padded (append d2 (make-list (- max-len (length d2)) 0)))

  (define-values (result-digits _)
    (for/fold ([result '()] [carry 0])
              ([a (reverse d1-padded)] [b (reverse d2-padded)])
      (define sum (+ a b carry))
      (values (cons (modulo sum p) result) (quotient sum p))))

  (padic-advanced-normalize (make-padic-advanced p result-digits min-val)))

(define (padic-advanced->real pa [precision 10])
  ;; Convert to real number approximation
  (define p (padic-advanced-prime pa))
  (define digits (padic-advanced-digits pa))
  (define val (padic-advanced-valuation pa))
  (for/sum ([(d i) (in-indexed (take digits (min precision (length digits))))])
    (* d (expt p (+ val i)))))

;; ============================================================================
;; 13. ImpreciseProbability - Interval-Valued Probabilities
;; ============================================================================

(struct imprecise-probability (lower upper) #:transparent)

(define (make-imprecise-probability lower upper)
  (unless (and (<= 0 lower) (<= lower upper) (<= upper 1))
    (error 'make-imprecise-probability "Must satisfy 0 ≤ lower ≤ upper ≤ 1"))
  (imprecise-probability lower upper))

(define (imprecise-prob-precise? ip [tolerance 1e-10])
  ;; Check if essentially a precise probability
  (< (- (imprecise-probability-upper ip) (imprecise-probability-lower ip)) tolerance))

(define (imprecise-prob-complement ip)
  ;; Complement: [1-u, 1-l]
  (make-imprecise-probability
   (- 1 (imprecise-probability-upper ip))
   (- 1 (imprecise-probability-lower ip))))

(define (imprecise-prob-and ip1 ip2)
  ;; Independent conjunction (lower bound)
  ;; [l1*l2, u1*u2]
  (make-imprecise-probability
   (* (imprecise-probability-lower ip1) (imprecise-probability-lower ip2))
   (* (imprecise-probability-upper ip1) (imprecise-probability-upper ip2))))

(define (imprecise-prob-or ip1 ip2)
  ;; Independent disjunction
  ;; [l1 + l2 - l1*l2, u1 + u2 - u1*u2]
  (define l1 (imprecise-probability-lower ip1))
  (define l2 (imprecise-probability-lower ip2))
  (define u1 (imprecise-probability-upper ip1))
  (define u2 (imprecise-probability-upper ip2))
  (make-imprecise-probability
   (- (+ l1 l2) (* l1 l2))
   (- (+ u1 u2) (* u1 u2))))

(define (imprecise-prob-update ip likelihood)
  ;; Bayesian update with imprecise priors
  ;; Returns updated interval using Bayes' rule bounds
  (define l (imprecise-probability-lower ip))
  (define u (imprecise-probability-upper ip))
  (make-imprecise-probability
   (/ (* l likelihood) (+ (* l likelihood) (- 1 l)))
   (/ (* u likelihood) (+ (* u likelihood) (- 1 u)))))

;; ============================================================================
;; 14. DempsterShafer - Belief Functions and Mass Assignments
;; ============================================================================

(struct dempster-shafer (frame masses) #:transparent)

(define (make-dempster-shafer frame masses)
  ;; frame: list of focal elements (sets)
  ;; masses: list of mass values (must sum to 1)
  (unless (and (list? frame) (list? masses))
    (error 'make-dempster-shafer "Frame and masses must be lists"))
  (unless (= (length frame) (length masses))
    (error 'make-dempster-shafer "Frame and masses must be same length"))
  (unless (< (abs (- (apply + masses) 1.0)) 0.001)
    (error 'make-dempster-shafer "Masses must sum to 1"))
  (dempster-shafer frame masses))

(define (ds-belief ds hypothesis)
  ;; Belief: sum of masses of subsets of hypothesis
  (define frame (dempster-shafer-frame ds))
  (define masses (dempster-shafer-masses ds))
  (for/sum ([focal frame] [mass masses])
    (if (subset? focal hypothesis)
        mass
        0)))

(define (ds-plausibility ds hypothesis)
  ;; Plausibility: sum of masses that intersect hypothesis
  (define frame (dempster-shafer-frame ds))
  (define masses (dempster-shafer-masses ds))
  (for/sum ([focal frame] [mass masses])
    (if (not (empty? (set-intersect focal hypothesis)))
        mass
        0)))

(define (ds-combine ds1 ds2)
  ;; Dempster's rule of combination
  (define frame1 (dempster-shafer-frame ds1))
  (define masses1 (dempster-shafer-masses ds1))
  (define frame2 (dempster-shafer-frame ds2))
  (define masses2 (dempster-shafer-masses ds2))

  ;; Compute intersections and their masses
  (define new-frame '())
  (define new-masses '())

  (for ([f1 frame1] [m1 masses1])
    (for ([f2 frame2] [m2 masses2])
      (define intersection (set-intersect f1 f2))
      (unless (empty? intersection)
        (set! new-frame (cons intersection new-frame))
        (set! new-masses (cons (* m1 m2) new-masses)))))

  ;; Normalize (exclude empty set mass)
  (define total-mass (apply + new-masses))
  (if (> total-mass 0)
      (make-dempster-shafer new-frame
                           (map (lambda (m) (/ m total-mass)) new-masses))
      (error 'ds-combine "Complete conflict: no valid combination")))

(define (subset? s1 s2)
  ;; Check if s1 is subset of s2
  (andmap (lambda (x) (member x s2)) s1))

(define (set-intersect s1 s2)
  ;; Return intersection of two sets
  (filter (lambda (x) (member x s2)) s1))

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
                    (lottery-number-sample ln)))

  (displayln "\n9. DistnumberBeta (Beta Distribution):")
  (define db (make-distnumber-beta 2 5))  ;; Beta(2,5) for success rate
  (displayln (format "   Beta(2,5) mean: ~a" (distnumber-beta-mean db)))
  (displayln (format "   Beta(2,5) variance: ~a" (distnumber-beta-variance db)))
  (displayln (format "   Sample: ~a" (distnumber-beta-sample db)))

  (displayln "\n10. Hyperreal (Non-Standard Analysis):")
  (define h1 (make-hyperreal 3 1))  ;; 3 + ε
  (define h2 (make-hyperreal 2 0.5))  ;; 2 + 0.5ε
  (define h-sum (hyperreal-add h1 h2))
  (displayln (format "   (3 + ε) + (2 + 0.5ε) = ~a + ~aε"
                    (hyperreal-finite h-sum) (hyperreal-infinitesimal h-sum)))
  (displayln (format "   Standard part: ~a" (hyperreal-standard-part h-sum)))

  (displayln "\n11. SurrealAdvanced (Surreal Numbers):")
  (displayln (format "   Zero: ~a" (surreal-to-real surreal-zero)))
  (displayln (format "   One: ~a" (surreal-to-real surreal-one)))
  (define s-sum (surreal-add surreal-one surreal-one))
  (displayln (format "   1 + 1 ≈ ~a" (surreal-to-real s-sum)))

  (displayln "\n12. PAdicAdvanced (p-Adic Arithmetic):")
  (define pa1 (make-padic-advanced 5 '(1 2 3)))
  (define pa2 (make-padic-advanced 5 '(2 1)))
  (define pa-sum (padic-advanced-add pa1 pa2))
  (displayln (format "   p=5: 321 + 12 (base 5)"))
  (displayln (format "   Result: ~a" (padic-advanced->real pa-sum)))

  (displayln "\n13. ImpreciseProbability (Interval Bounds):")
  (define ip (make-imprecise-probability 0.3 0.7))  ;; 30-70% confidence
  (define ip-comp (imprecise-prob-complement ip))
  (displayln (format "   P(A) ∈ [~a, ~a]"
                    (imprecise-probability-lower ip)
                    (imprecise-probability-upper ip)))
  (displayln (format "   P(¬A) ∈ [~a, ~a]"
                    (imprecise-probability-lower ip-comp)
                    (imprecise-probability-upper ip-comp)))

  (displayln "\n14. DempsterShafer (Belief Functions):")
  (define frame '((a) (b) (a b)))
  (define masses '(0.4 0.3 0.3))
  (define ds (make-dempster-shafer frame masses))
  (displayln (format "   Belief in {a}: ~a" (ds-belief ds '(a))))
  (displayln (format "   Plausibility of {a}: ~a" (ds-plausibility ds '(a)))))
