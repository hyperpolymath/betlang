-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
--
-- Aggregator entry point for the playground Idris2 test suite.

module Main

import Test.Spec
import PlaygroundTernaryTest
import PlaygroundProbabilityTest
import PlaygroundStructureTest
import System

%default covering

main : IO ()
main = do
  (p1, f1) <- runTestSuite "PlaygroundTernaryTest"     PlaygroundTernaryTest.allSuites
  (p2, f2) <- runTestSuite "PlaygroundProbabilityTest" PlaygroundProbabilityTest.allSuites
  (p3, f3) <- runTestSuite "PlaygroundStructureTest"   PlaygroundStructureTest.allSuites
  let totalPassed = p1 + p2 + p3
  let totalFailed = f1 + f2 + f3
  putStrLn ""
  putStrLn $ "=== Total: " ++ show totalPassed ++ " passed, " ++ show totalFailed ++ " failed ==="
  if totalFailed > 0
    then exitWith (ExitFailure 1)
    else pure ()
