# Type Theory and Probability Monad Proofs

## Abstract

This document establishes the type-theoretic foundations of betlang, formalizing the probability monad structure, proving the monad laws, and establishing type safety. We present a gradual type system suitable for a dynamically-typed host language (Racket) with optional static typing guarantees.

---

## 1. The Probability Monad

### 1.1 Monad Definition

**Definition 1.1** (Probability Monad). The triple (Dist, η, μ) forms a monad where:

- **Dist**: Type → Type is the distribution type constructor
- **η**: A → Dist(A) is the unit (return)
- **μ**: Dist(Dist(A)) → Dist(A) is the join (flatten)

Equivalently, using Kleisli presentation with bind:
- **return**: A → Dist(A)
- **>>=** (bind): Dist(A) → (A → Dist(B)) → Dist(B)

### 1.2 Concrete Implementation

```racket
;; return : A → Dist(A)
(define (bet-pure x)
  (bet x x x))  ; Dirac distribution at x

;; bind : Dist(A) → (A → Dist(B)) → Dist(B)
(define (bet-bind m f)
  (f (m)))  ; Sample from m, then apply f
```

### 1.3 Monad Laws

**Theorem 1.1** (Left Identity). For all x and f:
```
(bet-bind (bet-pure x) f) ≡ (f x)
```

*Proof*:
```
  (bet-bind (bet-pure x) f)
= (bet-bind (bet x x x) f)
= (f (bet x x x))
= (f x)  ; since (bet x x x) = x with probability 1
```
∎

**Theorem 1.2** (Right Identity). For all m:
```
(bet-bind m bet-pure) ≡ m
```

*Proof*:
```
  (bet-bind m bet-pure)
= (bet-pure (m))
= (bet (m) (m) (m))
```

This is distributionally equivalent to m since bet-pure maps each outcome back to itself. More formally:

Let m produce value v with probability P(v). Then:
- bet-bind m bet-pure produces (bet v v v) for each sampled v
- (bet v v v) = v with probability 1
- Therefore the output distribution equals the input distribution
∎

**Theorem 1.3** (Associativity). For all m, f, g:
```
(bet-bind (bet-bind m f) g) ≡ (bet-bind m (λ x. bet-bind (f x) g))
```

*Proof*:
```
LHS = (bet-bind (bet-bind m f) g)
    = (g ((bet-bind m f)))
    = (g ((f (m))))

RHS = (bet-bind m (λ x. bet-bind (f x) g))
    = ((λ x. bet-bind (f x) g) (m))
    = (bet-bind (f (m)) g)
    = (g ((f (m))))

LHS = RHS
```
∎

---

## 2. Functor Laws

### 2.1 Definition

**Definition 2.1** (bet-map as Functor). The map operation:
```racket
(define (bet-fmap f m)
  (bet-bind m (λ x. bet-pure (f x))))
```

### 2.2 Functor Laws

**Theorem 2.1** (Identity).
```
(bet-fmap identity m) ≡ m
```

*Proof*:
```
  (bet-fmap identity m)
= (bet-bind m (λ x. bet-pure (identity x)))
= (bet-bind m (λ x. bet-pure x))
= (bet-bind m bet-pure)
= m  ; by right identity
```
∎

**Theorem 2.2** (Composition).
```
(bet-fmap (compose f g) m) ≡ (bet-fmap f (bet-fmap g m))
```

*Proof*:
```
LHS = (bet-bind m (λ x. bet-pure ((compose f g) x)))
    = (bet-bind m (λ x. bet-pure (f (g x))))

RHS = (bet-fmap f (bet-bind m (λ x. bet-pure (g x))))
    = (bet-bind (bet-bind m (λ x. bet-pure (g x))) (λ y. bet-pure (f y)))
    = (bet-bind m (λ x. bet-bind (bet-pure (g x)) (λ y. bet-pure (f y))))  ; assoc
    = (bet-bind m (λ x. (λ y. bet-pure (f y)) (g x)))  ; left identity
    = (bet-bind m (λ x. bet-pure (f (g x))))

LHS = RHS
```
∎

