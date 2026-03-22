# SPDX-License-Identifier: PMPL-1.0-or-later
# Basic BetLang.jl examples

using BetLang

println("=== BetLang.jl Basic Examples ===\n")

# Example 1: Basic ternary bet
println("1. Basic ternary bet:")
result = bet("heads", "tails", "edge")
println("   Result: $result\n")

# Example 2: Multiple trials
println("2. Ten coin flips:")
results = bet_parallel(10, "heads", "tails", "edge")
println("   Results: $results\n")

# Example 3: Weighted bet
println("3. Weighted bet (70% common, 20% uncommon, 10% rare):")
result = bet_weighted([("common", 0.7), ("uncommon", 0.2), ("rare", 0.1)])
println("   Result: $result\n")

# Example 4: Probability estimation
println("4. Estimate probability of 'heads' (1000 trials):")
prob = bet_probability(1000, x -> x == "heads", "heads", "tails", "edge")
println("   Estimated probability: $prob (expected: 0.333)\n")

# Example 5: Entropy calculation
println("5. Shannon entropy of uniform ternary bet:")
H = bet_entropy(10000, "A", "B", "C")
println("   Entropy: $H bits (expected: $(log2(3)) bits)\n")

# Example 6: Conditional bet
println("6. Conditional bet:")
x = 7
result = bet_conditional(
    () -> x > 5,
    "high",
    "low",
    "unknown"
)
println("   x = $x, result: $result\n")

# Example 7: Deterministic (seeded) bet
println("7. Deterministic bets with same seed:")
result1 = bet_with_seed(42) do
    bet("A", "B", "C")
end
result2 = bet_with_seed(42) do
    bet("A", "B", "C")
end
println("   First: $result1, Second: $result2")
println("   Same result: $(result1 == result2)\n")

# Example 8: Frequency analysis
println("8. Frequency analysis (10000 trials):")
results = bet_parallel(10000, "win", "draw", "lose")
counts = Dict("win" => 0, "draw" => 0, "lose" => 0)
for r in results
    counts[r] += 1
end
println("   Win: $(counts["win"]) ($(counts["win"]/100)%)")
println("   Draw: $(counts["draw"]) ($(counts["draw"]/100)%)")
println("   Lose: $(counts["lose"]) ($(counts["lose"]/100)%)")
