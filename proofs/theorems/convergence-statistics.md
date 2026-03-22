# Convergence and Statistical Proofs for Betlang

## Abstract

This document provides rigorous proofs for the statistical properties of betlang, including limit theorems, convergence rates, error bounds, and the theoretical foundations of the statistical inference library.

---

## 1. Law of Large Numbers

### 1.1 Weak Law of Large Numbers

**Theorem 1.1** (WLLN for Ternary Bets). Let X₁, X₂, ..., Xₙ be i.i.d. samples from (bet A B C) where A, B, C ∈ ℝ. Then:

$$\bar{X}_n = \frac{1}{n}\sum_{i=1}^{n} X_i \xrightarrow{P} \mu = \frac{A + B + C}{3}$$

*Proof*:
By Chebyshev's inequality:
$$P(|\bar{X}_n - \mu| \geq \epsilon) \leq \frac{\text{Var}(\bar{X}_n)}{\epsilon^2} = \frac{\sigma^2}{n\epsilon^2}$$

where σ² = Var(X₁) = (A² + B² + C²)/3 - μ² < ∞.

As n → ∞, this bound → 0, so $\bar{X}_n \xrightarrow{P} \mu$. ∎

### 1.2 Strong Law of Large Numbers

**Theorem 1.2** (SLLN for Ternary Bets). Under the same conditions:

$$\bar{X}_n \xrightarrow{a.s.} \mu$$

*Proof*:
Since E[X₁⁴] < ∞ (finite fourth moment for bounded discrete distribution), Kolmogorov's SLLN applies directly. ∎

### 1.3 Weighted Bet Extension

**Theorem 1.3** (SLLN for Weighted Bets). For weighted bet with probabilities (p_A, p_B, p_C):

$$\bar{X}_n \xrightarrow{a.s.} p_A \cdot A + p_B \cdot B + p_C \cdot C$$

*Proof*: Same as Theorem 1.2 with modified expectation. ∎

---

## 2. Central Limit Theorem

### 2.1 Basic CLT

**Theorem 2.1** (CLT for Ternary Bets). For i.i.d. samples from (bet A B C):

$$\sqrt{n}\frac{\bar{X}_n - \mu}{\sigma} \xrightarrow{d} N(0, 1)$$

where μ = (A+B+C)/3 and σ² = E[X²] - μ².

*Proof*:
The moment generating function of Xᵢ exists and is finite (bounded support). By Lévy's CLT, the standardized sum converges to standard normal. ∎

### 2.2 Berry-Esseen Bound

**Theorem 2.2** (Convergence Rate). The CLT convergence rate is:

$$\sup_x |P(\sqrt{n}(\bar{X}_n - \mu)/\sigma \leq x) - \Phi(x)| \leq \frac{C \cdot \rho}{\sigma^3 \sqrt{n}}$$

where ρ = E[|X - μ|³] and C ≤ 0.4748 (Berry-Esseen constant).

*Proof*:
For ternary bets, the third absolute moment is bounded:
$$\rho = E[|X - \mu|^3] \leq \max(|A-\mu|, |B-\mu|, |C-\mu|)^3 < \infty$$

The Berry-Esseen theorem applies directly. ∎

**Corollary 2.1** (Practical Bound). For uniform (bet A B C) with A, B, C ∈ [-1, 1]:

$$\sup_x |F_n(x) - \Phi(x)| \leq \frac{0.95}{\sqrt{n}}$$

This means n ≥ 1000 gives ≤ 3% approximation error.

### 2.3 Multivariate CLT

**Theorem 2.3** (Multivariate CLT). For d-dimensional bet outcomes:

$$\sqrt{n}(\bar{\mathbf{X}}_n - \boldsymbol{\mu}) \xrightarrow{d} N_d(\mathbf{0}, \boldsymbol{\Sigma})$$

where Σ is the covariance matrix.

---

## 3. Probability Estimation Convergence

### 3.1 Frequency Estimator

**Theorem 3.1** (Frequency Convergence). The `bet-probability` estimator:
$$\hat{p}_n = \frac{1}{n}\sum_{i=1}^{n} \mathbf{1}[\text{pred}(X_i)]$$

converges at rate O(1/√n).

*Proof*:
$$\text{MSE}(\hat{p}_n) = \text{Var}(\hat{p}_n) = \frac{p(1-p)}{n}$$

Standard error: SE = √(p(1-p)/n) = O(1/√n). ∎

### 3.2 Confidence Interval Width

**Theorem 3.2** (CI Width). The 95% confidence interval width is:

