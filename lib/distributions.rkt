#lang racket
(require "../core/betlang.rkt")
(provide uniform
         bernoulli
         binomial
         geometric
         poisson
         exponential
         normal
         log-normal
         gamma
         beta
         chi-square
         student-t
         f-distribution
         weibull
         pareto
         cauchy
         laplace
         multinomial
         dirichlet
         categorical
         zipf
         sample-from
         pdf
         cdf
         quantile
         random-walk
         brownian-motion
         levy-flight)

;; Discrete distributions

(define (uniform a b)
  (+ a (random (+ 1 (- b a)))))

(define (bernoulli p)
  (if (< (random) p) 1 0))

(define (binomial n p)
  (for/sum ([i (in-range n)])
    (bernoulli p)))

(define (geometric p)
  (let loop ([trials 1])
    (if (< (random) p)
        trials
        (loop (+ trials 1)))))

(define (poisson lambda)
  (define L (exp (- lambda)))
  (let loop ([k 0] [p 1.0])
    (if (<= p L)
        (- k 1)
        (loop (+ k 1) (* p (random))))))

;; Continuous distributions

(define (exponential lambda)
  (/ (- (log (random))) lambda))

(define (normal mu sigma)
  ;; Box-Muller transform
  (define u1 (random))
  (define u2 (random))
  (define z0 (* (sqrt (* -2 (log u1))) (cos (* 2 pi u2))))
  (+ mu (* sigma z0)))

(define (log-normal mu sigma)
  (exp (normal mu sigma)))

(define (gamma shape scale)
  ;; Marsaglia and Tsang method for shape >= 1
  (if (< shape 1)
      (* (gamma (+ shape 1) scale) (expt (random) (/ 1 shape)))
      (let* ([d (- shape (/ 1 3))]
             [c (/ 1 (sqrt (* 9 d)))])
        (let loop ()
          (define z (normal 0 1))
          (define v (expt (+ 1 (* c z)) 3))
          (if (and (> v 0)
                   (< (log (random))
                      (+ (* 0.5 z z) (* d (- 1 v (log v))))))
              (* scale d v)
              (loop))))))

(define (beta alpha beta-param)
  (define x (gamma alpha 1))
  (define y (gamma beta-param 1))
  (/ x (+ x y)))

(define (chi-square k)
  (gamma (/ k 2) 2))

(define (student-t df)
  (define z (normal 0 1))
  (define v (chi-square df))
  (/ z (sqrt (/ v df))))

(define (f-distribution d1 d2)
  (define x1 (chi-square d1))
  (define x2 (chi-square d2))
  (/ (/ x1 d1) (/ x2 d2)))

(define (weibull shape scale)
  (* scale (expt (- (log (random))) (/ 1 shape))))

(define (pareto alpha xm)
  (/ xm (expt (random) (/ 1 alpha))))

(define (cauchy x0 gamma)
  (+ x0 (* gamma (tan (* pi (- (random) 0.5))))))

(define (laplace mu b)
  (define u (- (random) 0.5))
  (- mu (* b (sgn u) (log (- 1 (* 2 (abs u)))))))

;; Multivariate distributions

(define (multinomial n probs)
  (define results (make-vector (length probs) 0))
  (for ([i (in-range n)])
    (define r (random))
    (let loop ([ps probs] [idx 0] [cumsum 0])
      (unless (null? ps)
        (define new-sum (+ cumsum (first ps)))
        (if (< r new-sum)
            (vector-set! results idx (+ 1 (vector-ref results idx)))
            (loop (rest ps) (+ idx 1) new-sum)))))
  (vector->list results))

(define (dirichlet alphas)
  (define gammas (map (lambda (a) (gamma a 1)) alphas))
  (define total (apply + gammas))
  (map (lambda (g) (/ g total)) gammas))

(define (categorical probs)
  (define r (random))
  (let loop ([ps probs] [idx 0] [cumsum 0])
    (if (null? ps)
        (- idx 1)
        (let ([new-sum (+ cumsum (first ps))])
          (if (< r new-sum)
              idx
              (loop (rest ps) (+ idx 1) new-sum))))))

(define (zipf s n)
  ;; Zipf distribution using rejection sampling
  (define normalizer
    (for/sum ([i (in-range 1 (+ n 1))])
      (/ 1 (expt i s))))
  (let loop ()
    (define k (+ 1 (random n)))
    (define u (random))
    (if (< u (/ (/ 1 (expt k s)) (* normalizer (/ 1 k))))
        k
        (loop))))

;; Sampling utilities

(define (sample-from distribution n)
  (for/list ([i (in-range n)])
    (distribution)))

;; Distribution functions (simplified)

(define (pdf dist x)
  ;; Placeholder for probability density function
  ;; Would need specific implementation for each distribution
  0.0)

(define (cdf dist x)
  ;; Placeholder for cumulative distribution function
  0.0)

(define (quantile dist p)
  ;; Placeholder for quantile function
  0.0)

;; Stochastic processes

(define (random-walk n)
  (define steps (for/list ([i (in-range n)])
                  (bet -1 0 1)))
  (define positions (list 0))
  (for ([s steps])
    (set! positions (cons (+ (first positions) s) positions)))
  (reverse positions))

(define (brownian-motion n dt)
  (define sigma (sqrt dt))
  (define increments (for/list ([i (in-range n)])
                       (normal 0 sigma)))
  (define positions (list 0))
  (for ([inc increments])
    (set! positions (cons (+ (first positions) inc) positions)))
  (reverse positions))

(define (levy-flight n alpha)
  (define steps (for/list ([i (in-range n)])
                  (define r (random))
                  (define theta (* 2 pi (random)))
                  (define step-size (expt r (/ -1 alpha)))
                  (list (* step-size (cos theta))
                        (* step-size (sin theta)))))
  (define positions (list '(0 0)))
  (for ([step steps])
    (match-define (list dx dy) step)
    (match-define (list x y) (first positions))
    (set! positions (cons (list (+ x dx) (+ y dy)) positions)))
  (reverse positions))

;; Helper function
(define (sgn x)
  (cond
    [(> x 0) 1]
    [(< x 0) -1]
    [else 0]))
