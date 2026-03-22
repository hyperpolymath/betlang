# Category-Theoretic Foundations of Betlang

## Abstract

This document develops the category-theoretic foundations of betlang, formalizing the probability monad, establishing its relationship to the Giry monad, proving functoriality properties, and developing the Kleisli category structure that underlies compositional probabilistic programming.

---

## 1. Categorical Preliminaries

### 1.1 Category Definition

**Definition 1.1** (Category). A category C consists of:
- Objects: Ob(C)
- Morphisms: for each A, B ∈ Ob(C), a set Hom(A, B)
- Composition: ∘ : Hom(B, C) × Hom(A, B) → Hom(A, C)
- Identity: id_A ∈ Hom(A, A) for each A

satisfying associativity and identity laws.

### 1.2 Relevant Categories

| Category | Objects | Morphisms |
|----------|---------|-----------|
| **Set** | Sets | Functions |
| **Meas** | Measurable spaces | Measurable functions |
| **Prob** | Probability spaces | Measure-preserving maps |
| **Bet** | Ternary value spaces | Probabilistic functions |

---

## 2. Functors

### 2.1 Definition

**Definition 2.1** (Functor). A functor F: C → D consists of:
- Object mapping: F: Ob(C) → Ob(D)
- Morphism mapping: F: Hom_C(A, B) → Hom_D(F(A), F(B))

preserving identity and composition.

### 2.2 Dist Functor

**Definition 2.2** (Distribution Functor). Dist: **Set** → **Set**:
- Objects: Dist(A) = {probability distributions over A}
- Morphisms: For f: A → B, Dist(f): Dist(A) → Dist(B) is pushforward

$$(\text{Dist}(f))(μ)(B) = μ(f^{-1}(B))$$

**Theorem 2.1** (Dist is a Functor).
1. Dist(id_A) = id_{Dist(A)}
2. Dist(g ∘ f) = Dist(g) ∘ Dist(f)

*Proof*:
1. Pushforward by identity is identity on distributions
2. (Dist(g ∘ f))(μ)(C) = μ((g ∘ f)⁻¹(C)) = μ(f⁻¹(g⁻¹(C))) = (Dist(g)(Dist(f)(μ)))(C) ∎

### 2.3 bet-map as Functor Action

```racket
(bet-map f distribution) ≡ Dist(f)(distribution)
```

This is the functorial action on morphisms.

---

## 3. Monads

### 3.1 Definition

**Definition 3.1** (Monad). A monad on category C is a triple (T, η, μ) where:
- T: C → C is an endofunctor
- η: Id → T is a natural transformation (unit)
- μ: T² → T is a natural transformation (multiplication)

satisfying:
- μ ∘ Tμ = μ ∘ μT (associativity)
- μ ∘ ηT = μ ∘ Tη = id (unit laws)

### 3.2 Probability Monad (Giry Monad)

**Definition 3.2** (Giry Monad). On **Meas**:
- G(X) = probability measures on X with weak topology
- η_X: X → G(X) by η_X(x) = δ_x (Dirac measure)
- μ_X: G(G(X)) → G(X) by μ_X(Φ)(A) = ∫ μ(A) dΦ(μ)

### 3.3 Betlang Monad

**Definition 3.3** (Bet Monad). On **Set** (discrete case):
- Dist(A) = finitely-supported probability distributions on A
- η_A(a) = δ_a = Dirac at a
- μ_A(Φ) = ∫ μ dΦ = Σ_μ Φ(μ) · μ

**Theorem 3.1** (Bet Monad Laws). The triple (Dist, η, μ) satisfies monad laws.

*Proof of associativity*:
$$μ ∘ \text{Dist}(μ) = μ ∘ μ_{\text{Dist}}$$

For Φ ∈ Dist(Dist(Dist(A))):
$$LHS = μ(\text{Dist}(μ)(Φ)) = \sum_μ (\text{Dist}(μ)(Φ))(μ) · μ$$

Both sides yield the same flattening of nested distributions. ∎

*Proof of unit laws*:
$$μ ∘ η_{\text{Dist}} = id \quad \text{and} \quad μ ∘ \text{Dist}(η) = id$$

Inserting Dirac and then flattening returns original distribution. ∎

---

## 4. Kleisli Category

### 4.1 Definition

**Definition 4.1** (Kleisli Category). For monad (T, η, μ) on C, the Kleisli category C_T has:
- Objects: same as C
- Morphisms: Hom_{C_T}(A, B) = Hom_C(A, T(B))
- Composition: g ∘_K f = μ_C ∘ T(g) ∘ f
- Identity: η_A

### 4.2 Kleisli Category for Bet

**Definition 4.2** (Kleisli Morphisms for Bet).
$$\text{Hom}_{\text{Bet}}(A, B) = A → \text{Dist}(B)$$

These are "stochastic functions" or "probabilistic arrows."