$$W_n = 2 \cdot 1.96 \cdot \sqrt{\frac{\hat{p}(1-\hat{p})}{n}} = O\left(\frac{1}{\sqrt{n}}\right)$$

**Corollary 3.1** (Sample Size for Precision). To achieve margin of error ε:
$$n \geq \frac{z_{\alpha/2}^2 \cdot p(1-p)}{\epsilon^2}$$

For ε = 0.01 and 95% confidence: n ≥ 9604 (using p = 0.5 conservatively).

### 3.3 Hoeffding Bound

**Theorem 3.3** (Finite-Sample Bound). For any ε > 0:

$$P(|\hat{p}_n - p| \geq \epsilon) \leq 2\exp(-2n\epsilon^2)$$

*Proof*: Direct application of Hoeffding's inequality for bounded random variables in [0,1]. ∎

**Corollary 3.2** (Sample Size for High Confidence). For P(error > ε) ≤ δ:
$$n \geq \frac{\ln(2/\delta)}{2\epsilon^2}$$

---

## 4. Entropy Estimation

### 4.1 Plugin Estimator Bias

**Theorem 4.1** (Entropy Estimator Bias). The plugin entropy estimator:
$$\hat{H} = -\sum_{x} \hat{p}(x) \log_2 \hat{p}(x)$$

has negative bias:
$$\mathbb{E}[\hat{H}] = H - \frac{|S| - 1}{2n\ln 2} + O(n^{-2})$$

where |S| is the support size and H is the true entropy.

*Proof*: By Taylor expansion of -p log p around the true probability. ∎

**Corollary 4.1** (Bias for Ternary Bet). For uniform ternary bet (|S| = 3):
$$\text{Bias} \approx -\frac{1}{n\ln 2} \approx -\frac{1.44}{n}$$

### 4.2 Miller-Madow Correction

**Theorem 4.2** (Bias-Corrected Entropy). The Miller-Madow estimator:
$$\hat{H}_{MM} = \hat{H} + \frac{|S| - 1}{2n\ln 2}$$

is approximately unbiased to O(n⁻²).

### 4.3 Entropy Convergence Rate

**Theorem 4.3** (Entropy RMSE). The RMSE of the plugin estimator is:
$$\text{RMSE}(\hat{H}) = O\left(\frac{1}{\sqrt{n}}\right)$$

*Proof*: Using the delta method on the entropy functional. ∎

---

## 5. Distribution Convergence

### 5.1 Empirical Distribution

**Theorem 5.1** (Glivenko-Cantelli). The empirical CDF converges uniformly:
$$\sup_x |\hat{F}_n(x) - F(x)| \xrightarrow{a.s.} 0$$

For ternary bets (discrete), this is:
$$\max_{x \in \{A,B,C\}} |\hat{p}(x) - p(x)| \xrightarrow{a.s.} 0$$

### 5.2 DKW Inequality

**Theorem 5.2** (Finite-Sample CDF Bound).
$$P\left(\sup_x |\hat{F}_n(x) - F(x)| > \epsilon\right) \leq 2e^{-2n\epsilon^2}$$

*Proof*: Dvoretzky-Kiefer-Wolfowitz inequality. ∎

### 5.3 Kolmogorov-Smirnov Test Validity

**Theorem 5.3** (KS Test Asymptotics). For the KS statistic D_n:
$$\sqrt{n} D_n \xrightarrow{d} K$$

where K is the Kolmogorov distribution.

The `kolmogorov-smirnov` function uses this for goodness-of-fit testing.

---

## 6. Markov Chain Convergence

### 6.1 Ergodic Theorem

**Theorem 6.1** (MCMC Ergodicity). For an irreducible, aperiodic Markov chain with stationary distribution π:
$$\frac{1}{n}\sum_{i=1}^{n} f(X_i) \xrightarrow{a.s.} \mathbb{E}_\pi[f]$$

for any f with E_π[|f|] < ∞.

*Proof*: By the Markov chain ergodic theorem. ∎

### 6.2 Mixing Time

**Definition 6.1** (Mixing Time). The ε-mixing time is:
$$t_{mix}(\epsilon) = \min\{t : \max_x \|P^t(x, \cdot) - \pi\|_{TV} \leq \epsilon\}$$

**Theorem 6.2** (Mixing Time Bound). For the Metropolis-Hastings chain:
$$t_{mix}(\epsilon) \leq C \cdot \frac{\ln(1/(\epsilon \cdot \pi_{min}))}{\text{gap}}$$

