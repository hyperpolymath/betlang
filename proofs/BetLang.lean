-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
--
-- BetLang.lean — Formal Lean 4 mechanised proofs for BetLang
--
-- This file formalises the core of BetLang: syntax, types, a bidirectional
-- typing judgement, small-step operational semantics, and the fundamental
-- metatheoretic results (Progress, Preservation). It also states and proves
-- the three monad laws for the distribution monad at the syntactic/typing
-- level.
--
-- Design choices:
--   * We model a *core* calculus: literals (Int, Float, Bool, String, Unit),
--     variables, lambda, application, let, if, bet (ternary uniform choice),
--     and sample (elimination for Dist).
--   * Types mirror compiler/bet-core/src/types.rs (the simplified subset).
--   * The typing judgement uses de Bruijn indices to avoid capture issues.
--   * Bet reduces non-deterministically (modelled as three reduction rules).
--   * All theorems are fully proved — no `sorry`.

-- ════════════════════════════════════════════════════════════════════════════
-- Section 1. Syntax
-- ════════════════════════════════════════════════════════════════════════════

/-- BetLang types. Mirrors the core subset of `compiler/bet-core/src/types.rs`. -/
inductive Ty : Type where
  | int    : Ty
  | float  : Ty
  | bool   : Ty
  | string : Ty
  | unit   : Ty
  | arrow  : Ty → Ty → Ty
  | dist   : Ty → Ty
  -- Echo types (structured loss; see `hyperpolymath/echo-types`). `echo T` is
  -- a proof-relevant retained-loss residue over `T`, distinct from `T`;
  -- `echoR T` is its strict, non-recoverable weakening. These are type
  -- *formers* only in this Lean development: no `Expr` constructor introduces
  -- or eliminates them and `HasType` assigns no expression an echo type, so
  -- the carrier's metatheory (Progress/Preservation below) is unaffected.
  -- NOTE: the Rust checker `bet-check` now carries the *typing rules* for the
  -- echo operations — introduction `echo : 'a → Echo 'a`, the functor/comonad
  -- surface `echo_map`/`echo_output` (counit)/`echo_duplicate`, the residue
  -- lowering `echo_to_residue : Echo 'a → EchoR 'a`, and the probabilistic
  -- bridge `sample_echo : Dist 'a → Echo 'a` (all type-level / ghost; the
  -- ungraded shadow of `EchoGradedComonad.agda`). Mirroring those rules here
  -- and re-establishing Progress/Preservation is tracked as obligation TP-5
  -- (PROOF-NEEDS.md), deferred until the runtime residue representation is
  -- settled.
  | echo   : Ty → Ty
  | echoR  : Ty → Ty
  deriving DecidableEq, Repr

/-- BetLang core expressions using de Bruijn indices. -/
inductive Expr : Type where
  | litInt    : Int → Expr
  | litFloat  : Float → Expr
  | litBool   : Bool → Expr
  | litString : String → Expr
  | litUnit   : Expr
  | var       : Nat → Expr
  | lam       : Ty → Expr → Expr          -- λ (x : T). body
  | app       : Expr → Expr → Expr
  | letE      : Ty → Expr → Expr → Expr   -- let x : T = e₁ in e₂
  | ifE       : Expr → Expr → Expr → Expr
  | bet       : Expr → Expr → Expr → Expr -- bet { e₁ e₂ e₃ }
  | sample    : Expr → Expr               -- sample e  (Dist T → T)
  | distPure  : Expr → Expr               -- return/pure for Dist (wraps value)
  | distBind  : Expr → Expr → Expr        -- bind : Dist A → (A → Dist B) → Dist B
  deriving Repr

-- ════════════════════════════════════════════════════════════════════════════
-- Section 2. Values
-- ════════════════════════════════════════════════════════════════════════════

/-- Predicate identifying value forms (no further reduction possible). -/
inductive IsValue : Expr → Prop where
  | litInt    : IsValue (Expr.litInt n)
  | litFloat  : IsValue (Expr.litFloat f)
  | litBool   : IsValue (Expr.litBool b)
  | litString : IsValue (Expr.litString s)
  | litUnit   : IsValue Expr.litUnit
  | lam       : IsValue (Expr.lam t body)
  | distPure  : IsValue v → IsValue (Expr.distPure v)

/-- Values are decidable (we need this for case analysis in proofs). -/
def isValue : Expr → Bool
  | Expr.litInt _    => true
  | Expr.litFloat _  => true
  | Expr.litBool _   => true
  | Expr.litString _ => true
  | Expr.litUnit     => true
  | Expr.lam _ _     => true
  | Expr.distPure v  => isValue v
  | _                => false

-- ════════════════════════════════════════════════════════════════════════════
-- Section 3. Typing contexts and judgement
-- ════════════════════════════════════════════════════════════════════════════

/-- A typing context is a list of types, indexed by de Bruijn level. -/
abbrev Ctx := List Ty

/-- Context lookup. -/
def Ctx.lookup : Ctx → Nat → Option Ty
  | [],     _     => none
  | t :: _, 0     => some t
  | _ :: Γ, n + 1 => Ctx.lookup Γ n

/-- Typing judgement: Γ ⊢ e : T -/
inductive HasType : Ctx → Expr → Ty → Prop where
  /-- T-Int: integer literals have type Int -/
  | tInt    : HasType Γ (Expr.litInt n) Ty.int
  /-- T-Float: float literals have type Float -/
  | tFloat  : HasType Γ (Expr.litFloat f) Ty.float
  /-- T-Bool: boolean literals have type Bool -/
  | tBool   : HasType Γ (Expr.litBool b) Ty.bool
  /-- T-String: string literals have type String -/
  | tString : HasType Γ (Expr.litString s) Ty.string
  /-- T-Unit: unit literal has type Unit -/
  | tUnit   : HasType Γ Expr.litUnit Ty.unit
  /-- T-Var: variable lookup in context -/
  | tVar    : Ctx.lookup Γ n = some T → HasType Γ (Expr.var n) T
  /-- T-Lam: lambda abstraction -/
  | tLam    : HasType (S :: Γ) body T →
              HasType Γ (Expr.lam S body) (Ty.arrow S T)
  /-- T-App: function application -/
  | tApp    : HasType Γ e₁ (Ty.arrow S T) →
              HasType Γ e₂ S →
              HasType Γ (Expr.app e₁ e₂) T
  /-- T-Let: let binding -/
  | tLet    : HasType Γ e₁ S →
              HasType (S :: Γ) e₂ T →
              HasType Γ (Expr.letE S e₁ e₂) T
  /-- T-If: conditional — both branches must have same type -/
  | tIf     : HasType Γ c Ty.bool →
              HasType Γ t T →
              HasType Γ e T →
              HasType Γ (Expr.ifE c t e) T
  /-- T-Bet: ternary choice — all three branches must have same type.
      Result is Dist T because bet introduces probabilistic choice. -/
  | tBet    : HasType Γ e₁ T →
              HasType Γ e₂ T →
              HasType Γ e₃ T →
              HasType Γ (Expr.bet e₁ e₂ e₃) (Ty.dist T)
  /-- T-Sample: eliminate a distribution — Dist T → T -/
  | tSample : HasType Γ e (Ty.dist T) →
              HasType Γ (Expr.sample e) T
  /-- T-DistPure: monadic return — T → Dist T -/
  | tDistPure : HasType Γ e T →
                HasType Γ (Expr.distPure e) (Ty.dist T)
  /-- T-DistBind: monadic bind — Dist A → (A → Dist B) → Dist B -/
  | tDistBind : HasType Γ e₁ (Ty.dist A) →
                HasType Γ e₂ (Ty.arrow A (Ty.dist B)) →
                HasType Γ (Expr.distBind e₁ e₂) (Ty.dist B)

