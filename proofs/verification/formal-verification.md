# Formal Verification Specifications for Betlang

## Abstract

This document provides formal specifications for verifying betlang programs using established verification techniques: Hoare logic, refinement types, separation logic, and probabilistic model checking. We define invariants, pre/postconditions, and correctness criteria suitable for automated and interactive theorem provers.

---

## 1. Verification Framework

### 1.1 Specification Language

We use a probabilistic specification language extending Hoare logic:

```
Spec ::= {P} e {Q}                    ; Total correctness
       | {P} e {Q}_p                  ; Probabilistic guarantee
       | {P} e {Q}_≥p                 ; Lower bound
       | {P} e {Q}_[p,q]              ; Probability range
       | ⟨P⟩ e ⟨Q⟩                    ; Partial correctness
       | e ↓                          ; Termination
       | e ↓_p                        ; Probabilistic termination
```

### 1.2 Assertion Language

```
P, Q ::= true | false
       | e₁ = e₂ | e₁ < e₂ | e₁ ≤ e₂
       | P ∧ Q | P ∨ Q | ¬P | P ⟹ Q
       | ∀x. P | ∃x. P
       | P(X) with probability p      ; Probabilistic assertion
       | E[X] = μ                     ; Expectation assertion
       | Var(X) ≤ σ²                  ; Variance bound
       | H(X) ≤ h                     ; Entropy bound
       | X ~ D                        ; Distribution assertion
```

---

## 2. Core Primitive Specifications

### 2.1 Uniform Bet

**Specification**:
```
{true}
  (bet A B C)
{result ∈ {A, B, C}}₁

{true}
  (bet A B C)
{result = A}₁/₃

{true}
  (bet A B C)
{result = B}₁/₃

{true}
  (bet A B C)
{result = C}₁/₃
```

**Distribution Specification**:
```
{true}
  let X = (bet A B C)
{X ~ Uniform({A, B, C})}
```

**Expected Value Specification** (for numeric A, B, C):
```
{A, B, C ∈ ℝ}
  let X = (bet A B C)
{E[X] = (A + B + C)/3}

{A, B, C ∈ ℝ}
  let X = (bet A B C)
{Var(X) = (A² + B² + C²)/3 - ((A + B + C)/3)²}
```

**Entropy Specification**:
```
{A ≠ B ∨ B ≠ C}
  let X = (bet A B C)
{H(X) = log₂(3)}

{A = B = C}
  let X = (bet A B C)
{H(X) = 0}
```

### 2.2 Weighted Bet

**Specification**:
```
{wₐ ≥ 0 ∧ w_b ≥ 0 ∧ w_c ≥ 0 ∧ wₐ + w_b + w_c > 0}
  (bet/weighted '(A wₐ) '(B w_b) '(C w_c))
{result = A}_{wₐ/(wₐ+w_b+w_c)}
```

**Normalization Invariant**:
```
{W = wₐ + w_b + w_c > 0}
  let X = (bet/weighted ...)
{P(X = A) + P(X = B) + P(X = C) = 1}
```

### 2.3 Conditional Bet

**Specification**:
```
{true}
  (bet/conditional pred A B C)
{pred ⟹ result = A}₁

{¬pred}
  (bet/conditional pred A B C)
{result ∈ {B, C, A}}₁
```

### 2.4 Lazy Bet

**Specification**:
```
{true}
  (bet/lazy thunk_A thunk_B thunk_C)
{result = (thunk_A) ∨ result = (thunk_B) ∨ result = (thunk_C)}₁

;; Only one thunk is evaluated
{true}
  (bet/lazy thunk_A thunk_B thunk_C)
{exactly_one_evaluated(thunk_A, thunk_B, thunk_C)}₁
```

---

## 3. Iteration Specifications

### 3.1 bet-chain

**Specification**:
```
{n ≥ 0}
  (bet-chain n f init)
{result = fⁿ(init)}
```

where fⁿ denotes n-fold application of f.

**Loop Invariant**:
```
;; Invariant: after k iterations, state = f^k(init)
{n ≥ 0 ∧ ∀k ∈ [0,n]. f^k(init) is defined}
  (bet-chain n f init)
{result = fⁿ(init)}
```

**Termination**:
```
(bet-chain n f init) ↓  ⟺  n ∈ ℕ ∧ f terminates on all intermediate values
```

### 3.2 bet-until

**Specification**:
```
{P(pred(thunk())) = p > 0}
  (bet-until pred thunk)
{pred(result)}₁

{P(pred(thunk())) = p > 0}
  (bet-until pred thunk)
↓₁   ;; Almost sure termination
```

**Expected Iterations**:
```
{P(pred(thunk())) = p}
  let N = iterations in (bet-until pred thunk)
{E[N] = 1/p}
```