where gap is the spectral gap of the transition matrix.

### 6.3 Geometric Ergodicity

**Theorem 6.3** (Geometric Convergence). If the MH chain is geometrically ergodic:
$$\|P^n(x, \cdot) - \pi\|_{TV} \leq M(x) \cdot \rho^n$$

for some ρ < 1 and function M.

**Corollary 6.1** (Burn-in Recommendation). Use burn-in of:
$$B \geq \frac{\ln(M \cdot \epsilon)}{\ln(1/\rho)}$$

samples to achieve TV distance ≤ ε.

---

## 7. Monte Carlo Integration

### 7.1 Consistency

**Theorem 7.1** (MC Integration Consistency). For Monte Carlo estimate:
$$\hat{I}_n = \frac{1}{n}\sum_{i=1}^{n} f(X_i)$$

where Xᵢ ~ p, we have:
$$\hat{I}_n \xrightarrow{a.s.} \int f(x) p(x) dx = \mathbb{E}_p[f]$$

*Proof*: SLLN for i.i.d. samples. ∎

### 7.2 Variance Reduction

**Theorem 7.2** (Importance Sampling Variance). With importance distribution q:
$$\hat{I}_{IS} = \frac{1}{n}\sum_{i=1}^{n} \frac{f(X_i) p(X_i)}{q(X_i)}, \quad X_i \sim q$$

Variance:
$$\text{Var}(\hat{I}_{IS}) = \frac{1}{n}\left[\int \frac{f(x)^2 p(x)^2}{q(x)} dx - I^2\right]$$

Optimal q* ∝ |f(x)| p(x).

### 7.3 Antithetic Variates

