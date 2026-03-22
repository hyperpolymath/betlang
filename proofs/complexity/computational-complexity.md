# Computational Complexity Analysis of Betlang

## Abstract

This document provides a rigorous computational complexity analysis of betlang operations, establishing time and space bounds, analyzing the relationship to probabilistic complexity classes, and proving efficiency properties of the core language constructs and library functions.

---

## 1. Complexity Model

### 1.1 Computational Model

We analyze betlang under the **Random Access Machine (RAM) model** with unit-cost arithmetic and the following additions:

- **Random oracle**: O(1) cost per random number generation
- **Memory model**: Unbounded memory with O(1) access
- **Word size**: O(log n) bits for input size n

### 1.2 Probabilistic Complexity

For probabilistic analysis, we consider:
- **Expected time**: E[T(n)]
- **Worst-case time**: max T(n)
- **High-probability bounds**: P(T(n) > t) ≤ δ

---

## 2. Core Primitives

### 2.1 Basic Bet

**Operation**: `(bet A B C)`

| Metric | Complexity |
|--------|------------|
| Time | O(1) |
| Space | O(1) |
| Random bits consumed | O(log 3) ≈ 1.585 bits |

*Proof*:
```
(bet A B C) =
  1. Generate random r ∈ [0,1)    ; O(1)
  2. Compute i = floor(3r)        ; O(1)
  3. Return [A,B,C][i]            ; O(1)
Total: O(1)
```
∎

### 2.2 Weighted Bet

**Operation**: `(bet/weighted '(A wₐ) '(B w_b) '(C w_c))`

| Metric | Complexity |
|--------|------------|
| Time | O(1) |
| Space | O(1) |
| Random bits | O(log W) where W = sum of weights |

*Proof*:
```
1. Compute W = wₐ + w_b + w_c    ; O(1)
2. Generate r ∈ [0,W)            ; O(1)
3. Linear scan of 3 weights      ; O(1) - constant 3 items
4. Return selected value         ; O(1)
```
∎

### 2.3 Conditional Bet

**Operation**: `(bet/conditional pred A B C)`

| Metric | Complexity |
|--------|------------|
| Time | O(T_pred) |
| Space | O(S_pred) |

where T_pred, S_pred are complexities of evaluating the predicate.

### 2.4 Lazy Bet

**Operation**: `(bet/lazy thunk_A thunk_B thunk_C)`

| Metric | Best Case | Average Case | Worst Case |
|--------|-----------|--------------|------------|
| Time | O(min T_i) | O((T_A+T_B+T_C)/3) | O(max T_i) |
| Space | O(min S_i) | O((S_A+S_B+S_C)/3) | O(max S_i) |

*Key insight*: Lazy evaluation only computes the selected branch.

---

## 3. Iteration Constructs

### 3.1 bet-chain

**Operation**: `(bet-chain n f init)`

| Metric | Complexity |
|--------|------------|
| Time | O(n · T_f) |
| Space | O(S_f) (tail-recursive) |

*Proof*:
```racket
(define (bet-chain n f v)
  (if (= n 0)
      v
      (bet-chain (- n 1) f (f v))))
```

The function calls f exactly n times. If implemented tail-recursively, space is constant in n. ∎

### 3.2 bet-until

**Operation**: `(bet-until pred thunk)`

Let p = P(pred(thunk()) = true).

| Metric | Expected | Worst Case |
|--------|----------|------------|
| Time | O(T_thunk/p) | ∞ (unbounded) |
| Space | O(S_thunk) | O(S_thunk) |

*Proof*:
Number of iterations N ~ Geometric(p).
- E[N] = 1/p
- Var(N) = (1-p)/p²
- P(N > k) = (1-p)^k

Expected time: E[N] · (T_thunk + T_pred) = O((T_thunk + T_pred)/p). ∎

**Theorem 3.1** (High-Probability Bound). For p > 0:
$$P(N > \frac{\ln(1/\delta)}{p}) \leq \delta$$

*Proof*: P(N > k) = (1-p)^k ≤ e^{-pk}. Set e^{-pk} = δ, solve for k. ∎

### 3.3 bet-repeat

**Operation**: `(bet-repeat n thunk)`

| Metric | Complexity |
|--------|------------|
| Time | O(n · T_thunk) |
| Space | O(n) (storing results) |

### 3.4 bet-parallel

**Operation**: `(bet-parallel n A B C)`

| Metric | Complexity |
|--------|------------|
| Time | O(n) |
| Space | O(n) |
| Random bits | O(n · log 3) |

*Note*: This is sequential simulation of n independent trials, not actual parallelism.

---

## 4. Higher-Order Operations

### 4.1 bet-map

**Operation**: `(bet-map f lst)`