### 3.3 bet-repeat

**Specification**:
```
{n ≥ 0}
  (bet-repeat n thunk)
{|result| = n ∧ ∀i. result[i] = some evaluation of thunk}
```

**Independence**:
```
{n ≥ 0}
  (bet-repeat n thunk)
{result[i] ⊥ result[j] for i ≠ j}  ;; Independent samples
```

---

## 4. Probability Estimation Specifications

### 4.1 bet-probability

**Specification**:
```
{n > 0 ∧ p = P(pred(bet A B C))}
  let p̂ = (bet-probability n pred A B C)
{E[p̂] = p}                           ;; Unbiased

{n > 0}
  let p̂ = (bet-probability n pred A B C)
{Var(p̂) = p(1-p)/n}                  ;; Variance

{n > 0 ∧ ε > 0}
  let p̂ = (bet-probability n pred A B C)
{|p̂ - p| < ε}_{1 - 2exp(-2nε²)}       ;; Hoeffding bound
```

### 4.2 bet-expect

**Specification**:
```
{n > 0 ∧ μ = E[f(bet A B C)]}
  let μ̂ = (bet-expect n f A B C)
{E[μ̂] = μ}                           ;; Unbiased

{n > 0}
  let μ̂ = (bet-expect n f A B C)
{μ̂ → μ as n → ∞}                     ;; Consistency
```

### 4.3 bet-entropy

**Specification**:
```
{n > 0 ∧ samples from distribution with entropy H}
  let Ĥ = (bet-entropy samples)
{E[Ĥ] ≈ H - (k-1)/(2n ln 2)}         ;; Bias formula

{n → ∞}
  let Ĥ = (bet-entropy samples)
{Ĥ → H}                              ;; Consistency
```

---

## 5. Statistical Function Specifications

### 5.1 Descriptive Statistics

**Mean**:
```
{samples ≠ ∅}
  let μ̂ = (mean samples)
{μ̂ = Σᵢ samples[i] / |samples|}

{samples are i.i.d. with mean μ}
  let μ̂ = (mean samples)
{E[μ̂] = μ ∧ μ̂ → μ a.s.}
```

**Variance**:
```
{|samples| > 1}
  let σ̂² = (variance samples)
{σ̂² = Σᵢ (samples[i] - mean)² / (|samples| - 1)}  ;; Bessel correction

{samples are i.i.d. with variance σ²}
  let σ̂² = (variance samples)
{E[σ̂²] = σ²}                         ;; Unbiased
```

**Median**:
```
{samples ≠ ∅}
  let m = (median samples)
{|{x ∈ samples : x ≤ m}| ≥ |samples|/2 ∧
 |{x ∈ samples : x ≥ m}| ≥ |samples|/2}
```

### 5.2 Statistical Tests

**Chi-Square Test**:
```
{observed, expected have same length ∧ expected[i] > 0 ∀i}
  let (χ², p) = (chi-square-test observed expected)
{χ² = Σᵢ (observed[i] - expected[i])² / expected[i]}

{H₀: observed ~ expected}
  let (χ², p) = (chi-square-test observed expected)
{p = P(χ²_{k-1} > χ² | H₀)}
```

**Kolmogorov-Smirnov Test**:
```
{samples₁, samples₂ ≠ ∅}
  let D = (kolmogorov-smirnov samples₁ samples₂)
{D = sup_x |F₁(x) - F₂(x)|}          ;; Two-sample KS statistic
```

---

## 6. Distribution Specifications

### 6.1 Continuous Distributions

**Normal Distribution**:
```
{σ > 0}
  let X = (normal μ σ)
{X ~ N(μ, σ²)}

{true}
  let X = (normal μ σ)
{E[X] = μ ∧ Var(X) = σ²}
```

**Exponential Distribution**:
```
{λ > 0}
  let X = (exponential λ)
{X ~ Exp(λ) ∧ E[X] = 1/λ ∧ Var(X) = 1/λ²}
```

**Gamma Distribution**:
```
{α > 0 ∧ β > 0}
  let X = (gamma α β)
{X ~ Gamma(α, β) ∧ E[X] = αβ ∧ Var(X) = αβ²}
```

### 6.2 Discrete Distributions

**Binomial Distribution**:
```
{n ∈ ℕ ∧ 0 ≤ p ≤ 1}
  let X = (binomial n p)
{X ~ Binomial(n, p) ∧ E[X] = np ∧ Var(X) = np(1-p)}
```

**Poisson Distribution**:
```
{λ > 0}
  let X = (poisson λ)
{X ~ Poisson(λ) ∧ E[X] = λ ∧ Var(X) = λ}
```

---

