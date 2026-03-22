# Comparative Analysis: Betlang and Probabilistic Programming Languages

## Abstract

This document provides a rigorous comparison of betlang with established probabilistic programming languages (PPLs) including Church, Anglican, Stan, Pyro, Edward, Gen, and Turing. We analyze semantic foundations, expressiveness, inference capabilities, and theoretical properties.

---

## 1. Taxonomy of Probabilistic Programming Languages

### 1.1 Classification Dimensions

| Dimension | Categories |
|-----------|------------|
| Paradigm | Functional, Imperative, Declarative |
| Typing | Static, Dynamic, Gradual |
| Primitive | Binary flip, Continuous sample, Ternary bet |
| Inference | Exact, Sampling, Variational |
| Host language | Standalone, Embedded |

### 1.2 Language Overview

| Language | Year | Host | Paradigm | Primary Inference |
|----------|------|------|----------|-------------------|
| BUGS | 1989 | Standalone | Declarative | Gibbs |
| Church | 2008 | Scheme | Functional | MH, Enumeration |
| Stan | 2012 | Standalone | Imperative | HMC/NUTS |
| Anglican | 2014 | Clojure | Functional | SMC, MCMC |
| Edward | 2016 | Python/TF | Functional | VI |
| Pyro | 2017 | Python/PyTorch | Functional | VI, MCMC |
| Gen | 2019 | Julia | Functional | Programmable |
| Turing | 2018 | Julia | Functional | MCMC |
| **Betlang** | 2024 | Racket | Functional | Monte Carlo |

---

## 2. Primitive Operations

### 2.1 Comparison of Primitives

**Church** (Binary):
```scheme
(flip 0.5)  ; Bernoulli(0.5)
(uniform 0 1)  ; Uniform continuous
```

**Stan** (Continuous):
```stan
y ~ normal(0, 1);  ; Sampling statement
target += normal_lpdf(y | 0, 1);  ; Log-prob
```

**Pyro** (Tensor-based):
```python
pyro.sample("x", dist.Normal(0, 1))
```

**Betlang** (Ternary):
```racket
(bet A B C)  ; Uniform ternary
(bet/weighted '(A 1) '(B 2) '(C 1))  ; Weighted
```

### 2.2 Expressiveness Comparison

**Theorem 2.1** (Inter-reducibility). The following primitives are inter-reducible:
1. Binary flip ⟺ Ternary bet (with constant overhead)
2. Discrete uniform ⟺ Binary flip (logarithmic overhead)
3. Continuous uniform ⟺ Binary flip (infinite precision)

**Proof** (Binary from Ternary):
```racket
(define (flip p)
  (let ([x (bet 0 1 2)])  ; x ∈ {0, 1, 2}
    (if (< x (* 3 p))     ; Scale to [0,3)
        #t
        #f)))
```

For p = 0.5: P(x < 1.5) = P(x ∈ {0, 1}) = 2/3 ≠ 0.5

Better construction using rejection:
```racket
(define (flip-from-bet p)
  (let loop ()
    (let ([x (bet 0 1 2)])
      (cond [(= x 0) #t]
            [(= x 1) #f]
            [else (loop)]))))  ; P(heads) = P(tails) = 0.5
```

**Proof** (Ternary from Binary):
```racket
(define (bet-from-flip a b c)
  (let loop ()
    (let ([b1 (flip 0.5)]
          [b2 (flip 0.5)])
      (cond [(and (not b1) (not b2)) a]  ; 00 → A (1/4)
            [(and (not b1) b2) b]         ; 01 → B (1/4)
            [(and b1 (not b2)) c]         ; 10 → C (1/4)
            [else (loop)]))))             ; 11 → reject (1/4)
```

Expected flips: 8/3 ≈ 2.67 per ternary sample. ∎

### 2.3 Information Efficiency

| Primitive | Entropy/operation | Random bits |
|-----------|-------------------|-------------|
| flip | 1 bit | 1 |
| bet | log₂(3) ≈ 1.585 bits | 1.585 |
| uniform(0,1) | ∞ (continuous) | 32-64 |

**Theorem 2.2** (Entropy Efficiency). Ternary bet is the most efficient primitive for 3-way decisions.