| Metric | Complexity |
|--------|------------|
| Time | O(|lst| · T_f) |
| Space | O(|lst| + S_f) |

### 4.2 bet-filter

**Operation**: `(bet-filter pred lst)`

| Metric | Complexity |
|--------|------------|
| Time | O(|lst| · (T_pred + T_thunk)) |
| Space | O(|lst|) worst case |

*Expected output size*: Binomial(|lst|, p) where p = P(pred succeeds).

### 4.3 bet-fold

**Operation**: `(bet-fold f init lst)`

| Metric | Complexity |
|--------|------------|
| Time | O(|lst| · T_f) |
| Space | O(S_f) (tail-recursive) |

### 4.4 bet-compose

**Operation**: `(bet-compose f g h)`

| Metric | Complexity |
|--------|------------|
| Composition Time | O(1) |
| Application Time | O(max(T_f, T_g, T_h)) expected |
| Space | O(max(S_f, S_g, S_h)) |

---

## 5. Statistical Functions

### 5.1 bet-probability

**Operation**: `(bet-probability n pred A B C)`

| Metric | Complexity |
|--------|------------|
| Time | O(n · (T_bet + T_pred)) = O(n · T_pred) |
| Space | O(1) |
| Statistical precision | O(1/√n) |

### 5.2 bet-entropy

**Operation**: `(bet-entropy samples)`

| Metric | Complexity |
|--------|------------|
| Time | O(n log n) |
| Space | O(k) where k = # unique values |

*Proof*:
1. Count frequencies: O(n) with hash table
2. Compute -Σ p log p: O(k)
3. If using sorting: O(n log n)

### 5.3 bet-expect

**Operation**: `(bet-expect n f A B C)`

| Metric | Complexity |
|--------|------------|
| Time | O(n · (1 + T_f)) |
| Space | O(1) |

---

## 6. Statistical Library Complexity

### 6.1 Descriptive Statistics

| Function | Time | Space |
|----------|------|-------|
| mean | O(n) | O(1) |
| median | O(n) or O(n log n) | O(1) or O(n) |
| variance | O(n) | O(1) |
| stddev | O(n) | O(1) |
| percentile | O(n log n) | O(n) |
| mode | O(n) | O(k) |
| histogram | O(n) | O(bins) |

### 6.2 Correlation and Covariance

| Function | Time | Space |
|----------|------|-------|
| covariance | O(n) | O(1) |
| correlation | O(n) | O(1) |

### 6.3 Statistical Tests

| Function | Time | Space |
|----------|------|-------|
| chi-square-test | O(n) | O(k) |
| kolmogorov-smirnov | O(n log n) | O(n) |
| confidence-interval | O(n) | O(1) |

---

## 7. Distribution Sampling

### 7.1 Discrete Distributions

| Distribution | Time per sample | Method |
|--------------|-----------------|--------|
| uniform(a,b) | O(1) | Direct |
| bernoulli(p) | O(1) | Direct |
| binomial(n,p) | O(n) | Sum of Bernoullis |
| geometric(p) | O(1) | Inverse transform |
| poisson(λ) | O(λ) expected | Knuth's algorithm |
| categorical | O(k) or O(log k) | Linear or binary search |

**Theorem 7.1** (Binomial Complexity). Binomial(n,p) sampling is O(n) using Bernoulli sum or O(1) using normal approximation for large n.

### 7.2 Continuous Distributions

| Distribution | Time per sample | Method |
|--------------|-----------------|--------|
| normal | O(1) | Box-Muller |
| exponential | O(1) | Inverse transform |
| gamma(α,β) | O(1) amortized | Marsaglia-Tsang |
| beta(α,β) | O(1) | Ratio of gammas |
| chi-square(k) | O(1) | Sum of squared normals |

**Theorem 7.2** (Normal Sampling). Box-Muller generates 2 normal samples using 2 uniform samples and O(1) arithmetic operations.

*Proof*:
```
Z₁ = √(-2 ln U₁) cos(2π U₂)
Z₂ = √(-2 ln U₁) sin(2π U₂)
```
Operations: 1 log, 1 sqrt, 2 trig = O(1). ∎

### 7.3 Stochastic Processes

| Process | Time for n steps |
|---------|------------------|
| random-walk | O(n) |
| brownian-motion | O(n) |
| levy-flight | O(n) |

---

## 8. MCMC Complexity

### 8.1 Metropolis-Hastings

**Operation**: `(metropolis-hastings target proposal init n)`

| Metric | Complexity |
|--------|------------|
| Time | O(n · (T_target + T_proposal)) |
| Space | O(n) for samples |

**Mixing time complexity**: Depends on spectral gap of the chain.

### 8.2 Gibbs Sampling