**Theorem 7.3** (Antithetic Variance Reduction). For antithetic pairs (X, X'):
$$\text{Var}\left(\frac{f(X) + f(X')}{2}\right) = \frac{\text{Var}(f(X))}{2}(1 + \text{Corr}(f(X), f(X')))$$

If Corr < 0 (negatively correlated), variance is reduced.

**Corollary 7.1** (Ternary Antithetic). For symmetric bet A, 0, -A with f(x) = x:
$$\text{Corr}(f(A), f(-A)) = -1$$

giving maximum variance reduction.

---

## 8. Bootstrap Convergence

### 8.1 Bootstrap Consistency

**Theorem 8.1** (Bootstrap Validity). For the empirical bootstrap:
$$\hat{F}^*_n = \frac{1}{n}\sum_{i=1}^{n} \delta_{X_i}$$

the bootstrap distribution of √n(θ̂* - θ̂) converges to the true sampling distribution of √n(θ̂ - θ) for smooth functionals θ.

*Proof*: Efron's theorem on bootstrap consistency. ∎

### 8.2 Bootstrap Confidence Intervals

**Theorem 8.2** (Percentile Bootstrap). The percentile bootstrap interval:
$$[\hat{\theta}^*_{(\alpha/2)}, \hat{\theta}^*_{(1-\alpha/2)}]$$

has asymptotically correct coverage for symmetric distributions.

### 8.3 Bootstrap Variance Estimation

**Theorem 8.3** (Bootstrap Variance Consistency).
$$\hat{\text{Var}}_{boot}(\hat{\theta}) \xrightarrow{P} \text{Var}(\hat{\theta})$$

The `bootstrap` function in lib/statistics.rkt provides consistent variance estimates.

---

## 9. Bayesian Convergence

### 9.1 Posterior Consistency

**Theorem 9.1** (Bernstein-von Mises). Under regularity conditions, the posterior converges to a normal distribution:
$$\Pi_n(\cdot | X_1, ..., X_n) \xrightarrow{d} N(\hat{\theta}_{MLE}, I(\theta_0)^{-1}/n)$$

where I(θ₀) is the Fisher information.

### 9.2 Bayesian Consistency

**Theorem 9.2** (Doob's Consistency). For "most" priors (in a measure-theoretic sense), the posterior concentrates around the true parameter:
$$\Pi_n(|\theta - \theta_0| > \epsilon | \text{data}) \xrightarrow{a.s.} 0$$

### 9.3 Credible Interval Coverage

**Theorem 9.3** (Frequentist Coverage of Bayesian CIs). For regular problems:
$$P_{\theta_0}(\theta_0 \in CI_{1-\alpha}^{Bayes}) \to 1 - \alpha$$

as n → ∞. The Bayesian credible interval has asymptotically correct frequentist coverage.

---

## 10. Random Walk Analysis

### 10.1 Simple Random Walk

**Theorem 10.1** (Random Walk Expectation). For `(random-walk n)` with steps {-1, 0, +1}:
$$\mathbb{E}[S_n] = 0$$
$$\text{Var}(S_n) = \frac{2n}{3}$$

*Proof*:
- E[step] = (-1 + 0 + 1)/3 = 0
- E[step²] = (1 + 0 + 1)/3 = 2/3
- Var(step) = 2/3 - 0² = 2/3
- By independence, Var(Sₙ) = n · Var(step) = 2n/3 ∎

### 10.2 CLT for Random Walk

**Theorem 10.2** (Diffusive Scaling).
$$\frac{S_n}{\sqrt{n}} \xrightarrow{d} N\left(0, \frac{2}{3}\right)$$

### 10.3 Hitting Time

**Theorem 10.3** (Expected Hitting Time). For hitting level k starting from 0:
$$\mathbb{E}[T_k] = \infty$$ (for symmetric walk)

but
$$P(T_k < \infty) = 1$$ (recurrence)

---

## 11. Stochastic Process Convergence

### 11.1 Brownian Motion Approximation

**Theorem 11.1** (Donsker's Theorem). The rescaled random walk converges to Brownian motion:
$$\left(\frac{S_{\lfloor nt \rfloor}}{\sqrt{n}}\right)_{t \in [0,1]} \xrightarrow{d} \sigma B_t$$

where B_t is standard Brownian motion and σ² = 2/3.

### 11.2 Lévy Flight Heavy Tails

**Theorem 11.2** (Stable Law Convergence). Lévy flights with index α converge to α-stable distributions:
$$\frac{S_n}{n^{1/\alpha}} \xrightarrow{d} S_\alpha$$

where S_α is the α-stable distribution.

---

## 12. Complexity-Convergence Tradeoffs

### 12.1 Sample Complexity

**Theorem 12.1** (Sample Complexity for ε-δ Estimation). To estimate a probability p within ε with confidence 1-δ:
$$n = O\left(\frac{\log(1/\delta)}{\epsilon^2}\right)$$

### 12.2 Variance-Computation Tradeoff

**Theorem 12.2** (Bias-Variance-Computation). For time budget T and cost c per sample:
$$n = T/c$$
$$\text{MSE} = \frac{\text{Var}}{n} + \text{Bias}^2 = \frac{c \cdot \text{Var}}{T} + \text{Bias}^2$$

Optimal allocation balances these terms.

---

## 13. Error Analysis

### 13.1 Numerical Precision

**Theorem 13.1** (Floating-Point Error). For entropy calculation with n samples:
$$|\hat{H}_{computed} - \hat{H}_{exact}| \leq O(n \cdot \epsilon_{machine})$$

where ε_machine ≈ 2.2 × 10⁻¹⁶ for IEEE 754 double precision.

### 13.2 Pseudorandom Quality

**Theorem 13.2** (PRNG Period Effect). For Racket's PRNG with period P:

If n > P, samples become correlated, violating independence assumptions.

For Racket's random, P ≈ 2⁶¹, so this is not a practical concern for typical simulation sizes.

---

## 14. Summary of Convergence Rates

| Quantity | Rate | Theorem |
|----------|------|---------|
| Sample mean | O(1/√n) | CLT |
| Probability estimate | O(1/√n) | Thm 3.1 |
| Entropy estimate | O(1/√n) | Thm 4.3 |
| MCMC convergence | O(ρⁿ), ρ<1 | Thm 6.3 |
| Bootstrap variance | O(1/√n) | Thm 8.3 |
| KS statistic | O(1/√n) | Thm 5.2 |

---

## 15. TODOs and Open Problems

**TODO**: The following need further investigation:

1. **Optimal importance sampling**: Derive optimal proposal for betlang-specific computations
2. **Adaptive MCMC convergence**: Prove convergence for adaptive MH in lib/bayesian.rkt
3. **Sequential Monte Carlo**: Prove particle filter convergence rate
4. **Non-asymptotic Bayesian bounds**: Finite-sample posterior concentration

---

## References

1. Durrett, R. (2019). *Probability: Theory and Examples*
2. Van der Vaart, A.W. (1998). *Asymptotic Statistics*
3. Robert, C.P. & Casella, G. (2004). *Monte Carlo Statistical Methods*
4. Efron, B. & Tibshirani, R. (1993). *An Introduction to the Bootstrap*
5. Meyn, S.P. & Tweedie, R.L. (2009). *Markov Chains and Stochastic Stability*
