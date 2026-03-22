# Information-Theoretic Analysis of Betlang

## Abstract

This document provides a comprehensive information-theoretic analysis of betlang, covering Shannon entropy, mutual information, channel capacity, rate-distortion theory, and the information content of probabilistic programs.

---

## 1. Entropy Fundamentals

### 1.1 Shannon Entropy

**Definition 1.1** (Entropy). For random variable X with PMF p:
$$H(X) = -\sum_{x} p(x) \log_2 p(x)$$

**Theorem 1.1** (Uniform Ternary Entropy). For `(bet A B C)`:
$$H = \log_2(3) \approx 1.5849625 \text{ bits}$$

*Proof*:
$$H = -3 \cdot \frac{1}{3} \log_2 \frac{1}{3} = -\log_2 \frac{1}{3} = \log_2 3 \approx 1.585 ∎$$

**Theorem 1.2** (Maximum Entropy Principle). Among all ternary distributions, uniform achieves maximum entropy.

*Proof*: By Lagrange multipliers on H subject to Σp = 1:
$$\frac{\partial}{\partial p_i}\left[-\sum_j p_j \log p_j + λ(\sum_j p_j - 1)\right] = 0$$
$$-\log p_i - 1 + λ = 0 \Rightarrow p_i = e^{λ-1}$$

All probabilities equal ⟹ uniform distribution. ∎

### 1.2 Weighted Bet Entropy

**Theorem 1.3** (Weighted Entropy). For `(bet/weighted '(A wₐ) '(B w_b) '(C w_c))`:
$$H = -\sum_{i \in \{A,B,C\}} \frac{w_i}{W} \log_2 \frac{w_i}{W}$$

where W = wₐ + w_b + w_c.

**Example**: Weights (2, 1, 1):
- P(A) = 1/2, P(B) = P(C) = 1/4
- H = -1/2 log(1/2) - 2·(1/4)log(1/4) = 1/2 + 1 = 1.5 bits

### 1.3 Entropy Bounds

**Theorem 1.4** (Entropy Bounds for Ternary).
$$0 \leq H(X) \leq \log_2(3)$$

- H = 0 ⟺ deterministic (one outcome has probability 1)
- H = log₂(3) ⟺ uniform

---

## 2. Conditional Entropy and Mutual Information

### 2.1 Conditional Entropy

**Definition 2.1** (Conditional Entropy).
$$H(X|Y) = \sum_y p(y) H(X|Y=y) = -\sum_{x,y} p(x,y) \log_2 p(x|y)$$

**Theorem 2.1** (Independence and Conditional Entropy). For independent bets X, Y:
$$H(X|Y) = H(X)$$

*Proof*: Independence implies p(x|y) = p(x), so:
$$H(X|Y) = -\sum_{x,y} p(x,y) \log_2 p(x) = -\sum_x p(x) \log_2 p(x) = H(X) ∎$$

### 2.2 Mutual Information

**Definition 2.2** (Mutual Information).
$$I(X; Y) = H(X) - H(X|Y) = H(Y) - H(Y|X) = H(X) + H(Y) - H(X,Y)$$

**Theorem 2.2** (Independent Bets). For independent `(bet A B C)` and `(bet D E F)`:
$$I(X; Y) = 0$$

**Theorem 2.3** (Mutual Information Bounds).
$$0 \leq I(X; Y) \leq \min(H(X), H(Y))$$

### 2.3 Data Processing Inequality

**Theorem 2.4** (Data Processing for Bets). For Markov chain X → Y → Z:
$$I(X; Z) \leq I(X; Y)$$

Processing cannot increase mutual information.

---

## 3. Entropy Rate

### 3.1 Definition

**Definition 3.1** (Entropy Rate). For stochastic process {Xₙ}:
$$H_\infty = \lim_{n \to \infty} \frac{1}{n} H(X_1, ..., X_n)$$

