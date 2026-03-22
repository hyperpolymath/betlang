# Formal Semantics of Betlang

## Abstract

This document provides a complete formal semantics for betlang, including operational semantics (small-step and big-step), denotational semantics, and axiomatic semantics. We establish the semantic foundations required for formal reasoning about betlang programs.

---

## 1. Syntax

### 1.1 Abstract Grammar

```
e ::= v                           ; values
    | (bet e e e)                 ; ternary choice
    | (bet/weighted p p p)        ; weighted choice
    | (bet/conditional e e e e)   ; conditional choice
    | (bet/lazy e e e)            ; lazy choice
    | (bet-chain e e e)           ; iteration
    | (bet-compose e e e)         ; function composition
    | (e e)                       ; application
    | (λ (x) e)                   ; abstraction
    | (if e e e)                  ; conditional
    | x                           ; variable

v ::= n                           ; numbers
    | b                           ; booleans
    | 'sym                        ; symbols
    | (λ (x) e)                   ; closures
    | ()                          ; unit

p ::= '(e w)                      ; weighted pair (value, weight)

w ::= n where n ≥ 0              ; non-negative weight
```

### 1.2 Syntactic Sugar

```
(bet A B C) ≡ (bet/weighted '(A 1) '(B 1) '(C 1))
```

---

## 2. Operational Semantics

### 2.1 Configuration

A configuration is a triple ⟨e, σ, ω⟩ where:
- e: expression being evaluated
- σ: environment mapping variables to values
- ω: random stream (infinite sequence of values in [0,1))

### 2.2 Small-Step Semantics (→)

#### Values
$$\frac{}{⟨v, σ, ω⟩ → ⟨v, σ, ω⟩} \text{ (V-VALUE)}$$

#### Uniform Bet
$$\frac{r = \text{head}(ω) \quad ω' = \text{tail}(ω) \quad i = \lfloor 3r \rfloor}{⟨\text{(bet } v_0 \ v_1 \ v_2\text{)}, σ, ω⟩ → ⟨v_i, σ, ω'⟩} \text{ (E-BET)}$$

#### Weighted Bet
$$\frac{r = \text{head}(ω) \quad ω' = \text{tail}(ω) \quad W = w_0 + w_1 + w_2 \quad i = \text{select}(r \cdot W, [w_0, w_1, w_2])}{⟨\text{(bet/weighted } '(v_0 \ w_0) \ '(v_1 \ w_1) \ '(v_2 \ w_2)\text{)}, σ, ω⟩ → ⟨v_i, σ, ω'⟩} \text{ (E-WBET)}$$

where select(x, [w₀, w₁, w₂]) returns:
- 0 if x < w₀
- 1 if w₀ ≤ x < w₀ + w₁
- 2 otherwise

#### Conditional Bet
$$\frac{⟨e_{pred}, σ, ω⟩ →^* ⟨\text{true}, σ, ω'⟩}{⟨\text{(bet/conditional } e_{pred} \ e_a \ e_b \ e_c\text{)}, σ, ω⟩ → ⟨e_a, σ, ω'⟩} \text{ (E-COND-T)}$$

$$\frac{⟨e_{pred}, σ, ω⟩ →^* ⟨\text{false}, σ, ω'⟩}{⟨\text{(bet/conditional } e_{pred} \ e_a \ e_b \ e_c\text{)}, σ, ω⟩ → ⟨\text{(bet } e_b \ e_c \ e_a\text{)}, σ, ω'⟩} \text{ (E-COND-F)}$$

#### Lazy Bet
$$\frac{r = \text{head}(ω) \quad ω' = \text{tail}(ω) \quad i = \lfloor 3r \rfloor \quad ⟨(thunk_i), σ, ω'⟩ →^* ⟨v, σ, ω''⟩}{⟨\text{(bet/lazy } thunk_0 \ thunk_1 \ thunk_2\text{)}, σ, ω⟩ → ⟨v, σ, ω''⟩} \text{ (E-LAZY)}$$

