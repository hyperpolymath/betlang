# SPDX-License-Identifier: PMPL-1.0-or-later
# Generated from betlang source

using BetLang

println("Basic bet:")
result = bet("heads", "tails", "edge")
println(result)
println("Weighted bet:")
weighted = bet_weighted([("common", 0.7), ("rare", 0.3)])
println(weighted)
println("Ten parallel bets:")
many = bet_parallel(10, "win", "draw", "lose")
println(many)
x = 5
y = 3
sum = (x + y)
println(sum)
