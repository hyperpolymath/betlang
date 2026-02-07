# SPDX-License-Identifier: PMPL-1.0-or-later

using Test
using BetLang
using Random

@testset "BetLang.jl Tests" begin

@testset "Basic Bet" begin
    # Test that bet returns one of three values
    results = Set()
    for _ in 1:100
        result = bet("a", "b", "c")
        @test result in ["a", "b", "c"]
        push!(results, result)
    end
    # Should see all three values eventually
    @test length(results) == 3
end

@testset "Weighted Bet" begin
    # Test weighted probabilities
    counts = Dict("a" => 0, "b" => 0, "c" => 0)
    n = 10000
    for _ in 1:n
        result = bet_weighted([("a", 0.5), ("b", 0.3), ("c", 0.2)])
        counts[result] += 1
    end

    # Check approximate probabilities (within 5%)
    @test isapprox(counts["a"] / n, 0.5, atol=0.05)
    @test isapprox(counts["b"] / n, 0.3, atol=0.05)
    @test isapprox(counts["c"] / n, 0.2, atol=0.05)
end

@testset "Conditional Bet" begin
    # Test predicate-based selection
    result = bet_conditional(() -> true, "yes", "no", "maybe")
    @test result == "yes"

    result = bet_conditional(() -> false, "yes", "no", "maybe")
    @test result == "no"

    result = bet_conditional(() -> error("fail"), "yes", "no", "maybe")
    @test result == "maybe"
end

@testset "Parallel Bets" begin
    results = bet_parallel(10, 1, 2, 3)
    @test length(results) == 10
    @test all(r -> r in [1, 2, 3], results)
end

@testset "Deterministic Bet (Seeded)" begin
    # Same seed should give same result
    result1 = bet_with_seed(42) do
        bet("a", "b", "c")
    end

    result2 = bet_with_seed(42) do
        bet("a", "b", "c")
    end

    @test result1 == result2
end

@testset "Probability Estimation" begin
    # Estimate probability of "heads" in fair ternary bet
    prob = bet_probability(10000, x -> x == "heads", "heads", "tails", "edge")
    @test isapprox(prob, 1/3, atol=0.02)
end

@testset "Entropy Calculation" begin
    # Entropy of uniform ternary distribution should be log2(3) ≈ 1.585
    H = bet_entropy(10000, "a", "b", "c")
    @test isapprox(H, log2(3), atol=0.05)
end

@testset "Bet Repeat" begin
    results = bet_repeat(5) do
        bet(1, 2, 3)
    end
    @test length(results) == 5
    @test all(r -> r in [1, 2, 3], results)
end

end # testset