For n-way uniform choice:
- Using binary flips: ⌈log₂(n)⌉ flips, efficiency = log₂(n)/⌈log₂(n)⌉
- Using ternary bets: ⌈log₃(n)⌉ bets, efficiency = log₃(n)/⌈log₃(n)⌉

For n = 3: bet efficiency = 1.0, flip efficiency = log₂(3)/2 ≈ 0.79

---

## 3. Semantic Foundations

### 3.1 Denotational Semantics Comparison

**Church/Scheme-based**:
$$⟦\text{flip } p⟧ = p \cdot δ_{\text{true}} + (1-p) \cdot δ_{\text{false}}$$

**Stan** (Log-density semantics):
$$⟦y \sim D⟧(σ) = σ[y ↦ v], \text{score} += \log p_D(v)$$

**Betlang**:
$$⟦\text{bet } A \ B \ C⟧ = \frac{1}{3}δ_A + \frac{1}{3}δ_B + \frac{1}{3}δ_C$$

### 3.2 Monadic Structure

| Language | Monad | bind | return |
|----------|-------|------|--------|
| Church | Probability | stochastic function | deterministic |
| Haskell PPLs | Prob a | >>= | return |
| Betlang | Dist τ | bet-bind | bet-pure |

**Theorem 3.1** All listed PPLs satisfy monad laws (with appropriate interpretation).

### 3.3 Operational Semantics

**Church** (Call-by-value, eager):
- Expressions evaluated before random choice
- Memoization through `mem`

**Stan** (Two-phase):
- Data block: deterministic
- Model block: stochastic

**Betlang** (Call-by-value with lazy option):
- Default: eager evaluation
- `bet/lazy`: only selected branch evaluated

---

## 4. Inference Methods

### 4.1 Inference Comparison

| Method | Church | Stan | Pyro | Betlang |
|--------|--------|------|------|---------|
| Exact enumeration | ✓ | ✗ | ✗ | ✗ |
| Rejection sampling | ✓ | ✗ | ✓ | ✓ |
| Importance sampling | ✓ | ✗ | ✓ | ✓ |
| MH | ✓ | ✓ | ✓ | ✓ |
| HMC/NUTS | ✗ | ✓ | ✓ | ✓ (basic) |
| SMC | ✓ | ✗ | ✓ | ✓ |
| Variational | ✗ | ✓ | ✓ | ✗ |
| ABC | ✗ | ✗ | ✗ | ✓ |

### 4.2 Inference Correctness

**Definition 4.1** (Inference Correctness). An inference algorithm is correct if its output converges to the true posterior.

**Theorem 4.1** (MH Correctness - All PPLs). Metropolis-Hastings is correct if:
1. Detailed balance is satisfied
2. Chain is irreducible
3. Chain is aperiodic

**Betlang-specific**: The `metropolis-hastings` implementation satisfies these conditions for well-posed target distributions.

### 4.3 Convergence Diagnostics

| Diagnostic | Stan | Pyro | Betlang |
|------------|------|------|---------|
| R̂ (Gelman-Rubin) | ✓ | ✗ | ✗ |
| ESS | ✓ | ✓ | ✗ |
| Trace plots | ✓ | ✓ | Via analyzer |
| ELBO | ✓ | ✓ | ✗ |

**TODO**: Implement standard convergence diagnostics in Betlang.

---

## 5. Type Systems

### 5.1 Static vs Dynamic Typing

| Language | Typing | Probabilistic Types |
|----------|--------|---------------------|
| Church | Dynamic | No |
| Stan | Static | Implicit (bounded types) |
| Pyro | Dynamic | Tensor shapes |
| Gen | Gradual | Trace types |
| Betlang | Dynamic | Dist τ (informal) |

### 5.2 Type Soundness

**Stan**: Compile-time type checking prevents runtime errors.

**Pyro**: Shape errors detected at runtime; optional mypy support.

**Betlang**: Racket's contracts provide optional runtime checking. Type soundness theorem holds for the idealized type system.

### 5.3 Dependent Probability Types

Advanced PPLs explore dependent types:

