-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
--
-- Port of betlang Racket DSL test suite to Idris2 (estate-rollout port 5a/11).
-- Content-validation pattern: file-read + substring matching against the
-- canonical Racket sources. Does NOT reimplement (bet A B C); the Racket
-- random primitive has no pure Idris2 analogue.
--
-- Source files validated:
--   core/betlang.rkt             - DSL primitives and provides
--   lib/ternary.rkt              - ternary logic utilities
--   lib/statistics.rkt           - mean/variance/etc
--   lib/combinators.rkt          - pure combinators
--   lib/distributions.rkt        - distributions
--   tests/basics.rkt             - original Racket test suite
--   conformance/deterministic.rkt
--   conformance/stochastic-seeded.rkt

module DslStructureTest

import Test.Spec
import Data.String
import System.File

%default covering

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

public export
allSuites : List TestCase
allSuites =
  [ test "Unit: All 8 source .rkt files exist at expected paths" $ do
      a <- fileExists "core/betlang.rkt"
      b <- fileExists "lib/ternary.rkt"
      c <- fileExists "lib/statistics.rkt"
      d <- fileExists "lib/combinators.rkt"
      e <- fileExists "lib/distributions.rkt"
      f <- fileExists "tests/basics.rkt"
      g <- fileExists "conformance/deterministic.rkt"
      h <- fileExists "conformance/stochastic-seeded.rkt"
      assertTrue "all 8 .rkt source files present"
        (a && b && c && d && e && f && g && h)

  , test "Unit: All .rkt files declare #lang racket" $ do
      a <- readFileToString "core/betlang.rkt"
      b <- readFileToString "lib/ternary.rkt"
      c <- readFileToString "lib/statistics.rkt"
      d <- readFileToString "lib/combinators.rkt"
      e <- readFileToString "lib/distributions.rkt"
      f <- readFileToString "tests/basics.rkt"
      g <- readFileToString "conformance/deterministic.rkt"
      h <- readFileToString "conformance/stochastic-seeded.rkt"
      let needle = "#lang racket"
      allPass
        [ assertTrue "core/betlang.rkt #lang" (isInfixOf needle a)
        , assertTrue "lib/ternary.rkt #lang" (isInfixOf needle b)
        , assertTrue "lib/statistics.rkt #lang" (isInfixOf needle c)
        , assertTrue "lib/combinators.rkt #lang" (isInfixOf needle d)
        , assertTrue "lib/distributions.rkt #lang" (isInfixOf needle e)
        , assertTrue "tests/basics.rkt #lang" (isInfixOf needle f)
        , assertTrue "conformance/deterministic.rkt #lang" (isInfixOf needle g)
        , assertTrue "conformance/stochastic-seeded.rkt #lang" (isInfixOf needle h)
        ]

  , test "Unit: Conformance files have SPDX-License-Identifier headers" $ do
      -- NOTE: core/* and lib/* and tests/basics.rkt currently lack SPDX
      -- headers in the Racket sources. Only the two conformance files have
      -- them. This test pins the current truth; widening SPDX coverage to
      -- the rest of the .rkt corpus is tracked separately.
      d <- readFileToString "conformance/deterministic.rkt"
      s <- readFileToString "conformance/stochastic-seeded.rkt"
      let spdx = "SPDX-License-Identifier: MPL-2.0"
      allPass
        [ assertTrue "deterministic.rkt SPDX" (isInfixOf spdx d)
        , assertTrue "stochastic-seeded.rkt SPDX" (isInfixOf spdx s)
        ]

  , test "Unit: core/betlang.rkt provides all 20 documented primitives" $ do
      content <- readFileToString "core/betlang.rkt"
      allPass
        [ assertTrue "bet" (isInfixOf "bet" content)
        , assertTrue "bet/weighted" (isInfixOf "bet/weighted" content)
        , assertTrue "bet/conditional" (isInfixOf "bet/conditional" content)
        , assertTrue "bet/lazy" (isInfixOf "bet/lazy" content)
        , assertTrue "bet-chain" (isInfixOf "bet-chain" content)
        , assertTrue "bet-compose" (isInfixOf "bet-compose" content)
        , assertTrue "bet-map" (isInfixOf "bet-map" content)
        , assertTrue "bet-fold" (isInfixOf "bet-fold" content)
        , assertTrue "bet-filter" (isInfixOf "bet-filter" content)
        , assertTrue "bet-repeat" (isInfixOf "bet-repeat" content)
        , assertTrue "bet-until" (isInfixOf "bet-until" content)
        , assertTrue "bet-with-seed" (isInfixOf "bet-with-seed" content)
        , assertTrue "all-bets" (isInfixOf "all-bets" content)
        , assertTrue "any-bet" (isInfixOf "any-bet" content)
        , assertTrue "bet-sequence" (isInfixOf "bet-sequence" content)
        , assertTrue "bet-parallel" (isInfixOf "bet-parallel" content)
        , assertTrue "make-bet-generator" (isInfixOf "make-bet-generator" content)
        , assertTrue "bet-probability" (isInfixOf "bet-probability" content)
        , assertTrue "bet-entropy" (isInfixOf "bet-entropy" content)
        , assertTrue "bet-expect" (isInfixOf "bet-expect" content)
        ]

  , test "Unit: core/betlang.rkt uses (random 3) for ternary dispatch" $ do
      content <- readFileToString "core/betlang.rkt"
      allPass
        [ assertTrue "(random 3) ternary dispatch" (isInfixOf "(random 3)" content)
        , assertTrue "match clause for 0" (isInfixOf "[0 a]" content)
        , assertTrue "match clause for 1" (isInfixOf "[1 b]" content)
        ]

  , test "Unit: lib/ternary.rkt exports Kleene three-valued logic primitives" $ do
      content <- readFileToString "lib/ternary.rkt"
      allPass
        [ assertTrue "TRUE" (isInfixOf "TRUE" content)
        , assertTrue "FALSE" (isInfixOf "FALSE" content)
        , assertTrue "UNKNOWN" (isInfixOf "UNKNOWN" content)
        , assertTrue "ternary-and" (isInfixOf "ternary-and" content)
        , assertTrue "ternary-or" (isInfixOf "ternary-or" content)
        , assertTrue "ternary-not" (isInfixOf "ternary-not" content)
        , assertTrue "ternary-majority" (isInfixOf "ternary-majority" content)
        , assertTrue "ternary-median" (isInfixOf "ternary-median" content)
        ]

  , test "Unit: lib/statistics.rkt exports descriptive statistics primitives" $ do
      content <- readFileToString "lib/statistics.rkt"
      allPass
        [ assertTrue "mean" (isInfixOf "mean" content)
        , assertTrue "median" (isInfixOf "median" content)
        , assertTrue "mode" (isInfixOf "mode" content)
        , assertTrue "variance" (isInfixOf "variance" content)
        , assertTrue "stddev" (isInfixOf "stddev" content)
        , assertTrue "covariance" (isInfixOf "covariance" content)
        , assertTrue "correlation" (isInfixOf "correlation" content)
        , assertTrue "percentile" (isInfixOf "percentile" content)
        , assertTrue "histogram" (isInfixOf "histogram" content)
        , assertTrue "bootstrap" (isInfixOf "bootstrap" content)
        ]

  , test "Unit: lib/combinators.rkt exports monadic + control combinators" $ do
      content <- readFileToString "lib/combinators.rkt"
      allPass
        [ assertTrue "bet-pure" (isInfixOf "bet-pure" content)
        , assertTrue "bet-bind" (isInfixOf "bet-bind" content)
        , assertTrue "bet-join" (isInfixOf "bet-join" content)
        , assertTrue "bet-lift" (isInfixOf "bet-lift" content)
        , assertTrue "bet-ap" (isInfixOf "bet-ap" content)
        , assertTrue "bet-memoize" (isInfixOf "bet-memoize" content)
        , assertTrue "bet-pipeline" (isInfixOf "bet-pipeline" content)
        , assertTrue "bet-retry" (isInfixOf "bet-retry" content)
        , assertTrue "bet-fallback" (isInfixOf "bet-fallback" content)
        ]

  , test "Unit: lib/distributions.rkt exports common probability distributions" $ do
      content <- readFileToString "lib/distributions.rkt"
      allPass
        [ assertTrue "uniform" (isInfixOf "uniform" content)
        , assertTrue "bernoulli" (isInfixOf "bernoulli" content)
        , assertTrue "binomial" (isInfixOf "binomial" content)
        , assertTrue "geometric" (isInfixOf "geometric" content)
        , assertTrue "poisson" (isInfixOf "poisson" content)
        , assertTrue "exponential" (isInfixOf "exponential" content)
        , assertTrue "normal" (isInfixOf "normal" content)
        , assertTrue "gamma" (isInfixOf "gamma" content)
        , assertTrue "beta" (isInfixOf "beta" content)
        , assertTrue "random-walk" (isInfixOf "random-walk" content)
        , assertTrue "brownian-motion" (isInfixOf "brownian-motion" content)
        ]

  , test "Unit: lib/*.rkt files require core/betlang.rkt" $ do
      a <- readFileToString "lib/ternary.rkt"
      b <- readFileToString "lib/statistics.rkt"
      c <- readFileToString "lib/combinators.rkt"
      d <- readFileToString "lib/distributions.rkt"
      let needle = "(require \"../core/betlang.rkt\")"
      allPass
        [ assertTrue "ternary requires core" (isInfixOf needle a)
        , assertTrue "statistics requires core" (isInfixOf needle b)
        , assertTrue "combinators requires core" (isInfixOf needle c)
        , assertTrue "distributions requires core" (isInfixOf needle d)
        ]

  , test "Unit: tests/basics.rkt uses rackunit + requires all libs" $ do
      content <- readFileToString "tests/basics.rkt"
      allPass
        [ assertTrue "rackunit" (isInfixOf "(require rackunit)" content)
        , assertTrue "core/betlang require" (isInfixOf "../core/betlang.rkt" content)
        , assertTrue "statistics require" (isInfixOf "../lib/statistics.rkt" content)
        , assertTrue "combinators require" (isInfixOf "../lib/combinators.rkt" content)
        , assertTrue "distributions require" (isInfixOf "../lib/distributions.rkt" content)
        ]

  , test "Unit: conformance/deterministic.rkt requires core + uses rackunit" $ do
      content <- readFileToString "conformance/deterministic.rkt"
      allPass
        [ assertTrue "rackunit" (isInfixOf "(require rackunit)" content)
        , assertTrue "core require" (isInfixOf "../core/betlang.rkt" content)
        , assertTrue "check-equal" (isInfixOf "check-equal?" content)
        ]

  , test "Unit: conformance/stochastic-seeded.rkt declares MASTER-SEED" $ do
      content <- readFileToString "conformance/stochastic-seeded.rkt"
      allPass
        [ assertTrue "MASTER-SEED define" (isInfixOf "(define MASTER-SEED" content)
        , assertTrue "20250101 literal" (isInfixOf "20250101" content)
        , assertTrue "bet-with-seed call" (isInfixOf "bet-with-seed" content)
        ]

  , test "Unit: core/betlang.rkt bet primitive has exactly 3 positional args" $ do
      content <- readFileToString "core/betlang.rkt"
      -- The canonical definition is (define (bet a b c) ...). The exact
      -- substring uniquely identifies the ternary arity.
      assertTrue "(define (bet a b c)" (isInfixOf "(define (bet a b c)" content)

  , test "Unit: bet/lazy takes 3 thunks (lazy ternary form)" $ do
      content <- readFileToString "core/betlang.rkt"
      assertTrue "(define (bet/lazy thunk-a thunk-b thunk-c)"
        (isInfixOf "(define (bet/lazy thunk-a thunk-b thunk-c)" content)

  , test "Unit: all-bets returns a 3-element list (ternary shape)" $ do
      content <- readFileToString "core/betlang.rkt"
      allPass
        [ assertTrue "(define (all-bets a b c)" (isInfixOf "(define (all-bets a b c)" content)
        , assertTrue "(list a b c)" (isInfixOf "(list a b c)" content)
        ]
  ]
