-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
--
-- Port of playground/test/ternary_test.ts to Idris2, estate-rollout port 5b/11.
-- The playground ternary module is pure Kleene 3-value logic, so it is
-- re-implemented inline below and the original Deno test cases are ported
-- literally as assertEq calls.

module PlaygroundTernaryTest

import Test.Spec

%default total

-- Inline Kleene 3-value logic --------------------------------------------------

public export
data Tri = T | F | U

public export
Eq Tri where
  T == T = True
  F == F = True
  U == U = True
  _ == _ = False

public export
Show Tri where
  show T = "T"
  show F = "F"
  show U = "U"

-- F < U < T encoded as ranks 0, 1, 2.
rankT : Tri -> Nat
rankT F = 0
rankT U = 1
rankT T = 2

byRank : Nat -> Tri
byRank Z         = F
byRank (S Z)     = U
byRank (S (S _)) = T

minNat : Nat -> Nat -> Nat
minNat Z     _     = Z
minNat _     Z     = Z
minNat (S a) (S b) = S (minNat a b)

maxNat : Nat -> Nat -> Nat
maxNat Z     b     = b
maxNat a     Z     = a
maxNat (S a) (S b) = S (maxNat a b)

public export
notT : Tri -> Tri
notT T = F
notT F = T
notT U = U

public export
andT : Tri -> Tri -> Tri
andT a b = byRank (minNat (rankT a) (rankT b))

public export
orT : Tri -> Tri -> Tri
orT a b = byRank (maxNat (rankT a) (rankT b))

public export
impliesT : Tri -> Tri -> Tri
impliesT a b = orT (notT a) b

-- Lazy ternary choice. Idris2 is lazy-by-need; we mark the branches Lazy so
-- only the selected one is forced, mirroring the Deno thunk behaviour.
public export
betT : Tri -> Lazy a -> Lazy a -> Lazy a -> a
betT T onTrue _         _        = onTrue
betT F _      _         onFalse  = onFalse
betT U _      onUnknown _        = onUnknown

-- Helpers ---------------------------------------------------------------------

all3 : List Tri
all3 = [T, U, F]

-- Tests -----------------------------------------------------------------------

public export
allSuites : List TestCase
allSuites =
  [ test "negation is involutive and fixes Unknown" $
      allPass
        [ assertEq (notT T) F
        , assertEq (notT F) T
        , assertEq (notT U) U
        , assertEq (notT (notT T)) T
        , assertEq (notT (notT U)) U
        , assertEq (notT (notT F)) F
        ]

  , test "AND matches the documented Justfile truth table (min)" $
      allPass
        [ assertEq (andT T T) T
        , assertEq (andT T U) U
        , assertEq (andT T F) F
        , assertEq (andT U U) U
        , assertEq (andT U F) F
        , assertEq (andT F F) F
        ]

  , test "OR matches the dual truth table (max)" $
      allPass
        [ assertEq (orT T T) T
        , assertEq (orT T U) T
        , assertEq (orT T F) T
        , assertEq (orT U U) U
        , assertEq (orT U F) U
        , assertEq (orT F F) F
        ]

  , test "AND is commutative across all 9 pairs" $
      allPass
        [ assertEq (andT a b) (andT b a)
        | a <- all3, b <- all3
        ]

  , test "OR is commutative across all 9 pairs" $
      allPass
        [ assertEq (orT a b) (orT b a)
        | a <- all3, b <- all3
        ]

  , test "De Morgan: not(and a b) == or(not a, not b)" $
      allPass
        [ assertEq (notT (andT a b)) (orT (notT a) (notT b))
        | a <- all3, b <- all3
        ]

  , test "De Morgan dual: not(or a b) == and(not a, not b)" $
      allPass
        [ assertEq (notT (orT a b)) (andT (notT a) (notT b))
        | a <- all3, b <- all3
        ]

  , test "implies(a,b) == or(not a, b)" $
      allPass
        [ assertEq (impliesT a b) (orT (notT a) b)
        | a <- all3, b <- all3
        ]

  , test "implies(T,b) == b (modus ponens shape)" $
      allPass
        [ assertEq (impliesT T T) T
        , assertEq (impliesT T U) U
        , assertEq (impliesT T F) F
        ]

  , test "implies(F,_) == T (ex falso)" $
      allPass
        [ assertEq (impliesT F T) T
        , assertEq (impliesT F U) T
        , assertEq (impliesT F F) T
        ]

  , test "bet on F takes the false branch (returns 3)" $
      assertEq (betT F 1 2 3) 3

  , test "bet on T takes the true branch (returns 1)" $
      assertEq (betT T 1 2 3) 1

  , test "bet on U takes the unknown branch (returns 2)" $
      assertEq (betT U 1 2 3) 2
  ]