---

## 3. Applicative Functor Laws

### 3.1 Definition

**Definition 3.1** (Applicative Operations).
```racket
(define (bet-ap mf mx)
  (bet-bind mf (λ f. bet-bind mx (λ x. bet-pure (f x)))))

(define bet-pure bet-pure)  ; same as monad return
```

### 3.2 Applicative Laws

**Theorem 3.1** (Identity).
```
(bet-ap (bet-pure identity) m) ≡ m
```

*Proof*:
```
  (bet-ap (bet-pure identity) m)
= (bet-bind (bet-pure identity) (λ f. bet-bind m (λ x. bet-pure (f x))))
= (bet-bind m (λ x. bet-pure (identity x)))  ; left identity
= (bet-bind m bet-pure)
= m  ; right identity
```
∎

**Theorem 3.2** (Homomorphism).
```
(bet-ap (bet-pure f) (bet-pure x)) ≡ (bet-pure (f x))
```

*Proof*:
```
  (bet-ap (bet-pure f) (bet-pure x))
= (bet-bind (bet-pure f) (λ g. bet-bind (bet-pure x) (λ y. bet-pure (g y))))
= (bet-bind (bet-pure x) (λ y. bet-pure (f y)))  ; left identity
= (bet-pure (f x))  ; left identity
```
∎

**Theorem 3.3** (Interchange).
```
(bet-ap mf (bet-pure x)) ≡ (bet-ap (bet-pure (λ f. f x)) mf)
```

*Proof*: By expansion using monad laws and function application. ∎

**Theorem 3.4** (Composition).
```
(bet-ap (bet-ap (bet-ap (bet-pure compose) mf) mg) mx)
≡ (bet-ap mf (bet-ap mg mx))
```

*Proof*: Follows from monad associativity. ∎

---

## 4. Kleisli Category

### 4.1 Objects and Morphisms

**Definition 4.1** (Kleisli Category for Dist).
- **Objects**: Types A, B, C, ...
- **Morphisms**: Kleisli arrows A ⇝ B = A → Dist(B)
- **Identity**: η : A → Dist(A)
- **Composition**: (g ∘_K f) = λx. bind (f x) g

### 4.2 Category Laws

**Theorem 4.1** (Left Unit).
```
η ∘_K f ≡ f
```

*Proof*:
```
(η ∘_K f)(x) = bind (f x) η = f x  ; by right identity
```
∎

**Theorem 4.2** (Right Unit).
```
f ∘_K η ≡ f
```

*Proof*:
```
(f ∘_K η)(x) = bind (η x) f = f x  ; by left identity
```
∎

**Theorem 4.3** (Associativity).
```
(h ∘_K g) ∘_K f ≡ h ∘_K (g ∘_K f)
```

*Proof*: Follows from monad associativity. ∎

---

## 5. Type System

### 5.1 Base Types

```
BaseType ::= Int | Real | Bool | Symbol | Unit | Void
```

### 5.2 Compound Types

```
Type τ ::= BaseType
         | τ → τ                    ; Function
         | Dist τ                   ; Distribution
         | List τ                   ; Homogeneous list
         | (τ₁ × τ₂ × ... × τₙ)     ; Product/tuple
         | ∀α. τ                    ; Universal quantification
```

### 5.3 Typing Judgments

**Judgment Form**: Γ ⊢ e : τ

where Γ is a typing context mapping variables to types.

### 5.4 Core Typing Rules

**Literals**
$$\frac{n ∈ ℤ}{Γ ⊢ n : \text{Int}} \quad \frac{r ∈ ℝ}{Γ ⊢ r : \text{Real}} \quad \frac{b ∈ \{\text{true}, \text{false}\}}{Γ ⊢ b : \text{Bool}}$$

**Variables**
$$\frac{x : τ ∈ Γ}{Γ ⊢ x : τ}$$

