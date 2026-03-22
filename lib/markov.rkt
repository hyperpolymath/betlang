#lang racket
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")

;; Markov Chain utilities for betlang

(provide make-markov-chain
         markov-step
         markov-simulate
         markov-stationary
         markov-classify
         make-transition-matrix
         estimate-transitions
         markov-entropy
         hmm-viterbi
         generate-text-markov
         ternary-markov)

;; Markov chain structure
(struct markov-chain (states transitions initial) #:transparent)

;; Create a markov chain
(define (make-markov-chain states transitions initial-state)
  (markov-chain states transitions initial-state))

;; Take one step in the Markov chain
(define (markov-step chain current-state)
  (define transitions (markov-chain-transitions chain))
  (define next-states (hash-ref transitions current-state '()))

  (if (null? next-states)
      current-state
      (let ([total-prob (apply + (map second next-states))]
            [r (random)])
        (let loop ([options next-states] [cumsum 0])
          (match options
            ['() current-state]
            [(cons (list next-state prob) rest)
             (define new-cumsum (+ cumsum (/ prob total-prob)))
             (if (< r new-cumsum)
                 next-state
                 (loop rest new-cumsum))])))))

;; Simulate markov chain for n steps
(define (markov-simulate chain n)
  (define initial (markov-chain-initial chain))
  (let loop ([state initial] [steps n] [path (list initial)])
    (if (<= steps 0)
        (reverse path)
        (let ([next-state (markov-step chain state)])
          (loop next-state (- steps 1) (cons next-state path))))))

;; Estimate stationary distribution
(define (markov-stationary chain n-simulations)
  (define simulation (markov-simulate chain n-simulations))
  (define freq (frequency-table (drop simulation (quotient n-simulations 2))))
  (for/list ([state (markov-chain-states chain)])
    (define count (cdr (assoc state freq (lambda (a b) (equal? a b)) '(state . 0))))
    (list state (/ count (exact->inexact (length (drop simulation (quotient n-simulations 2)))))))
)

;; Create transition matrix from data
(define (estimate-transitions data)
  (define transitions (make-hash))

  (for ([i (in-range (- (length data) 1))])
    (define current (list-ref data i))
    (define next (list-ref data (+ i 1)))

    (define current-transitions (hash-ref transitions current '()))
    (define next-count
      (for/fold ([count 0])
                ([pair current-transitions]
                 #:when (equal? (first pair) next))
        (+ count (second pair))))

    (define updated-transitions
      (if (> next-count 0)
          (for/list ([pair current-transitions])
            (if (equal? (first pair) next)
                (list next (+ (second pair) 1))
                pair))
          (cons (list next 1) current-transitions)))

    (hash-set! transitions current updated-transitions))

  transitions)

;; Classify sequence using Markov model
(define (markov-classify chain sequence)
  (define prob
    (for/product ([i (in-range (- (length sequence) 1))])
      (define current (list-ref sequence i))
      (define next (list-ref sequence (+ i 1)))
      (define transitions (hash-ref (markov-chain-transitions chain) current '()))
      (define next-prob
        (for/first ([pair transitions]
                   #:when (equal? (first pair) next))
          (second pair)))
      (if next-prob next-prob 0.001))) ;; Small prob for unseen transitions
  prob)

;; Make transition matrix
(define (make-transition-matrix states transition-fn)
  (for/hash ([from states])
    (values from
            (for/list ([to states])
              (list to (transition-fn from to))))))

;; Calculate entropy of markov chain
(define (markov-entropy chain)
  (define states (markov-chain-states chain))
  (define transitions (markov-chain-transitions chain))

  (for/sum ([state states])
    (define state-transitions (hash-ref transitions state '()))
    (if (null? state-transitions)
        0
        (let ([total (apply + (map second state-transitions))])
          (for/sum ([pair state-transitions])
            (define prob (/ (second pair) total))
            (* prob (log prob 2)))))))

;; Hidden Markov Model - Viterbi algorithm (simplified)
(define (hmm-viterbi observations states start-prob trans-prob emit-prob)
  (define n (length observations))
  (define n-states (length states))

  ;; Initialize
  (define viterbi (make-vector n (make-vector n-states 0)))
  (define path (make-vector n (make-vector n-states 0)))

  ;; First observation
  (for ([s (in-naturals)]
        [state states])
    (vector-set! (vector-ref viterbi 0) s
                 (* (hash-ref start-prob state 0)
                    (hash-ref (hash-ref emit-prob state (hash)) (first observations) 0))))

  ;; Forward pass
  (for ([t (in-range 1 n)])
    (for ([s (in-naturals)]
          [state states])
      (define max-prob
        (for/fold ([max-val 0] [max-state 0])
                  ([s2 (in-naturals)]
                   [state2 states])
          (define prob
            (* (vector-ref (vector-ref viterbi (- t 1)) s2)
               (hash-ref (hash-ref trans-prob state2 (hash)) state 0)
               (hash-ref (hash-ref emit-prob state (hash)) (list-ref observations t) 0)))
          (if (> prob max-val)
              (values prob s2)
              (values max-val max-state))))
      (vector-set! (vector-ref viterbi t) s max-prob)))

  ;; Find best final state
  (define best-final-state
    (for/fold ([max-idx 0] [max-val 0])
              ([s (in-naturals)]
               [state states])
      (define val (vector-ref (vector-ref viterbi (- n 1)) s))
      (if (> val max-val)
          (values s val)
          (values max-idx max-val))))

  best-final-state)

;; Text generation with Markov chains
(define (generate-text-markov training-text n-words)
  (define words (string-split training-text))
  (define transitions (estimate-transitions words))

  (define chain (markov-chain
                (remove-duplicates words)
                transitions
                (first words)))

  (string-join (markov-simulate chain n-words) " "))

;; Ternary Markov chain specifically for betlang
(define (ternary-markov a b c)
  ;; Create a 3-state markov chain
  (define states '(A B C))
  (define transitions
    (hash 'A (list (list 'A a) (list 'B b) (list 'C c))
          'B (list (list 'A a) (list 'B b) (list 'C c))
          'C (list (list 'A a) (list 'B b) (list 'C c))))

  (make-markov-chain states transitions 'A))

;; Example usage
(module+ main
  (displayln "=== Markov Chain Examples ===\n")

  ;; Example 1: Simple weather model
  (displayln "1. Weather Markov Chain:")
  (define weather-transitions
    (hash 'sunny (list '(sunny 0.7) '(cloudy 0.2) '(rainy 0.1))
          'cloudy (list '(sunny 0.3) '(cloudy 0.4) '(rainy 0.3))
          'rainy (list '(sunny 0.2) '(cloudy 0.3) '(rainy 0.5))))

  (define weather-chain
    (markov-chain '(sunny cloudy rainy) weather-transitions 'sunny))

  (define weather-sim (markov-simulate weather-chain 30))
  (displayln (format "   30-day weather: ~a" weather-sim))

  (define weather-steady (markov-stationary weather-chain 10000))
  (displayln "\n   Stationary distribution:")
  (for ([pair weather-steady])
    (displayln (format "     ~a: ~a%" (first pair) (* 100 (second pair)))))

  ;; Example 2: Ternary markov
  (displayln "\n2. Ternary Markov (Equal Transitions):")
  (define tern-chain (ternary-markov 1 1 1))
  (define tern-sim (markov-simulate tern-chain 20))
  (displayln (format "   Sequence: ~a" tern-sim))

  ;; Example 3: Biased ternary markov
  (displayln "\n3. Biased Ternary Markov:")
  (define biased-chain (ternary-markov 5 3 2))
  (define biased-sim (markov-simulate biased-chain 100))
  (define biased-freq (frequency-table biased-sim))
  (displayln "   100-step simulation:")
  (for ([pair biased-freq])
    (displayln (format "     ~a: ~a times" (car pair) (cdr pair))))

  ;; Example 4: Learn from data
  (displayln "\n4. Learn Transitions from Data:")
  (define sample-data '(A B A C A B C C A B A A B C))
  (define learned-transitions (estimate-transitions sample-data))
  (displayln (format "   Data: ~a" sample-data))
  (displayln "   Learned transitions:")
  (for ([(state transitions) (in-hash learned-transitions)])
    (displayln (format "     ~a -> ~a" state transitions)))

  (displayln "\n=== Markov Chain Examples Complete ==="))
