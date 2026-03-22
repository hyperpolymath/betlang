# betlang Tutorial

Welcome to betlang! This tutorial will guide you through the ternary probabilistic programming language from basics to advanced topics.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation & Setup](#installation--setup)
3. [Your First Bet](#your-first-bet)
4. [Understanding Probability](#understanding-probability)
5. [Weighted Bets](#weighted-bets)
6. [Composition & Chaining](#composition--chaining)
7. [Statistical Analysis](#statistical-analysis)
8. [Probability Distributions](#probability-distributions)
9. [Markov Chains](#markov-chains)
10. [Real-World Applications](#real-world-applications)

## Introduction

Betlang is a Domain-Specific Language (DSL) for probabilistic programming built on Racket. Its core primitive is the **ternary bet** - a choice between three values with associated probabilities. This simple primitive enables elegant expression of complex probabilistic models.

### Why Ternary?

Most probabilistic languages focus on binary choices. Betlang's ternary approach:
- Models three-way decisions naturally (win/draw/lose, yes/no/maybe)
- Provides richer expressiveness than binary
- Inspired by musical ternary form (A-B-A)
- Creates interesting emergent properties

## Installation & Setup

### Prerequisites

- Racket 7.0 or later

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/hyperpolymath/betlang.git
cd betlang
```

2. Start the REPL:
```bash
racket repl/shell.rkt
```

3. You should see:
```
╔══════════════════════════════════════════╗
║   🎰 Welcome to betlang REPL v2.0       ║
║   Ternary Probabilistic Programming     ║
╚══════════════════════════════════════════╝
```

## Your First Bet

Let's start with the simplest bet - choosing between three values:

```racket
betlang> (bet 1 2 3)
2
betlang> (bet 1 2 3)
3
betlang> (bet 1 2 3)
1
```

Each time you run it, you get one of the three values. Each has equal probability: 1/3.

### Try Different Types

Bets work with any values:

```racket
betlang> (bet 'red 'green 'blue)
green

betlang> (bet "Win" "Draw" "Lose")
"Win"

betlang> (bet #t #f 'maybe)
maybe
```

### Bets in Expressions

Bets integrate seamlessly with Racket code:

```racket
betlang> (+ 10 (bet 1 2 3))
12

betlang> (if (equal? (bet 'a 'b 'c) 'a)
             "Got A!"
             "Not A")
"Got A!"
```

## Keyword-delimited Syntax (preferred)

Betlang now exposes a Julia-style keyword syntax where each block ends with `end`.
It desugars directly to the same AST as the brace-based form, but many users find it
easier to read and write. The brace form is still accepted for backwards compatibility,
but the `end` style is the recommended one going forward.

```betlang
if x > 0 then
  bet "Positive" "Zero" "Negative"
else
  "No change"
end

let result = bet "A" "B" "C" in
  if (equal? result "B") then
    "Hit"
  else
    "Miss"
  end
end

match value
  'A -> "Alpha"
  'B -> "Beta"
  other -> "Other"
end

do
  x <- sample normal
  y <- sample normal
  return (+ x y)
end

parallel 4 do
  sample normal
end
```

Each keyword form (`bet`, `let … in`, `if … then … else`, `match`, `do`, `parallel`) can resurface general-purpose expressions, and the trailing `end` keeps the structure explicit. Old-style `{}`/`,` grouping is still parsed, but aim for `end` blocks for new code.

## Understanding Probability

### Running Multiple Trials

Use `bet-parallel` to run many trials:

```racket
betlang> (bet-parallel 10 'heads 'tails 'edge)
(heads edge tails heads heads tails heads edge tails heads)
```

### Estimating Probabilities

Let's verify the 1/3 probability:

```racket
betlang> (bet-probability 10000
           (lambda (x) (equal? x 'heads))
           'heads 'tails 'edge)
0.3337
```

Close to 0.333 (1/3)!

### Frequency Analysis

Count how often each outcome appears:

```racket
betlang> (require "../lib/statistics.rkt")
betlang> (define results (bet-parallel 1000 'A 'B 'C))
betlang> (frequency-table results)
((A . 334) (B . 333) (C . 333))
```

## Weighted Bets

Sometimes you need non-uniform probabilities:

```racket
betlang> (bet/weighted '(common 6) '(uncommon 3) '(rare 1))
```

This creates:
- P(common) = 6/10 = 60%
- P(uncommon) = 3/10 = 30%
- P(rare) = 1/10 = 10%

### Practical Example: Loot Drops

Simulating game loot with varying rarity:

```racket
(define (drop-loot)
  (bet/weighted
    '("Common Sword" 7)
    '("Rare Shield" 2)
    '("Legendary Armor" 1)))

;; Simulate 100 drops
(define drops (for/list ([i (in-range 100)])
                (drop-loot)))

(frequency-table drops)
```

## Composition & Chaining

### Bet Composition

Create higher-order probabilistic functions:

```racket
betlang> (define random-op (bet-compose add1 identity sub1))
betlang> (random-op 10)
11
betlang> (random-op 10)
9
betlang> (random-op 10)
10
```

### Chaining Bets

Thread values through multiple probabilistic transformations:

```racket
(define (step x)
  (+ x (bet -1 0 1)))

(bet-chain 10 step 0)
; Random walk of 10 steps
```

### Lazy Evaluation

Only compute the branch you need:

```racket
(bet/lazy
  (lambda () (displayln "Computing A") 'A)
  (lambda () (displayln "Computing B") 'B)
  (lambda () (displayln "Computing C") 'C))

;; Prints only one message!
```

## Statistical Analysis

### Basic Statistics

```racket
(require "../lib/statistics.rkt")

(define data (bet-parallel 1000 1 2 3))

(mean data)        ; ~2.0
(median data)      ; 2
(stddev data)      ; ~0.816
(variance data)    ; ~0.666
```

### Entropy

Measure information content:

```racket
(define samples (bet-parallel 1000 'A 'B 'C))
(bet-entropy samples)
; ~1.585 bits (maximum for 3 outcomes)
```

### Expected Value

```racket
(bet-expect 10000
  (lambda (x) x)  ; identity function
  1 2 3)
; ~2.0 (mean of 1, 2, 3)
```

### Correlation

```racket
(define x-vals '(1 2 3 4 5))
(define y-vals '(2 4 6 8 10))
(correlation x-vals y-vals)
; 1.0 (perfect positive correlation)
```

## Probability Distributions

Betlang provides many standard distributions:

### Discrete Distributions

```racket
(require "../lib/distributions.rkt")

;; Binomial: n trials, probability p
(binomial 10 0.5)
; Number of heads in 10 fair coin flips

;; Geometric: trials until first success
(geometric 0.3)
; Number of attempts until success

;; Poisson: events in fixed interval
(poisson 5.0)
; Number of events with rate 5
```

### Continuous Distributions

```racket
;; Normal (Gaussian)
(normal 0 1)
; Standard normal

(normal 100 15)
; IQ scores (mean=100, sd=15)

;; Exponential
(exponential 0.5)
; Time between events

;; Gamma
(gamma 2 2)
; Waiting time for multiple events
```

### Stochastic Processes

```racket
;; Random Walk
(random-walk 100)
; 100-step random walk

;; Brownian Motion
(brownian-motion 100 0.01)
; Continuous random walk
```

## Markov Chains

Model systems with state transitions:

```racket
(require "../lib/markov.rkt")

;; Weather model
(define weather-chain
  (make-markov-chain
    '(sunny cloudy rainy)
    (hash 'sunny  '((sunny 0.7) (cloudy 0.2) (rainy 0.1))
          'cloudy '((sunny 0.3) (cloudy 0.4) (rainy 0.3))
          'rainy  '((sunny 0.2) (cloudy 0.3) (rainy 0.5)))
    'sunny))

;; Simulate a week
(markov-simulate weather-chain 7)
; (sunny sunny cloudy rainy rainy rainy cloudy sunny)
```

### Learning from Data

```racket
;; Observe sequence
(define data '(A B A C A B C C A B A))

;; Learn transition probabilities
(define transitions (estimate-transitions data))

;; Create chain
(define learned-chain
  (make-markov-chain
    '(A B C)
    transitions
    'A))

;; Generate new sequences
(markov-simulate learned-chain 20)
```

## Real-World Applications

### 1. Monte Carlo Simulation

Estimate π using random sampling:

```racket
(require "../examples/monte-carlo.rkt")

(monte-carlo-pi 100000)
; ~3.14159
```

### 2. Game Theory

Simulate repeated games:

```racket
(require "../examples/game-theory.rkt")

;; Rock-Paper-Scissors tournament
(define (random-rps) (bet 'rock 'paper 'scissors))
(play-rps random-rps random-rps 1000)
```

### 3. A/B Testing

Compare two strategies:

```racket
(define strategy-a
  (lambda () (bet/weighted '(success 3) '(neutral 1) '(failure 1))))

(define strategy-b
  (lambda () (bet/weighted '(success 2) '(neutral 2) '(failure 1))))

(define results-a (bet-repeat 1000 strategy-a))
(define results-b (bet-repeat 1000 strategy-b))

(mean (map (lambda (x) (if (equal? x 'success) 1 0)) results-a))
; Success rate for A

(mean (map (lambda (x) (if (equal? x 'success) 1 0)) results-b))
; Success rate for B
```

### 4. Risk Analysis

Model uncertain outcomes:

```racket
(define (project-outcome)
  (bet/weighted
    '("Success: $1M" 5)
    '("Partial: $200K" 3)
    '("Failure: -$500K" 2)))

;; Run 10000 scenarios
(define outcomes (bet-repeat 10000 project-outcome))

;; Analyze risk
(frequency-table outcomes)
```

### 5. Traffic Simulation

```racket
(define (traffic-light-duration)
  (bet 30 45 60))  ; Seconds

(define (cars-passing)
  (bet/weighted '(5 3) '(10 5) '(15 2)))

;; Simulate hour
(define total-cars
  (for/sum ([i (in-range 60)])  ; 60 cycles per hour
    (cars-passing)))
```

## Advanced Topics

### Conditional Betting

Make decisions based on conditions:

```racket
(define temperature 75)

(bet/conditional
  (> temperature 70)
  'swim        ; If hot
  'hike        ; If not (random between these)
  'read)       ; If not
```

### Bet Combinators

Build complex behaviors from simple parts:

```racket
(require "../lib/combinators.rkt")

;; Retry with fallback
(define risky-operation
  (bet-fallback
    (lambda () (bet 'success 'failure 'failure))
    (lambda () 'backup-success)))

;; Memoize expensive computation
(define expensive-bet
  (bet-memoize
    (lambda ()
      (sleep 1)
      (bet 'cached-a 'cached-b 'cached-c))))

;; First call: slow
(expensive-bet)  ; 1 second delay

;; Subsequent calls: instant
(expensive-bet)  ; Returns immediately!
```

### Analysis Tools

```racket
(require "../tools/analyzer.rkt")

;; Comprehensive analysis
(analyze-bet
  (lambda () (bet 'A 'B 'C))
  10000)

;; Convergence check
(convergence-analysis
  (lambda () (bet 'heads 'tails 'edge))
  'heads
  10000)
```

## Best Practices

### 1. Use Seeds for Reproducibility

```racket
(bet-with-seed 42
  (lambda () (bet-parallel 10 1 2 3)))
; Same result every time with seed 42
```

### 2. Verify Distributions

Always check your probability distribution makes sense:

```racket
(define results (bet-parallel 10000 'A 'B 'C))
(frequency-table results)
; Should be roughly equal
```

### 3. Test with Large Samples

Probabilistic code needs statistical validation:

```racket
(define prob (bet-probability 100000
               (lambda (x) (equal? x 'rare))
               'common 'uncommon 'rare))

(check-true (< 0.30 prob 0.36))  ; Should be ~1/3
```

### 4. Profile Performance

```racket
(time
  (bet-parallel 1000000 1 2 3))
; Measure execution time
```

## Next Steps

- Explore the [API Reference](api-reference.md) for complete function documentation
- Read the [Semantics](semantics.md) for formal specifications
- Check out [examples/](../examples/) for more complex programs
- Run tests with `racket tests/basics.rkt`
- Try the analyzer: `racket tools/analyzer.rkt`

## Getting Help

- Type `:help` in the REPL for command reference
- Type `:examples` for quick examples
- Read the source code - it's well-commented!
- Check out the examples in `examples/` directory

## Summary

You've learned:
- ✓ Basic bet syntax and usage
- ✓ Weighted probability distributions
- ✓ Composition and chaining
- ✓ Statistical analysis
- ✓ Probability distributions
- ✓ Markov chains
- ✓ Real-world applications

Happy betting! 🎲
