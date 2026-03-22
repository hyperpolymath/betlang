# betlang API Reference

Complete API documentation for all betlang functions and libraries.

## Core Module (`core/betlang.rkt`)

### Basic Operations

#### `(bet A B C)`
Randomly selects one of three values with equal probability (1/3 each).

**Parameters:**
- `A`, `B`, `C`: Any Racket values

**Returns:** One of the three input values

**Example:**
```racket
(bet 1 2 3) ; Returns 1, 2, or 3
```

---

#### `(bet/weighted '(A weight-a) '(B weight-b) '(C weight-c))`
Selects one of three weighted choices where probabilities are proportional to weights.

**Parameters:**
- Three lists, each containing a value and its weight

**Returns:** One of the three values, weighted by probability

**Example:**
```racket
(bet/weighted '(rare 1) '(uncommon 3) '(common 6))
; 10% rare, 30% uncommon, 60% common
```

---

#### `(bet/conditional predicate A B C)`
Returns A if predicate is true, otherwise performs bet between B and C.

**Parameters:**
- `predicate`: Boolean expression
- `A`, `B`, `C`: Any Racket values

**Returns:** A value based on the condition

**Example:**
```racket
(bet/conditional (> x 10) 'large 'medium 'small)
```

---

#### `(bet/lazy thunk-a thunk-b thunk-c)`
Lazy version that only evaluates the selected branch.

**Parameters:**
- Three zero-argument functions (thunks)

**Returns:** Result of calling the selected thunk

**Example:**
```racket
(bet/lazy
  (lambda () (expensive-a))
  (lambda () (expensive-b))
  (lambda () (expensive-c)))
```

---

### Composition Operations

#### `(bet-chain n f init)`
Chains n bets together, threading results through function f.

**Parameters:**
- `n`: Number of iterations
- `f`: Function to apply at each step
- `init`: Initial value

**Returns:** Final value after n applications

**Example:**
```racket
(bet-chain 10 (lambda (x) (+ x 1)) 0)
; Returns 10
```

---

#### `(bet-compose f g h)`
Creates a function that randomly selects one of three functions to apply.

**Parameters:**
- `f`, `g`, `h`: Functions to compose

**Returns:** A function that applies one of f, g, or h

**Example:**
```racket
(define process (bet-compose add1 identity sub1))
(process 10) ; Returns 9, 10, or 11
```

---

#### `(bet-map f lst)`
Maps a function over a list with probabilistic selection.

**Parameters:**
- `f`: Function to apply
- `lst`: List of values

**Returns:** Transformed list

---

#### `(bet-fold f init lst)`
Fold operation with probabilistic choices.

**Parameters:**
- `f`: Binary function
- `init`: Initial accumulator
- `lst`: List to fold over

**Returns:** Accumulated value

---

#### `(bet-filter pred lst)`
Filters list with probabilistic predicate evaluation.

**Parameters:**
- `pred`: Predicate function
- `lst`: List to filter

**Returns:** Filtered list

---

### Parallel Operations

#### `(bet-parallel n A B C)`
Runs n independent trials and returns all results.

**Parameters:**
- `n`: Number of trials
- `A`, `B`, `C`: Values to bet on

**Returns:** List of n results

**Example:**
```racket
(bet-parallel 100 'heads 'tails 'edge)
; Returns list of 100 outcomes
```

---

#### `(bet-repeat n thunk)`
Repeats a bet n times and collects results.

**Parameters:**
- `n`: Number of repetitions
- `thunk`: Zero-argument function to call

**Returns:** List of n results

---

#### `(bet-sequence bet1 bet2 bet3 ...)`
Executes bets in sequence.

**Parameters:**
- Variable number of bet specifications

**Returns:** List of results

---

### Control Flow

#### `(bet-until predicate thunk)`
Repeats bet until predicate is satisfied.

**Parameters:**
- `predicate`: Function testing result
- `thunk`: Zero-argument function generating results

**Returns:** First result satisfying predicate

**Example:**
```racket
(bet-until
  (lambda (x) (equal? x 'target))
  (lambda () (bet 'target 'miss 'miss)))
```

---

#### `(bet-with-seed seed thunk)`
Executes bet with specific random seed.

