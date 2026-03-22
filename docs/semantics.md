# betlang Semantics

Betlang is a ternary DSL for probabilistic modeling and symbolic wagers, implemented in Racket. It provides a rich set of primitives for expressing and analyzing probabilistic computations.

## Core Philosophy

The language is built around the ternary principle - all fundamental operations involve three choices, inspired by musical ternary form (A–B–A). This design creates a natural framework for modeling uncertainty and probabilistic decision-making.

## Core Forms

### Basic Bet

```racket
(bet A B C) → randomly selects one of three values
```

The fundamental primitive of betlang. Each value has equal probability (1/3) of being selected.

**Examples:**
```racket
(bet 1 2 3)          ; Returns 1, 2, or 3
(bet 'win 'draw 'lose)  ; Returns one of three symbols
(bet "A" "B" "C")    ; Returns one of three strings
```

**Formal Semantics:**
```
⟦bet A B C⟧ = A  with probability 1/3
             = B  with probability 1/3
             = C  with probability 1/3
```

### Weighted Bet

```racket
(bet/weighted '(A weight-a) '(B weight-b) '(C weight-c))
```

Allows non-uniform probability distributions where probabilities are proportional to weights.

**Examples:**
```racket
(bet/weighted '(rare 1) '(uncommon 3) '(common 6))
; P(rare) = 0.1, P(uncommon) = 0.3, P(common) = 0.6
```

**Formal Semantics:**
```
⟦bet/weighted (A w₁) (B w₂) (C w₃)⟧ = A  with probability w₁/(w₁+w₂+w₃)
                                     = B  with probability w₂/(w₁+w₂+w₃)
                                     = C  with probability w₃/(w₁+w₂+w₃)
```

### Conditional Bet

```racket
(bet/conditional predicate A B C)
```

Deterministically returns A if predicate is true, otherwise performs a bet between B and C (with fallback to A).

**Examples:**
```racket
(bet/conditional (> x 10) 'large 'medium 'small)
```

**Formal Semantics:**
```
⟦bet/conditional pred A B C⟧ = A           if pred = true
                              = ⟦bet B C A⟧  if pred = false
```

### Lazy Bet

```racket
(bet/lazy thunk-a thunk-b thunk-c)
```

Delays evaluation - only the selected branch is computed, avoiding unnecessary computation.

**Examples:**
```racket
(bet/lazy
  (lambda () (expensive-computation-a))
  (lambda () (expensive-computation-b))
  (lambda () (expensive-computation-c)))
```

## Composition Operators

### Bet Chain

```racket
(bet-chain n f init)
```

Chains n probabilistic transformations together, threading results through function f.

**Formal Semantics:**
```
⟦bet-chain 0 f init⟧ = init
⟦bet-chain n f init⟧ = ⟦bet-chain (n-1) f (f init)⟧
```

### Bet Compose

```racket
(bet-compose f g h)
```

Creates a higher-order function that randomly selects one of three functions to apply.

**Examples:**
```racket
(define process (bet-compose add1 identity sub1))
(process 10)  ; Returns 11, 10, or 9
```

### Bet Map

```racket
(bet-map f lst)
```

Probabilistically applies a function to list elements.

### Bet Fold

```racket
(bet-fold f init lst)
```

Fold with probabilistic choices at each step.

## Parallel Operations

### Bet Parallel

```racket
(bet-parallel n A B C)
```

Runs n independent trials of the bet and returns all results as a list.

**Examples:**
```racket
(bet-parallel 100 'heads 'tails 'edge)
; Returns list of 100 coin flips
```

### Bet Sequence

```racket
(bet-sequence bet1 bet2 bet3 ...)
```

Executes multiple bets in sequence and returns all results.

## Control Flow

### Bet Until

```racket
(bet-until predicate thunk)
```

Repeatedly executes bet until predicate is satisfied.

**Formal Semantics:**
```
⟦bet-until pred thunk⟧ = result  where result is first value from thunk satisfying pred
```