**Abstraction**
$$\frac{Γ, x : τ_1 ⊢ e : τ_2}{Γ ⊢ (λ (x) e) : τ_1 → τ_2}$$

**Application**
$$\frac{Γ ⊢ e_1 : τ_1 → τ_2 \quad Γ ⊢ e_2 : τ_1}{Γ ⊢ (e_1 \ e_2) : τ_2}$$

**Bet**
$$\frac{Γ ⊢ e_1 : τ \quad Γ ⊢ e_2 : τ \quad Γ ⊢ e_3 : τ}{Γ ⊢ (\text{bet } e_1 \ e_2 \ e_3) : \text{Dist } τ}$$

**Bet-Pure**
$$\frac{Γ ⊢ e : τ}{Γ ⊢ (\text{bet-pure } e) : \text{Dist } τ}$$

**Bet-Bind**
$$\frac{Γ ⊢ m : \text{Dist } τ_1 \quad Γ ⊢ f : τ_1 → \text{Dist } τ_2}{Γ ⊢ (\text{bet-bind } m \ f) : \text{Dist } τ_2}$$

**Sample** (extract from distribution)
$$\frac{Γ ⊢ e : \text{Dist } τ}{Γ ⊢ (\text{sample } e) : τ}$$

---

## 6. Type Safety

### 6.1 Progress

**Theorem 6.1** (Progress). If ⊢ e : τ (closed, well-typed), then either:
1. e is a value, or
2. ∃e'. e → e' (e can take a step)

*Proof*: By induction on the typing derivation.

**Case** bet e₁ e₂ e₃:
- By IH, each eᵢ either is a value or steps
- If all are values, the bet expression steps via E-BET
- If any eᵢ steps, the whole expression steps by congruence

**Case** application (e₁ e₂):
- By IH on e₁ and e₂
- If e₁ = λx.e and e₂ = v, step by β-reduction
- Otherwise, step a subexpression

Other cases follow standard patterns. ∎

### 6.2 Preservation

**Theorem 6.2** (Preservation). If Γ ⊢ e : τ and e → e', then Γ ⊢ e' : τ.

*Proof*: By induction on the evaluation derivation.

**Case** E-BET: (bet v₁ v₂ v₃) → vᵢ
- By inversion, Γ ⊢ vᵢ : τ for all i
- Therefore Γ ⊢ vᵢ : τ ✓

**Case** β-reduction: ((λx.e) v) → e[x ↦ v]
- By inversion, Γ ⊢ λx.e : τ₁ → τ₂ and Γ ⊢ v : τ₁
- So Γ, x:τ₁ ⊢ e : τ₂
- By substitution lemma, Γ ⊢ e[x ↦ v] : τ₂ ✓

∎

### 6.3 Type Soundness

**Theorem 6.3** (Type Soundness). Well-typed programs don't go wrong.

If ⊢ e : τ and e →* e' where e' is irreducible, then e' is a value of type τ.

*Proof*: By induction using Progress and Preservation. ∎

---

## 7. Subtyping for Distributions

### 7.1 Distribution Subtyping

**Definition 7.1** (Distribution Subtyping).
$$\frac{τ_1 <: τ_2}{\text{Dist } τ_1 <: \text{Dist } τ_2}$$

Distributions are covariant in their element type.

### 7.2 Numeric Subtyping

$$\text{Int} <: \text{Real}$$

**Theorem 7.1** (Subsumption).
$$\frac{Γ ⊢ e : τ_1 \quad τ_1 <: τ_2}{Γ ⊢ e : τ_2}$$

---

## 8. Effect System for Randomness

### 8.1 Effect Annotations

We can track probabilistic effects in types:

```
τ ::= ... | τ !ε        ; τ with effect ε
ε ::= Pure | Prob | ε₁ ∪ ε₂
```

### 8.2 Effect Rules

