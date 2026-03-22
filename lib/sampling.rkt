#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/distributions.rkt")

;; Advanced Sampling Methods for betlang

(provide stratified-sampling
         latin-hypercube-sampling
         sobol-sequence
         halton-sequence
         importance-resampling
         sequential-monte-carlo
         adaptive-rejection-sampling
         slice-sampling
         hamiltonian-monte-carlo
         parallel-tempering
         nested-sampling
         ternary-sampling
         antithetic-variates
         control-variates
         weighted-resampling)

;; Stratified Sampling
(define (stratified-sampling n-strata samples-per-stratum)
  (apply append
         (for/list ([stratum (in-range n-strata)])
           (define lower (/ stratum (exact->inexact n-strata)))
           (define upper (/ (+ stratum 1) (exact->inexact n-strata)))
           (for/list ([i (in-range samples-per-stratum)])
             (+ lower (* (random) (- upper lower)))))))

;; Latin Hypercube Sampling
(define (latin-hypercube-sampling n-samples n-dims)
  (for/list ([dim (in-range n-dims)])
    (define intervals (shuffle (range n-samples)))
    (for/list ([i intervals])
      (+ (/ i (exact->inexact n-samples))
         (/ (random) n-samples)))))

;; Sobol Sequence (simplified implementation)
(define (sobol-sequence n-points dims)
  (for/list ([i (in-range n-points)])
    (for/list ([d (in-range dims)])
      (/ (modulo (* i (+ d 1)) (expt 2 16))
         (exact->inexact (expt 2 16))))))

;; Halton Sequence
(define (halton-sequence n base)
  (define (halton-number n base)
    (let loop ([n n] [f (/ 1.0 base)] [r 0.0])
      (if (<= n 0)
          r
          (loop (quotient n base)
                (/ f base)
                (+ r (* f (modulo n base)))))))

  (for/list ([i (in-range n)])
    (halton-number i base)))

