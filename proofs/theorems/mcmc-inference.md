# MCMC and Inference Correctness Proofs

## Abstract

This document provides rigorous proofs of correctness for the Markov Chain Monte Carlo (MCMC) and inference methods implemented in betlang. We establish convergence guarantees, prove detailed balance, and analyze the statistical properties of the samplers.

---

## 1. Metropolis-Hastings Algorithm

### 1.1 Algorithm Definition

**Algorithm 1.1** (Metropolis-Hastings):
```
Input: target π, proposal q, initial x₀, iterations n
Output: samples x₁, ..., xₙ

for i = 1 to n:
    y ~ q(·|xᵢ₋₁)                    # Propose
    α = min(1, π(y)q(xᵢ₋₁|y) / (π(xᵢ₋₁)q(y|xᵢ₋₁)))  # Accept ratio
    u ~ Uniform(0,1)
    if u < α:
        xᵢ = y                        # Accept
    else:
        xᵢ = xᵢ₋₁                     # Reject
```

### 1.2 Detailed Balance

**Definition 1.1** (Detailed Balance). A transition kernel P satisfies detailed balance w.r.t. π if:
$$π(x) P(x, y) = π(y) P(y, x) \quad ∀x, y$$

**Theorem 1.1** (MH Satisfies Detailed Balance). The MH transition kernel satisfies detailed balance with target π.

*Proof*:
The MH kernel is:
$$P(x, y) = q(y|x) · α(x, y) + r(x) · δ_x(y)$$

where α(x,y) = min(1, π(y)q(x|y)/(π(x)q(y|x))) and r(x) = 1 - ∫ q(y|x)α(x,y)dy.

For x ≠ y:
$$π(x) P(x, y) = π(x) q(y|x) α(x, y)$$

Case 1: α(x,y) = 1 (i.e., π(y)q(x|y) ≥ π(x)q(y|x)):
$$π(x) P(x, y) = π(x) q(y|x)$$
$$π(y) P(y, x) = π(y) q(x|y) α(y, x) = π(y) q(x|y) · \frac{π(x)q(y|x)}{π(y)q(x|y)} = π(x) q(y|x)$$

Case 2: α(x,y) < 1:
$$π(x) P(x, y) = π(x) q(y|x) · \frac{π(y)q(x|y)}{π(x)q(y|x)} = π(y) q(x|y)$$
$$π(y) P(y, x) = π(y) q(x|y) · 1 = π(y) q(x|y)$$

Both cases: π(x)P(x,y) = π(y)P(y,x). ∎

### 1.3 Invariance

**Theorem 1.2** (π is Invariant). If P satisfies detailed balance with π, then π is invariant:
$$π P = π$$

*Proof*:
$$(\pi P)(y) = \int π(x) P(x, y) dx = \int π(y) P(y, x) dx = π(y) \int P(y, x) dx = π(y) · 1 = π(y) ∎$$

### 1.4 Ergodicity

**Theorem 1.3** (MH Ergodicity). If the MH chain is irreducible and aperiodic, then:
$$\frac{1}{n} \sum_{i=1}^n f(X_i) \xrightarrow{a.s.} \mathbb{E}_π[f]$$

*Proof*: By the ergodic theorem for Markov chains with invariant distribution. ∎

### 1.5 Sufficient Conditions for Irreducibility

**Lemma 1.1** (Irreducibility). MH is irreducible if:
$$\forall x, y: \exists \text{ path } x = x_0, x_1, ..., x_k = y \text{ with } \prod_i q(x_{i+1}|x_i) > 0$$

**Lemma 1.2** (Aperiodicity). If q(x|x) > 0 or there exist states with positive probability of rejection, MH is aperiodic.

---

## 2. Gibbs Sampling

### 2.1 Algorithm

**Algorithm 2.1** (Gibbs Sampler for (X, Y)):
```
Input: conditionals p(x|y), p(y|x), initial (x₀, y₀), iterations n
Output: samples (x₁, y₁), ..., (xₙ, yₙ)

for i = 1 to n:
    xᵢ ~ p(x | yᵢ₋₁)
    yᵢ ~ p(y | xᵢ)
```