**Hakaru**: Dependent types for measure-theoretic semantics

**Betlang** (potential extension):
```racket
;; Dependent probability type (future work)
(: bet (∀ (A) (→ A A A (Dist_{1/3} A))))
```

---

## 6. Expressiveness

### 6.1 Turing Completeness

**Theorem 6.1**. All listed PPLs are Turing complete.

*Proof sketch*: Each can encode lambda calculus with fixed-point combinators. ∎

### 6.2 Stochastic Lambda Calculus

All functional PPLs are instances of stochastic lambda calculus:

$$\Lambda_\text{prob} ::= x \mid λx.M \mid M N \mid \text{sample}(D)$$

**Betlang addition**:
$$\Lambda_\text{bet} ::= ... \mid \text{bet}(M_1, M_2, M_3)$$

### 6.3 Higher-Order Probability

| Language | Higher-order distributions | Distribution-valued returns |
|----------|---------------------------|----------------------------|
| Church | ✓ | ✓ |
| Stan | ✗ | ✗ |
| Pyro | ✓ | ✓ |
| Betlang | ✓ | ✓ |

**Example** (Betlang):
```racket
(define (distribution-of-distributions)
  (bet (λ () (normal 0 1))
       (λ () (exponential 1))
       (λ () (uniform 0 1))))
```

---

## 7. Implementation Comparison

### 7.1 Compilation Strategies

| Language | Strategy | Target |
|----------|----------|--------|
| Stan | Transpile | C++ |
| Pyro | JIT | PyTorch |
| Betlang | Interpret | Racket VM |

### 7.2 Performance Characteristics

| Language | Compilation | Inference | Strengths |
|----------|-------------|-----------|-----------|
| Stan | Slow | Fast | Production HMC |
| Pyro | N/A | GPU | Deep learning integration |
| Betlang | N/A | Moderate | Simplicity, education |

### 7.3 Memory Model

**Stan**: Static memory allocation based on data block declarations.

**Pyro**: PyTorch tensor memory management; GPU support.

**Betlang**: Racket garbage collection; functional data structures.

---

## 8. Modeling Capabilities

### 8.1 Model Expressiveness

| Model Type | Church | Stan | Pyro | Betlang |
|------------|--------|------|------|---------|
| Discrete | ✓ | ✓ | ✓ | ✓ |
| Continuous | ✓ | ✓ | ✓ | ✓ |
| Hierarchical | ✓ | ✓ | ✓ | ✓ |
| Non-parametric | ✓ | Limited | ✓ | Limited |
| Time series | ✓ | ✓ | ✓ | ✓ |
| Neural networks | ✗ | ✗ | ✓ | ✗ |

### 8.2 Conjugate Prior Support

| Prior-Likelihood | Stan | Betlang |
|------------------|------|---------|
| Beta-Binomial | Auto | Manual (provided) |
| Normal-Normal | Auto | Manual (provided) |
| Gamma-Poisson | Auto | ✗ |
| Dirichlet-Categorical | Auto | ✗ |

**TODO**: Implement additional conjugate pairs in Betlang.

### 8.3 Mixture Models

**Stan**:
```stan
target += log_mix(theta, normal_lpdf(y | mu1, sigma1),
                         normal_lpdf(y | mu2, sigma2));
```

**Betlang**:
```racket
(define (mixture-sample)
  (let ([component (bet 'comp1 'comp2 'comp3)])
    (case component
      [(comp1) (normal 0 1)]
      [(comp2) (normal 5 2)]
      [(comp3) (exponential 1)])))
```

Natural ternary structure for 3-component mixtures.

---

## 9. Theoretical Contributions

### 9.1 Unique to Betlang

1. **Ternary primitive**: No other PPL uses ternary as fundamental
2. **Musical inspiration**: A-B-A form connection
3. **Ternary logic library**: Kleene three-valued logic integration
4. **Ternary-optimized algorithms**: ternary search, skip lists with P(up)=1/3

### 9.2 Contributions from Other PPLs Incorporated

From **Church**: Stochastic lambda calculus semantics
From **Stan**: Probabilistic Hoare logic approach
From **Pyro**: Effect handling concepts (implicit)