## 7. MCMC Specifications

### 7.1 Metropolis-Hastings

**Detailed Balance**:
```
{π is target distribution, q is proposal}
  (metropolis-hastings π q x₀ n)
{∀x,y. π(x)P(x→y) = π(y)P(y→x)}      ;; Detailed balance
```

**Convergence**:
```
{chain is irreducible and aperiodic}
  let samples = (metropolis-hastings π q x₀ n)
{empirical_distribution(samples) → π as n → ∞}
```

**Acceptance Rate**:
```
{true}
  let (samples, accepts) = (metropolis-hastings π q x₀ n)
{E[accepts/n] depends on proposal tuning}
```

### 7.2 Gibbs Sampling

**Full Conditional Correctness**:
```
{p(x|y) and p(y|x) are correct full conditionals of joint p(x,y)}
  (gibbs-sampler cond-x cond-y x₀ y₀ n)
{samples → p(x,y) as n → ∞}
```

---

## 8. Bayesian Inference Specifications

### 8.1 Bayes Theorem

**Correctness**:
```
{prior_prob > 0 ∧ evidence_prob > 0}
  let posterior = (bayes-theorem prior_prob likelihood evidence_prob)
{posterior = (prior_prob × likelihood) / evidence_prob}

{prior_prob × likelihood / evidence_prob ∈ [0,1]}
  (bayes-theorem prior_prob likelihood evidence_prob)
{result ∈ [0, 1]}                    ;; Valid probability
```

### 8.2 Conjugate Priors

**Beta-Binomial**:
```
{α > 0 ∧ β > 0 ∧ k ≥ 0 ∧ n ≥ 0}
  let posterior = (conjugate-beta-binomial α β k (n-k))
{posterior ~ Beta(α + k, β + n - k)}
```

**Normal-Normal**:
```
{σ₀² > 0 ∧ σ² > 0}
  let posterior = (conjugate-normal μ₀ σ₀² σ² observations)
{posterior ~ N(μ_n, σ_n²) where μ_n, σ_n² are posterior parameters}
```

### 8.3 Credible Intervals

```
{0 < α < 1}
  let (lo, hi) = (credible-interval samples α)
{P(lo ≤ θ ≤ hi | data) = 1 - α}
```

---

## 9. Markov Chain Specifications

### 9.1 Transition Matrix

**Stochasticity**:
```
{transitions is a valid transition specification}
  let chain = (make-markov-chain states transitions initial)
{∀s. Σ_s' P(s→s') = 1}              ;; Rows sum to 1
```

### 9.2 Simulation

**Markov Property**:
```
{chain is a valid Markov chain}
  let path = (markov-simulate chain n)
{P(path[k+1] | path[0..k]) = P(path[k+1] | path[k])}
```

**Stationary Distribution**:
```
{chain is irreducible and aperiodic}
  let π̂ = (markov-stationary chain n)
{π̂ → π as n → ∞ where π is the unique stationary distribution}
```

---

## 10. Refinement Types

### 10.1 Probability-Indexed Types

```
Prob_p T    ;; Type T with associated probability p
```

**Subtyping**:
```
p ≤ q ⟹ Prob_p T <: Prob_q T
```

### 10.2 Refined Bet Types

```
bet : ∀A. A → A → A → Prob_{1/3} A

bet/weighted : ∀A. (A, w₁:ℕ) → (A, w₂:ℕ) → (A, w₃:ℕ) →
               Prob_{w₁/(w₁+w₂+w₃)} A
```

### 10.3 Dependent Probability Types

```
;; Type depending on probability
type BetResult[p : Prob] = { x : Value | P(selected) = p }

bet : A → B → C → Σ(i : Fin 3). BetResult[1/3]
```

---

## 11. Separation Logic for Probabilistic State

### 11.1 Probabilistic Heap Assertions

```
P ::= ...
    | x ↦ v                        ;; x points to value v
    | x ↦_p v                      ;; x points to v with probability p
    | P * Q                        ;; Separating conjunction
    | P -* Q                       ;; Magic wand
```

### 11.2 Frame Rule

```
{P} e {Q}
─────────────────
{P * R} e {Q * R}
```

The frame rule preserves separate resources.

### 11.3 Random State Assertions

```
{rng_state = s}
  (bet A B C)
{rng_state = next(s) ∧ result = select(s, A, B, C)}
```

---

## 12. Verification Conditions

### 12.1 Weakest Precondition