**Composition**:
$$(g ∘_K f)(a) = \sum_{b} f(a)(b) \cdot g(b)$$

### 4.3 bet-bind as Kleisli Composition

```racket
(bet-bind m f) = (m >>=_K f)
```

where >>=_K is Kleisli bind:
$$(m >>= f) = μ ∘ \text{Dist}(f) ∘ m$$

For sampled m:
```racket
(bet-bind m f) = (f (sample m))
```

---

## 5. Natural Transformations

### 5.1 Definition

**Definition 5.1** (Natural Transformation). α: F ⇒ G is a family of morphisms:
$$α_A: F(A) → G(A)$$

such that for f: A → B:
$$α_B ∘ F(f) = G(f) ∘ α_A$$

### 5.2 Unit as Natural Transformation

**Theorem 5.1** (η is Natural). For f: A → B:
$$η_B ∘ f = \text{Dist}(f) ∘ η_A$$

*Proof*:
$$(η_B ∘ f)(a) = δ_{f(a)}$$
$$(\text{Dist}(f) ∘ η_A)(a) = \text{Dist}(f)(δ_a) = δ_{f(a)}$$

Equal. ∎

### 5.3 Multiplication as Natural Transformation

**Theorem 5.2** (μ is Natural). For f: A → B:
$$μ_B ∘ \text{Dist}(\text{Dist}(f)) = \text{Dist}(f) ∘ μ_A$$

---

## 6. Monoidal Structure

### 6.1 Monoidal Category

**Definition 6.1** (Monoidal Category). A monoidal category (C, ⊗, I) has:
- Bifunctor ⊗: C × C → C
- Unit object I
- Natural isomorphisms for associativity and unit

### 6.2 Product of Distributions

**Definition 6.2** (Distribution Product). For μ ∈ Dist(A), ν ∈ Dist(B):
$$μ ⊗ ν ∈ \text{Dist}(A × B)$$
$$(μ ⊗ ν)(a, b) = μ(a) · ν(b)$$

This is the product measure (independence).

### 6.3 Dist as Monoidal Functor

**Theorem 6.1** (Dist is Lax Monoidal).
$$\text{Dist}(A) × \text{Dist}(B) → \text{Dist}(A × B)$$

by the product operation.

---

## 7. Commutative Monad

### 7.1 Definition

**Definition 7.1** (Commutative Monad). A monad T is commutative if:
$$\text{strength}: T(A) × B → T(A × B)$$

satisfies commutativity conditions.

### 7.2 Commutativity of Bet Monad

**Theorem 7.1** (Bet Monad is Commutative). The order of sampling doesn't affect joint distribution:

```racket
(bet-bind m₁ (λ (x) (bet-bind m₂ (λ (y) (pair x y)))))
≡
(bet-bind m₂ (λ (y) (bet-bind m₁ (λ (x) (pair x y)))))
```

*Proof*: Both produce the product measure m₁ ⊗ m₂. ∎

---

## 8. Algebras and Eilenberg-Moore Category

### 8.1 T-Algebra

**Definition 8.1** (T-Algebra). For monad T, a T-algebra is (A, h) where:
- A is an object
- h: T(A) → A is the structure map

satisfying:
- h ∘ η_A = id_A
- h ∘ μ_A = h ∘ T(h)

### 8.2 Dist-Algebras

**Theorem 8.1** (Dist-Algebras are Convex Spaces). A Dist-algebra is equivalently a convex set: a set with affine combinations.

For ternary: ability to form weighted averages of three elements.

### 8.3 Free Algebra

**Definition 8.2** (Free Dist-Algebra). The free Dist-algebra on set A is (Dist(A), μ_A).

---

## 9. Adjunctions

### 9.1 Definition

**Definition 9.1** (Adjunction). F: C → D is left adjoint to G: D → C (F ⊣ G) if:
$$\text{Hom}_D(F(A), B) ≅ \text{Hom}_C(A, G(B))$$

naturally in A and B.

### 9.2 Monad from Adjunction

Every monad arises from an adjunction:
$$T = G ∘ F$$
$$η = \text{unit of adjunction}$$
$$μ = G(ε_F) \text{ where } ε \text{ is counit}$$

### 9.3 Kleisli Adjunction

**Theorem 9.1** (Kleisli Adjunction). There is an adjunction:
$$F_T: C → C_T, \quad G_T: C_T → C$$

where F_T(A) = A and G_T(A) = T(A).

---

## 10. Strength and Enrichment

### 10.1 Strength

**Definition 10.1** (Strength). A strength for monad T is:
$$\text{st}_{A,B}: A × T(B) → T(A × B)$$

natural in A, B, satisfying coherence conditions.

### 10.2 Strength for Dist

```racket
(define (strength a dist-b)
  (bet-map (λ (b) (cons a b)) dist-b))
```

This pairs a fixed value with all outcomes.

### 10.3 Enrichment in Probability

