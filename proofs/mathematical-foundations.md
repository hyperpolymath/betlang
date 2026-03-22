# Mathematical Foundations of Betlang

## Abstract

This document establishes the rigorous mathematical foundations underlying betlang, a ternary probabilistic programming language. We formalize the probability spaces, measure-theoretic foundations, and algebraic structures that govern the language semantics.

---

## 1. Probability Space Foundation

### 1.1 Sample Space Definition

**Definition 1.1** (Ternary Sample Space). For any bet expression `(bet A B C)`, the sample space is:

$$\Omega = \{A, B, C\}$$

where $|\Omega| = 3$ (the ternary cardinality).

**Definition 1.2** (σ-algebra). The σ-algebra over Ω is the power set:

$$\mathcal{F} = \mathcal{P}(\Omega) = \{\emptyset, \{A\}, \{B\}, \{C\}, \{A,B\}, \{A,C\}, \{B,C\}, \{A,B,C\}\}$$

with $|\mathcal{F}| = 2^3 = 8$ events.

### 1.2 Probability Measure

**Definition 1.3** (Uniform Ternary Measure). The probability measure P: F → [0,1] for uniform bet is:

$$P(\{x\}) = \frac{1}{3} \quad \forall x \in \Omega$$

**Theorem 1.1** (Measure Axioms). The betlang probability measure satisfies Kolmogorov's axioms:

1. **Non-negativity**: $P(E) \geq 0$ for all $E \in \mathcal{F}$
2. **Unitarity**: $P(\Omega) = P(\{A,B,C\}) = 1$
3. **σ-additivity**: For disjoint events $E_i$: $P(\bigcup_i E_i) = \sum_i P(E_i)$

*Proof*:
1. By construction, $P(\{x\}) = 1/3 \geq 0$ and additivity preserves non-negativity.
2. $P(\Omega) = P(\{A\}) + P(\{B\}) + P(\{C\}) = 1/3 + 1/3 + 1/3 = 1$ ∎
3. For finite discrete spaces, σ-additivity reduces to finite additivity, which holds by definition of P on compound events. ∎

### 1.3 Weighted Probability Measure

**Definition 1.4** (Weighted Ternary Measure). For `(bet/weighted '(A wₐ) '(B w_b) '(C w_c))`:

$$P(\{x\}) = \frac{w_x}{\sum_{y \in \Omega} w_y}$$

**Theorem 1.2** (Weighted Measure Validity). The weighted measure satisfies probability axioms iff all weights are non-negative and at least one is positive.

*Proof*:
- If $w_x \geq 0$ for all x and $\sum w_x > 0$, then $P(\{x\}) \geq 0$ and $\sum P(\{x\}) = 1$.
- Conversely, if any $w_x < 0$, then $P(\{x\})$ may be negative, violating axiom 1. ∎

---

## 2. Measure-Theoretic Framework

### 2.1 Random Variables

**Definition 2.1** (Bet Random Variable). A bet expression defines a random variable:

$$X: \Omega \to E$$

where E is the outcome space (which may differ from Ω when outcomes are transformed).

**Definition 2.2** (Pushforward Measure). For a function f: E → F, the pushforward measure is:

$$P_f(B) = P(f^{-1}(B)) = P(\{\omega : f(X(\omega)) \in B\})$$

This formalizes `bet-map` operations.

### 2.2 Independence

**Definition 2.3** (Independent Bets). Two bet expressions X and Y are independent iff:

$$P(X = x, Y = y) = P(X = x) \cdot P(Y = y)$$

**Theorem 2.1** (bet-parallel Independence). Each trial in `(bet-parallel n A B C)` is independent.

*Proof*: Each trial uses an independent random number generator call. By construction of the pseudorandom number generator (assuming proper seeding), successive calls produce independent uniform samples. ∎

**Remark**: True independence requires cryptographic-quality randomness. Racket's `random` provides pseudorandom independence sufficient for simulation purposes.

### 2.3 Conditional Probability

**Definition 2.4** (Conditional Bet). For `(bet/conditional pred A B C)`:

$$P(X = x | \text{pred}) = \begin{cases} 1 & \text{if pred is true and } x = A \\ P_{\text{bet}}(x) & \text{if pred is false} \end{cases}$$

where $P_{\text{bet}}$ is the standard ternary measure over {B, C, A} when pred is false.

---

## 3. Expected Value Theory

### 3.1 Expectation Definition

**Definition 3.1** (Expected Value). For a bet random variable X with outcomes {a, b, c}:

$$\mathbb{E}[X] = \frac{1}{3}(a + b + c)$$

when a, b, c are numeric and uniformly weighted.

**Definition 3.2** (Weighted Expectation). For weighted bets:

$$\mathbb{E}[X] = \sum_{x \in \{a,b,c\}} P(\{x\}) \cdot x = \frac{w_a \cdot a + w_b \cdot b + w_c \cdot c}{w_a + w_b + w_c}$$

### 3.2 Moments

**Theorem 3.1** (Variance Formula). For uniform ternary bet:

