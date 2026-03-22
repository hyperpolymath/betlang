#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/distributions.rkt")

;; Bayesian Inference Utilities for betlang

(provide bayes-theorem
         posterior
         likelihood
         prior
         evidence
         conjugate-beta-binomial
         conjugate-normal
         metropolis-hastings
         gibbs-sampler
         importance-sampling
         rejection-sampling
         abc-algorithm
         bayesian-update
         credible-interval
         hpd-interval
         bayes-factor
         posterior-predictive
         empirical-bayes)

;; Basic Bayes' Theorem
(define (bayes-theorem prior-prob likelihood evidence-prob)
  (/ (* likelihood prior-prob) evidence-prob))

;; General posterior calculation
(define (posterior prior-fn likelihood-fn data)
  (lambda (theta)
    (* (prior-fn theta) (likelihood-fn theta data))))

;; Prior distributions (simplified representations)
(define (prior type . params)
  (match type
    ['uniform (lambda (x) 1.0)]
    ['beta (lambda (x)
             (define alpha (first params))
             (define beta-param (second params))
             ;; Simplified beta PDF
             (expt x (- alpha 1)) (* (expt (- 1 x) (- beta-param 1))))]
    ['normal (lambda (x)
              (define mu (first params))
              (define sigma (second params))
              ;; Simplified normal PDF
              (exp (* -0.5 (expt (/ (- x mu) sigma) 2))))]))

;; Likelihood function helper
(define (likelihood model data)
  (lambda (theta)
    (apply * (map (lambda (obs) (model theta obs)) data))))

;; Evidence (marginal likelihood)
(define (evidence prior-fn likelihood-fn data samples)
  (define theta-samples (samples))
  (mean (map (lambda (theta)
               (* (prior-fn theta) (likelihood-fn theta data)))
             theta-samples)))

;; Conjugate Priors

;; Beta-Binomial conjugacy
(define (conjugate-beta-binomial alpha beta successes failures)
  ;; Posterior is Beta(alpha + successes, beta + failures)
  (lambda ()
    (bet (/ (+ alpha successes) (+ alpha beta successes failures))
         (beta (+ alpha successes) (+ beta failures))
         (beta (+ alpha successes) (+ beta failures)))))

;; Normal-Normal conjugacy (known variance)
(define (conjugate-normal prior-mean prior-var data-var observations)
  (define n (length observations))
  (define data-mean (mean observations))

  (define posterior-var
    (/ 1 (+ (/ 1 prior-var) (/ n data-var))))

  (define posterior-mean
    (* posterior-var
       (+ (/ prior-mean prior-var)
          (/ (* n data-mean) data-var))))

  (list posterior-mean posterior-var))

;; MCMC Methods

;; Metropolis-Hastings algorithm
(define (metropolis-hastings target-log-density proposal initial n-samples)
  (define samples (make-vector n-samples 0))
  (vector-set! samples 0 initial)

  (for ([i (in-range 1 n-samples)])
    (define current (vector-ref samples (- i 1)))
    (define proposed (+ current (proposal)))

    (define log-acceptance-ratio
      (- (target-log-density proposed) (target-log-density current)))

    (define accept? (< (log (random)) log-acceptance-ratio))

    (vector-set! samples i (if accept? proposed current)))

  (vector->list samples))

;; Gibbs Sampler (simplified 2D case)
(define (gibbs-sampler conditional-x conditional-y init-x init-y n-samples)
  (define samples-x (make-vector n-samples 0))
  (define samples-y (make-vector n-samples 0))

  (vector-set! samples-x 0 init-x)
  (vector-set! samples-y 0 init-y)

  (for ([i (in-range 1 n-samples)])
    (define prev-x (vector-ref samples-x (- i 1)))
    (define prev-y (vector-ref samples-y (- i 1)))

    (define new-x (conditional-x prev-y))
    (define new-y (conditional-y new-x))

    (vector-set! samples-x i new-x)
    (vector-set! samples-y i new-y))

  (list (vector->list samples-x) (vector->list samples-y)))

;; Importance Sampling
(define (importance-sampling target-fn proposal-sample proposal-density n-samples)
  (define samples
    (for/list ([i (in-range n-samples)])
      (proposal-sample)))

  (define weights
    (for/list ([s samples])
      (/ (target-fn s) (proposal-density s))))

  (define normalized-weights
    (let ([sum-weights (apply + weights)])
      (map (lambda (w) (/ w sum-weights)) weights)))

  (list samples weights normalized-weights))

;; Rejection Sampling
(define (rejection-sampling target-fn proposal-sample M max-attempts)
  (let loop ([attempts 0])
    (if (>= attempts max-attempts)
        #f
        (let* ([x (proposal-sample)]
               [u (random)]
               [accept? (<= u (/ (target-fn x) (* M (proposal-sample))))])
          (if accept?
              x
              (loop (+ attempts 1)))))))

