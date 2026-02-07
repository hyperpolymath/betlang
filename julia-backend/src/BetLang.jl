# SPDX-License-Identifier: PMPL-1.0-or-later
# BetLang.jl - Julia backend for betlang probabilistic programming language
# Copyright (C) 2026 Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>

module BetLang

using Distributions
using StatsBase
using Random

export bet, bet_weighted, bet_conditional, bet_parallel, bet_repeat
export bet_with_seed, bet_probability, bet_entropy

# ============================================================================
# Core Bet Primitives
# ============================================================================

"""
    bet(options...)

Basic ternary bet - randomly select one of three values with equal probability.

# Examples
```julia
result = bet('heads', 'tails', 'edge')
result = bet(1, 2, 3)
result = bet("win", "draw", "lose")
```
"""
function bet(a, b, c)
    dist = Categorical([1/3, 1/3, 1/3])
    choice = rand(dist)
    return choice == 1 ? a : (choice == 2 ? b : c)
end

"""
    bet_weighted(options::Vector{Tuple{T, Float64}}) where T

Weighted bet - select from options with specified probabilities.

# Arguments
- `options`: Vector of (value, weight) tuples

# Examples
```julia
result = bet_weighted([("common", 7.0), ("uncommon", 2.0), ("rare", 1.0)])
```
"""
function bet_weighted(options::Vector{Tuple{T, Float64}}) where T
    values = [opt[1] for opt in options]
    weights = [opt[2] for opt in options]

    # Normalize weights to probabilities
    probs = weights ./ sum(weights)

    dist = Categorical(probs)
    choice = rand(dist)
    return values[choice]
end

"""
    bet_conditional(predicate::Function, if_true, if_false, uncertain)

Conditional bet based on a predicate function.

# Examples
```julia
x = 5
result = bet_conditional(
    () -> x > 3,
    "high",
    "low",
    "unknown"
)
```
"""
function bet_conditional(predicate::Function, if_true, if_false, uncertain)
    try
        if predicate()
            return if_true
        else
            return if_false
        end
    catch
        return uncertain
    end
end

"""
    bet_parallel(n::Int, a, b, c)

Run multiple parallel trials of a ternary bet.

# Examples
```julia
results = bet_parallel(10, 'heads', 'tails', 'edge')
# => ['heads', 'tails', 'heads', 'edge', 'tails', ...]
```
"""
function bet_parallel(n::Int, a, b, c)
    dist = Categorical([1/3, 1/3, 1/3])
    choices = rand(dist, n)
    return [choice == 1 ? a : (choice == 2 ? b : c) for choice in choices]
end

"""
    bet_repeat(n::Int, fn::Function)

Repeat a bet function n times.

# Examples
```julia
results = bet_repeat(100) do
    bet('heads', 'tails', 'edge')
end
```
"""
function bet_repeat(fn::Function, n::Int)
    return [fn() for _ in 1:n]
end

"""
    bet_with_seed(seed::Int, fn::Function)

Execute a bet with deterministic random seed for reproducibility.

# Examples
```julia
result = bet_with_seed(42) do
    bet("heads", "tails", "edge")
end
```
"""
function bet_with_seed(fn::Function, seed::Int)
    Random.seed!(seed)
    return fn()
end

# ============================================================================
# Statistical Utilities
# ============================================================================

"""
    bet_probability(n::Int, predicate::Function, a, b, c)

Estimate probability that a bet satisfies a predicate.

# Examples
```julia
prob = bet_probability(1000, x -> x == 'heads', 'heads', 'tails', 'edge')
# => ≈ 0.333
```
"""
function bet_probability(n::Int, predicate::Function, a, b, c)
    results = bet_parallel(n, a, b, c)
    matches = count(predicate, results)
    return matches / n
end

"""
    bet_entropy(n::Int, a, b, c)

Estimate Shannon entropy of a bet distribution.

# Examples
```julia
H = bet_entropy(10000, 'heads', 'tails', 'edge')
# => ≈ 1.585 bits (log2(3) for uniform ternary)
```
"""
function bet_entropy(n::Int, a, b, c)
    results = bet_parallel(n, a, b, c)

    # Count occurrences
    counts = Dict{Any, Int}()
    for result in results
        counts[result] = get(counts, result, 0) + 1
    end

    # Calculate entropy
    H = 0.0
    for count in values(counts)
        p = count / n
        if p > 0
            H -= p * log2(p)
        end
    end

    return H
end

# ============================================================================
# Version Information
# ============================================================================

"""
    version()

Return BetLang.jl version string.
"""
function version()
    return "0.8.0-dev"
end

end # module BetLang