$$\text{Var}(X) = \mathbb{E}[X^2] - \mathbb{E}[X]^2 = \frac{a^2 + b^2 + c^2}{3} - \left(\frac{a+b+c}{3}\right)^2$$

*Proof*: Direct application of the variance definition with discrete uniform distribution. ∎

**Corollary 3.1** (Symmetric Bet Variance). If a = -d, b = 0, c = d for some d ≠ 0:

$$\text{Var}(X) = \frac{d^2 + 0 + d^2}{3} - 0 = \frac{2d^2}{3}$$

---

## 4. Information Theory

### 4.1 Shannon Entropy

**Definition 4.1** (Entropy of Ternary Bet). The Shannon entropy of a bet distribution:

$$H(X) = -\sum_{x \in \Omega} P(x) \log_2 P(x)$$

**Theorem 4.1** (Maximum Entropy). The uniform ternary bet achieves maximum entropy:

$$H_{\max} = \log_2(3) \approx 1.585 \text{ bits}$$

*Proof*: By the maximum entropy theorem, entropy is maximized when all outcomes are equiprobable. For uniform distribution P(x) = 1/3:

$$H = -3 \cdot \frac{1}{3} \log_2\frac{1}{3} = \log_2(3) ∎$$

**Theorem 4.2** (Entropy Bounds). For any ternary bet:

$$0 \leq H(X) \leq \log_2(3)$$

with H = 0 iff one outcome has probability 1 (deterministic), and H = log₂(3) iff uniform.

### 4.2 Mutual Information

**Definition 4.2** (Mutual Information). For two bet expressions X and Y:

$$I(X; Y) = H(X) + H(Y) - H(X, Y)$$

**Theorem 4.3** (Independent Bets). If X and Y are independent:

$$I(X; Y) = 0$$

*Proof*: For independent variables, H(X,Y) = H(X) + H(Y), so I(X;Y) = 0. ∎

---

## 5. Algebraic Structure

### 5.1 Probability Monad

**Definition 5.1** (Bet Monad). The bet construction forms a monad (Bet, return, bind):

**return** (unit): $\eta: A \to \text{Bet}(A)$
```
return x = (bet x x x)
```

**bind** (>>=): $\mu: \text{Bet}(A) \to (A \to \text{Bet}(B)) \to \text{Bet}(B)$
```
bind m f = evaluate m, then apply f to result
```

**Theorem 5.1** (Monad Laws). The bet monad satisfies:

1. **Left identity**: `(bind (return x) f) = (f x)`
2. **Right identity**: `(bind m return) = m`
3. **Associativity**: `(bind (bind m f) g) = (bind m (λ x. bind (f x) g))`

*Proof*:
1. `(bind (bet x x x) f)` always returns `(f x)` since all outcomes are x. ∎
2. `(bind m return)` returns `(bet m m m)` which equals m distributionally. ∎
3. Follows from associativity of function composition over probabilistic choice. ∎

### 5.2 Kleisli Category

**Definition 5.2** (Kleisli Morphisms). The Kleisli category for Bet has:
- Objects: Types A, B, C, ...
- Morphisms: Functions A → Bet(B)
- Composition: Kleisli composition via bind

**Theorem 5.2** (Category Laws). Kleisli composition satisfies identity and associativity laws by the monad laws.

### 5.3 Semiring Structure

**Definition 5.3** (Probability Semiring). Bet probabilities form a semiring (ℝ≥0, +, ×, 0, 1):

- Addition: corresponds to disjoint union of events
- Multiplication: corresponds to independent conjunction

---

## 6. Ternary Logic Foundation

### 6.1 Kleene's Three-Valued Logic

**Definition 6.1** (Truth Values). The ternary truth domain:

$$\mathbb{T}_3 = \{0, \frac{1}{2}, 1\}$$

representing FALSE, UNKNOWN, TRUE respectively.

**Definition 6.2** (Logical Operations).

| Operation | Definition |
|-----------|------------|
| AND(a,b,c) | min(a, b, c) |
| OR(a,b,c) | max(a, b, c) |
| NOT(a) | 1 - a |

