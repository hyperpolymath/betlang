# SPDX-License-Identifier: PMPL-1.0-or-later
# Generated from betlang source

using BetLang

println("=== Coin Flip Game ===")
println("Single flip:")
flip = bet("heads", "tails", "edge")
println(flip)
println("Ten flips:")
flips = bet_parallel(10, "heads", "tails", "edge")
println(flips)
println("Estimating probability of heads (1000 trials):")
prob_heads = bet_probability(1000, (x) -> begin (x == "heads") end, "heads", "tails", "edge")
println(prob_heads)
println("Unfair coin (70% heads, 20% tails, 10% edge):")
unfair = bet_weighted([("heads", 0.7), ("tails", 0.2), ("edge", 0.1)])
println(unfair)
println("100 flips entropy:")
H = bet_entropy(100, "heads", "tails", "edge")
println(H)