#### Function Application
$$\frac{⟨e_1, σ, ω⟩ →^* ⟨(λ (x) e), σ, ω'⟩ \quad ⟨e_2, σ, ω'⟩ →^* ⟨v, σ, ω''⟩ \quad ⟨e, σ[x ↦ v], ω''⟩ →^* ⟨v', σ, ω'''⟩}{⟨(e_1 \ e_2), σ, ω⟩ → ⟨v', σ, ω'''⟩} \text{ (E-APP)}$$

#### Bet Chain
$$\frac{n = 0}{⟨\text{(bet-chain } 0 \ f \ v\text{)}, σ, ω⟩ → ⟨v, σ, ω⟩} \text{ (E-CHAIN-0)}$$

$$\frac{n > 0 \quad ⟨(f \ v), σ, ω⟩ →^* ⟨v', σ, ω'⟩ \quad ⟨\text{(bet-chain } (n-1) \ f \ v'\text{)}, σ, ω'⟩ →^* ⟨v'', σ, ω''⟩}{⟨\text{(bet-chain } n \ f \ v\text{)}, σ, ω⟩ → ⟨v'', σ, ω''⟩} \text{ (E-CHAIN-N)}$$

### 2.3 Big-Step Semantics (⇓)

#### Values
$$\frac{}{⟨v, σ, ω⟩ ⇓ ⟨v, ω⟩} \text{ (B-VALUE)}$$

#### Uniform Bet
$$\frac{⟨e_0, σ, ω⟩ ⇓ ⟨v_0, ω_0⟩ \quad ⟨e_1, σ, ω_0⟩ ⇓ ⟨v_1, ω_1⟩ \quad ⟨e_2, σ, ω_1⟩ ⇓ ⟨v_2, ω_2⟩ \quad r = \text{head}(ω_2) \quad i = \lfloor 3r \rfloor}{⟨\text{(bet } e_0 \ e_1 \ e_2\text{)}, σ, ω⟩ ⇓ ⟨v_i, \text{tail}(ω_2)⟩} \text{ (B-BET)}$$

**Note**: This evaluates all three branches before selecting (eager evaluation). For lazy semantics, see B-LAZY below.

#### Lazy Bet (Big-Step)
$$\frac{r = \text{head}(ω) \quad i = \lfloor 3r \rfloor \quad ⟨(thunk_i), σ, \text{tail}(ω)⟩ ⇓ ⟨v, ω'⟩}{⟨\text{(bet/lazy } thunk_0 \ thunk_1 \ thunk_2\text{)}, σ, ω⟩ ⇓ ⟨v, ω'⟩} \text{ (B-LAZY)}$$

---

## 3. Denotational Semantics

### 3.1 Semantic Domains

**Domain D** (semantic values):
```
D = ℤ + ℝ + Bool + Symbol + (D → Dist(D)) + Unit + Error
```

**Distribution Domain** Dist(D):
```
Dist(D) = D → [0, 1] such that Σ_d P(d) = 1
```

### 3.2 Semantic Function

$$⟦ \cdot ⟧ : \text{Expr} → \text{Env} → \text{Dist}(D)$$

#### Literals
$$⟦ n ⟧ρ = δ_n$$ (Dirac distribution at n)
$$⟦ \text{true} ⟧ρ = δ_{\text{true}}$$
$$⟦ 'sym ⟧ρ = δ_{sym}$$

#### Variables
$$⟦ x ⟧ρ = δ_{ρ(x)}$$

#### Uniform Bet
$$⟦ \text{(bet } e_0 \ e_1 \ e_2\text{)} ⟧ρ = \frac{1}{3}⟦e_0⟧ρ + \frac{1}{3}⟦e_1⟧ρ + \frac{1}{3}⟦e_2⟧ρ$$

where + denotes mixture of distributions.

#### Weighted Bet
$$⟦ \text{(bet/weighted } '(e_0 \ w_0) \ '(e_1 \ w_1) \ '(e_2 \ w_2)\text{)} ⟧ρ = \sum_{i=0}^{2} \frac{w_i}{W} ⟦e_i⟧ρ$$

where W = w₀ + w₁ + w₂.

#### Conditional
$$⟦ \text{(if } e_c \ e_t \ e_f\text{)} ⟧ρ = ⟦e_c⟧ρ \rhd λb. \begin{cases} ⟦e_t⟧ρ & \text{if } b = \text{true} \\ ⟦e_f⟧ρ & \text{if } b = \text{false} \end{cases}$$

where ▷ is monadic bind for distributions.

#### Abstraction
$$⟦ (λ (x) e) ⟧ρ = δ_{λv. ⟦e⟧ρ[x ↦ v]}$$

#### Application
$$⟦ (e_1 \ e_2) ⟧ρ = ⟦e_1⟧ρ \rhd λf. ⟦e_2⟧ρ \rhd λv. f(v)$$

### 3.3 Distribution Operations

**Mixture**: For distributions μ, ν and weight p ∈ [0,1]:
$$(p \cdot μ + (1-p) \cdot ν)(x) = p \cdot μ(x) + (1-p) \cdot ν(x)$$

**Bind** (▷): For distribution μ and function f: D → Dist(D):
$$(μ \rhd f)(y) = \sum_{x} μ(x) \cdot f(x)(y)$$

**Expected Value**:
$$\mathbb{E}[⟦e⟧ρ] = \sum_{v} v \cdot ⟦e⟧ρ(v)$$

---

## 4. Axiomatic Semantics

### 4.1 Probabilistic Hoare Logic

We extend Hoare triples to probabilistic settings:

$$\{P\} \ e \ \{Q\}_p$$

meaning: if precondition P holds, then after evaluating e, postcondition Q holds with probability at least p.

### 4.2 Axioms

#### Bet Axiom
$$\{P\} \ \text{(bet } A \ B \ C\text{)} \ \{Q\}_{1} \quad \text{if} \ P \Rightarrow Q[A/x] \land Q[B/x] \land Q[C/x]$$

"If Q holds for all possible outcomes, it holds with certainty."

#### Probabilistic Bet Axiom
$$\{P\} \ \text{(bet } A \ B \ C\text{)} \ \{x = A\}_{1/3}$$
$$\{P\} \ \text{(bet } A \ B \ C\text{)} \ \{x = B\}_{1/3}$$
$$\{P\} \ \text{(bet } A \ B \ C\text{)} \ \{x = C\}_{1/3}$$

#### Weighted Bet Axiom
$$\{P\} \ \text{(bet/weighted } '(A \ w_A) \ '(B \ w_B) \ '(C \ w_C)\text{)} \ \{x = A\}_{w_A/W}$$

where W = wₐ + w_B + w_C.

#### Conditional Axiom
$$\frac{\{P ∧ b\} \ e_1 \ \{Q\}_p \quad \{P ∧ ¬b\} \ e_2 \ \{Q\}_p}{\{P\} \ \text{(if } b \ e_1 \ e_2\text{)} \ \{Q\}_p}$$

#### Sequence Rule
$$\frac{\{P\} \ e_1 \ \{R\}_{p_1} \quad \{R\} \ e_2 \ \{Q\}_{p_2}}{\{P\} \ e_1; e_2 \ \{Q\}_{p_1 \cdot p_2}}$$

#### Consequence Rule
$$\frac{P' \Rightarrow P \quad \{P\} \ e \ \{Q\}_p \quad Q \Rightarrow Q' \quad p \leq p'}{\{P'\} \ e \ \{Q'\}_{p'}}$$

### 4.3 Expectation Calculus

For numeric expressions, we can reason about expected values:

$$\text{wp}[\text{(bet } A \ B \ C\text{)}](f) = \frac{1}{3}(f(A) + f(B) + f(C))$$

where wp is the weakest precondition transformer and f is a post-expectation.

**Theorem 4.1** (Expectation Soundness). If ⟦e⟧ρ = μ, then:
$$\mathbb{E}_{x \sim μ}[f(x)] = \text{wp}[e](f)$$

---

## 5. Semantic Properties

### 5.1 Determinism and Confluence

**Theorem 5.1** (Probabilistic Confluence). Betlang is confluent up to distribution:

If ⟨e, σ, ω⟩ →* ⟨v₁, σ, ω₁⟩ and ⟨e, σ, ω⟩ →* ⟨v₂, σ, ω₂⟩ with the same random stream ω, then v₁ = v₂.

*Proof*: The operational semantics is deterministic given a fixed random stream. ∎

**Corollary 5.1** (Distribution Determinism). The distribution ⟦e⟧ρ is uniquely determined by e and ρ.

### 5.2 Adequacy

**Theorem 5.2** (Computational Adequacy). For closed expressions e:

$$P(⟨e, ∅, ω⟩ ⇓ ⟨v, ω'⟩) = ⟦e⟧∅(v)$$

where the left side is probability over random streams ω.

*Proof sketch*: By structural induction on e, showing operational and denotational semantics agree. ∎

### 5.3 Compositionality

**Theorem 5.3** (Compositionality). The denotational semantics is compositional:

$$⟦e⟧ρ \text{ depends only on } ⟦e_i⟧ρ \text{ for immediate subexpressions } e_i$$

*Proof*: By inspection of the semantic equations. ∎

---

## 6. Type System

### 6.1 Types

```
τ ::= Int | Real | Bool | Symbol | Unit
    | τ → τ                    ; function type
    | Dist(τ)                  ; distribution type
    | ∀α. τ                    ; polymorphism
```

### 6.2 Typing Rules

$$\frac{}{Γ ⊢ n : \text{Int}} \text{ (T-INT)}$$

$$\frac{}{Γ ⊢ r : \text{Real}} \text{ (T-REAL)}$$

$$\frac{Γ ⊢ e_0 : τ \quad Γ ⊢ e_1 : τ \quad Γ ⊢ e_2 : τ}{Γ ⊢ \text{(bet } e_0 \ e_1 \ e_2\text{)} : \text{Dist}(τ)} \text{ (T-BET)}$$

$$\frac{Γ, x : τ_1 ⊢ e : τ_2}{Γ ⊢ (λ (x) e) : τ_1 → τ_2} \text{ (T-ABS)}$$

$$\frac{Γ ⊢ e_1 : τ_1 → τ_2 \quad Γ ⊢ e_2 : τ_1}{Γ ⊢ (e_1 \ e_2) : τ_2} \text{ (T-APP)}$$

$$\frac{Γ ⊢ e : \text{Dist}(τ_1) \quad Γ ⊢ f : τ_1 → \text{Dist}(τ_2)}{Γ ⊢ \text{(bind } e \ f\text{)} : \text{Dist}(τ_2)} \text{ (T-BIND)}$$

### 6.3 Type Soundness

**Theorem 6.1** (Type Soundness). If Γ ⊢ e : τ and ⟨e, σ, ω⟩ ⇓ ⟨v, ω'⟩ where σ ⊨ Γ, then v : τ.

*Proof*: By induction on the typing derivation and evaluation derivation. ∎

---

## 7. Semantic Equivalences

### 7.1 Observational Equivalence

**Definition 7.1** (Observational Equivalence). e₁ ≅ e₂ iff for all contexts C[·]:
$$⟦C[e_1]⟧∅ = ⟦C[e_2]⟧∅$$

### 7.2 Distribution Equivalence

**Definition 7.2** (Distribution Equivalence). e₁ ≡_d e₂ iff:
$$⟦e_1⟧ρ = ⟦e_2⟧ρ$$ for all ρ.

**Theorem 7.1** (Bet Symmetry). For any permutation π of {0,1,2}:
$$(bet \ e_0 \ e_1 \ e_2) ≡_d (bet \ e_{π(0)} \ e_{π(1)} \ e_{π(2)})$$

*Proof*: Both expressions produce the same uniform distribution over {⟦e₀⟧ρ, ⟦e₁⟧ρ, ⟦e₂⟧ρ}. ∎

**Theorem 7.2** (Idempotence).
$$(bet \ e \ e \ e) ≡_d e$$

*Proof*: ⟦(bet e e e)⟧ρ = (1/3)⟦e⟧ρ + (1/3)⟦e⟧ρ + (1/3)⟦e⟧ρ = ⟦e⟧ρ. ∎

**Theorem 7.3** (Linearity of Expectation).
$$\mathbb{E}[\text{(bet } A \ B \ C\text{)}] = \frac{1}{3}(\mathbb{E}[A] + \mathbb{E}[B] + \mathbb{E}[C])$$

---

## 8. Fixed Points and Recursion

### 8.1 Domain-Theoretic Foundation

**Definition 8.1** (CPO of Distributions). Dist(D) forms a complete partial order under:
$$μ ⊑ ν \iff ∀x. μ(x) ≤ ν(x)$$

**Theorem 8.1** (Kleene Fixed Point). Recursive bet definitions have least fixed points:

For F: Dist(D) → Dist(D) continuous, there exists least fixed point μ* = ⊔ₙ Fⁿ(⊥).

### 8.2 Recursive Bets

The `bet-chain` and `bet-until` constructs define recursive computations:

$$⟦\text{(bet-until } p \ t\text{)}⟧ρ = \mu X. (⟦t⟧ρ \rhd λv. \text{if } p(v) \text{ then } δ_v \text{ else } X)$$

---

## 9. Game Semantics (Optional Advanced Framework)

### 9.1 Games for Probabilistic Choice

A probabilistic game consists of:
- Positions P
- Player/Opponent moves at each position
- A probability distribution over nature's moves

**Definition 9.1** (Bet Game). The game for (bet A B C):
- Initial position: ?
- Nature moves: selects branch with probability 1/3 each
- Player responds: plays the selected subgame

### 9.2 Strategy Composition

Strategies compose via:
- Sequential: play one game then another
- Probabilistic: nature selects which strategy

---

## 10. Semantic Gaps and Future Work

### 10.1 Currently Unformalized

**TODO**: The following require additional formalization:

1. **Continuous distributions**: Current semantics handle discrete distributions only
2. **Measure-theoretic continuous semantics**: Need σ-algebra over function spaces
3. **Concurrency semantics**: No formal model for parallel bet execution
4. **Effect handlers**: Exception propagation not fully formalized

### 10.2 Proposed Extensions

1. **Probabilistic Effect System**: Track probabilistic effects in types
2. **Gradual Typing**: Allow dynamic probabilistic types
3. **Dependent Types**: Types depending on probability values

---

## Appendix: Inference Rules Summary

### Operational Rules
| Rule | Form |
|------|------|
| E-BET | Uniform random selection |
| E-WBET | Weighted random selection |
| E-COND-T/F | Conditional branching |
| E-LAZY | Lazy evaluation |
| E-CHAIN | Iterative chaining |

### Typing Rules
| Rule | Judgment |
|------|----------|
| T-BET | (bet e e e) : Dist(τ) |
| T-BIND | (bind e f) : Dist(τ₂) |
| T-ABS | (λ x e) : τ₁ → τ₂ |

---

## References

1. Plotkin, G. (1981). "A Structural Approach to Operational Semantics"
2. Moggi, E. (1991). "Notions of Computation and Monads"
3. Kozen, D. (1981). "Semantics of Probabilistic Programs"
4. McIver, A. & Morgan, C. (2005). "Abstraction, Refinement and Proof for Probabilistic Systems"
5. Ramsey, N. & Pfeffer, A. (2002). "Stochastic Lambda Calculus"
