-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
--
-- Port of playground/test/probability_test.ts to Idris2, estate-rollout 5b/11.
-- The probability layer mixes pure deterministic ops (expectation reduction,
-- conditional dispatch) with PRNG-driven Monte Carlo. We port the pure parts
-- directly (weighted-choice over [0,1) inputs, analytic expectation,
-- conditional dispatch via Tri) and keep content-validation for the parts
-- that need the Deno mulberry32 PRNG.

module PlaygroundProbabilityTest

import Test.Spec
import PlaygroundTernaryTest
import Data.String
import System.File

%default covering

-- Inline weighted-branch model -----------------------------------------------

-- A branch carries a weight and a lazy value. Mirrors playground/src/probability.ts.
public export
record Branch a where
  constructor MkBranch
  weight : Double
  value  : Lazy a

-- Sum of weights.
totalWeight : List (Branch a) -> Double
totalWeight = foldr (\b, acc => weight b + acc) 0.0

-- Pick a branch given a draw in [0,1). Returns Nothing when weights are
-- non-positive or the input list is empty (we model the runtime error
-- defensively so tests can assert on it).
public export
betWeighted : List (Branch a) -> Double -> Maybe a
betWeighted []                 _    = Nothing
betWeighted branches@(b0 :: _) draw =
  let tot = totalWeight branches in
  if tot <= 0.0
    then Nothing
    else Just (pick branches (draw * tot) (Force (value b0)))
  where
    pick : List (Branch a) -> Double -> a -> a
    pick []        _ fallback = fallback
    pick (b :: bs) r fallback =
      let r' = r - weight b in
      if r' <= 0.0
        then Force (value b)
        else pick bs r' fallback

-- Predicate-driven Tri dispatch: total under U. Mirrors the Deno
-- betConditional, which is just a Tri-flavoured wrapper over bet().
public export
betConditional : Tri -> Lazy a -> Lazy a -> Lazy a -> a
betConditional T onT _   _   = onT
betConditional U _   onU _   = onU
betConditional F _   _   onF = onF

-- Analytic expectation of a numeric weighted bet (no sampling needed).
public export
analyticEV : List (Branch Double) -> Double
analyticEV bs =
  let tot = totalWeight bs in
  if tot <= 0.0
    then 0.0
    else foldr (\b, acc => weight b * Force (value b) + acc) 0.0 bs / tot

-- File helpers for content validation ----------------------------------------

readFileToString : String -> IO String
readFileToString path = do
  Right contents <- readFile path
    | Left _ => pure ""
  pure contents

fileExists : String -> IO Bool
fileExists path = do
  Right _ <- readFile path
    | Left _ => pure False
  pure True

-- Sample weighted payout used in the Deno expectation test.
samplePayout : List (Branch Double)
samplePayout =
  [ MkBranch 1.0 100.0
  , MkBranch 2.0 10.0
  , MkBranch 7.0 0.0
  ]

-- Tests ----------------------------------------------------------------------

public export
allSuites : List TestCase
allSuites =
  [ test "analyticEV computes the documented EV = 12 for the sample payout" $
      assertEq (analyticEV samplePayout) 12.0

  , test "betWeighted with draw=0.0 picks the first branch" $ do
      let bs : List (Branch String) =
            [ MkBranch 0.6 "T", MkBranch 0.3 "U", MkBranch 0.1 "F" ]
      assertEq (betWeighted bs 0.0) (Just "T")

  , test "betWeighted with draw=0.5 picks branch with cumulative weight" $ do
      -- weights 0.6/0.3/0.1; draw 0.5 -> target 0.5 -> consume 0.6 first -> T
      let bs : List (Branch String) =
            [ MkBranch 0.6 "T", MkBranch 0.3 "U", MkBranch 0.1 "F" ]
      assertEq (betWeighted bs 0.5) (Just "T")

  , test "betWeighted with draw 0.75 picks the middle branch" $ do
      -- target 0.75; 0.6 used by T (-0.15 leftover -> would early-return)
      -- Actually 0.75*1.0 = 0.75; 0.75 - 0.6 = 0.15 > 0, then 0.15 - 0.3 < 0 -> U
      let bs : List (Branch String) =
            [ MkBranch 0.6 "T", MkBranch 0.3 "U", MkBranch 0.1 "F" ]
      assertEq (betWeighted bs 0.75) (Just "U")

  , test "betWeighted with draw 0.95 picks the tail branch" $ do
      let bs : List (Branch String) =
            [ MkBranch 0.6 "T", MkBranch 0.3 "U", MkBranch 0.1 "F" ]
      assertEq (betWeighted bs 0.95) (Just "F")

  , test "betWeighted rejects empty branch list" $ do
      let bs : List (Branch String) = []
      assertEq (betWeighted bs 0.5) Nothing

  , test "betWeighted rejects non-positive total weight" $ do
      let bs : List (Branch String) = [ MkBranch 0.0 "x" ]
      assertEq (betWeighted bs 0.5) Nothing

  , test "betConditional defers to the uncertain branch on Unknown" $
      assertEq (betConditional U "yes" "maybe" "no") "maybe"

  , test "betConditional commits on True" $
      assertEq (betConditional T "yes" "maybe" "no") "yes"

  , test "betConditional declines on False" $
      assertEq (betConditional F "yes" "maybe" "no") "no"

  -- Content validation for the parts of probability.ts that rely on Deno's
  -- mulberry32 PRNG: we cannot port the bit-twiddling RNG identically into
  -- Idris2, so we cross-check that the TS source still exposes the public API
  -- the tests above mirror.
  , test "Structure: probability.ts exports rng/betWeighted/betConditional/expectation" $ do
      content <- readFileToString "src/probability.ts"
      allPass
        [ assertTrue "export rng"            (isInfixOf "export function rng" content)
        , assertTrue "export betWeighted"    (isInfixOf "export function betWeighted" content)
        , assertTrue "export betConditional" (isInfixOf "export function betConditional" content)
        , assertTrue "export expectation"    (isInfixOf "export function expectation" content)
        ]

  , test "Structure: probability.ts uses mulberry32 PRNG (deterministic, seedable)" $ do
      content <- readFileToString "src/probability.ts"
      allPass
        [ assertTrue "mulberry32 mention"     (isInfixOf "mulberry32" content)
        , assertTrue "0x6d2b79f5 constant"    (isInfixOf "0x6d2b79f5" content)
        , assertTrue "Math.imul"              (isInfixOf "Math.imul" content)
        ]
  ]
