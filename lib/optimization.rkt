#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")

;; Optimization Algorithms with Probabilistic Elements

(provide simulated-annealing
         genetic-algorithm
         particle-swarm
         hill-climbing
         random-search
         evolutionary-strategy
         cross-entropy-method
         gradient-descent-stochastic
         differential-evolution
         tabu-search
         ant-colony
         firefly-algorithm
         ternary-search
         golden-section-search)

;; Simulated Annealing
(define (simulated-annealing objective initial temp-schedule max-iter neighbor-fn)
  (let loop ([current initial]
             [current-score (objective initial)]
             [best current]
             [best-score (objective initial)]
             [iter 0])
    (if (>= iter max-iter)
        (list best best-score)
        (let* ([temp (temp-schedule iter)]
               [neighbor (neighbor-fn current)]
               [neighbor-score (objective neighbor)]
               [delta (- neighbor-score current-score)]
               [accept? (or (> delta 0)
                           (< (random) (exp (/ delta temp))))])
          (if accept?
              (loop neighbor
                    neighbor-score
                    (if (> neighbor-score best-score) neighbor best)
                    (if (> neighbor-score best-score) neighbor-score best-score)
                    (+ iter 1))
              (loop current
                    current-score
                    best
                    best-score
                    (+ iter 1)))))))