;; Approximate Bayesian Computation (ABC)
(define (abc-algorithm simulator observed-data distance threshold n-particles)
  (define accepted-params '())

  (let loop ([i 0])
    (when (< (length accepted-params) n-particles)
      (define candidate-param (bet (random) (random) (random)))
      (define simulated-data (simulator candidate-param))
      (define dist (distance simulated-data observed-data))

      (when (< dist threshold)
        (set! accepted-params (cons candidate-param accepted-params)))

      (loop (+ i 1))))

  accepted-params)

;; Bayesian Update (sequential)
(define (bayesian-update prior likelihood-fn new-data)
  (lambda (theta)
    (* (prior theta) (likelihood-fn theta new-data))))

;; Credible Interval
(define (credible-interval samples alpha)
  (define sorted (sort samples <))
  (define n (length sorted))
  (define lower-idx (exact->inexact (floor (* (/ alpha 2) n))))
  (define upper-idx (exact->inexact (floor (* (- 1 (/ alpha 2)) n))))

  (list (list-ref sorted (min (inexact->exact lower-idx) (- n 1)))
        (list-ref sorted (min (inexact->exact upper-idx) (- n 1)))))

;; Highest Posterior Density (HPD) interval (simplified)
(define (hpd-interval samples alpha)
  ;; Simplified version - use credible interval
  ;; True HPD requires density estimation
  (credible-interval samples alpha))

;; Bayes Factor
(define (bayes-factor marginal-likelihood-1 marginal-likelihood-2)
  (/ marginal-likelihood-1 marginal-likelihood-2))

;; Posterior Predictive Distribution
(define (posterior-predictive posterior-samples likelihood-fn n-predictions)
  (for/list ([i (in-range n-predictions)])
    (define theta (list-ref posterior-samples
                           (random (length posterior-samples))))
    (likelihood-fn theta)))

;; Empirical Bayes
(define (empirical-bayes data estimate-hyperparams)
  ;; Estimate hyperparameters from data
  (define hyperparams (estimate-hyperparams data))
  ;; Return prior with estimated hyperparameters
  hyperparams)

;; Example usage
(module+ main
  (displayln "=== Bayesian Inference Examples ===\n")

  ;; Example 1: Coin flip with Beta-Binomial
  (displayln "1. Coin Flip Inference (Beta-Binomial)")
  (define coin-posterior (conjugate-beta-binomial 1 1 7 3))
  (displayln "   Prior: Beta(1,1) - uniform")
  (displayln "   Data: 7 heads, 3 tails")
  (displayln (format "   Posterior mean: ~a" ((coin-posterior))))
  (displayln "   Posterior: Beta(8, 4)\n")

  ;; Example 2: Normal-Normal conjugacy
  (displayln "2. Normal Mean Inference")
  (define observations '(10.2 9.8 10.1 10.3 9.9))
  (define posterior-params (conjugate-normal 10.0 1.0 0.1 observations))
  (displayln (format "   Prior: N(10, 1)"))
  (displayln (format "   Data: ~a" observations))
  (displayln (format "   Posterior: N(~a, ~a)"
                     (first posterior-params)
                     (second posterior-params)))
  (displayln "")

  ;; Example 3: Metropolis-Hastings
  (displayln "3. Metropolis-Hastings Sampling")
  (define (target-log-density x)
    ;; Log of normal distribution
    (* -0.5 (expt x 2)))

  (define (proposal) (normal 0 0.5))

  (define mh-samples (metropolis-hastings target-log-density proposal 0.0 1000))
  (displayln (format "   Target: Standard Normal"))
  (displayln (format "   Samples: ~a" (length mh-samples)))
  (displayln (format "   Sample mean: ~a" (mean mh-samples)))
  (displayln (format "   Sample std: ~a" (stddev mh-samples)))
  (displayln "")

  ;; Example 4: Credible interval
  (displayln "4. Credible Interval")
  (define posterior-samples
    (for/list ([i (in-range 10000)])
      (normal 0 1)))
  (define ci-95 (credible-interval posterior-samples 0.05))
  (displayln (format "   95% Credible Interval: ~a" ci-95))
  (displayln "   (Should be approximately [-1.96, 1.96])\n")

  ;; Example 5: Bayes Factor
  (displayln "5. Bayes Factor (Model Comparison)")
  (define ml1 0.3)  ;; Marginal likelihood model 1
  (define ml2 0.1)  ;; Marginal likelihood model 2
  (define bf (bayes-factor ml1 ml2))
  (displayln (format "   Model 1 marginal likelihood: ~a" ml1))
  (displayln (format "   Model 2 marginal likelihood: ~a" ml2))
  (displayln (format "   Bayes Factor (M1/M2): ~a" bf))
  (displayln (format "   Evidence: ~a"
                     (cond
                       [(> bf 10) "Strong support for M1"]
                       [(> bf 3) "Moderate support for M1"]
                       [(< bf 0.33) "Strong support for M2"]
                       [else "Inconclusive"])))

  (displayln "\n=== Bayesian Inference Examples Complete ==="))