**Parameters:**
- `seed`: Integer seed for random number generator
- `thunk`: Zero-argument function to execute

**Returns:** Result of thunk with deterministic randomness

**Example:**
```racket
(bet-with-seed 42 (lambda () (bet 1 2 3)))
; Always returns same value
```

---

### Utility Functions

#### `(all-bets A B C)`
Returns all three possible outcomes as a list.

**Returns:** List containing all outcomes

---

#### `(make-bet-generator A B C)`
Creates a generator function for repeated betting.

**Returns:** Zero-argument function that generates bet results

---

### Statistical Functions

#### `(bet-probability n predicate A B C)`
Estimates probability that predicate holds over n trials.

**Parameters:**
- `n`: Number of trials
- `predicate`: Function testing outcomes
- `A`, `B`, `C`: Bet values

**Returns:** Probability estimate (0.0 to 1.0)

**Example:**
```racket
(bet-probability 10000
  (lambda (x) (equal? x 'A))
  'A 'B 'C)
; ≈ 0.333
```

---

#### `(bet-entropy samples)`
Calculates Shannon entropy of samples in bits.

**Parameters:**
- `samples`: List of sample values

**Returns:** Entropy value

---

#### `(bet-expect n f A B C)`
Calculates expected value of function f over n trials.

**Parameters:**
- `n`: Number of trials
- `f`: Function to apply to outcomes
- `A`, `B`, `C`: Bet values

**Returns:** Expected value

---

## Statistics Module (`lib/statistics.rkt`)

### Descriptive Statistics

#### `(mean samples)`
Arithmetic mean of samples.

---

#### `(median samples)`
Median value.

---

#### `(mode samples)`
Most frequent value(s).

---

#### `(variance samples)`
Sample variance.

---

#### `(stddev samples)`
Standard deviation.

---

#### `(percentile samples p)`
p-th percentile (p between 0 and 1).

---

### Correlation and Covariance

#### `(covariance samples1 samples2)`
Covariance between two samples.

---

#### `(correlation samples1 samples2)`
Pearson correlation coefficient.

---

### Statistical Tests

#### `(chi-square-test observed expected)`
Chi-square goodness of fit test.

**Parameters:**
- `observed`: List of observed frequencies
- `expected`: List of expected frequencies

**Returns:** Chi-square statistic

---

#### `(kolmogorov-smirnov samples1 samples2)`
KS test statistic for comparing distributions.

---

### Resampling Methods

#### `(bootstrap samples n statistic)`
Bootstrap resampling.

**Parameters:**
- `samples`: Original sample
- `n`: Number of bootstrap samples
- `statistic`: Function to compute on each resample

**Returns:** List of n statistic values

---

#### `(jackknife samples statistic)`
Jackknife resampling.

---

### Time Series

#### `(moving-average samples window)`
Moving average with window size.

---

#### `(exponential-smoothing samples alpha)`
Exponential smoothing with parameter alpha.

---

### Simulation

#### `(run-simulation n experiment)`
Runs experiment n times.

---

#### `(monte-carlo-pi n)`
Estimates π using Monte Carlo method.

---

## Distributions Module (`lib/distributions.rkt`)

### Discrete Distributions

#### `(uniform a b)`
Discrete uniform distribution from a to b.

---

#### `(bernoulli p)`
Bernoulli trial with success probability p.

---

#### `(binomial n p)`
Binomial distribution: n trials, probability p.

---

#### `(geometric p)`
Geometric distribution: trials until first success.

---

#### `(poisson lambda)`
Poisson distribution with rate lambda.

---

#### `(categorical probs)`
Categorical distribution given probability list.

---

### Continuous Distributions

#### `(normal mu sigma)`
Normal (Gaussian) distribution.

**Parameters:**
- `mu`: Mean
- `sigma`: Standard deviation

---

#### `(exponential lambda)`
Exponential distribution with rate lambda.

---

#### `(gamma shape scale)`
Gamma distribution.

---

#### `(beta alpha beta)`
Beta distribution.

---

#### `(student-t df)`
Student's t-distribution with df degrees of freedom.

---

#### `(chi-square k)`
Chi-square distribution with k degrees of freedom.

---

### Stochastic Processes

#### `(random-walk n)`
Random walk of n steps.