;; Importance Resampling
(define (importance-resampling samples weights n-resample)
  (define normalized-weights
    (let ([sum-w (apply + weights)])
      (map (lambda (w) (/ w sum-w)) weights)))

  (define cumulative
    (let loop ([ws normalized-weights] [acc 0] [result '()])
      (if (null? ws)
          (reverse result)
          (loop (rest ws)
                (+ acc (first ws))
                (cons (+ acc (first ws)) result)))))

  (for/list ([i (in-range n-resample)])
    (define r (random))
    (define idx
      (for/first ([c cumulative]
                  [j (in-naturals)]
                  #:when (< r c))
        j))
    (list-ref samples (or idx (- (length samples) 1)))))

;; Sequential Monte Carlo (Particle Filter)
(define (sequential-monte-carlo prior transition observation n-particles n-steps)
  (let loop ([particles (for/list ([i (in-range n-particles)])
                         (prior))]
             [step 0]
             [history '()])
    (if (>= step n-steps)
        (reverse history)
        (let* ([obs (observation step)]
               [weights
                (for/list ([p particles])
                  (exp (* -0.5 (expt (- p obs) 2))))]
               [resampled (importance-resampling particles weights n-particles)]
               [new-particles
                (for/list ([p resampled])
                  (transition p))])
          (loop new-particles
                (+ step 1)
                (cons (mean new-particles) history))))))

;; Slice Sampling
(define (slice-sampling log-density initial n-samples w)
  (let loop ([x initial] [samples '()] [n 0])
    (if (>= n n-samples)
        (reverse samples)
        (let* ([y (+ (log (random)) (log-density x))]
               [x-left (- x (* (random) w))]
               [x-right (+ x-left w)]
               [new-x
                (let sample-loop ()
                  (define x-proposed (+ x-left (* (random) (- x-right x-left))))
                  (if (>= (log-density x-proposed) y)
                      x-proposed
                      (begin
                        (if (< x-proposed x)
                            (set! x-left x-proposed)
                            (set! x-right x-proposed))
                        (sample-loop))))])
          (loop new-x (cons new-x samples) (+ n 1))))))

;; Adaptive Rejection Sampling (simplified)
(define (adaptive-rejection-sampling log-density bounds n-samples)
  (define (envelope x) 0.0)  ;; Simplified
  (for/list ([i (in-range n-samples)])
    (let loop ()
      (define x (+ (first bounds)
                   (* (random) (- (second bounds) (first bounds)))))
      (define u (random))
      (if (< (log u) (- (log-density x) (envelope x)))
          x
          (loop)))))

;; Ternary Sampling (betlang-specific)
(define (ternary-sampling sampler-a sampler-b sampler-c n-samples)
  (for/list ([i (in-range n-samples)])
    (bet (sampler-a) (sampler-b) (sampler-c))))

;; Antithetic Variates
(define (antithetic-variates sampler n-pairs)
  (apply append
         (for/list ([i (in-range n-pairs)])
           (define u (random))
           (list (sampler u) (sampler (- 1 u))))))

;; Control Variates
(define (control-variates target-fn control-fn control-mean n-samples)
  (define samples (for/list ([i (in-range n-samples)])
                   (define x (random))
                   (list (target-fn x) (control-fn x))))

  (define y-vals (map first samples))
  (define c-vals (map second samples))

  (define cov (covariance y-vals c-vals))
  (define var-c (variance c-vals))
  (define c-star (/ cov var-c))

  (define adjusted
    (for/list ([y y-vals] [c c-vals])
      (- y (* c-star (- c control-mean)))))

  adjusted)

;; Weighted Resampling
(define (weighted-resampling samples weights)
  (importance-resampling samples weights (length samples)))

;; Hamiltonian Monte Carlo (simplified)
(define (hamiltonian-monte-carlo log-prob grad-log-prob initial n-samples epsilon L)
  (define (leapfrog q p)
    (define p-half (for/list ([p-i p] [g (grad-log-prob q)])
                     (+ p-i (* 0.5 epsilon g))))
    (define q-new (for/list ([q-i q] [p-i p-half])
                    (+ q-i (* epsilon p-i))))
    (define p-new (for/list ([p-i p-half] [g (grad-log-prob q-new)])
                    (+ p-i (* 0.5 epsilon g))))
    (list q-new p-new))

  (let loop ([q initial] [samples '()] [n 0])
    (if (>= n n-samples)
        (reverse samples)
        (let* ([p (for/list ([qi q]) (normal 0 1))]
               [result
                (for/fold ([q-curr q] [p-curr p])
                          ([i (in-range L)])
                  (match-define (list q-next p-next) (leapfrog q-curr p-curr))
                  (values q-next p-next))]
               [q-prop (first result)]
               [p-prop (second result)]
               [current-energy (+ (- (log-prob q))
                                 (* 0.5 (apply + (map (lambda (x) (* x x)) p))))]
               [proposed-energy (+ (- (log-prob q-prop))
                                  (* 0.5 (apply + (map (lambda (x) (* x x)) p-prop))))]
               [accept? (< (log (random)) (- current-energy proposed-energy))]
               [new-q (if accept? q-prop q)])
          (loop new-q (cons new-q samples) (+ n 1))))))

;; Example usage
(module+ main
  (displayln "=== Advanced Sampling Methods ===\n")

  (displayln "1. Stratified Sampling")
  (define strat-samples (stratified-sampling 10 5))
  (displayln (format "   50 samples from 10 strata: ~a..."
                     (take strat-samples 10)))
  (displayln "")

  (displayln "2. Latin Hypercube Sampling (3D)")
  (define lhs-samples (latin-hypercube-sampling 10 3))
  (displayln (format "   First 3 samples: ~a"
                     (take (first lhs-samples) 3)))
  (displayln "")

  (displayln "3. Halton Sequence")
  (define halton-2 (halton-sequence 10 2))
  (displayln (format "   Base-2 Halton: ~a" halton-2))
  (displayln "")

  (displayln "4. Ternary Sampling")
  (define ternary-samples
    (ternary-sampling
     (lambda () (normal 0 1))
     (lambda () (normal 5 1))
     (lambda () (normal -5 1))
     100))
  (displayln (format "   Mean: ~a" (mean ternary-samples)))
  (displayln (format "   Std: ~a" (stddev ternary-samples)))
  (displayln "")

  (displayln "=== Sampling Methods Complete ==="))
