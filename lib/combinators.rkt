#lang racket
(require "../core/betlang.rkt")
(provide bet-pure
         bet-bind
         bet-join
         bet-lift
         bet-ap
         bet-or
         bet-and
         bet-xor
         bet-not
         bet-maybe
         bet-either
         bet-try
         bet-guard
         bet-when
         bet-unless
         bet-cond
         bet-case
         bet-match
         bet-memoize
         bet-cache
         bet-throttle
         bet-debounce
         bet-pipeline
         bet-fork-join
         bet-race
         bet-timeout
         bet-retry
         bet-fallback)

;; Monadic operations

(define (bet-pure x)
  (lambda () x))

(define (bet-bind m f)
  (lambda ()
    (f (m))))

(define (bet-join mm)
  (lambda ()
    ((mm))))

(define (bet-lift f)
  (lambda args
    (lambda ()
      (apply f (map (lambda (m) (m)) args)))))

(define (bet-ap mf mx)
  (lambda ()
    ((mf) (mx))))

;; Logical combinators

(define (bet-or a b c)
  (or a (bet b c a)))

(define (bet-and a b c)
  (and a (bet b c a)))

(define (bet-xor a b c)
  (bet (and a (not b))
       (and b (not c))
       (and c (not a))))

(define (bet-not a b c)
  (bet (not a) (not b) (not c)))

;; Maybe/Either combinators

(define (bet-maybe default f mx)
  (lambda ()
    (define x (mx))
    (if x (f x) default)))

(define (bet-either left right mx)
  (lambda ()
    (define x (mx))
    (if (procedure? x)
        (left x)
        (right x))))

;; Error handling combinators

(define (bet-try thunk handler)
  (with-handlers ([exn:fail? handler])
    (thunk)))

(define (bet-guard pred thunk fallback)
  (lambda ()
    (define result (thunk))
    (if (pred result) result (fallback))))

;; Conditional combinators

(define (bet-when pred thunk)
  (lambda ()
    (when pred (thunk))))

(define (bet-unless pred thunk)
  (lambda ()
    (unless pred (thunk))))

(define (bet-cond . clauses)
  (lambda ()
    (let loop ([cs clauses])
      (match cs
        ['() (void)]
        [(cons (list pred thunk) rest)
         (if (pred)
             (thunk)
             (loop rest))]))))

(define (bet-case val . clauses)
  (lambda ()
    (let loop ([cs clauses])
      (match cs
        ['() (void)]
        [(cons (list test thunk) rest)
         (if (equal? val test)
             (thunk)
             (loop rest))]))))

(define (bet-match val . patterns)
  (lambda ()
    (let loop ([ps patterns])
      (match ps
        ['() (error 'bet-match "no matching pattern")]
        [(cons (list pattern guard thunk) rest)
         (if (and (equal? val pattern) (guard))
             (thunk)
             (loop rest))]))))

;; Performance combinators

(define (bet-memoize thunk)
  (define cache #f)
  (define cached? #f)
  (lambda ()
    (if cached?
        cache
        (begin
          (set! cache (thunk))
          (set! cached? #t)
          cache))))

(define (bet-cache ttl thunk)
  (define cache #f)
  (define timestamp #f)
  (lambda ()
    (define now (current-inexact-milliseconds))
    (if (and timestamp (< (- now timestamp) ttl))
        cache
        (begin
          (set! cache (thunk))
          (set! timestamp now)
          cache))))

(define (bet-throttle interval thunk)
  (define last-call #f)
  (lambda args
    (define now (current-inexact-milliseconds))
    (when (or (not last-call) (>= (- now last-call) interval))
      (set! last-call now)
      (apply thunk args))))

(define (bet-debounce delay thunk)
  (define timer #f)
  (lambda args
    (when timer (kill-thread timer))
    (set! timer
          (thread
           (lambda ()
             (sleep (/ delay 1000))
             (apply thunk args))))))

;; Composition combinators

(define (bet-pipeline . stages)
  (lambda (x)
    (foldl (lambda (stage acc) (stage acc)) x stages)))

(define (bet-fork-join fork . joins)
  (lambda (x)
    (define results (map (lambda (f) (f x)) (fork x)))
    (apply (bet (first joins) (second joins) (third joins))
           results)))

(define (bet-race . thunks)
  (lambda ()
    (bet (lambda () ((first thunks)))
         (lambda () ((second thunks)))
         (lambda () ((third thunks))))))

;; Resilience combinators

(define (bet-timeout ms thunk default)
  (lambda ()
    (define result-box (box #f))
    (define worker
      (thread
       (lambda ()
         (set-box! result-box (thunk)))))
    (sleep (/ ms 1000))
    (if (thread-running? worker)
        (begin
          (kill-thread worker)
          default)
        (unbox result-box))))

(define (bet-retry n thunk)
  (lambda ()
    (let loop ([attempts n])
      (with-handlers ([exn:fail?
                       (lambda (e)
                         (if (> attempts 1)
                             (loop (- attempts 1))
                             (raise e)))])
        (thunk)))))

(define (bet-fallback . thunks)
  (lambda ()
    (let loop ([ts thunks])
      (match ts
        ['() (error 'bet-fallback "all fallbacks failed")]
        [(cons t rest)
         (with-handlers ([exn:fail? (lambda (e) (loop rest))])
           (t))]))))