**Definition**:
```
wp[bet A B C](Q) = (1/3)Q[A] + (1/3)Q[B] + (1/3)Q[C]

wp[bet/weighted (A,wₐ) (B,w_b) (C,w_c)](Q) =
  (wₐ/W)Q[A] + (w_b/W)Q[B] + (w_c/W)Q[C]

wp[if b then e₁ else e₂](Q) =
  (b ⟹ wp[e₁](Q)) ∧ (¬b ⟹ wp[e₂](Q))

wp[let x = e₁ in e₂](Q) = wp[e₁](λv. wp[e₂[x↦v]](Q))
```

### 12.2 Verification Condition Generation

For probabilistic programs:

```
VC({P} e {Q}_p) = P ⟹ wp[e](Q) ≥ p
```

### 12.3 Automated Verification

Decidable fragments:
- Linear arithmetic assertions
- Polynomial probability expressions
- Finite-state distributions

---

## 13. Model Checking Specifications

### 13.1 PCTL Properties

Probabilistic Computation Tree Logic for betlang:

```
φ ::= true | a | ¬φ | φ ∧ ψ
    | P_{∼p}[ψ]              ;; Probability operator
    | X φ                     ;; Next
    | φ U ψ                   ;; Until
    | F φ                     ;; Eventually
    | G φ                     ;; Always
```

### 13.2 Example Properties

**Termination**:
```
P_{=1}[F terminated]          ;; Almost sure termination
```

**Reachability**:
```
P_{≥0.9}[F goal]              ;; Reach goal with prob ≥ 0.9
```

**Safety**:
```
P_{=1}[G ¬error]              ;; Never reach error state
```

### 13.3 Continuous Stochastic Logic

For continuous-time extensions:

```
φ ::= ... | P_{∼p}[φ U^{≤t} ψ]    ;; Time-bounded until
```

---

## 14. Proof Obligations

### 14.1 For Each Core Primitive

1. **bet**: Uniformity, independence, termination
2. **bet/weighted**: Normalization, non-negativity, termination
3. **bet/conditional**: Determinism under true predicate
4. **bet/lazy**: Lazy evaluation correctness

### 14.2 For Statistical Functions

1. **Unbiasedness**: E[estimator] = true value
2. **Consistency**: Convergence to true value
3. **Efficiency**: Variance bounds (Cramér-Rao)
4. **Robustness**: Behavior under model misspecification

### 14.3 For MCMC Methods

1. **Detailed balance**: Reversibility
2. **Irreducibility**: All states reachable
3. **Aperiodicity**: No cycling
4. **Geometric ergodicity**: Convergence rate bounds

---

## 15. Mechanized Proofs

### 15.1 Coq Encoding (Sketch)

```coq
Inductive BetExpr (A : Type) : Type :=
  | Bet : A -> A -> A -> BetExpr A
  | BetWeighted : (A * nat) -> (A * nat) -> (A * nat) -> BetExpr A.

Definition bet_probability {A} (e : BetExpr A) (x : A) : Q :=
  match e with
  | Bet a b c =>
      if dec_eq x a then 1/3 else
      if dec_eq x b then 1/3 else
      if dec_eq x c then 1/3 else 0
  | BetWeighted (a, wa) (b, wb) (c, wc) =>
      let W := wa + wb + wc in
      if dec_eq x a then wa/W else ...
  end.

Theorem bet_uniform : forall A (a b c : A),
  bet_probability (Bet a b c) a = 1/3 /\
  bet_probability (Bet a b c) b = 1/3 /\
  bet_probability (Bet a b c) c = 1/3.
Proof. ... Qed.
```

### 15.2 Lean Encoding (Sketch)

```lean
def bet_dist (a b c : α) : Pmf α :=
  Pmf.ofFinset {a, b, c} (by simp)

theorem bet_uniform (a b c : α) (h : a ≠ b ∧ b ≠ c ∧ a ≠ c) :
  (bet_dist a b c) a = 1/3 ∧ (bet_dist a b c) b = 1/3 ∧ (bet_dist a b c) c = 1/3 := by
  ...
```

---

## 16. TODOs: Incomplete Specifications

**TODO**: The following require formal specification:

1. **Continuous distribution correctness**: PDF matching for all distributions
2. **Numerical stability**: Floating-point error bounds for all functions
3. **Concurrency**: Specifications for parallel/concurrent bet operations
4. **Streaming**: Specifications for online/streaming statistics
5. **Approximate inference**: Error bounds for ABC, variational methods

---

## References

1. Morgan, C., McIver, A., & Seidel, K. (1996). "Probabilistic Predicate Transformers"
2. Barthe, G., et al. (2012). "Probabilistic Relational Reasoning"
3. Batz, K., et al. (2021). "Foundations of Probabilistic Programming"
4. Hurd, J. (2003). "Formal Verification of Probabilistic Algorithms"
5. Kwiatkowska, M., Norman, G., & Parker, D. (2011). "PRISM 4.0: Verification of Probabilistic Real-Time Systems"