### 9.3 Open Questions

1. **Ternary quantum**: Connection to qutrit quantum computing
2. **Optimal ternary inference**: Specialized inference for ternary structure
3. **Ternary information geometry**: Geometry of ternary simplex

---

## 10. Case Study: Expressing the Same Model

### 10.1 Bayesian Linear Regression

**Stan**:
```stan
data {
  int N;
  vector[N] x;
  vector[N] y;
}
parameters {
  real alpha;
  real beta;
  real<lower=0> sigma;
}
model {
  alpha ~ normal(0, 10);
  beta ~ normal(0, 10);
  sigma ~ exponential(1);
  y ~ normal(alpha + beta * x, sigma);
}
```

**Pyro**:
```python
def model(x, y):
    alpha = pyro.sample("alpha", dist.Normal(0, 10))
    beta = pyro.sample("beta", dist.Normal(0, 10))
    sigma = pyro.sample("sigma", dist.Exponential(1))
    with pyro.plate("data", len(x)):
        pyro.sample("obs", dist.Normal(alpha + beta * x, sigma), obs=y)
```

**Betlang**:
```racket
(define (linear-regression-model x-data y-data)
  (define alpha (normal 0 10))
  (define beta (normal 0 10))
  (define sigma (exponential 1))
  (define (log-likelihood)
    (for/sum ([x x-data] [y y-data])
      (normal-log-pdf y (+ alpha (* beta x)) sigma)))
  (list alpha beta sigma (log-likelihood)))
```

### 10.2 Mixture Model with Three Components

**Betlang** (natural):
```racket
(define (three-component-mixture)
  (bet (normal 0 1)
       (normal 5 1)
       (normal 10 1)))
```

**Stan** (requires indexing):
```stan
simplex[3] theta;
ordered[3] mu;
y ~ normal(mu[categorical_rng(theta)], sigma);
```

---

## 11. Limitations and TODOs

### 11.1 Current Betlang Limitations

| Feature | Status | Priority |
|---------|--------|----------|
| Automatic differentiation | Missing | High |
| Variational inference | Missing | High |
| GPU acceleration | Missing | Medium |
| Distributed computing | Missing | Medium |
| Advanced diagnostics | Missing | Medium |
| Continuous semantics | Incomplete | High |

### 11.2 Comparison Summary

| Criterion | Best | Betlang Position |
|-----------|------|------------------|
| Production inference | Stan | Developing |
| Deep learning | Pyro | Not targeted |
| Theoretical elegance | Church | Comparable |
| Education | Church | Strong |
| Ternary modeling | **Betlang** | Leader |

---

## 12. Recommendations

### 12.1 When to Use Betlang

1. **Three-way decisions**: Natural ternary structure
2. **Education**: Clear, minimal semantics
3. **Racket ecosystem**: Integration needed
4. **Monte Carlo**: Simple sampling applications
5. **Research**: Novel probabilistic semantics

### 12.2 When to Use Alternatives

1. **Production HMC**: Stan
2. **Deep learning**: Pyro
3. **Program synthesis**: Church
4. **Flexible inference**: Gen

---

## 13. Conclusion

Betlang occupies a unique position in the PPL landscape with its ternary primitive. While not intended to replace production systems like Stan or Pyro, it offers:

1. **Theoretical novelty**: First ternary-based PPL
2. **Clean semantics**: Ideal for formal study
3. **Natural fit**: Three-way uncertainty modeling

The comparison reveals that betlang is expressively equivalent to other PPLs while offering unique structure for ternary problems.

---

## References

1. Goodman, N.D., et al. (2008). "Church: A language for generative models."
2. Carpenter, B., et al. (2017). "Stan: A probabilistic programming language."
3. Bingham, E., et al. (2019). "Pyro: Deep universal probabilistic programming."
4. Cusumano-Towner, M.F., et al. (2019). "Gen: A general-purpose probabilistic programming system."
5. Ge, H., Xu, K., & Ghahramani, Z. (2018). "Turing: A language for flexible probabilistic inference."
6. Wood, F., van de Meent, J.W., & Mansinghka, V. (2014). "A new approach to probabilistic programming inference."