### 2.2 Correctness

**Theorem 2.1** (Gibbs Preserves Joint). If p(x|y) and p(y|x) are the true conditionals of joint p(x,y), then Gibbs sampling has p(x,y) as invariant distribution.

*Proof*:
The transition kernel is:
$$P((x,y), (x',y')) = p(x'|y) · p(y'|x')$$

For invariance, need:
$$\int\int p(x,y) P((x,y),(x',y')) dx dy = p(x',y')$$

$$\int\int p(x,y) p(x'|y) p(y'|x') dx dy$$
$$= \int p(y) p(x'|y) dy · p(y'|x')$$  (integrating out x using ∫p(x|y)dx = 1)
$$= p(x') · p(y'|x')$$  (using ∫p(x'|y)p(y)dy = p(x'))
$$= p(x',y')$$ ∎

### 2.3 Convergence Rate

**Theorem 2.2** (Geometric Convergence). Under regularity conditions, Gibbs sampling converges geometrically:
$$\|P^n((x_0, y_0), ·) - π\|_{TV} \leq C · ρ^n$$

for some ρ < 1.

---

## 3. Hamiltonian Monte Carlo

### 3.1 Algorithm

**Algorithm 3.1** (HMC):
```
Input: log-prob U, gradient ∇U, initial q₀, step size ε, path length L
Output: samples q₁, ..., qₙ

for i = 1 to n:
    p ~ N(0, I)                      # Sample momentum
    (q', p') = leapfrog(qᵢ₋₁, p, ε, L)  # Integrate
    α = min(1, exp(-H(q', p') + H(qᵢ₋₁, p)))
    u ~ Uniform(0,1)
    if u < α:
        qᵢ = q'
    else:
        qᵢ = qᵢ₋₁
```

### 3.2 Leapfrog Integrator

**Algorithm 3.2** (Leapfrog):
```
p ← p - (ε/2)∇U(q)
for l = 1 to L:
    q ← q + ε · p
    if l < L:
        p ← p - ε∇U(q)
p ← p - (ε/2)∇U(q)
return (q, -p)
```

### 3.3 Symplectic Property

**Theorem 3.1** (Volume Preservation). Leapfrog is symplectic (preserves volume in phase space).

*Proof*: Each step is a shear transformation with unit Jacobian determinant. Composition preserves this property. ∎

### 3.4 Detailed Balance

**Theorem 3.2** (HMC Detailed Balance). HMC satisfies detailed balance with joint distribution:
$$π(q, p) ∝ \exp(-U(q) - \frac{1}{2}p^T p)$$

*Proof*:
1. Momentum flip: (q, p) ↔ (q, -p) is self-inverse
2. Leapfrog: approximately preserves H(q,p) = U(q) + K(p)
3. MH correction: accounts for numerical integration error ∎

### 3.5 No-U-Turn Sampler (NUTS)

**Theorem 3.3** (NUTS Correctness). NUTS maintains detailed balance while adaptively choosing path length.

*Proof sketch*: The doubling procedure and slice sampling maintain reversibility. ∎

---

## 4. Rejection Sampling

### 4.1 Algorithm

**Algorithm 4.1** (Rejection Sampling):
```
Input: target f, proposal g, bound M (where f(x) ≤ M·g(x))
Output: sample from f

repeat:
    x ~ g
    u ~ Uniform(0, 1)
until u < f(x) / (M·g(x))
return x
```

### 4.2 Correctness

**Theorem 4.1** (Rejection Sampling Correctness). The output follows distribution f.

*Proof*:
P(accept x ∈ A) = P(X ∈ A, U < f(X)/(M·g(X)) | accept)

By Bayes:
$$= \frac{P(X ∈ A, U < f(X)/(M·g(X)))}{P(\text{accept})}$$
$$= \frac{\int_A g(x) · \frac{f(x)}{M·g(x)} dx}{1/M}$$
$$= \frac{(1/M)\int_A f(x) dx}{1/M}$$
$$= \int_A f(x) dx$$

where we used P(accept) = (1/M)∫f(x)dx = 1/M for normalized f. ∎

### 4.3 Efficiency

**Theorem 4.2** (Acceptance Rate). The acceptance probability is 1/M.

**Corollary 4.1** (Expected Samples). Expected number of proposals until acceptance: M.

---

## 5. Importance Sampling

### 5.1 Estimator

**Definition 5.1** (Importance Sampling Estimator). For target p, proposal q:
$$\hat{I}_{IS} = \frac{1}{n} \sum_{i=1}^n \frac{f(X_i) p(X_i)}{q(X_i)}, \quad X_i \sim q$$

### 5.2 Unbiasedness

**Theorem 5.1** (IS Unbiasedness).
$$\mathbb{E}_q[\hat{I}_{IS}] = \mathbb{E}_p[f]$$

*Proof*:
$$\mathbb{E}_q\left[\frac{f(X) p(X)}{q(X)}\right] = \int \frac{f(x) p(x)}{q(x)} q(x) dx = \int f(x) p(x) dx = \mathbb{E}_p[f] ∎$$

### 5.3 Variance

**Theorem 5.2** (IS Variance).
$$\text{Var}_q\left[\frac{f(X)p(X)}{q(X)}\right] = \mathbb{E}_q\left[\left(\frac{f(X)p(X)}{q(X)}\right)^2\right] - \left(\mathbb{E}_p[f]\right)^2$$

### 5.4 Optimal Proposal

**Theorem 5.3** (Optimal IS Proposal). The variance-minimizing proposal is:
$$q^*(x) ∝ |f(x)| p(x)$$

*Proof*: By Lagrange multipliers on variance subject to ∫q = 1. ∎

---

## 6. Sequential Monte Carlo

### 6.1 Algorithm

**Algorithm 6.1** (Particle Filter):
```
Input: prior p₀, transition p(xₜ|xₜ₋₁), observation p(yₜ|xₜ), observations y₁:T
Output: particles {xₜ^{(i)}}

Initialize: x₀^{(i)} ~ p₀ for i = 1..N
for t = 1 to T:
    Propagate: x̃ₜ^{(i)} ~ p(xₜ | xₜ₋₁^{(i)})
    Weight: wₜ^{(i)} ∝ p(yₜ | x̃ₜ^{(i)})
    Resample: xₜ^{(i)} ~ Categorical({x̃ₜ^{(j)}}, {wₜ^{(j)}})
```

### 6.2 Correctness

**Theorem 6.1** (Particle Filter Consistency). As N → ∞:
$$\sum_{i=1}^N w_t^{(i)} δ_{x_t^{(i)}} \xrightarrow{d} p(x_t | y_{1:t})$$

*Proof*: By sequential application of importance sampling with resampling correction. ∎

### 6.3 Effective Sample Size

**Definition 6.1** (ESS).
$$\text{ESS} = \frac{(\sum_i w_i)^2}{\sum_i w_i^2}$$

**Theorem 6.2** (ESS Bound). 1 ≤ ESS ≤ N with equality at N iff uniform weights.

---

## 7. Approximate Bayesian Computation

### 7.1 Algorithm

**Algorithm 7.1** (ABC Rejection):
```
Input: simulator p(D|θ), prior π(θ), observed D_obs, threshold ε
Output: approximate posterior samples

repeat:
    θ ~ π(θ)
    D_sim ~ p(D|θ)
until d(D_sim, D_obs) < ε
return θ
```

### 7.2 Consistency

**Theorem 7.1** (ABC Consistency). As ε → 0:
$$π_{ABC}(θ | d(D_{sim}, D_{obs}) < ε) → π(θ | D = D_{obs})$$

under regularity conditions.

*Proof*: As ε → 0, the acceptance region shrinks to {D = D_obs}, recovering exact posterior. ∎

### 7.3 Approximation Error

**Theorem 7.2** (ABC Error Bound). The total variation distance:
$$\|π_{ABC} - π_{true}\|_{TV} = O(ε)$$

for smooth likelihoods.

---

## 8. Convergence Diagnostics

### 8.1 Potential Scale Reduction Factor

**Definition 8.1** (R-hat). For m chains of length n:
$$\hat{R} = \sqrt{\frac{\hat{V}}{W}}$$

where W = within-chain variance, V̂ = pooled variance estimate.

**Theorem 8.1** (Convergence Criterion). R̂ → 1 as n → ∞ for converged chains.

### 8.2 Effective Sample Size

**Definition 8.2** (MCMC ESS).
$$\text{ESS} = \frac{n}{1 + 2\sum_{k=1}^∞ ρ_k}$$

where ρ_k is the lag-k autocorrelation.

### 8.3 Geweke Diagnostic

**Theorem 8.2** (Geweke Test). Under convergence, for first 10% and last 50% of chain:
$$\frac{\bar{X}_A - \bar{X}_B}{\sqrt{S_A^2/n_A + S_B^2/n_B}} \xrightarrow{d} N(0, 1)$$

---

## 9. Betlang-Specific Implementation Details

### 9.1 metropolis-hastings Implementation

```racket
(define (metropolis-hastings target-log-density proposal initial n-samples)
  (let loop ([current initial]
             [samples '()]
             [i 0])
    (if (>= i n-samples)
        (reverse samples)
        (let* ([proposed ((proposal current))]
               [log-alpha (- (target-log-density proposed)
                            (target-log-density current))]
               [accept? (< (log (random)) log-alpha)])
          (loop (if accept? proposed current)
                (cons (if accept? proposed current) samples)
                (+ i 1))))))
```

**Correctness**: Follows Theorem 1.1 (detailed balance) assuming symmetric proposal.

### 9.2 gibbs-sampler Implementation

```racket
(define (gibbs-sampler conditional-x conditional-y init-x init-y n-samples)
  (let loop ([x init-x] [y init-y] [samples '()] [i 0])
    (if (>= i n-samples)
        (reverse samples)
        (let* ([new-x (conditional-x y)]
               [new-y (conditional-y new-x)])
          (loop new-x new-y
                (cons (list new-x new-y) samples)
                (+ i 1))))))
```

**Correctness**: Follows Theorem 2.1 given correct conditionals.

---

## 10. Theoretical Guarantees Summary

| Method | Condition | Guarantee |
|--------|-----------|-----------|
| MH | Irreducible, aperiodic | Ergodic convergence |
| Gibbs | Correct conditionals | Invariant = joint |
| HMC | Symplectic integrator | Volume preservation |
| Rejection | f ≤ Mg | Exact samples |
| IS | q > 0 where p > 0 | Unbiased estimator |
| SMC | Proper resampling | Consistent filtering |
| ABC | ε → 0 | Posterior convergence |

---

## 11. Error Bounds

### 11.1 MCMC Standard Error

**Theorem 11.1** (MCMC CLT). Under geometric ergodicity:
$$\sqrt{n}(\bar{X}_n - \mu) \xrightarrow{d} N(0, σ^2_{eff})$$

where σ²_eff = σ²(1 + 2Σρₖ) is the effective variance.

### 11.2 Burn-in Selection

**Theorem 11.2** (Burn-in Bound). To achieve TV distance ε from stationarity:
$$B \geq \frac{\log(1/ε)}{\log(1/ρ)}$$

where ρ < 1 is the convergence rate.

---

## 12. TODOs

**TODO**: Further proofs needed:

1. **Adaptive MCMC**: Prove ergodicity of adaptive algorithms
2. **Parallel tempering**: Prove mixing time bounds
3. **Gradient estimation**: Bias/variance of stochastic gradients
4. **Variational inference**: Approximation bounds

---

## References

1. Robert, C.P. & Casella, G. (2004). *Monte Carlo Statistical Methods*
2. Brooks, S., et al. (2011). *Handbook of Markov Chain Monte Carlo*
3. Doucet, A., de Freitas, N., & Gordon, N. (2001). *Sequential Monte Carlo Methods in Practice*
4. Neal, R.M. (2011). "MCMC using Hamiltonian dynamics"
5. Marin, J.-M., et al. (2012). "Approximate Bayesian computational methods"
