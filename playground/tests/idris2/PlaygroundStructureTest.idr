-- SPDX-License-Identifier: PMPL-1.0-or-later
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
--
-- Content-validation tests for the playground sub-project, estate-rollout 5b/11.
-- Mirrors the Deno test-file inventory: each .ts source must exist, carry the
-- SPDX header, and expose the public API the unit tests depend on.

module PlaygroundStructureTest

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

spdxLine : String
spdxLine = "SPDX-License-Identifier: PMPL-1.0-or-later"

public export
allSuites : List TestCase
allSuites =
  [ test "Structure: all 4 .ts files plus deno.json exist" $ do
      a <- fileExists "src/ternary.ts"
      b <- fileExists "src/probability.ts"
      c <- fileExists "test/ternary_test.ts"
      d <- fileExists "test/probability_test.ts"
      e <- fileExists "deno.json"
      assertTrue "all 5 playground files present" (a && b && c && d && e)

  , test "Structure: ternary.ts carries SPDX header" $ do
      content <- readFileToString "src/ternary.ts"
      assertTrue "ternary.ts SPDX" (isInfixOf spdxLine content)

  , test "Structure: probability.ts carries SPDX header" $ do
      content <- readFileToString "src/probability.ts"
      assertTrue "probability.ts SPDX" (isInfixOf spdxLine content)

  , test "Structure: ternary_test.ts carries SPDX header" $ do
      content <- readFileToString "test/ternary_test.ts"
      assertTrue "ternary_test.ts SPDX" (isInfixOf spdxLine content)

  , test "Structure: probability_test.ts carries SPDX header" $ do
      content <- readFileToString "test/probability_test.ts"
      assertTrue "probability_test.ts SPDX" (isInfixOf spdxLine content)

  , test "Structure: ternary.ts exports the Tri type and all 5 operators" $ do
      content <- readFileToString "src/ternary.ts"
      allPass
        [ assertTrue "export Tri"       (isInfixOf "export type Tri" content)
        , assertTrue "export not"       (isInfixOf "export function not" content)
        , assertTrue "export and"       (isInfixOf "export function and" content)
        , assertTrue "export or"        (isInfixOf "export function or"  content)
        , assertTrue "export implies"   (isInfixOf "export function implies" content)
        , assertTrue "export bet"       (isInfixOf "export function bet" content)
        ]

  , test "Structure: ternary.ts documents the F < U < T order" $ do
      content <- readFileToString "src/ternary.ts"
      assertTrue "F < U < T order documented" (isInfixOf "F < U < T" content)

  , test "Structure: ternary.ts uses min/max from Math (Kleene = min/max)" $ do
      content <- readFileToString "src/ternary.ts"
      allPass
        [ assertTrue "Math.min present" (isInfixOf "Math.min" content)
        , assertTrue "Math.max present" (isInfixOf "Math.max" content)
        ]

  , test "Structure: probability.ts imports from ternary.ts" $ do
      content <- readFileToString "src/probability.ts"
      allPass
        [ assertTrue "imports from ./ternary.ts" (isInfixOf "from './ternary.ts'" content)
        , assertTrue "imports Tri type"          (isInfixOf "type Tri" content)
        , assertTrue "imports bet"               (isInfixOf "bet" content)
        ]

  , test "Structure: probability.ts exports the Branch interface" $ do
      content <- readFileToString "src/probability.ts"
      assertTrue "export interface Branch" (isInfixOf "export interface Branch" content)

  , test "Structure: ternary_test.ts uses @std/assert" $ do
      content <- readFileToString "test/ternary_test.ts"
      allPass
        [ assertTrue "imports assertEquals" (isInfixOf "assertEquals" content)
        , assertTrue "uses Deno.test"       (isInfixOf "Deno.test" content)
        ]

  , test "Structure: probability_test.ts uses assertThrows for weight-validation" $ do
      content <- readFileToString "test/probability_test.ts"
      allPass
        [ assertTrue "assertThrows present" (isInfixOf "assertThrows" content)
        , assertTrue "Deno.test present"    (isInfixOf "Deno.test" content)
        ]

  , test "Structure: deno.json declares the betlang-playground name" $ do
      content <- readFileToString "deno.json"
      allPass
        [ assertTrue "name field"   (isInfixOf "betlang-playground" content)
        , assertTrue "test task"    (isInfixOf "\"test\"" content)
        , assertTrue "@std/assert"  (isInfixOf "@std/assert" content)
        ]

  , test "Structure: deno.json enables strict TypeScript" $ do
      content <- readFileToString "deno.json"
      allPass
        [ assertTrue "strict mode"        (isInfixOf "\"strict\": true" content)
        , assertTrue "noImplicitAny"      (isInfixOf "noImplicitAny" content)
        , assertTrue "strictNullChecks"   (isInfixOf "strictNullChecks" content)
        ]
  ]