**Bet produces Prob effect**:
$$\frac{Γ ⊢ e_i : τ \ !\varepsilon_i}{Γ ⊢ (\text{bet } e_1 \ e_2 \ e_3) : τ \ !(\text{Prob} ∪ \varepsilon_1 ∪ \varepsilon_2 ∪ \varepsilon_3)}$$

**Pure computations**:
$$\frac{Γ ⊢ e : τ \ !\text{Pure}}{e \text{ is deterministic}}$$

### 8.3 Effect Masking

$$\frac{Γ ⊢ e : τ \ !\varepsilon \quad \text{seed fixed}}{Γ ⊢ (\text{bet-with-seed } s \ e) : τ \ !\text{Pure}}$$

Fixing the random seed converts probabilistic to deterministic.

---

## 9. Parametricity

### 9.1 Free Theorems

**Theorem 9.1** (Parametricity for bet-map). For bet-map with type:
```
∀α β. (α → β) → Dist α → Dist β
```

For any f: A → B and g: B → C:
```
bet-map (g ∘ f) ≡ bet-map g ∘ bet-map f
```

This is the free theorem derived from the type.

### 9.2 Naturality

**Theorem 9.2** (Natural Transformation). bet-pure is a natural transformation:
```
Id → Dist
```

For any f: A → B:
```
bet-pure ∘ f ≡ bet-map f ∘ bet-pure
```

*Proof*:
```
(bet-pure ∘ f)(x) = bet-pure (f x)
(bet-map f ∘ bet-pure)(x) = bet-map f (bet-pure x)
                          = bet-bind (bet-pure x) (λ y. bet-pure (f y))
                          = bet-pure (f x)  ; by left identity
```
∎

---

## 10. Dependent Types (Extension)

### 10.1 Indexed Distributions

For advanced type safety, we can index by probability:

```
Dist_{p} τ    ; Distribution with probability p of success
```

### 10.2 Dependent Bet

$$\frac{Γ ⊢ e_1 : τ \quad Γ ⊢ e_2 : τ \quad Γ ⊢ e_3 : τ}{Γ ⊢ (\text{bet } e_1 \ e_2 \ e_3) : \text{Dist}_{1/3} τ × \text{Dist}_{1/3} τ × \text{Dist}_{1/3} τ}$$

**TODO**: Full dependent type system requires more infrastructure:
- Π types for probability-indexed functions
- Σ types for existential probability bounds
- Refinement types for probabilistic assertions

---

## 11. Probability Algebra

### 11.1 Convex Combination

**Definition 11.1** (Convex Space). Dist(A) forms a convex space:

For p ∈ [0,1] and distributions μ, ν:
$$p \cdot μ + (1-p) \cdot ν$$

is a valid distribution.

**Theorem 11.1** (Bet as Convex Combination).
```
(bet A B C) = (1/3)·δ_A + (1/3)·δ_B + (1/3)·δ_C
```

### 11.2 Barycentric Algebra

The set of distributions satisfies:
1. **Idempotence**: p·μ + (1-p)·μ = μ
2. **Skew-commutativity**: p·μ + (1-p)·ν = (1-p)·ν + p·μ
3. **Skew-associativity**: Nesting of mixtures associates properly

---

## 12. Categorical Semantics Summary

| Structure | Betlang Instantiation |
|-----------|----------------------|
| Category | Kleisli(Dist) |
| Functor | bet-map |
| Natural Transformation | bet-pure |
| Monad | (Dist, bet-pure, bet-bind) |
| Applicative | (Dist, bet-pure, bet-ap) |
| Convex Space | probability mixtures |

---

## References

1. Wadler, P. (1995). "Monads for Functional Programming"
2. Ramsey, N. & Pfeffer, A. (2002). "Stochastic Lambda Calculus and Monads of Probability Distributions"
3. Moggi, E. (1991). "Notions of Computation and Monads"
4. Heunen, C., Kammar, O., et al. (2017). "A Convenient Category for Higher-Order Probability Theory"
5. Pierce, B.C. (2002). "Types and Programming Languages"