;; Genetic Algorithm
(define (genetic-algorithm objective pop-size generations crossover-rate mutation-rate gene-length)
  ;; Initialize population
  (define (random-individual)
    (for/list ([i (in-range gene-length)])
      (bet 0 1 0)))  ;; Binary genes with ternary bet

  (define (fitness individual)
    (objective individual))

  (define (select population fitnesses)
    ;; Tournament selection with ternary choice
    (define idx1 (random (length population)))
    (define idx2 (random (length population)))
    (define idx3 (random (length population)))
    (define best-idx (bet idx1 idx2 idx3))
    (list-ref population
              (if (> (list-ref fitnesses idx1) (list-ref fitnesses idx2))
                  (if (> (list-ref fitnesses idx1) (list-ref fitnesses idx3))
                      idx1
                      idx3)
                  (if (> (list-ref fitnesses idx2) (list-ref fitnesses idx3))
                      idx2
                      idx3))))

  (define (crossover parent1 parent2)
    (if (< (random) crossover-rate)
        (let ([point (random (length parent1))])
          (append (take parent1 point) (drop parent2 point)))
        parent1))

  (define (mutate individual)
    (for/list ([gene individual])
      (if (< (random) mutation-rate)
          (bet 0 1 (- 1 gene))
          gene)))

  (let loop ([population (for/list ([i (in-range pop-size)])
                          (random-individual))]
             [gen 0])
    (if (>= gen generations)
        (let* ([fitnesses (map fitness population)]
               [best-idx (argmax identity (range (length fitnesses))
                                 #:key (lambda (i) (list-ref fitnesses i)))])
          (list (list-ref population best-idx)
                (list-ref fitnesses best-idx)))
        (let* ([fitnesses (map fitness population)]
               [new-pop
                (for/list ([i (in-range pop-size)])
                  (define parent1 (select population fitnesses))
                  (define parent2 (select population fitnesses))
                  (mutate (crossover parent1 parent2)))])
          (loop new-pop (+ gen 1))))))

;; Particle Swarm Optimization
(define (particle-swarm objective dim n-particles n-iterations bounds)
  (define (random-position)
    (for/list ([i (in-range dim)])
      (+ (first bounds) (* (random) (- (second bounds) (first bounds))))))

  (define (random-velocity)
    (for/list ([i (in-range dim)])
      (* 0.1 (- (random) 0.5))))

  (define particles
    (for/list ([i (in-range n-particles)])
      (define pos (random-position))
      (list pos (random-velocity) pos (objective pos))))

  (define (global-best particles)
    (argmax (lambda (p) (fourth p)) particles))

  (let loop ([swarm particles]
             [iter 0])
    (if (>= iter n-iterations)
        (global-best swarm)
        (let* ([g-best (global-best swarm)]
               [g-best-pos (first g-best)]
               [new-swarm
                (for/list ([particle swarm])
                  (match-define (list pos vel p-best p-best-score) particle)
                  (define w 0.7)  ;; Inertia
                  (define c1 1.5) ;; Cognitive
                  (define c2 1.5) ;; Social

                  (define new-vel
                    (for/list ([v vel]
                               [x pos]
                               [pb p-best]
                               [gb g-best-pos])
                      (+ (* w v)
                         (* c1 (random) (- pb x))
                         (* c2 (random) (- gb x)))))

                  (define new-pos
                    (for/list ([x pos] [v new-vel])
                      (max (first bounds)
                           (min (second bounds) (+ x v)))))

                  (define new-score (objective new-pos))
                  (define new-p-best
                    (if (> new-score p-best-score) new-pos p-best))
                  (define new-p-best-score
                    (if (> new-score p-best-score) new-score p-best-score))

                  (list new-pos new-vel new-p-best new-p-best-score))])
          (loop new-swarm (+ iter 1))))))

;; Hill Climbing with Ternary Moves
(define (hill-climbing objective initial neighbor-fn max-iter)
  (let loop ([current initial]
             [current-score (objective initial)]
             [iter 0])
    (if (>= iter max-iter)
        (list current current-score)
        (let* ([neighbor1 (neighbor-fn current)]
               [neighbor2 (neighbor-fn current)]
               [neighbor3 (neighbor-fn current)]
               [score1 (objective neighbor1)]
               [score2 (objective neighbor2)]
               [score3 (objective neighbor3)]
               [best-neighbor (bet neighbor1 neighbor2 neighbor3)]
               [best-score (bet score1 score2 score3)])
          (if (> best-score current-score)
              (loop best-neighbor best-score (+ iter 1))
              (loop current current-score (+ iter 1)))))))

;; Random Search
(define (random-search objective sample-fn n-samples)
  (define samples
    (for/list ([i (in-range n-samples)])
      (sample-fn)))
  (define scores (map objective samples))
  (define best-idx (argmax identity (range n-samples) #:key (lambda (i) (list-ref scores i))))
  (list (list-ref samples best-idx) (list-ref scores best-idx)))

;; Evolutionary Strategy (ES)
(define (evolutionary-strategy objective dim mu lambda sigma max-gen bounds)
  (define (random-individual)
    (for/list ([i (in-range dim)])
      (+ (first bounds) (* (random) (- (second bounds) (first bounds))))))

  (let loop ([population (for/list ([i (in-range mu)])
                          (random-individual))]
             [gen 0])
    (if (>= gen max-gen)
        (let* ([scores (map objective population)]
               [best-idx (argmax identity (range mu) #:key (lambda (i) (list-ref scores i)))])
          (list (list-ref population best-idx) (list-ref scores best-idx)))
        (let* ([offspring
                (for/list ([i (in-range lambda)])
                  (define parent (list-ref population (random mu)))
                  (for/list ([gene parent])
                    (+ gene (* sigma (- (random) 0.5)))))]
               [all-individuals (append population offspring)]
               [all-scores (map objective all-individuals)]
               [sorted-indices
                (sort (range (+ mu lambda))
                      > #:key (lambda (i) (list-ref all-scores i)))]
               [new-pop (for/list ([i (in-range mu)])
                         (list-ref all-individuals (list-ref sorted-indices i)))])
          (loop new-pop (+ gen 1))))))

;; Cross-Entropy Method
(define (cross-entropy-method objective dim n-samples elite-frac max-iter)
  (define n-elite (inexact->exact (floor (* elite-frac n-samples))))

  (let loop ([mean (make-list dim 0.0)]
             [std (make-list dim 1.0)]
             [iter 0])
    (if (>= iter max-iter)
        (list mean (objective mean))
        (let* ([samples
                (for/list ([i (in-range n-samples)])
                  (for/list ([m mean] [s std])
                    (+ m (* s (- (random) 0.5)))))]
               [scores (map objective samples)]
               [sorted-indices (sort (range n-samples) > #:key (lambda (i) (list-ref scores i)))]
               [elite-samples
                (for/list ([i (in-range n-elite)])
                  (list-ref samples (list-ref sorted-indices i)))]
               [new-mean
                (for/list ([d (in-range dim)])
                  (mean (for/list ([s elite-samples])
                         (list-ref s d))))]
               [new-std
                (for/list ([d (in-range dim)])
                  (stddev (for/list ([s elite-samples])
                           (list-ref s d))))])
          (loop new-mean new-std (+ iter 1))))))

;; Ternary Search (for unimodal functions)
(define (ternary-search f left right epsilon)
  (if (< (- right left) epsilon)
      (/ (+ left right) 2)
      (let* ([third (/ (- right left) 3)]
             [m1 (+ left third)]
             [m2 (- right third)]
             [fm1 (f m1)]
             [fm2 (f m2)])
        (if (< fm1 fm2)
            (ternary-search f m1 right epsilon)
            (ternary-search f left m2 epsilon)))))

;; Example usage
(module+ main
  (displayln "=== Optimization Algorithms ===\n")

  ;; Example 1: Simulated Annealing for TSP-like problem
  (displayln "1. Simulated Annealing")
  (define (sphere x)
    ;; Minimize -sum(x^2)
    (- (apply + (map (lambda (v) (* v v)) x))))

  (define (neighbor x)
    (for/list ([v x])
      (+ v (* 0.1 (- (random) 0.5)))))

  (define sa-result
    (simulated-annealing
     sphere
     '(5.0 5.0)
     (lambda (t) (/ 100 (+ t 1)))
     1000
     neighbor))

  (displayln (format "   Best solution: ~a" (first sa-result)))
  (displayln (format "   Best score: ~a" (second sa-result)))
  (displayln "")

  ;; Example 2: Ternary Search
  (displayln "2. Ternary Search (find minimum of parabola)")
  (define (parabola x) (* (- x 3) (- x 3)))
  (define ts-result (ternary-search parabola -10.0 10.0 0.0001))
  (displayln (format "   Minimum at x = ~a (should be ~3)" ts-result))
  (displayln "")

  (displayln "=== Optimization Complete ==="))