**Operation**: `(gibbs-sampler cond-x cond-y init-x init-y n)`

| Metric | Complexity |
|--------|------------|
| Time | O(n · (T_cond-x + T_cond-y)) |
| Space | O(n) |

### 8.3 Hamiltonian Monte Carlo

**Operation**: `(hamiltonian-monte-carlo log-prob grad init n ε L)`

| Metric | Complexity |
|--------|------------|
| Time per sample | O(L · (T_grad + d)) where d = dimension |
| Space | O(d) |

*Key insight*: L leapfrog steps, each requiring gradient computation.

---

## 9. Optimization Complexity

### 9.1 Simulated Annealing

| Metric | Complexity |
|--------|------------|
| Time | O(iterations · T_objective) |
| Space | O(d) |

Convergence guarantee: Logarithmic cooling schedule guarantees convergence to global optimum but requires exponentially many iterations.

### 9.2 Genetic Algorithm

| Metric | Complexity |
|--------|------------|
| Time | O(generations · population · (T_objective + gene_length)) |
| Space | O(population · gene_length) |

### 9.3 Particle Swarm Optimization

| Metric | Complexity |
|--------|------------|
| Time | O(iterations · particles · (T_objective + d)) |
| Space | O(particles · d) |

### 9.4 Ternary Search

| Metric | Complexity |
|--------|------------|
| Time | O(log_{3/2}(range/ε) · T_f) |
| Space | O(1) |

*Proof*: Each iteration reduces search range by factor of 2/3. ∎

---

## 10. Bayesian Inference Complexity

### 10.1 Exact Inference

For discrete distributions with k states:

| Operation | Complexity |
|-----------|------------|
| Prior evaluation | O(1) |
| Likelihood computation | O(n) for n data points |
| Posterior (conjugate) | O(1) |
| Evidence (marginal) | O(k) discrete, intractable continuous |

### 10.2 Approximate Inference

| Method | Time | Space |
|--------|------|-------|
| Rejection sampling | O(n · M) where M = bound ratio | O(n) |
| Importance sampling | O(n) | O(n) |
| ABC | O(n · simulations) | O(n) |

### 10.3 Sequential Monte Carlo

| Metric | Complexity |
|--------|------------|
| Time | O(T · N · (T_transition + T_observation)) |
| Space | O(N · d) |

where T = timesteps, N = particles, d = dimension.

---

## 11. Markov Chain Complexity

### 11.1 Transition

| Operation | Complexity |
|-----------|------------|
| markov-step | O(k) for k states |
| markov-simulate n steps | O(n · k) |

With sparse transitions: O(n · average_degree)

### 11.2 Stationary Distribution

| Method | Complexity |
|--------|------------|
| Power iteration | O(iter · k²) |
| Monte Carlo estimate | O(n · k) |

### 11.3 Viterbi Algorithm

| Metric | Complexity |
|--------|------------|
| Time | O(T · k²) |
| Space | O(T · k) |

where T = sequence length, k = states.

---

## 12. Probabilistic Data Structures

### 12.1 Skip Lists

| Operation | Expected Time |
|-----------|---------------|
| Search | O(log n) |
| Insert | O(log n) |
| Delete | O(log n) |
| Space | O(n) expected |

*Proof*: Expected number of levels is O(log n) with geometric distribution. ∎

### 12.2 Bloom Filters

| Metric | Complexity |
|--------|------------|
| Insert | O(k) |
| Query | O(k) |
| Space | O(m) bits |

False positive rate: (1 - e^{-kn/m})^k

Optimal k = (m/n) ln 2

### 12.3 HyperLogLog

| Metric | Complexity |
|--------|------------|
| Insert | O(1) |
| Query | O(m) |
| Space | O(m) registers |

Standard error: 1.04/√m

---

## 13. Probabilistic Complexity Classes

### 13.1 Relationship to Complexity Classes

Betlang expressions relate to probabilistic complexity classes:

| Class | Betlang Correspondence |
|-------|----------------------|
| BPP | Poly-time with bet, 2/3 majority |
| RP | Poly-time, no false positives |
| ZPP | Expected poly-time, always correct |
| PP | Poly-time, majority correct |

### 13.2 Expressiveness

**Theorem 13.1** (BPP Simulation). Any BPP algorithm can be simulated in betlang with polynomial overhead.

*Proof sketch*:
1. BPP uses polynomial random bits
2. Each random bit can be simulated with 2 bet operations
3. Polynomial composition of polynomial = polynomial ∎

**Theorem 13.2** (Amplification). Probability of error can be reduced from 1/3 to 2^{-n} using O(n) repetitions and majority vote.

---

## 14. Space Complexity Analysis

