#lang racket
(provide bet
         bet/weighted
         bet/conditional
         bet/lazy
         bet-chain
         bet-compose
         bet-map
         bet-fold
         bet-filter
         bet-repeat
         bet-until
         bet-with-seed
         all-bets
         any-bet
         bet-sequence
         bet-parallel
         make-bet-generator
         bet-probability
         bet-entropy
         bet-expect)

;; (bet A B C) randomly picks one of three values with equal probability.
(define (bet a b c)
  (match (random 3)
    [0 a]
    [1 b]
    [_ c]))

;; (bet/weighted [A weight-a] [B weight-b] [C weight-c])
;; Weighted version where probabilities are proportional to weights
(define (bet/weighted . weighted-choices)
  (unless (= (length weighted-choices) 3)
    (error 'bet/weighted "expected exactly 3 weighted choices"))
  (define total-weight (apply + (map second weighted-choices)))
  (define r (* (random) total-weight))
  (let loop ([choices weighted-choices] [acc 0])
    (match choices
      ['() (first (third weighted-choices))]
      [(cons (list val weight) rest)
       (if (< r (+ acc weight))
           val
           (loop rest (+ acc weight)))])))

;; (bet/conditional predicate then-bet else-bet)
;; Conditional bet that evaluates predicate first
(define (bet/conditional pred a b c)
  (if pred a (bet b c a)))

;; (bet/lazy thunk-a thunk-b thunk-c)
;; Lazy version - only evaluates the selected branch
(define (bet/lazy thunk-a thunk-b thunk-c)
  (match (random 3)
    [0 (thunk-a)]
    [1 (thunk-b)]
    [_ (thunk-c)]))

;; (bet-chain n f init)
;; Chain n bets together, threading result through function f
(define (bet-chain n f init)
  (if (<= n 0)
      init
      (bet-chain (- n 1) f (f init))))

;; (bet-compose f g h)
;; Compose three functions into a bet
(define (bet-compose f g h)
  (lambda (x)
    (match (random 3)
      [0 (f x)]
      [1 (g x)]
      [_ (h x)])))

;; (bet-map f lst)
;; Map a function over a list with probabilistic selection
(define (bet-map f lst)
  (map (lambda (x) (bet (f x) x (f (f x)))) lst))

;; (bet-fold f init lst)
;; Fold with probabilistic choices
(define (bet-fold f init lst)
  (foldl (lambda (x acc) (bet (f x acc) acc (f acc x))) init lst))

;; (bet-filter pred lst)
;; Probabilistic filter
(define (bet-filter pred lst)
  (filter (lambda (x) (bet (pred x) #t #f)) lst))

;; (bet-repeat n thunk)
;; Repeat a bet n times and collect results
(define (bet-repeat n thunk)
  (for/list ([i (in-range n)])
    (thunk)))

;; (bet-until pred thunk)
;; Repeat bet until predicate is satisfied
(define (bet-until pred thunk)
  (let loop ()
    (define result (thunk))
    (if (pred result)
        result
        (loop))))

;; (bet-with-seed seed thunk)
;; Execute bet with specific random seed
(define (bet-with-seed seed thunk)
  (parameterize ([current-pseudo-random-generator
                  (make-pseudo-random-generator)])
    (random-seed seed)
    (thunk)))

;; (all-bets a b c)
;; Returns all three possible outcomes as a list
(define (all-bets a b c)
  (list a b c))

;; (any-bet a b c)
;; Alias for bet, emphasizing that any outcome is possible
(define (any-bet a b c)
  (bet a b c))

;; (bet-sequence . bets)
;; Execute a sequence of bets and return results as list
(define (bet-sequence . bets)
  (map (lambda (b) (apply bet b)) bets))

;; (bet-parallel n a b c)
;; Run n parallel bets and return list of results
(define (bet-parallel n a b c)
  (for/list ([i (in-range n)])
    (bet a b c)))

;; (make-bet-generator a b c)
;; Create a generator that yields bet results
(define (make-bet-generator a b c)
  (lambda () (bet a b c)))

;; Statistical utilities

;; (bet-probability n pred a b c)
;; Estimate probability that predicate holds over n trials
(define (bet-probability n pred a b c)
  (/ (count pred (bet-parallel n a b c)) n))

;; (bet-entropy samples)
;; Calculate Shannon entropy of bet outcomes
(define (bet-entropy samples)
  (define counts (make-hash))
  (for ([s samples])
    (hash-set! counts s (+ 1 (hash-ref counts s 0))))
  (define total (length samples))
  (define probs (hash-values counts))
  (- (apply + (map (lambda (p)
                     (define prob (/ p total))
                     (* prob (log prob 2)))
                   probs))))

;; (bet-expect n f a b c)
;; Calculate expected value of function f over n trials
(define (bet-expect n f a b c)
  (/ (apply + (map f (bet-parallel n a b c))) n))