### Bet Repeat

```racket
(bet-repeat n thunk)
```

Executes bet n times and collects results.

## Statistical Utilities

### Bet Probability

```racket
(bet-probability n predicate A B C)
```

Estimates probability that predicate holds by running n trials.

**Examples:**
```racket
(bet-probability 10000 (lambda (x) (equal? x 'A)) 'A 'B 'C)
; Returns approximately 0.333
```

### Bet Entropy

```racket
(bet-entropy samples)
```

Calculates Shannon entropy of bet outcomes in bits.

**Formula:**
```
H(X) = -Σ P(xᵢ) log₂ P(xᵢ)
```

For uniform ternary bet: H = log₂(3) ≈ 1.585 bits

### Bet Expect

```racket
(bet-expect n f A B C)
```

Calculates expected value of function f over n trials.

**Formula:**
```
E[f(X)] ≈ (1/n) Σ f(xᵢ)
```

## Determinism and Reproducibility

### Bet With Seed

```racket
(bet-with-seed seed thunk)
```

Executes bet with a specific random seed for reproducible results.

**Examples:**
```racket
(bet-with-seed 42 (lambda () (bet 1 2 3)))
; Always returns the same value for seed 42
```

## Type System (Informal)

Betlang is dynamically typed through Racket, but conceptually:

```
Bet[A, B, C] :: Type A → Type B → Type C → Bet (A | B | C)
```

Where `Bet T` represents a probabilistic value of type T.

## Probability Monad (Informal)

The bet operations form a monad-like structure:

```racket
return x = (bet x x x)           ; Pure/deterministic value
bind m f = (bet (f (bet A B C))  ; Monadic bind
                (f (bet A B C))
                (f (bet A B C)))
```

More formally available through `bet-pure` and `bet-bind` in combinators.

## Equational Properties

### Symmetry
For uniform bet: `(bet A B C) ≡ (bet B C A) ≡ (bet C A B)` (distributionally)

### Idempotence
`(bet X X X) = X` (deterministic)

### Commutativity (distributional)
`(bet A B C)` and `(bet B A C)` have same distribution up to permutation

## Error Semantics

All bet operations propagate Racket exceptions. Invalid arguments raise `exn:fail` exceptions.

## Memory Semantics

Bets are evaluated eagerly by default (except `bet/lazy`). Results are not memoized unless explicitly using `bet-memoize` combinator.

## Concurrency Semantics

Bet operations are not thread-safe by default. Use Racket's synchronization primitives for concurrent access.

## Integration with Racket

Betlang is implemented as a Racket library. All Racket primitives and libraries are available. Bets can be freely mixed with Racket code:

```racket
(define result (bet 1 2 3))
(if (> result 2)
    (displayln "Large!")
    (displayln "Small!"))
```

## Advanced Semantics

### Probability Distributions

Through `lib/distributions.rkt`, betlang provides:
- Discrete: binomial, geometric, Poisson, multinomial
- Continuous: normal, exponential, gamma, beta
- Stochastic processes: random walks, Brownian motion

### Markov Chains

Through `lib/markov.rkt`, betlang supports:
- Discrete-time Markov chains
- Transition matrix learning
- Stationary distribution estimation
- Hidden Markov Models (simplified)

### Statistical Inference

Through `lib/statistics.rkt`:
- Descriptive statistics
- Hypothesis testing
- Resampling methods (bootstrap, jackknife)
- Time series analysis

## Notation Conventions

Throughout this documentation:
- `⟦expr⟧` denotes semantic interpretation
- `A ≡ B` denotes distributional equivalence
- `P(event)` denotes probability
- `E[X]` denotes expected value
- `H(X)` denotes entropy

## Future Extensions

Potential semantic extensions under consideration:
- Continuous probability support
- Bayesian inference primitives
- Automatic differentiation for probabilistic programs
- Parallel/distributed execution model