**Returns:** List of positions

---

#### `(brownian-motion n dt)`
Brownian motion simulation.

**Parameters:**
- `n`: Number of steps
- `dt`: Time increment

---

#### `(levy-flight n alpha)`
Lévy flight with stability parameter alpha.

---

## Combinators Module (`lib/combinators.rkt`)

### Monadic Operations

#### `(bet-pure x)`
Returns deterministic bet always returning x.

---

#### `(bet-bind m f)`
Monadic bind operation.

---

### Logical Combinators

#### `(bet-or a b c)`, `(bet-and a b c)`, `(bet-xor a b c)`
Logical operations on ternary values.

---

### Error Handling

#### `(bet-try thunk handler)`
Try-catch for bets.

---

#### `(bet-fallback thunk1 thunk2 ...)`
Try thunks in order until one succeeds.

---

#### `(bet-retry n thunk)`
Retry up to n times on failure.

---

### Performance

#### `(bet-memoize thunk)`
Memoize bet result (cache permanently).

---

#### `(bet-cache ttl thunk)`
Cache with time-to-live in milliseconds.

---

#### `(bet-throttle interval thunk)`
Throttle execution (minimum interval between calls).

---

## Markov Chains Module (`lib/markov.rkt`)

#### `(make-markov-chain states transitions initial)`
Creates a Markov chain.

**Parameters:**
- `states`: List of possible states
- `transitions`: Hash of transition probabilities
- `initial`: Initial state

---

#### `(markov-step chain current-state)`
Takes one step in the chain.

---

#### `(markov-simulate chain n)`
Simulates chain for n steps.

---

#### `(markov-stationary chain n-simulations)`
Estimates stationary distribution.

---

#### `(estimate-transitions data)`
Learns transition matrix from observed data.

---

#### `(ternary-markov a b c)`
Creates 3-state Markov chain with given transition weights.

---

## Analysis Tools (`tools/analyzer.rkt`)

#### `(analyze-bet bet-fn n)`
Comprehensive analysis of a bet function.

**Parameters:**
- `bet-fn`: Zero-argument function to analyze
- `n`: Number of samples

**Displays:** Frequency distribution, statistics, entropy, histogram

---

#### `(compare-bets bet-fns names n)`
Compare multiple betting strategies.

---

#### `(convergence-analysis bet-fn target-outcome trials)`
Analyze convergence to theoretical probability.

---

#### `(text-histogram freq max-width)`
Display text-based histogram.

---

## REPL Commands

- `:help` - Show help message
- `:stats` - Display session statistics
- `:reset-stats` - Reset statistics
- `:history` - Show command history
- `:examples` - Show example usage
- `:quit` - Exit REPL

---

## Type Signatures (Informal)

```
bet : α → β → γ → (α | β | γ)
bet/weighted : (α × ℝ) → (β × ℝ) → (γ × ℝ) → (α | β | γ)
bet-parallel : ℕ → α → β → γ → List[α | β | γ]
bet-probability : ℕ → (α → 𝔹) → α → β → γ → ℝ ∈ [0,1]
mean : List[ℝ] → ℝ
normal : ℝ → ℝ → ℝ
markov-simulate : MarkovChain → ℕ → List[State]
```

---

## Constants and Special Values

- `pi` - Mathematical constant π (from Racket)
- Random seed can be any integer for `bet-with-seed`

---

## Error Conditions

All functions raise `exn:fail?` exceptions on invalid input:
- `bet/weighted` requires exactly 3 weighted choices
- Statistical functions require non-empty lists
- Probabilities must be in range [0, 1]
- Markov chains require valid state transitions

---

## Performance Notes

- `bet-parallel` is O(n) in number of trials
- `bet-chain` is O(n) in chain length
- Distribution sampling varies (normal: O(1), gamma: varies)
- Markov simulation is O(steps × states)

---

## Thread Safety

Bet operations are **not thread-safe** by default. Use Racket's synchronization primitives (`semaphore`, `channel`, etc.) for concurrent access.

---

## Version

API Version: 2.0
Last Updated: 2025

---

For more information, see:
- [Tutorial](tutorial.md) - Learn by example
- [Semantics](semantics.md) - Formal specifications
- [Examples](../examples/) - Code examples