### 14.1 Memory Usage Patterns

| Operation | Stack Depth | Heap Usage |
|-----------|-------------|------------|
| bet | O(1) | O(1) |
| bet-chain n | O(1) if tail-recursive | O(1) |
| bet-map | O(1) | O(n) |
| bet-parallel n | O(1) | O(n) |

### 14.2 Streaming Complexity

For streaming algorithms on samples:

| Statistic | Space | Update Time |
|-----------|-------|-------------|
| mean | O(1) | O(1) |
| variance | O(1) | O(1) |
| median | O(n) or O(log n) approx | varies |
| mode | O(k) | O(1) |
| entropy | O(k) | O(1) |

---

## 15. Amortized Analysis

### 15.1 Memoized Operations

**Operation**: `(bet-memoize thunk)`

| Call | Time |
|------|------|
| First | O(T_thunk) |
| Subsequent | O(1) |
| Amortized over n calls | O(T_thunk/n) → O(1) |

### 15.2 Cached Operations

**Operation**: `(bet-cache ttl thunk)`

| Scenario | Time |
|----------|------|
| Cache hit | O(1) |
| Cache miss | O(T_thunk) |
| Miss rate r | O(1 + r · T_thunk) |

---

## 16. Lower Bounds

### 16.1 Random Bit Lower Bound

**Theorem 16.1** (Entropy Lower Bound). Sampling from a distribution with entropy H requires at least H random bits on average.

*Proof*: Shannon's source coding theorem. ∎

**Corollary 16.1** Uniform ternary bet requires ≥ log₂(3) ≈ 1.585 random bits.

### 16.2 Estimation Lower Bounds

**Theorem 16.2** (Sample Complexity Lower Bound). Estimating a probability p to within ε requires Ω(1/ε²) samples.

*Proof*: Cramér-Rao lower bound. ∎

### 16.3 Simulation Lower Bound

**Theorem 16.3** (Coin Flipping Lower Bound). Generating a fair ternary outcome from fair coins requires ≥ 1.585 coin flips on average.

---

## 17. Parallel Complexity

### 17.1 Inherent Parallelism

| Operation | Parallel Time | Span |
|-----------|---------------|------|
| bet-parallel n | O(1) | O(1) |
| bet-map | O(1) | O(1) |
| bet-fold (associative f) | O(log n) | O(log n) |

### 17.2 Work-Span Analysis

| Operation | Work | Span | Parallelism |
|-----------|------|------|-------------|
| bet-parallel | O(n) | O(1) | O(n) |
| Monte Carlo n trials | O(n) | O(1) | O(n) |
| MCMC | O(n) | O(n) | O(1) - inherently sequential |

---

## 18. Complexity Summary Tables

### Core Operations

| Operation | Time | Space | Random Bits |
|-----------|------|-------|-------------|
| bet | O(1) | O(1) | 1.585 |
| bet/weighted | O(1) | O(1) | varies |
| bet/lazy | O(T_i) | O(S_i) | 1.585 |
| bet-chain n | O(n·T_f) | O(1) | n·1.585 |
| bet-until | O(T/p) exp | O(S) | 1.585/p exp |

### Statistical Functions

| Function | Time | Space |
|----------|------|-------|
| bet-probability n | O(n) | O(1) |
| bet-entropy | O(n log n) | O(k) |
| bet-expect n | O(n) | O(1) |
| bootstrap n B | O(n·B) | O(B) |

### Inference Methods

| Method | Time per sample | Total for n samples |
|--------|-----------------|---------------------|
| Rejection | O(M·T) | O(n·M·T) |
| MH | O(T_target) | O(n·T_target) |
| Gibbs | O(d·T_cond) | O(n·d·T_cond) |
| HMC | O(L·T_grad) | O(n·L·T_grad) |

---

## 19. TODOs and Open Problems

**TODO**: The following require further complexity analysis:

1. **Tight bounds for MCMC mixing**: Precise mixing time for MH on specific targets
2. **Adaptive algorithm analysis**: Amortized complexity of adaptive MCMC
3. **Cache-oblivious bounds**: Memory hierarchy effects on large simulations
4. **Communication complexity**: For distributed betlang implementations
5. **Quantum speedups**: Potential for quantum Monte Carlo acceleration

---

## References

1. Cormen, T.H., et al. (2009). *Introduction to Algorithms*, 3rd ed.
2. Motwani, R. & Raghavan, P. (1995). *Randomized Algorithms*
3. Mitzenmacher, M. & Upfal, E. (2017). *Probability and Computing*
4. Arora, S. & Barak, B. (2009). *Computational Complexity: A Modern Approach*
5. Robert, C.P. & Casella, G. (2004). *Monte Carlo Statistical Methods*