**Theorem 6.1** (De Morgan's Laws). For ternary logic:

$$\neg(a \land b \land c) = \neg a \lor \neg b \lor \neg c$$
$$\neg(a \lor b \lor c) = \neg a \land \neg b \land \neg c$$

*Proof*:
- NOT(min(a,b,c)) = 1 - min(a,b,c) = max(1-a, 1-b, 1-c) = max(NOT a, NOT b, NOT c) ∎

### 6.2 Relationship to Probability

**Theorem 6.2** (Probabilistic Interpretation). Ternary truth values can be interpreted as probabilities:

- TRUE (1): Certain to hold
- FALSE (0): Certainly does not hold
- UNKNOWN (1/2): Maximum uncertainty (entropy = 1 bit)

The ternary AND/OR operations correspond to:
- AND: Probability of all events (under positive correlation assumption: min)
- OR: Probability of at least one event (under positive correlation: max)

---

## 7. Convergence Theory

### 7.1 Law of Large Numbers

**Theorem 7.1** (Weak LLN for Bets). For i.i.d. bet trials X₁, X₂, ..., Xₙ:

$$\bar{X}_n = \frac{1}{n}\sum_{i=1}^{n} X_i \xrightarrow{P} \mathbb{E}[X]$$

as n → ∞.

*Proof*: Follows directly from the classical Weak LLN since bet outcomes are i.i.d. with finite expectation. ∎

**Theorem 7.2** (Strong LLN for Bets). Under the same conditions:

$$\bar{X}_n \xrightarrow{a.s.} \mathbb{E}[X]$$

*Proof*: Follows from the Strong LLN since Var(X) is finite for ternary bets. ∎

### 7.2 Central Limit Theorem

**Theorem 7.3** (CLT for Bets). For standardized sample mean:

$$\sqrt{n}\frac{\bar{X}_n - \mu}{\sigma} \xrightarrow{d} N(0, 1)$$

where μ = E[X] and σ² = Var(X).

*Proof*: Standard CLT applies since bet outcomes have finite variance. ∎

**Corollary 7.1** (Confidence Intervals). For large n, approximate 95% CI for true probability:

$$\hat{p} \pm 1.96\sqrt{\frac{\hat{p}(1-\hat{p})}{n}}$$

This justifies the `bet-probability` function's convergence.

---

## 8. Martingale Theory

### 8.1 Bet Sequences as Martingales

**Definition 8.1** (Cumulative Bet Sum). Let Sₙ = Σᵢ₌₁ⁿ Xᵢ where Xᵢ are centered i.i.d. bets.

**Theorem 8.1** (Martingale Property). If E[Xᵢ] = 0, then {Sₙ} is a martingale:

$$\mathbb{E}[S_{n+1} | S_1, ..., S_n] = S_n$$

*Proof*:
$$\mathbb{E}[S_{n+1} | S_n] = \mathbb{E}[S_n + X_{n+1} | S_n] = S_n + \mathbb{E}[X_{n+1}] = S_n + 0 = S_n ∎$$

### 8.2 Optional Stopping

**Theorem 8.2** (Optional Stopping for bet-until). For `(bet-until pred thunk)` with stopping time τ:

If E[τ] < ∞ and the bet sum is a martingale, then E[S_τ] = E[S₀] = 0.

---

## 9. Measure-Theoretic Probability

### 9.1 Lebesgue Integration

**Definition 9.1** (Expectation as Lebesgue Integral). For bet random variable X:

$$\mathbb{E}[X] = \int_\Omega X(\omega) \, dP(\omega)$$

For discrete ternary space, this reduces to the sum:

$$\mathbb{E}[X] = \sum_{\omega \in \Omega} X(\omega) P(\{\omega\})$$

### 9.2 Radon-Nikodym Derivative

**Theorem 9.1** (Density for Weighted Bets). Given uniform measure Q and weighted measure P:

$$\frac{dP}{dQ}(\omega) = \frac{3w_\omega}{\sum w}$$

This is the likelihood ratio used in importance sampling.

---

## 10. Category-Theoretic Foundations

### 10.1 The Giry Monad

**Definition 10.1** (Connection to Giry Monad). Betlang's probability monad is a discrete instantiation of the Giry monad on the category of measurable spaces.

The Giry monad G: **Meas** → **Meas** maps:
- Objects: Measurable space (X, Σ) ↦ (G(X), Σ_G)
- Morphisms: Measurable f ↦ G(f) via pushforward

**Theorem 10.1**. The discrete Bet monad is the restriction of the Giry monad to finite discrete spaces.

### 10.2 Lawvere Theory

**Definition 10.2** (Ternary Lawvere Theory). The algebraic theory of betlang is generated by:
- One ternary operation: bet(-, -, -)
- Subject to idempotency: bet(x, x, x) = x
- Symmetry under outcome permutation (distributionally)

---

## Appendix A: Notation Summary

| Symbol | Meaning |
|--------|---------|
| Ω | Sample space {A, B, C} |
| F | σ-algebra (power set) |
| P | Probability measure |
| E[X] | Expected value |
| Var(X) | Variance |
| H(X) | Shannon entropy |
| Bet(A) | Probability monad over type A |
| T₃ | Ternary truth domain {0, ½, 1} |

## Appendix B: Key Results Summary

1. Uniform bet achieves maximum entropy log₂(3) ≈ 1.585 bits
2. Bet monad satisfies all three monad laws
3. LLN and CLT apply to bet sequences
4. Ternary logic satisfies De Morgan's laws
5. Weighted bets define valid probability measures iff weights are non-negative

---

## References

1. Kolmogorov, A.N. (1933). *Foundations of the Theory of Probability*
2. Kleene, S.C. (1952). *Introduction to Metamathematics*
3. Giry, M. (1982). "A categorical approach to probability theory"
4. Lawvere, F.W. (1963). "Functorial Semantics of Algebraic Theories"
5. Shannon, C.E. (1948). "A Mathematical Theory of Communication"
