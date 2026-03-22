#lang racket
(require "../core/betlang.rkt")
(provide mean
         median
         mode
         variance
         stddev
         covariance
         correlation
         percentile
         z-score
         normalize
         histogram
         frequency-table
         confidence-interval
         chi-square-test
         kolmogorov-smirnov
         run-simulation
         monte-carlo-pi
         bootstrap
         jackknife
         moving-average
         exponential-smoothing)

;; Basic descriptive statistics

(define (mean samples)
  (/ (apply + samples) (length samples)))

(define (median samples)
  (define sorted (sort samples <))
  (define n (length sorted))
  (if (odd? n)
      (list-ref sorted (quotient n 2))
      (/ (+ (list-ref sorted (quotient n 2))
            (list-ref sorted (- (quotient n 2) 1)))
         2)))

(define (mode samples)
  (define freq (make-hash))
  (for ([s samples])
    (hash-set! freq s (+ 1 (hash-ref freq s 0))))
  (define max-count (apply max (hash-values freq)))
  (for/list ([(k v) (in-hash freq)]
             #:when (= v max-count))
    k))

(define (variance samples)
  (define m (mean samples))
  (mean (map (lambda (x) (expt (- x m) 2)) samples)))

(define (stddev samples)
  (sqrt (variance samples)))

(define (covariance samples1 samples2)
  (define m1 (mean samples1))
  (define m2 (mean samples2))
  (mean (map (lambda (x y) (* (- x m1) (- y m2)))
             samples1 samples2)))

(define (correlation samples1 samples2)
  (/ (covariance samples1 samples2)
     (* (stddev samples1) (stddev samples2))))

(define (percentile samples p)
  (define sorted (sort samples <))
  (define idx (inexact->exact (floor (* p (length sorted)))))
  (list-ref sorted (min idx (- (length sorted) 1))))

(define (z-score x samples)
  (/ (- x (mean samples)) (stddev samples)))

(define (normalize samples)
  (define m (mean samples))
  (define s (stddev samples))
  (map (lambda (x) (/ (- x m) s)) samples))

;; Distribution analysis

(define (histogram samples bins)
  (define min-val (apply min samples))
  (define max-val (apply max samples))
  (define bin-width (/ (- max-val min-val) bins))
  (define counts (make-vector bins 0))
  (for ([s samples])
    (define bin (min (- bins 1)
                     (inexact->exact (floor (/ (- s min-val) bin-width)))))
    (vector-set! counts bin (+ 1 (vector-ref counts bin))))
  (for/list ([i (in-range bins)])
    (list (+ min-val (* i bin-width))
          (+ min-val (* (+ i 1) bin-width))
          (vector-ref counts i))))

(define (frequency-table samples)
  (define freq (make-hash))
  (for ([s samples])
    (hash-set! freq s (+ 1 (hash-ref freq s 0))))
  (hash->list freq))

;; Statistical tests

(define (confidence-interval samples alpha)
  (define m (mean samples))
  (define se (/ (stddev samples) (sqrt (length samples))))
  (define z (/ (- 1 alpha) 2))
  (list (- m (* z se)) (+ m (* z se))))

(define (chi-square-test observed expected)
  (apply + (map (lambda (o e)
                  (/ (expt (- o e) 2) e))
                observed expected)))

(define (kolmogorov-smirnov samples1 samples2)
  (define sorted1 (sort samples1 <))
  (define sorted2 (sort samples2 <))
  (define n1 (length sorted1))
  (define n2 (length sorted2))
  (define all-vals (sort (append sorted1 sorted2) <))
  (define (ecdf samples x)
    (/ (count (lambda (s) (<= s x)) samples)
       (length samples)))
  (apply max
         (map (lambda (x)
                (abs (- (ecdf sorted1 x) (ecdf sorted2 x))))
              all-vals)))

;; Simulation utilities

(define (run-simulation n experiment)
  (for/list ([i (in-range n)])
    (experiment)))

(define (monte-carlo-pi n)
  (define hits
    (count (lambda (_)
             (define x (random))
             (define y (random))
             (<= (+ (* x x) (* y y)) 1))
           (range n)))
  (* 4.0 (/ hits n)))

;; Resampling methods

(define (bootstrap samples n statistic)
  (for/list ([i (in-range n)])
    (define resampled
      (for/list ([j (in-range (length samples))])
        (list-ref samples (random (length samples)))))
    (statistic resampled)))

(define (jackknife samples statistic)
  (for/list ([i (in-range (length samples))])
    (define subset (append (take samples i)
                          (drop samples (+ i 1))))
    (statistic subset)))

;; Time series utilities

(define (moving-average samples window)
  (for/list ([i (in-range (- (length samples) window -1))])
    (mean (take (drop samples i) window))))

(define (exponential-smoothing samples alpha)
  (define result (list (first samples)))
  (for ([s (rest samples)])
    (define prev (first result))
    (set! result (cons (+ (* alpha s) (* (- 1 alpha) prev))
                      result)))
  (reverse result))