**Theorem 3.1** (i.i.d. Bet Sequence). For i.i.d. `(bet A B C)` sequence:
$$H_\infty = H(X_1) = \log_2(3) \approx 1.585 \text{ bits/symbol}$$

### 3.2 Markov Chain Entropy Rate

**Theorem 3.2** (Markov Entropy Rate). For Markov chain with transition matrix P and stationary distribution π:
$$H_\infty = -\sum_i π_i \sum_j P_{ij} \log_2 P_{ij}$$

**Application**: Ternary Markov chain with uniform stationary distribution:
$$H_\infty = H(\text{transition from any state})$$

### 3.3 Random Walk Entropy

**Theorem 3.3** (Ternary Random Walk Entropy). For walk with steps {-1, 0, +1}:
$$H_\infty = \log_2(3) \approx 1.585 \text{ bits/step}$$

Each step is an independent ternary choice.

---

## 4. Source Coding

### 4.1 Optimal Code Length

**Theorem 4.1** (Shannon's Source Coding Theorem). For source X with entropy H(X), the optimal expected code length L* satisfies:
$$H(X) \leq L^* < H(X) + 1$$

**Corollary 4.1** (Ternary Bet Coding). To encode outcomes of (bet A B C):
- Minimum: 1.585 bits average
- Achievable: 2 bits (fixed length) or 1.585 bits (optimal variable length)

### 4.2 Huffman Coding for Ternary

For uniform ternary (p = 1/3 each):

**Optimal code**: Not possible with binary prefix codes at exactly log₂(3) bits. Best binary Huffman: one outcome gets 1 bit, two get 2 bits.
- Average: 1·(1/3) + 2·(1/3) + 2·(1/3) = 5/3 ≈ 1.67 bits

**Efficiency**: 1.585/1.67 ≈ 95%

### 4.3 Arithmetic Coding

**Theorem 4.2** (Arithmetic Coding Optimality). Arithmetic coding achieves rate H + ε for any ε > 0.

For ternary bets, arithmetic coding achieves ~1.585 bits/symbol asymptotically.

---

## 5. Channel Capacity

### 5.1 Ternary Channel

**Definition 5.1** (Ternary Symmetric Channel). Input/output alphabet {0, 1, 2} with crossover probability ε:
$$P(Y=j|X=i) = \begin{cases} 1-ε & i=j \\ ε/2 & i≠j \end{cases}$$

**Theorem 5.1** (TSC Capacity).
$$C_{TSC} = \log_2(3) - H(\epsilon, \epsilon/2, \epsilon/2)$$

where H(·) is the entropy function.

For ε = 0 (noiseless): C = log₂(3) ≈ 1.585 bits.

### 5.2 Channel Coding with Bets

**Theorem 5.2** (Achievability). For BSC with capacity C, we can reliably transmit at rate R < C using ternary-encoded messages decoded with probabilistic algorithms.

### 5.3 Noisy Bet Channel

**Model**: `(noisy-bet A B C ε)` - intended outcome corrupted with probability ε.

**Capacity**:
$$C = \log_2(3) \cdot (1 - H_2(\epsilon))$$

where H₂ is binary entropy.

---

## 6. Rate-Distortion Theory

### 6.1 Distortion Measure

**Definition 6.1** (Hamming Distortion for Ternary).
$$d(x, \hat{x}) = \begin{cases} 0 & x = \hat{x} \\ 1 & x ≠ \hat{x} \end{cases}$$

### 6.2 Rate-Distortion Function

**Theorem 6.1** (Ternary Rate-Distortion). For uniform ternary source:
$$R(D) = \log_2(3) - H(D, (1-D)/2, (1-D)/2) \text{ for } D \leq 2/3$$

### 6.3 Compression of Bet Sequences

To compress sequence of bets to rate R bits/symbol with distortion D:
- R = 0: D = 2/3 (always guess one outcome)
- R = log₂(3): D = 0 (lossless)

---

## 7. Kullback-Leibler Divergence

### 7.1 Definition

**Definition 7.1** (KL Divergence).
$$D_{KL}(P || Q) = \sum_x P(x) \log_2 \frac{P(x)}{Q(x)}$$

### 7.2 Uniform vs Weighted Bet

**Theorem 7.1** (Divergence from Uniform). For weighted bet P vs uniform Q:
$$D_{KL}(P || Q) = \log_2(3) - H(P)$$

*Proof*:
$$D_{KL}(P || Q) = \sum_x P(x) \log_2 \frac{P(x)}{1/3} = \sum_x P(x) \log_2 P(x) + \log_2 3 = -H(P) + \log_2 3 ∎$$

### 7.3 Information Geometry

The space of ternary distributions forms a 2-simplex:
$$\Delta_2 = \{(p_1, p_2, p_3) : p_i \geq 0, \sum p_i = 1\}$$

This is an information-geometric manifold with Fisher metric.

---

## 8. Fisher Information

### 8.1 Definition

**Definition 8.1** (Fisher Information). For parametric family p(x;θ):
$$I(\theta) = E\left[\left(\frac{\partial}{\partial θ} \log p(X;θ)\right)^2\right]$$

### 8.2 Ternary Parametric Family

For ternary distribution parameterized by (θ₁, θ₂):
$$P(X=1) = θ_1, \quad P(X=2) = θ_2, \quad P(X=3) = 1-θ_1-θ_2$$

**Fisher Information Matrix**:
$$I_{ij} = \frac{∂}{∂θ_i} \frac{∂}{∂θ_j} D_{KL}(P_θ || P_{θ'}) |_{θ'=θ}$$

### 8.3 Cramér-Rao Bound

**Theorem 8.1** (Cramér-Rao for Ternary Estimation). For unbiased estimator θ̂ of ternary parameter:
$$\text{Var}(\hat{θ}) \geq \frac{1}{n \cdot I(θ)}$$

---

## 9. Entropy of Programs

### 9.1 Program Entropy

**Definition 9.1** (Program Entropy). For probabilistic program P producing distribution D:
$$H(P) = H(D) = -\sum_x D(x) \log_2 D(x)$$

### 9.2 Entropy Composition

**Theorem 9.1** (Sequential Composition). For independent programs P₁, P₂:
$$H(P_1 ; P_2) = H(P_1) + H(P_2)$$

**Theorem 9.2** (Bet Composition).
$$H(\text{bet } P_1 \ P_2 \ P_3) = \log_2(3) + \frac{1}{3}(H(P_1) + H(P_2) + H(P_3))$$

### 9.3 Entropy of bet-chain

**Theorem 9.3** (Chain Entropy). For `(bet-chain n f init)` where f is deterministic:
$$H(\text{bet-chain } n \ f \ \text{init}) = 0$$

If f involves randomness:
$$H(\text{bet-chain } n \ f \ \text{init}) = n \cdot H_f$$

where Hf is the entropy contribution per iteration.

---

## 10. Entropy Estimation in Betlang

### 10.1 Plugin Estimator

The `bet-entropy` function uses:
$$\hat{H} = -\sum_x \hat{p}(x) \log_2 \hat{p}(x)$$

where p̂(x) = count(x)/n.

**Theorem 10.1** (Bias of Plugin Estimator).
$$E[\hat{H}] = H - \frac{|S| - 1}{2n \ln 2} + O(n^{-2})$$

For ternary (|S| = 3): Bias ≈ -1.44/n

### 10.2 Miller-Madow Correction

**Corrected estimator**:
$$\hat{H}_{MM} = \hat{H} + \frac{|\hat{S}| - 1}{2n \ln 2}$$

### 10.3 Confidence Interval for Entropy

**Theorem 10.2** (Asymptotic Variance).
$$\text{Var}(\hat{H}) \approx \frac{1}{n}\left[\sum_x p(x) (\log p(x))^2 - H^2\right]$$

---

## 11. Information in Statistical Inference

### 11.1 Bayesian Information

**Definition 11.1** (Information Gain).
$$I_{gain} = H(\text{prior}) - H(\text{posterior})$$

For conjugate updates, this quantifies learning.

### 11.2 Expected Information Gain

**Theorem 11.1** (Expected IG). For observation X:
$$E[I_{gain}] = I(\Theta; X)$$

mutual information between parameter and observation.

### 11.3 Sequential Experimental Design

**Criterion**: Choose experiment maximizing expected information gain.

For ternary outcomes, optimal design balances:
$$\max_{\text{design}} E[H(\text{prior}) - H(\text{posterior} | X)]$$

---

## 12. Minimum Description Length

### 12.1 MDL Principle

**Definition 12.1** (Two-Part MDL). Model complexity:
$$L(M) + L(D|M)$$

where L(M) = model description length, L(D|M) = data given model.

### 12.2 Ternary Model Selection

For selecting between ternary distributions:
- Uniform: L(M) = 0 (no parameters)
- Weighted (2 params): L(M) = 2 · precision bits
- Full: L(M) = (|S|-1) · precision bits

### 12.3 Normalized Maximum Likelihood

**Definition 12.2** (NML). The NML distribution:
$$p_{NML}(x) = \frac{p(x | \hat{θ}_{ML}(x))}{∑_{x'} p(x' | \hat{θ}_{ML}(x'))}$$

---

## 13. Typicality and AEP

### 13.1 Asymptotic Equipartition Property

**Theorem 13.1** (AEP for Ternary i.i.d.). For X₁, ..., Xₙ i.i.d. uniform ternary:
$$-\frac{1}{n} \log_2 p(X_1, ..., X_n) \xrightarrow{P} H(X) = \log_2(3)$$

### 13.2 Typical Set

**Definition 13.1** (Typical Set).
$$A_\epsilon^{(n)} = \{(x_1,...,x_n) : |{-}\frac{1}{n} \log_2 p - H| < \epsilon\}$$

**Theorem 13.2** (Typical Set Properties).
1. P(A_ε^{(n)}) → 1 as n → ∞
2. |A_ε^{(n)}| ≈ 2^{nH}
3. All elements have probability ≈ 2^{-nH}

For ternary: |A_ε^{(n)}| ≈ 3^n

---

## 14. Information-Theoretic Security

### 14.1 Perfect Secrecy

**Theorem 14.1** (Shannon Secrecy). Perfect secrecy requires:
$$H(K) \geq H(M)$$

For ternary message space, need ternary key.

### 14.2 Ternary One-Time Pad

**Definition 14.1** (Ternary OTP).
$$C = (M + K) \mod 3$$

**Theorem 14.2** (Ternary OTP Security). If K ~ Uniform{0,1,2} independent of M:
$$I(M; C) = 0$$

Perfect secrecy.

---

## 15. Summary of Key Results

| Quantity | Ternary Value | Binary Comparison |
|----------|---------------|-------------------|
| Maximum entropy | log₂(3) ≈ 1.585 | 1 bit |
| Entropy rate (i.i.d.) | 1.585 bits/symbol | 1 bit/symbol |
| Optimal code length | ≥ 1.585 bits | ≥ 1 bit |
| TSC capacity (noiseless) | 1.585 bits | 1 bit |
| Fisher information (uniform) | 3 | 4 |

---

## 16. TODOs

**TODO**: Further development needed:

1. **Continuous entropy**: Differential entropy for continuous distributions
2. **Multivariate information**: Information in joint ternary distributions
3. **Network information theory**: Multiple ternary sources/channels
4. **Quantum information**: Connection to qutrit systems

---

## References

1. Cover, T.M. & Thomas, J.A. (2006). *Elements of Information Theory*, 2nd ed.
2. MacKay, D.J.C. (2003). *Information Theory, Inference, and Learning Algorithms*
3. Shannon, C.E. (1948). "A Mathematical Theory of Communication"
4. Rissanen, J. (1978). "Modeling by shortest data description"