The Kleisli category for Dist is enriched over probability spaces: hom-sets carry probabilistic structure.

---

## 11. Lawvere Theory

### 11.1 Definition

**Definition 11.1** (Lawvere Theory). A Lawvere theory L is a category with:
- Objects: natural numbers 0, 1, 2, ...
- n = 1 + 1 + ... + 1 (n times)
- Product: n × m = n + m

### 11.2 Theory of Bet

**Definition 11.2** (Ternary Theory). The Lawvere theory for betlang has:
- One ternary operation: bet: 3 → 1
- Equations: idempotency, symmetry (up to distribution)

**Ternary operation**:
$$\text{bet}: A × A × A → \text{Dist}(A)$$

### 11.3 Models

A model of this theory is a set A with:
- bet: A³ → Dist(A)
- satisfying idempotency: bet(a, a, a) = δ_a

---

## 12. Operad Structure

### 12.1 Definition

**Definition 12.1** (Operad). An operad P consists of:
- Sets P(n) for n ≥ 0 (n-ary operations)
- Composition: P(n) × P(k₁) × ... × P(kₙ) → P(k₁ + ... + kₙ)
- Identity in P(1)

### 12.2 Bet Operad

**Definition 12.2** (Ternary Probability Operad).
- P(1) = {id}
- P(3) = {bet} ∪ weighted versions
- P(n) = compositions of ternary bets

### 12.3 Algebra over Operad

An algebra over this operad is a set with well-defined ternary probabilistic choice.

---

## 13. Categorical Probability

### 13.1 Markov Categories

**Definition 13.1** (Markov Category). A Markov category is a symmetric monoidal category where:
- Each object has a "copy" morphism
- Copy is natural w.r.t. deterministic morphisms

### 13.2 Betlang as Markov Category

The Kleisli category for Dist is a Markov category:
- Monoidal: via distribution product
- Copying: μ ↦ μ ⊗ μ (independent copies)

### 13.3 Conditional Independence

**Definition 13.2** (Conditional Independence). X ⊥ Y | Z iff:
$$P(X, Y | Z) = P(X | Z) · P(Y | Z)$$

Categorically: factorization through Z.

---

## 14. String Diagrams

### 14.1 Graphical Calculus

String diagrams for the Dist monad:

```
    A           Dist(A)
    |              |
    η              |
    ↓              |
 Dist(A)    =      |
```

```
Dist(Dist(A))      Dist(A)
      |               |
      μ               |
      ↓               |
   Dist(A)      =     |
```

### 14.2 Bet Diagram

```
  A   A   A
   \  |  /
    \ | /
    bet
      |
   Dist(A)
```

Represents ternary mixing.

---

## 15. Higher Category Theory

### 15.1 2-Category of Monads

Monads form a 2-category:
- 0-cells: Categories
- 1-cells: Monads
- 2-cells: Monad morphisms

### 15.2 Dist in the 2-Category

The Dist monad is related to other probability monads by:
- Embedding: Dist ↪ Giry (discrete into continuous)
- Quotient: Giry → Dist (discretization)

### 15.3 Monad Transformers

**Definition 15.1** (Monad Transformer). Monad transformer T lifts monads:
$$T(M) \text{ is a monad when } M \text{ is}$$

DistT: adding probabilistic effects to other monads.

---

## 16. Topos-Theoretic Aspects

### 16.1 Presheaf Topos

**Definition 16.1** (Presheaf). A presheaf on C is a functor:
$$F: C^{op} → \textbf{Set}$$

### 16.2 Probability Distributions as Presheaves

Distributions can be viewed as certain presheaves satisfying normalization.

### 16.3 Internal Logic

The internal logic of a topos with probability monads yields probabilistic reasoning principles.

---

## 17. Summary: Categorical Structure of Betlang

| Structure | Betlang Instance |
|-----------|------------------|
| Functor | Dist: Set → Set |
| Monad | (Dist, η, μ) |
| Kleisli Category | Stochastic functions |
| Monoidal | Product distributions |
| Commutative | Order-independent sampling |
| Lawvere Theory | Ternary operations |
| Markov Category | With copying |

---

## 18. TODOs

**TODO**: Further categorical development:

1. **∞-categorical structure**: Higher homotopy in probabilistic programming
2. **Profunctors**: Probabilistic relations
3. **Comonads**: For Bayesian updates
4. **Polynomial functors**: For recursive types

---

## References

1. Giry, M. (1982). "A categorical approach to probability theory."
2. Lawvere, F.W. (1963). "Functorial Semantics of Algebraic Theories."
3. Moggi, E. (1991). "Notions of Computation and Monads."
4. Fritz, T. (2020). "A synthetic approach to Markov kernels, conditional independence and theorems on sufficient statistics."
5. Perrone, P. (2021). "Markov categories and entropy."
6. Jacobs, B. (2019). "Categorical Probability."
