#lang racket
(require racket/readline racket/format racket/pretty)
(require "../core/betlang.rkt")
(require "../lib/statistics.rkt")
(require "../lib/combinators.rkt")
(require "../lib/distributions.rkt")

(define logfile "../logs/session.log")
(define history-file "../logs/history.txt")
(define stats (make-hash))

;; Ensure logs directory exists
(make-directory* "../logs")

;; Statistics tracking
(define (track-result result)
  (hash-set! stats result (+ 1 (hash-ref stats result 0))))

(define (show-stats)
  (displayln "\n📊 Session Statistics:")
  (for ([(k v) (in-hash stats)])
    (displayln (format "  ~a: ~a times" k v)))
  (displayln (format "  Total evaluations: ~a" (apply + (hash-values stats)))))

(define (reset-stats)
  (set! stats (make-hash))
  (displayln "Statistics reset."))

;; Logging
(define (log-line txt)
  (with-output-to-file logfile #:exists 'append
    (lambda () (displayln txt))))

(define (save-history expr)
  (with-output-to-file history-file #:exists 'append
    (lambda () (displayln expr))))

;; Help system
(define (show-help)
  (displayln "
🎰 betlang REPL - Ternary Probabilistic Programming Language

BASIC COMMANDS:
  (bet A B C)              - Randomly select one of three values
  (bet/weighted '(A w1) '(B w2) '(C w3)) - Weighted selection
  (bet/conditional pred A B C) - Conditional bet
  (bet/lazy thunk-a thunk-b thunk-c) - Lazy evaluation

COMPOSITION:
  (bet-chain n f init)     - Chain n bets together
  (bet-compose f g h)      - Compose functions into a bet
  (bet-map f lst)          - Map with probabilistic selection
  (bet-parallel n A B C)   - Run n parallel bets

STATISTICS:
  (bet-probability n pred A B C) - Estimate probability
  (bet-entropy samples)    - Calculate entropy
  (bet-expect n f A B C)   - Calculate expected value
  (mean samples)           - Calculate mean
  (stddev samples)         - Standard deviation

DISTRIBUTIONS:
  (normal mu sigma)        - Normal distribution
  (binomial n p)          - Binomial distribution
  (poisson lambda)        - Poisson distribution
  (exponential lambda)    - Exponential distribution
  (random-walk n)         - Random walk process

REPL COMMANDS:
  :help or :h             - Show this help
  :stats                  - Show session statistics
  :reset-stats            - Reset statistics
  :history                - Show command history
  :clear                  - Clear screen
  :quit or :q             - Exit REPL
  :examples               - Show example usage

Try: (bet 1 2 3) or (bet-parallel 10 'heads 'tails 'edge)
"))

(define (show-examples)
  (displayln "
📚 EXAMPLE USAGE:

1. Basic bet:
   (bet \"Win\" \"Draw\" \"Lose\")

2. Weighted bet (50%, 30%, 20%):
   (bet/weighted '(\"Common\" 5) '(\"Uncommon\" 3) '(\"Rare\" 2))

3. Run 100 trials:
   (bet-parallel 100 'A 'B 'C)

4. Calculate probability:
   (bet-probability 1000 (lambda (x) (equal? x 'A)) 'A 'B 'C)

5. Expected value:
   (bet-expect 1000 (lambda (x) x) 1 2 3)

6. Random walk:
   (random-walk 50)

7. Normal distribution sample:
   (sample-from (lambda () (normal 0 1)) 100)

8. Composed bet:
   (define coin-flip (bet-compose add1 identity sub1))
   (coin-flip 10)
"))

;; REPL commands
(define (handle-command cmd)
  (match cmd
    [(or ':help ':h) (show-help) #t]
    [':stats (show-stats) #t]
    [':reset-stats (reset-stats) #t]
    [':history (show-command-history) #t]
    [':clear (system "clear") #t]
    [(or ':quit ':q) (displayln "Goodbye! 🎲") (exit)]
    [':examples (show-examples) #t]
    [_ #f]))

(define command-history '())

(define (save-to-history expr)
  (set! command-history (cons expr command-history)))

(define (show-command-history)
  (displayln "\n📜 Command History:")
  (for ([expr (reverse command-history)]
        [i (in-naturals 1)])
    (displayln (format "  ~a: ~a" i expr))))

;; Main REPL
(displayln "╔══════════════════════════════════════════╗")
(displayln "║   🎰 Welcome to betlang REPL v2.0       ║")
(displayln "║   Ternary Probabilistic Programming     ║")
(displayln "╚══════════════════════════════════════════╝")
(displayln "")
(displayln "Type :help for commands, :examples for usage")
(displayln "Type :quit to exit")
(displayln "")

(let loop ()
  (display "betlang> ") (flush-output)
  (define input (read))

  (unless (eof-object? input)
    (cond
      ;; Handle REPL commands
      [(handle-command input) (loop)]

      ;; Evaluate expressions
      [else
       (with-handlers ([exn:fail?
                        (lambda (e)
                          (displayln (format "❌ Error: ~a" (exn-message e)))
                          (log-line (format "ERROR: ~a | ~a" input (exn-message e))))])
         (save-to-history input)
         (save-history input)
         (define result (eval input))
         (pretty-print result)
         (track-result result)
         (log-line (format "INPUT: ~a | OUTPUT: ~a" input result)))
       (loop)])))