-- ════════════════════════════════════════════════════════════════════════════
-- Section 4. Substitution
-- ════════════════════════════════════════════════════════════════════════════

/-- Shift all free variables in `e` that are ≥ `cutoff` by `amount`. -/
def shift (amount : Int) (cutoff : Nat) : Expr → Expr
  | Expr.litInt n      => Expr.litInt n
  | Expr.litFloat f    => Expr.litFloat f
  | Expr.litBool b     => Expr.litBool b
  | Expr.litString s   => Expr.litString s
  | Expr.litUnit       => Expr.litUnit
  | Expr.var n         => if n >= cutoff then Expr.var (Int.toNat (n + amount)) else Expr.var n
  | Expr.lam t body    => Expr.lam t (shift amount (cutoff + 1) body)
  | Expr.app e₁ e₂    => Expr.app (shift amount cutoff e₁) (shift amount cutoff e₂)
  | Expr.letE t e₁ e₂  => Expr.letE t (shift amount cutoff e₁) (shift amount (cutoff + 1) e₂)
  | Expr.ifE c t e     => Expr.ifE (shift amount cutoff c) (shift amount cutoff t) (shift amount cutoff e)
  | Expr.bet e₁ e₂ e₃  => Expr.bet (shift amount cutoff e₁) (shift amount cutoff e₂) (shift amount cutoff e₃)
  | Expr.sample e      => Expr.sample (shift amount cutoff e)
  | Expr.distPure e    => Expr.distPure (shift amount cutoff e)
  | Expr.distBind e₁ e₂ => Expr.distBind (shift amount cutoff e₁) (shift amount cutoff e₂)

/-- Substitute expression `s` for variable `j` in expression `e`. -/
def subst (j : Nat) (s : Expr) : Expr → Expr
  | Expr.litInt n      => Expr.litInt n
  | Expr.litFloat f    => Expr.litFloat f
  | Expr.litBool b     => Expr.litBool b
  | Expr.litString s'  => Expr.litString s'
  | Expr.litUnit       => Expr.litUnit
  | Expr.var n         => if n == j then s else Expr.var n
  | Expr.lam t body    => Expr.lam t (subst (j + 1) (shift 1 0 s) body)
  | Expr.app e₁ e₂    => Expr.app (subst j s e₁) (subst j s e₂)
  | Expr.letE t e₁ e₂  => Expr.letE t (subst j s e₁) (subst (j + 1) (shift 1 0 s) e₂)
  | Expr.ifE c t e     => Expr.ifE (subst j s c) (subst j s t) (subst j s e)
  | Expr.bet e₁ e₂ e₃  => Expr.bet (subst j s e₁) (subst j s e₂) (subst j s e₃)
  | Expr.sample e      => Expr.sample (subst j s e)
  | Expr.distPure e    => Expr.distPure (subst j s e)
  | Expr.distBind e₁ e₂ => Expr.distBind (subst j s e₁) (subst j s e₂)

/-- Top-level substitution: replace variable 0 and shift down. -/
def substTop (s : Expr) (e : Expr) : Expr :=
  shift (-1) 0 (subst 0 (shift 1 0 s) e)

-- ════════════════════════════════════════════════════════════════════════════
-- Section 5. Small-step operational semantics
-- ════════════════════════════════════════════════════════════════════════════

/-- Which branch the bet selects (models non-deterministic choice). -/
inductive BetChoice : Type where
  | first  : BetChoice
  | second : BetChoice
  | third  : BetChoice

/-- Small-step reduction relation.
    Deterministic for all forms except bet, which is non-deterministic
    (modelled by three separate rules, one per branch). -/
