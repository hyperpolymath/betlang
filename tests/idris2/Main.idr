-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
module Main

import Test.Spec
import DslStructureTest
import DslContractsTest
import System

%default covering

main : IO ()
main = do
  (p1, f1) <- runTestSuite "DslStructureTest" DslStructureTest.allSuites
  (p2, f2) <- runTestSuite "DslContractsTest" DslContractsTest.allSuites
  let totalPassed = p1 + p2
  let totalFailed = f1 + f2
  putStrLn ""
  putStrLn $ "=== Total: " ++ show totalPassed ++ " passed, " ++ show totalFailed ++ " failed ==="
  if totalFailed > 0
    then exitWith (ExitFailure 1)
    else pure ()