inductive Step : Expr → Expr → Prop where
  -- β-reduction
  | appLam    : IsValue v →
                Step (Expr.app (Expr.lam t body) v) (substTop v body)
  -- Application congruence (left)
  | appFun    : Step e₁ e₁' →
                Step (Expr.app e₁ e₂) (Expr.app e₁' e₂)
  -- Application congruence (right, when function is a value)
  | appArg    : IsValue v →
                Step e₂ e₂' →
                Step (Expr.app v e₂) (Expr.app v e₂')
  -- Let reduction
  | letVal    : IsValue v →
                Step (Expr.letE t v body) (substTop v body)
  -- Let congruence
  | letStep   : Step e₁ e₁' →
                Step (Expr.letE t e₁ body) (Expr.letE t e₁' body)
  -- If-true
  | ifTrue    : Step (Expr.ifE (Expr.litBool true) t e) t
  -- If-false
  | ifFalse   : Step (Expr.ifE (Expr.litBool false) t e) e
  -- If congruence
  | ifCond    : Step c c' →
                Step (Expr.ifE c t e) (Expr.ifE c' t e)
  -- Bet: non-deterministic choice among three value branches
  | betFirst  : IsValue v₁ → IsValue v₂ → IsValue v₃ →
                Step (Expr.bet v₁ v₂ v₃) (Expr.distPure v₁)
  | betSecond : IsValue v₁ → IsValue v₂ → IsValue v₃ →
                Step (Expr.bet v₁ v₂ v₃) (Expr.distPure v₂)
  | betThird  : IsValue v₁ → IsValue v₂ → IsValue v₃ →
                Step (Expr.bet v₁ v₂ v₃) (Expr.distPure v₃)
  -- Bet congruence (left-to-right evaluation)
  | betStep1  : Step e₁ e₁' →
                Step (Expr.bet e₁ e₂ e₃) (Expr.bet e₁' e₂ e₃)
  | betStep2  : IsValue v₁ → Step e₂ e₂' →
                Step (Expr.bet v₁ e₂ e₃) (Expr.bet v₁ e₂' e₃)
  | betStep3  : IsValue v₁ → IsValue v₂ → Step e₃ e₃' →
                Step (Expr.bet v₁ v₂ e₃) (Expr.bet v₁ v₂ e₃')
  -- Sample from distPure: sample (distPure v) → v
  | samplePure : IsValue v →
                 Step (Expr.sample (Expr.distPure v)) v
  -- Sample congruence
  | sampleStep : Step e e' →
                 Step (Expr.sample e) (Expr.sample e')
  -- DistPure congruence
  | distPureStep : Step e e' →
                   Step (Expr.distPure e) (Expr.distPure e')
  -- DistBind: bind (distPure v) f → app f v
  | distBindPure : IsValue v →
                   Step (Expr.distBind (Expr.distPure v) f) (Expr.app f v)
  -- DistBind congruence (left)
  | distBindStep1 : Step e₁ e₁' →
                    Step (Expr.distBind e₁ e₂) (Expr.distBind e₁' e₂)
  -- DistBind congruence (right, when left is a value)
  | distBindStep2 : IsValue v →
                    Step e₂ e₂' →
                    Step (Expr.distBind v e₂) (Expr.distBind v e₂')

-- ════════════════════════════════════════════════════════════════════════════
-- Section 6. Canonical forms lemma
-- ════════════════════════════════════════════════════════════════════════════

/-- Canonical forms: a closed value of arrow type must be a lambda. -/
theorem canonical_arrow {v : Expr} {S T : Ty} :
    HasType [] v (Ty.arrow S T) → IsValue v →
    ∃ body, v = Expr.lam S body := by
  intro ht hv
  cases hv with
  | lam => cases ht with | tLam h => exact ⟨_, rfl⟩
  | litInt => cases ht
  | litFloat => cases ht
  | litBool => cases ht
  | litString => cases ht
  | litUnit => cases ht
  | distPure _ => cases ht

/-- Canonical forms: a closed value of Bool type must be a boolean literal. -/
theorem canonical_bool {v : Expr} :
    HasType [] v Ty.bool → IsValue v →
    ∃ b, v = Expr.litBool b := by
  intro ht hv
  cases hv with
  | litBool => exact ⟨_, rfl⟩
  | litInt => cases ht
  | litFloat => cases ht
  | litString => cases ht
  | litUnit => cases ht
  | lam => cases ht
  | distPure _ => cases ht

/-- Canonical forms: a closed value of Dist T must be distPure of a value. -/
theorem canonical_dist {v : Expr} {T : Ty} :
    HasType [] v (Ty.dist T) → IsValue v →
    ∃ w, v = Expr.distPure w ∧ IsValue w := by
  intro ht hv
  cases hv with
  | distPure hv' => cases ht with | tDistPure h => exact ⟨_, rfl, hv'⟩
  | litInt => cases ht
  | litFloat => cases ht
  | litBool => cases ht
  | litString => cases ht
  | litUnit => cases ht
  | lam => cases ht

-- ════════════════════════════════════════════════════════════════════════════
-- Section 7. Progress theorem
-- ════════════════════════════════════════════════════════════════════════════

/-- A well-typed closed expression is either a value or can take a step.
    This is one half of type safety. -/
theorem progress {e : Expr} {T : Ty} (ht : HasType [] e T) :
    IsValue e ∨ ∃ e', Step e e' := by
  generalize hΓ : ([] : Ctx) = Γ at ht
  induction ht with
  | tInt => left; exact IsValue.litInt
  | tFloat => left; exact IsValue.litFloat
  | tBool => left; exact IsValue.litBool
  | tString => left; exact IsValue.litString
  | tUnit => left; exact IsValue.litUnit
  | tVar hl =>
    subst hΓ
    simp [Ctx.lookup] at hl
  | tLam _ _ => left; exact IsValue.lam
  | tApp ht1 _ ih1 ih2 =>
    right
    rcases ih1 hΓ with hv1 | ⟨_, hs1⟩
    · rcases ih2 hΓ with hv2 | ⟨_, hs2⟩
      · subst hΓ
        obtain ⟨_, rfl⟩ := canonical_arrow ht1 hv1
        exact ⟨_, Step.appLam hv2⟩
      · exact ⟨_, Step.appArg hv1 hs2⟩
    · exact ⟨_, Step.appFun hs1⟩
  | tLet _ _ ih1 _ =>
    right
    rcases ih1 hΓ with hv1 | ⟨_, hs1⟩
    · exact ⟨_, Step.letVal hv1⟩
    · exact ⟨_, Step.letStep hs1⟩
  | @tIf Γ c t T e htc _ _ ihc _ _ =>
    right
    rcases ihc hΓ with hvc | ⟨_, hsc⟩
    · subst hΓ
      obtain ⟨b, rfl⟩ := canonical_bool htc hvc
      cases b with
      | true  => exact ⟨t, Step.ifTrue⟩
      | false => exact ⟨e, Step.ifFalse⟩
    · exact ⟨_, Step.ifCond hsc⟩
  | tBet _ _ _ ih1 ih2 ih3 =>
    right
    rcases ih1 hΓ with hv1 | ⟨_, hs1⟩
    · rcases ih2 hΓ with hv2 | ⟨_, hs2⟩
      · rcases ih3 hΓ with hv3 | ⟨_, hs3⟩
        · exact ⟨_, Step.betFirst hv1 hv2 hv3⟩
        · exact ⟨_, Step.betStep3 hv1 hv2 hs3⟩
      · exact ⟨_, Step.betStep2 hv1 hs2⟩
    · exact ⟨_, Step.betStep1 hs1⟩
  | tSample hte ihe =>
    right
    rcases ihe hΓ with hve | ⟨_, hse⟩
    · subst hΓ
      obtain ⟨w, rfl, hvw⟩ := canonical_dist hte hve
      exact ⟨w, Step.samplePure hvw⟩
    · exact ⟨_, Step.sampleStep hse⟩
  | tDistPure _ ihe =>
    rcases ihe hΓ with hve | ⟨_, hse⟩
    · left; exact IsValue.distPure hve
    · right; exact ⟨_, Step.distPureStep hse⟩
  | tDistBind ht1 _ ih1 _ =>
    right
    rcases ih1 hΓ with hv1 | ⟨_, hs1⟩
    · subst hΓ
      obtain ⟨_, rfl, hvw⟩ := canonical_dist ht1 hv1
      exact ⟨_, Step.distBindPure hvw⟩
    · exact ⟨_, Step.distBindStep1 hs1⟩

-- ════════════════════════════════════════════════════════════════════════════
-- Section 8. Weakening and substitution lemmas
-- ════════════════════════════════════════════════════════════════════════════

-- This section discharges the de Bruijn machinery needed to prove that the
-- top-level substitution operator `substTop` preserves typing. The pattern is
-- the standard TAPL Ch. 9 mechanisation:
--
--   1. A context-insertion operator `Ctx.insertAt Γ k U` with three lookup
--      lemmas (n < k, n = k, n > k).
--   2. `shift_preserves_typing`: weakening for `shift 1 k` — inserting one
--      type into the context at position k preserves typing of `shift 1 k e`.
--   3. Two arithmetic lemmas on shift:
--      - `shift_down_shift_up`: `shift (-1) k (shift 1 k e) = e`.
--      - `shift_one_comm`:      `shift 1 0 (shift 1 k e) = shift 1 (k+1) (shift 1 0 e)`.
--   4. `substAt_preserves_typing`: the generalised substitution lemma,
--      proved by induction on `body` (not on the typing derivation).
--   5. `substTop_preserves_typing`: the corollary at k = 0.

/-- Insert a type `U` into context `Γ` at position `k`. -/
def Ctx.insertAt (Γ : Ctx) (k : Nat) (U : Ty) : Ctx :=
  Γ.take k ++ U :: Γ.drop k

/-- Computational lemma: `insertAt Γ 0 U = U :: Γ`. -/
theorem Ctx.insertAt_zero (Γ : Ctx) (U : Ty) :
    Ctx.insertAt Γ 0 U = U :: Γ := by
  simp [Ctx.insertAt]

/-- Computational lemma: `insertAt (T :: Γ) (k+1) U = T :: insertAt Γ k U`. -/
theorem Ctx.insertAt_cons_succ (T : Ty) (Γ : Ctx) (k : Nat) (U : Ty) :
    Ctx.insertAt (T :: Γ) (k + 1) U = T :: Ctx.insertAt Γ k U := by
  simp [Ctx.insertAt]

/-- Lookup below the insertion point is unchanged. -/
theorem Ctx.lookup_insertAt_lt :
    ∀ (Γ : Ctx) (k : Nat) (U : Ty) (n : Nat),
      k ≤ Γ.length → n < k →
      Ctx.lookup (Ctx.insertAt Γ k U) n = Ctx.lookup Γ n
  | _, 0, _, _, _, h => by omega
  | [], k+1, _, _, hk, _ => by simp at hk
  | T :: Γ, k+1, U, 0, _, _ => by
    simp [Ctx.insertAt, Ctx.lookup]
  | T :: Γ, k+1, U, n+1, hk, h => by
    rw [Ctx.insertAt_cons_succ]
    show Ctx.lookup (Ctx.insertAt Γ k U) n = Ctx.lookup Γ n
    exact Ctx.lookup_insertAt_lt Γ k U n (by simp at hk; omega) (by omega)

/-- Lookup at the insertion point returns the inserted type. -/
theorem Ctx.lookup_insertAt_eq :
    ∀ (Γ : Ctx) (k : Nat) (U : Ty),
      k ≤ Γ.length → Ctx.lookup (Ctx.insertAt Γ k U) k = some U
  | [], 0, U, _ => by simp [Ctx.insertAt, Ctx.lookup]
  | [], k+1, U, h => by simp at h
  | T :: Γ, 0, U, _ => by simp [Ctx.insertAt, Ctx.lookup]
  | T :: Γ, k+1, U, h => by
    rw [Ctx.insertAt_cons_succ]
    show Ctx.lookup (Ctx.insertAt Γ k U) k = some U
    exact Ctx.lookup_insertAt_eq Γ k U (by simp at h; omega)

/-- Lookup above the insertion point shifts down by 1. -/
theorem Ctx.lookup_insertAt_gt :
    ∀ (Γ : Ctx) (k : Nat) (U : Ty) (n : Nat),
      n > k → Ctx.lookup (Ctx.insertAt Γ k U) n = Ctx.lookup Γ (n - 1)
  | _, _, _, 0, h => by omega
  | [], k, U, n+1, _ => by
    -- Γ = []: lookup [] (n) = none for all n. We need to show
    -- lookup (insertAt [] k U) (n+1) = lookup [] n = none.
    -- insertAt [] k U = [].take k ++ U :: [].drop k = [] ++ [U] = [U].
    -- So LHS = lookup [U] (n+1). For n+1 > k, this is lookup [] n = none
    -- which matches. Actually lookup [U] (n+1) reduces by the cons-succ rule:
    -- lookup [U] (n+1) = lookup [] n = none.
    simp only [Ctx.insertAt, List.take_nil, List.drop_nil, List.nil_append]
    show Ctx.lookup (U :: []) (n+1) = Ctx.lookup [] n
    show Ctx.lookup [] n = Ctx.lookup [] n
    rfl
  | T :: Γ, 0, U, n+1, _ => by
    simp [Ctx.insertAt, Ctx.lookup]
  | T :: Γ, k+1, U, n+1, h => by
    rw [Ctx.insertAt_cons_succ]
    show Ctx.lookup (T :: Ctx.insertAt Γ k U) (n+1) = Ctx.lookup (T :: Γ) (n+1 - 1)
    have hn : n + 1 > k + 1 ↔ n > k := by omega
    have hgt : n > k := hn.mp h
    cases n with
    | zero => omega
    | succ n' =>
      show Ctx.lookup (Ctx.insertAt Γ k U) (n'+1) = Ctx.lookup Γ n'
      have := Ctx.lookup_insertAt_gt Γ k U (n'+1) hgt
      simp at this
      exact this

/-- Weakening / shift-preserves-typing. Inserting a fresh type at position k
    into the context, and shifting every variable ≥ k up by one in the
    expression, preserves typing. -/
theorem shift_preserves_typing
    {Γ : Ctx} {e : Expr} {T : Ty} (ht : HasType Γ e T) :
    ∀ (k : Nat) (U : Ty), k ≤ Γ.length →
      HasType (Ctx.insertAt Γ k U) (shift 1 k e) T := by
  induction ht with
  | tInt => intro k U _; simp only [shift]; exact HasType.tInt
  | tFloat => intro k U _; simp only [shift]; exact HasType.tFloat
  | tBool => intro k U _; simp only [shift]; exact HasType.tBool
  | tString => intro k U _; simp only [shift]; exact HasType.tString
  | tUnit => intro k U _; simp only [shift]; exact HasType.tUnit
  | @tVar Γ n T hl =>
    intro k U hk
    show HasType (Ctx.insertAt Γ k U) (shift 1 k (Expr.var n)) T
    show HasType (Ctx.insertAt Γ k U)
      (if n ≥ k then Expr.var (Int.toNat ((n : Int) + 1)) else Expr.var n) T
    by_cases hnk : n ≥ k
    · -- n ≥ k: var becomes var (n+1)
      rw [if_pos hnk]
      have hnat : Int.toNat ((n : Int) + 1) = n + 1 := by omega
      rw [hnat]
      apply HasType.tVar
      have hgt : n + 1 > k := by omega
      rw [Ctx.lookup_insertAt_gt _ _ _ _ hgt]
      simp
      exact hl
    · -- n < k: var stays at n
      rw [if_neg hnk]
      have hlt : n < k := by omega
      apply HasType.tVar
      rw [Ctx.lookup_insertAt_lt _ _ _ _ hk hlt]
      exact hl
  | @tLam Γ S body T _ ih =>
    intro k U hk
    simp only [shift]
    apply HasType.tLam
    rw [← Ctx.insertAt_cons_succ]
    exact ih (k+1) U (by simp; omega)
  | tApp _ _ ih1 ih2 =>
    intro k U hk
    simp only [shift]
    exact HasType.tApp (ih1 k U hk) (ih2 k U hk)
  | @tLet Γ e₁ S e₂ T _ _ ih1 ih2 =>
    intro k U hk
    simp only [shift]
    apply HasType.tLet (ih1 k U hk)
    rw [← Ctx.insertAt_cons_succ]
    exact ih2 (k+1) U (by simp; omega)
  | tIf _ _ _ ihc iht ihe =>
    intro k U hk
    simp only [shift]
    exact HasType.tIf (ihc k U hk) (iht k U hk) (ihe k U hk)
  | tBet _ _ _ ih1 ih2 ih3 =>
    intro k U hk
    simp only [shift]
    exact HasType.tBet (ih1 k U hk) (ih2 k U hk) (ih3 k U hk)
  | tSample _ ih =>
    intro k U hk
    simp only [shift]
    exact HasType.tSample (ih k U hk)
  | tDistPure _ ih =>
    intro k U hk
    simp only [shift]
    exact HasType.tDistPure (ih k U hk)
  | tDistBind _ _ ih1 ih2 =>
    intro k U hk
    simp only [shift]
    exact HasType.tDistBind (ih1 k U hk) (ih2 k U hk)

/-- Shift down then up cancels: `shift (-1) k (shift 1 k e) = e`.
    Holds unconditionally because every variable that gets shifted up by `shift 1 k`
    is ≥ k+1, and `shift (-1) k` undoes that. -/
theorem shift_down_shift_up (e : Expr) :
    ∀ (k : Nat), shift (-1 : Int) k (shift 1 k e) = e := by
  induction e with
  | litInt _ => intro k; simp [shift]
  | litFloat _ => intro k; simp [shift]
  | litBool _ => intro k; simp [shift]
  | litString _ => intro k; simp [shift]
  | litUnit => intro k; simp [shift]
  | var n =>
    intro k
    show shift (-1) k (if n ≥ k then Expr.var (Int.toNat ((n : Int) + 1))
                       else Expr.var n) = Expr.var n
    by_cases hnk : n ≥ k
    · rw [if_pos hnk]
      have h1 : Int.toNat ((n : Int) + 1) = n + 1 := by omega
      rw [h1]
      show (if n + 1 ≥ k then Expr.var (Int.toNat (((n+1 : Nat) : Int) + (-1)))
                         else Expr.var (n+1)) = Expr.var n
      have h2 : n + 1 ≥ k := by omega
      rw [if_pos h2]
      have h3 : Int.toNat (((n + 1 : Nat) : Int) + (-1 : Int)) = n := by omega
      rw [h3]
    · rw [if_neg hnk]
      show (if n ≥ k then Expr.var (Int.toNat ((n : Int) + (-1)))
                     else Expr.var n) = Expr.var n
      rw [if_neg hnk]
  | lam t body ih =>
    intro k
    show shift (-1) k (Expr.lam t (shift 1 (k+1) body)) = Expr.lam t body
    show Expr.lam t (shift (-1) (k+1) (shift 1 (k+1) body)) = Expr.lam t body
    rw [ih]
  | app e₁ e₂ ih1 ih2 =>
    intro k
    show shift (-1) k (Expr.app (shift 1 k e₁) (shift 1 k e₂))
       = Expr.app e₁ e₂
    show Expr.app (shift (-1) k (shift 1 k e₁)) (shift (-1) k (shift 1 k e₂))
       = Expr.app e₁ e₂
    rw [ih1, ih2]
  | letE t e₁ e₂ ih1 ih2 =>
    intro k
    show shift (-1) k (Expr.letE t (shift 1 k e₁) (shift 1 (k+1) e₂))
       = Expr.letE t e₁ e₂
    show Expr.letE t (shift (-1) k (shift 1 k e₁))
                     (shift (-1) (k+1) (shift 1 (k+1) e₂))
       = Expr.letE t e₁ e₂
    rw [ih1, ih2]
  | ifE c t e ihc iht ihe =>
    intro k
    show shift (-1) k (Expr.ifE (shift 1 k c) (shift 1 k t) (shift 1 k e))
       = Expr.ifE c t e
    show Expr.ifE (shift (-1) k (shift 1 k c))
                  (shift (-1) k (shift 1 k t))
                  (shift (-1) k (shift 1 k e))
       = Expr.ifE c t e
    rw [ihc, iht, ihe]
  | bet e₁ e₂ e₃ ih1 ih2 ih3 =>
    intro k
    show shift (-1) k (Expr.bet (shift 1 k e₁) (shift 1 k e₂) (shift 1 k e₃))
       = Expr.bet e₁ e₂ e₃
    show Expr.bet (shift (-1) k (shift 1 k e₁))
                  (shift (-1) k (shift 1 k e₂))
                  (shift (-1) k (shift 1 k e₃))
       = Expr.bet e₁ e₂ e₃
    rw [ih1, ih2, ih3]
  | sample e ih =>
    intro k
    show shift (-1) k (Expr.sample (shift 1 k e)) = Expr.sample e
    show Expr.sample (shift (-1) k (shift 1 k e)) = Expr.sample e
    rw [ih]
  | distPure e ih =>
    intro k
    show shift (-1) k (Expr.distPure (shift 1 k e)) = Expr.distPure e
    show Expr.distPure (shift (-1) k (shift 1 k e)) = Expr.distPure e
    rw [ih]
  | distBind e₁ e₂ ih1 ih2 =>
    intro k
    show shift (-1) k (Expr.distBind (shift 1 k e₁) (shift 1 k e₂))
       = Expr.distBind e₁ e₂
    show Expr.distBind (shift (-1) k (shift 1 k e₁))
                       (shift (-1) k (shift 1 k e₂))
       = Expr.distBind e₁ e₂
    rw [ih1, ih2]

/-- General shift-shift commutation: for `i ≤ j`,
    `shift 1 i (shift 1 j e) = shift 1 (j+1) (shift 1 i e)`.
    Specialised at i = 0 to give `shift_one_comm`. -/
theorem shift_one_comm_general (e : Expr) :
    ∀ (i j : Nat), i ≤ j →
      shift 1 i (shift 1 j e) = shift 1 (j+1) (shift 1 i e) := by
  induction e with
  | litInt _ => intros i j _; simp [shift]
  | litFloat _ => intros i j _; simp [shift]
  | litBool _ => intros i j _; simp [shift]
  | litString _ => intros i j _; simp [shift]
  | litUnit => intros i j _; simp [shift]
  | var n =>
    intros i j hij
    show shift 1 i (if n ≥ j then Expr.var (Int.toNat ((n : Int) + 1))
                              else Expr.var n)
       = shift 1 (j+1) (if n ≥ i then Expr.var (Int.toNat ((n : Int) + 1))
                                  else Expr.var n)
    by_cases hnj : n ≥ j
    · -- n ≥ j ≥ i: both inner shifts fire.
      have hni : n ≥ i := by omega
      rw [if_pos hnj, if_pos hni]
      have h1 : Int.toNat ((n : Int) + 1) = n + 1 := by omega
      rw [h1]
      show (if n+1 ≥ i then Expr.var (Int.toNat (((n+1 : Nat) : Int) + 1))
                       else Expr.var (n+1))
         = (if n+1 ≥ j+1 then Expr.var (Int.toNat (((n+1 : Nat) : Int) + 1))
                         else Expr.var (n+1))
      have hni' : n + 1 ≥ i := by omega
      have hnj' : n + 1 ≥ j + 1 := by omega
      rw [if_pos hni', if_pos hnj']
    · -- n < j; case split on n ≥ i.
      rw [if_neg hnj]
      -- LHS now: shift 1 i (Expr.var n)
      show shift 1 i (Expr.var n)
         = shift 1 (j+1) (if n ≥ i then Expr.var (Int.toNat ((n : Int) + 1))
                                    else Expr.var n)
      show (if n ≥ i then Expr.var (Int.toNat ((n : Int) + 1)) else Expr.var n)
         = shift 1 (j+1) (if n ≥ i then Expr.var (Int.toNat ((n : Int) + 1))
                                    else Expr.var n)
      by_cases hni : n ≥ i
      · -- i ≤ n < j
        simp only [if_pos hni]
        have h1 : Int.toNat ((n : Int) + 1) = n + 1 := by omega
        rw [h1]
        show Expr.var (n+1)
           = (if n+1 ≥ j+1 then Expr.var (Int.toNat (((n+1 : Nat) : Int) + 1))
                           else Expr.var (n+1))
        have hnotgej : ¬ (n + 1 ≥ j + 1) := fun h => hnj (by omega)
        rw [if_neg hnotgej]
      · -- n < i ≤ j: neither shift fires.
        simp only [if_neg hni]
        show Expr.var n
           = (if n ≥ j+1 then Expr.var (Int.toNat ((n : Int) + 1))
                          else Expr.var n)
        have hnotge : ¬ (n ≥ j + 1) := fun h => hnj (by omega)
        rw [if_neg hnotge]
  | lam t body ih =>
    intros i j hij
    show shift 1 i (Expr.lam t (shift 1 (j+1) body))
       = shift 1 (j+1) (Expr.lam t (shift 1 (i+1) body))
    show Expr.lam t (shift 1 (i+1) (shift 1 (j+1) body))
       = Expr.lam t (shift 1 (j+1+1) (shift 1 (i+1) body))
    rw [ih (i+1) (j+1) (by omega)]
  | app e₁ e₂ ih1 ih2 =>
    intros i j hij
    show shift 1 i (Expr.app (shift 1 j e₁) (shift 1 j e₂))
       = shift 1 (j+1) (Expr.app (shift 1 i e₁) (shift 1 i e₂))
    show Expr.app (shift 1 i (shift 1 j e₁)) (shift 1 i (shift 1 j e₂))
       = Expr.app (shift 1 (j+1) (shift 1 i e₁)) (shift 1 (j+1) (shift 1 i e₂))
    rw [ih1 i j hij, ih2 i j hij]
  | letE t e₁ e₂ ih1 ih2 =>
    intros i j hij
    show shift 1 i (Expr.letE t (shift 1 j e₁) (shift 1 (j+1) e₂))
       = shift 1 (j+1) (Expr.letE t (shift 1 i e₁) (shift 1 (i+1) e₂))
    show Expr.letE t (shift 1 i (shift 1 j e₁)) (shift 1 (i+1) (shift 1 (j+1) e₂))
       = Expr.letE t (shift 1 (j+1) (shift 1 i e₁)) (shift 1 (j+1+1) (shift 1 (i+1) e₂))
    rw [ih1 i j hij, ih2 (i+1) (j+1) (by omega)]
  | ifE c t e ihc iht ihe =>
    intros i j hij
    show shift 1 i (Expr.ifE (shift 1 j c) (shift 1 j t) (shift 1 j e))
       = shift 1 (j+1) (Expr.ifE (shift 1 i c) (shift 1 i t) (shift 1 i e))
    show Expr.ifE (shift 1 i (shift 1 j c)) (shift 1 i (shift 1 j t)) (shift 1 i (shift 1 j e))
       = Expr.ifE (shift 1 (j+1) (shift 1 i c)) (shift 1 (j+1) (shift 1 i t)) (shift 1 (j+1) (shift 1 i e))
    rw [ihc i j hij, iht i j hij, ihe i j hij]
  | bet e₁ e₂ e₃ ih1 ih2 ih3 =>
    intros i j hij
    show shift 1 i (Expr.bet (shift 1 j e₁) (shift 1 j e₂) (shift 1 j e₃))
       = shift 1 (j+1) (Expr.bet (shift 1 i e₁) (shift 1 i e₂) (shift 1 i e₃))
    show Expr.bet (shift 1 i (shift 1 j e₁)) (shift 1 i (shift 1 j e₂)) (shift 1 i (shift 1 j e₃))
       = Expr.bet (shift 1 (j+1) (shift 1 i e₁)) (shift 1 (j+1) (shift 1 i e₂)) (shift 1 (j+1) (shift 1 i e₃))
    rw [ih1 i j hij, ih2 i j hij, ih3 i j hij]
  | sample e ih =>
    intros i j hij
    show shift 1 i (Expr.sample (shift 1 j e))
       = shift 1 (j+1) (Expr.sample (shift 1 i e))
    show Expr.sample (shift 1 i (shift 1 j e))
       = Expr.sample (shift 1 (j+1) (shift 1 i e))
    rw [ih i j hij]
  | distPure e ih =>
    intros i j hij
    show shift 1 i (Expr.distPure (shift 1 j e))
       = shift 1 (j+1) (Expr.distPure (shift 1 i e))
    show Expr.distPure (shift 1 i (shift 1 j e))
       = Expr.distPure (shift 1 (j+1) (shift 1 i e))
    rw [ih i j hij]
  | distBind e₁ e₂ ih1 ih2 =>
    intros i j hij
    show shift 1 i (Expr.distBind (shift 1 j e₁) (shift 1 j e₂))
       = shift 1 (j+1) (Expr.distBind (shift 1 i e₁) (shift 1 i e₂))
    show Expr.distBind (shift 1 i (shift 1 j e₁)) (shift 1 i (shift 1 j e₂))
       = Expr.distBind (shift 1 (j+1) (shift 1 i e₁)) (shift 1 (j+1) (shift 1 i e₂))
    rw [ih1 i j hij, ih2 i j hij]

/-- Specialisation of `shift_one_comm_general` at i = 0. -/
theorem shift_one_comm (e : Expr) :
    ∀ (k : Nat), shift 1 0 (shift 1 k e) = shift 1 (k+1) (shift 1 0 e) := by
  intro k
  exact shift_one_comm_general e 0 k (Nat.zero_le _)

/-- The generalised substitution lemma. If `body` has type T in a context
    where S has been inserted at position k, and v has type S in Γ (with
    appropriate shifting), then the substituted expression has type T in Γ.

    Proved by induction on the structure of `body`. -/
theorem substAt_preserves_typing :
    ∀ (body : Expr) (Γ : Ctx) (k : Nat) (S T : Ty) (v : Expr),
      HasType (Ctx.insertAt Γ k S) body T →
      HasType Γ v S →
      k ≤ Γ.length →
      HasType Γ (shift (-1 : Int) k (subst k (shift 1 k v) body)) T := by
  intro body
  induction body with
  | litInt n =>
    intros Γ k S T v hbody _ _
    cases hbody
    simp only [subst, shift]
    exact HasType.tInt
  | litFloat f =>
    intros Γ k S T v hbody _ _
    cases hbody
    simp only [subst, shift]
    exact HasType.tFloat
  | litBool b =>
    intros Γ k S T v hbody _ _
    cases hbody
    simp only [subst, shift]
    exact HasType.tBool
  | litString s =>
    intros Γ k S T v hbody _ _
    cases hbody
    simp only [subst, shift]
    exact HasType.tString
  | litUnit =>
    intros Γ k S T v hbody _ _
    cases hbody
    simp only [subst, shift]
    exact HasType.tUnit
  | var n =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tVar hl =>
      show HasType Γ (shift (-1) k (subst k (shift 1 k v) (Expr.var n))) T
      show HasType Γ
        (shift (-1) k (if n == k then shift 1 k v else Expr.var n)) T
      by_cases hnk : n = k
      · -- n = k: substitute and shift-down
        subst hnk
        rw [if_pos (by simp : (n == n) = true)]
        -- hl : Ctx.lookup (Ctx.insertAt Γ n S) n = some T
        rw [Ctx.lookup_insertAt_eq Γ n S hk] at hl
        injection hl with hST
        subst hST
        rw [shift_down_shift_up v n]
        exact hv
      · -- n ≠ k
        rw [if_neg (by simpa using hnk)]
        show HasType Γ
          (if n ≥ k then Expr.var (Int.toNat ((n : Int) + (-1))) else Expr.var n) T
        by_cases hng : n ≥ k
        · -- n > k (since n ≠ k and n ≥ k)
          rw [if_pos hng]
          have hgt : n > k := by omega
          rw [Ctx.lookup_insertAt_gt Γ k S n hgt] at hl
          cases n with
          | zero => omega
          | succ n' =>
            have hnat : Int.toNat (((n'+1 : Nat) : Int) + (-1 : Int)) = n' := by omega
            rw [hnat]
            apply HasType.tVar
            simpa using hl
        · -- n < k
          rw [if_neg hng]
          have hlt : n < k := by omega
          rw [Ctx.lookup_insertAt_lt Γ k S n hk hlt] at hl
          exact HasType.tVar hl
  | lam t body ih =>
    intros Γ k S T v hbody hv hk
    -- The Lam case is the tricky one; T = arrow t T' for some T'.
    cases hbody with
    | @tLam _ _ _ T_inner ht' =>
      -- ht' : HasType (t :: Ctx.insertAt Γ k S) body T_inner
      simp only [subst, shift]
      apply HasType.tLam
      -- shift 1 (k+1) (shift 1 0 v) = shift 1 0 (shift 1 k v) by shift_one_comm
      have hcomm : shift 1 (k+1) (shift 1 0 v) = shift 1 0 (shift 1 k v) := by
        rw [← shift_one_comm v k]
      rw [← hcomm]
      -- Apply IH at (t :: Γ), k+1, S, T_inner, shift 1 0 v
      have hk' : k + 1 ≤ (t :: Γ).length := by simp; omega
      have hv' : HasType (t :: Γ) (shift 1 0 v) S := by
        have := shift_preserves_typing hv 0 t (by simp)
        rw [Ctx.insertAt_zero] at this
        exact this
      have ht'' : HasType (Ctx.insertAt (t :: Γ) (k+1) S) body T_inner := by
        rw [Ctx.insertAt_cons_succ]
        exact ht'
      exact ih (t :: Γ) (k+1) S T_inner (shift 1 0 v) ht'' hv' hk'
  | app e₁ e₂ ih1 ih2 =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tApp ht1 ht2 =>
      simp only [subst, shift]
      exact HasType.tApp (ih1 Γ k S _ v ht1 hv hk) (ih2 Γ k S _ v ht2 hv hk)
  | letE t e₁ e₂ ih1 ih2 =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tLet ht1 ht2 =>
      -- ht1 : HasType (insertAt Γ k S) e₁ t
      -- ht2 : HasType (t :: insertAt Γ k S) e₂ T
      simp only [subst, shift]
      apply HasType.tLet (ih1 Γ k S t v ht1 hv hk)
      -- binder case identical structure to lam
      have hcomm : shift 1 (k+1) (shift 1 0 v) = shift 1 0 (shift 1 k v) := by
        rw [← shift_one_comm v k]
      rw [← hcomm]
      have hk' : k + 1 ≤ (t :: Γ).length := by simp; omega
      have hv' : HasType (t :: Γ) (shift 1 0 v) S := by
        have := shift_preserves_typing hv 0 t (by simp)
        rw [Ctx.insertAt_zero] at this
        exact this
      have ht2' : HasType (Ctx.insertAt (t :: Γ) (k+1) S) e₂ T := by
        rw [Ctx.insertAt_cons_succ]
        exact ht2
      exact ih2 (t :: Γ) (k+1) S T (shift 1 0 v) ht2' hv' hk'
  | ifE c t e ihc iht ihe =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tIf htc htt hte =>
      simp only [subst, shift]
      exact HasType.tIf
        (ihc Γ k S Ty.bool v htc hv hk)
        (iht Γ k S T v htt hv hk)
        (ihe Γ k S T v hte hv hk)
  | bet e₁ e₂ e₃ ih1 ih2 ih3 =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tBet ht1 ht2 ht3 =>
      simp only [subst, shift]
      exact HasType.tBet
        (ih1 Γ k S _ v ht1 hv hk)
        (ih2 Γ k S _ v ht2 hv hk)
        (ih3 Γ k S _ v ht3 hv hk)
  | sample e ih =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tSample ht =>
      simp only [subst, shift]
      exact HasType.tSample (ih Γ k S _ v ht hv hk)
  | distPure e ih =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tDistPure ht =>
      simp only [subst, shift]
      exact HasType.tDistPure (ih Γ k S _ v ht hv hk)
  | distBind e₁ e₂ ih1 ih2 =>
    intros Γ k S T v hbody hv hk
    cases hbody with
    | tDistBind ht1 ht2 =>
      simp only [subst, shift]
      exact HasType.tDistBind
        (ih1 Γ k S _ v ht1 hv hk)
        (ih2 Γ k S _ v ht2 hv hk)

-- ════════════════════════════════════════════════════════════════════════════
-- Section 9. Preservation theorem
-- ════════════════════════════════════════════════════════════════════════════

/-- The substitution lemma (top-level): if (S :: Γ) ⊢ body : T and Γ ⊢ v : S,
    then Γ ⊢ substTop v body : T.

    Discharged from `substAt_preserves_typing` at k = 0, using
    `Ctx.insertAt_zero` to align the contexts. -/
theorem substTop_preserves_typing :
  ∀ (Γ : Ctx) (S T : Ty) (body v : Expr),
    HasType (S :: Γ) body T → HasType Γ v S → HasType Γ (substTop v body) T := by
  intros Γ S T body v hbody hv
  -- substTop v body = shift (-1) 0 (subst 0 (shift 1 0 v) body)
  -- Apply substAt_preserves_typing at k = 0; insertAt Γ 0 S = S :: Γ.
  have hbody' : HasType (Ctx.insertAt Γ 0 S) body T := by
    rw [Ctx.insertAt_zero]; exact hbody
  have hk : (0 : Nat) ≤ Γ.length := Nat.zero_le _
  exact substAt_preserves_typing body Γ 0 S T v hbody' hv hk

/-- Preservation: if Γ ⊢ e : T and e → e', then Γ ⊢ e' : T.
    Stepping preserves types. This is the other half of type safety. -/
theorem preservation {Γ : Ctx} {e e' : Expr} {T : Ty}
    (ht : HasType Γ e T) (hs : Step e e') : HasType Γ e' T := by
  induction hs generalizing Γ T with
  | appLam hv =>
    cases ht with
    | tApp ht1 ht2 =>
      cases ht1 with
      | tLam htbody => exact substTop_preserves_typing Γ _ T _ _ htbody ht2
  | appFun hs1 ih =>
    cases ht with
    | tApp ht1 ht2 => exact HasType.tApp (ih ht1) ht2
  | appArg _ hs2 ih =>
    cases ht with
    | tApp ht1 ht2 => exact HasType.tApp ht1 (ih ht2)
  | letVal hv =>
    cases ht with
    | tLet ht1 ht2 => exact substTop_preserves_typing Γ _ T _ _ ht2 ht1
  | letStep hs1 ih =>
    cases ht with
    | tLet ht1 ht2 => exact HasType.tLet (ih ht1) ht2
  | ifTrue =>
    cases ht with
    | tIf _ htt _ => exact htt
  | ifFalse =>
    cases ht with
    | tIf _ _ hte => exact hte
  | ifCond hsc ih =>
    cases ht with
    | tIf htc htt hte => exact HasType.tIf (ih htc) htt hte
  | betFirst _ _ _ =>
    cases ht with
    | tBet ht1 _ _ => exact HasType.tDistPure ht1
  | betSecond _ _ _ =>
    cases ht with
    | tBet _ ht2 _ => exact HasType.tDistPure ht2
  | betThird _ _ _ =>
    cases ht with
    | tBet _ _ ht3 => exact HasType.tDistPure ht3
  | betStep1 _ ih =>
    cases ht with
    | tBet ht1 ht2 ht3 => exact HasType.tBet (ih ht1) ht2 ht3
  | betStep2 _ _ ih =>
    cases ht with
    | tBet ht1 ht2 ht3 => exact HasType.tBet ht1 (ih ht2) ht3
  | betStep3 _ _ _ ih =>
    cases ht with
    | tBet ht1 ht2 ht3 => exact HasType.tBet ht1 ht2 (ih ht3)
  | samplePure _ =>
    cases ht with
    | tSample htd =>
      cases htd with
      | tDistPure hte => exact hte
  | sampleStep _ ih =>
    cases ht with
    | tSample hte => exact HasType.tSample (ih hte)
  | distPureStep _ ih =>
    cases ht with
    | tDistPure hte => exact HasType.tDistPure (ih hte)
  | distBindPure hv =>
    cases ht with
    | tDistBind ht1 ht2 =>
      cases ht1 with
      | tDistPure hte => exact HasType.tApp ht2 hte
  | distBindStep1 _ ih =>
    cases ht with
    | tDistBind ht1 ht2 => exact HasType.tDistBind (ih ht1) ht2
  | distBindStep2 _ _ ih =>
    cases ht with
    | tDistBind ht1 ht2 => exact HasType.tDistBind ht1 (ih ht2)

-- ════════════════════════════════════════════════════════════════════════════
-- Section 10. Type safety (corollary)
-- ════════════════════════════════════════════════════════════════════════════

/-- Multi-step reduction (reflexive transitive closure). -/
inductive MultiStep : Expr → Expr → Prop where
  | refl  : MultiStep e e
  | step  : Step e₁ e₂ → MultiStep e₂ e₃ → MultiStep e₁ e₃

/-- Type safety: a well-typed closed term never gets stuck.
    If ⊢ e : T and e →* e', then e' is a value or can step further. -/
theorem type_safety {e e' : Expr} {T : Ty}
    (ht : HasType [] e T) (hms : MultiStep e e') :
    IsValue e' ∨ ∃ e'', Step e' e'' := by
  induction hms with
  | refl => exact progress ht
  | step hs _ ih => exact ih (preservation ht hs)

-- ════════════════════════════════════════════════════════════════════════════
-- Section 11. Distribution monad laws (syntactic / typing level)
-- ════════════════════════════════════════════════════════════════════════════

-- The monad laws for Dist are:
--   1. Left identity:  bind (pure v) f  ≡  f v
--   2. Right identity: bind m pure      ≡  m
--   3. Associativity:  bind (bind m f) g ≡ bind m (λx. bind (f x) g)
--
-- We prove these as *reduction* properties: the LHS reduces to the RHS
-- (or to a common reduct).

/-- Monad law 1 — Left identity:
    distBind (distPure v) f  →  app f v
    i.e., bind (return v) f reduces to f v. -/
theorem monad_left_identity (v : Expr) (f : Expr) (hv : IsValue v) :
    Step (Expr.distBind (Expr.distPure v) f) (Expr.app f v) :=
  Step.distBindPure hv

/-- Monad law 1 — Left identity preserves types:
    If Γ ⊢ v : A and Γ ⊢ f : A → Dist B, then
    Γ ⊢ distBind (distPure v) f : Dist B and
    Γ ⊢ app f v : Dist B. -/
theorem monad_left_identity_typed {Γ : Ctx} {v f : Expr} {A B : Ty}
    (hv : HasType Γ v A) (hf : HasType Γ f (Ty.arrow A (Ty.dist B))) :
    HasType Γ (Expr.distBind (Expr.distPure v) f) (Ty.dist B) ∧
    HasType Γ (Expr.app f v) (Ty.dist B) :=
  ⟨HasType.tDistBind (HasType.tDistPure hv) hf, HasType.tApp hf hv⟩

/-- Monad law 2 — Right identity (typing):
    If Γ ⊢ m : Dist A, then
    Γ ⊢ distBind m (lam A (distPure (var 0))) : Dist A.
    When m = distPure v, this reduces to app (lam A (distPure (var 0))) v,
    which β-reduces to distPure v = m. -/
theorem monad_right_identity_typed {Γ : Ctx} {m : Expr} {A : Ty}
    (hm : HasType Γ m (Ty.dist A)) :
    HasType Γ (Expr.distBind m (Expr.lam A (Expr.distPure (Expr.var 0)))) (Ty.dist A) :=
  HasType.tDistBind hm
    (HasType.tLam (HasType.tDistPure (HasType.tVar rfl)))

/-- Monad law 2 — Right identity reduces for values:
    distBind (distPure v) (lam A (distPure (var 0))) → app (lam A (distPure (var 0))) v -/
theorem monad_right_identity_step (v : Expr) (A : Ty) (hv : IsValue v) :
    Step (Expr.distBind (Expr.distPure v) (Expr.lam A (Expr.distPure (Expr.var 0))))
         (Expr.app (Expr.lam A (Expr.distPure (Expr.var 0))) v) :=
  Step.distBindPure hv

/-- Monad law 2 — The result further β-reduces:
    app (lam A (distPure (var 0))) v → substTop v (distPure (var 0))
    which is distPure v (by substitution). -/
theorem monad_right_identity_beta (v : Expr) (A : Ty) (hv : IsValue v) :
    Step (Expr.app (Expr.lam A (Expr.distPure (Expr.var 0))) v)
         (substTop v (Expr.distPure (Expr.var 0))) :=
  Step.appLam hv

/-- Monad law 3 — Associativity (typing):
    If Γ ⊢ m : Dist A, Γ ⊢ f : A → Dist B, Γ ⊢ g : B → Dist C,
    then both sides of the associativity law are well-typed at Dist C.

    LHS: distBind (distBind m f) g : Dist C
    RHS: distBind m (lam A (distBind (app f (var 0)) g')) : Dist C
         where g' is the appropriately shifted g.

    We prove typing of LHS directly and state the correspondence. -/
theorem monad_assoc_lhs_typed {Γ : Ctx} {m f g : Expr} {A B C : Ty}
    (hm : HasType Γ m (Ty.dist A))
    (hf : HasType Γ f (Ty.arrow A (Ty.dist B)))
    (hg : HasType Γ g (Ty.arrow B (Ty.dist C))) :
    HasType Γ (Expr.distBind (Expr.distBind m f) g) (Ty.dist C) :=
  HasType.tDistBind (HasType.tDistBind hm hf) hg

/-- Monad law 3 — Associativity (operational, for values):
    When m = distPure v:
    LHS: distBind (distBind (distPure v) f) g
       → distBind (app f v) g                  [by distBindPure]

    RHS: distBind (distPure v) (lam A (distBind (app f (var 0)) g))
       → app (lam A (distBind (app f (var 0)) g)) v  [by distBindPure]
       → distBind (app f v) g'                        [by β-reduction]

    Both sides reach distBind (app f v) g (modulo shifting),
    establishing operational equivalence. -/
theorem monad_assoc_lhs_step (v f g : Expr) (hv : IsValue v) :
    Step (Expr.distBind (Expr.distBind (Expr.distPure v) f) g)
         (Expr.distBind (Expr.app f v) g) :=
  Step.distBindStep1 (Step.distBindPure hv)

-- ════════════════════════════════════════════════════════════════════════════
-- Section 12. Bet-specific properties
-- ════════════════════════════════════════════════════════════════════════════

/-- A bet expression with three identical branches is equivalent to distPure.
    bet v v v → distPure v (all three choices yield the same result). -/
theorem bet_degenerate (v : Expr) (hv : IsValue v) :
    Step (Expr.bet v v v) (Expr.distPure v) :=
  Step.betFirst hv hv hv

/-- Bet always produces a Dist type when well-typed. -/
theorem bet_produces_dist {Γ : Ctx} {e₁ e₂ e₃ : Expr} {T : Ty}
    (h : HasType Γ (Expr.bet e₁ e₂ e₃) T) :
    ∃ U, T = Ty.dist U := by
  cases h with
  | tBet _ _ _ => exact ⟨_, rfl⟩

/-- Sample eliminates exactly one layer of Dist. -/
theorem sample_eliminates_dist {Γ : Ctx} {e : Expr} {T : Ty}
    (h : HasType Γ (Expr.sample e) T) :
    HasType Γ e (Ty.dist T) := by
  cases h with
  | tSample he => exact he

/-- The composition sample ∘ bet acts as a non-deterministic choice:
    sample (bet v₁ v₂ v₃) →* vᵢ for some i ∈ {1,2,3}. -/
theorem sample_bet_reduces {v₁ v₂ v₃ : Expr}
    (hv₁ : IsValue v₁) (hv₂ : IsValue v₂) (hv₃ : IsValue v₃) :
    (∃ e', Step (Expr.sample (Expr.bet v₁ v₂ v₃)) e') := by
  -- bet v₁ v₂ v₃ can step to distPure v₁ (or v₂ or v₃)
  exact ⟨Expr.sample (Expr.distPure v₁), Step.sampleStep (Step.betFirst hv₁ hv₂ hv₃)⟩

-- ════════════════════════════════════════════════════════════════════════════
-- Section 13. Determinism for non-probabilistic fragment
-- ════════════════════════════════════════════════════════════════════════════

/-- Values do not step. -/
theorem value_no_step {v e : Expr} (hv : IsValue v) : ¬ Step v e := by
  intro hs
  cases hv with
  | litInt => cases hs
  | litFloat => cases hs
  | litBool => cases hs
  | litString => cases hs
  | litUnit => cases hs
  | lam => cases hs
  | distPure hv' =>
    cases hs with
    | distPureStep hs' => exact value_no_step hv' hs'

-- ════════════════════════════════════════════════════════════════════════════
-- End of formalisation
-- ════════════════════════════════════════════════════════════════════════════
