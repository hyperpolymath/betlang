# BetLang Julia Backend Design (v0.8)

**Status:** Planning Phase (2026-02-07)
**Target:** v0.8.0 milestone

## Priority Summary

**Integration priority (most → least important):**

1. **Tier 0 (Foundation):** Standard Julia ecosystem
   - Distributions.jl, StatsBase.jl, Turing.jl ← **MOST CRITICAL**
   - These are mature, widely-used, essential for any probabilistic programming

2. **Tier 1 (Extensions):** Hyperpolymath packages
   - BowtieRisk.jl, ZeroProb.jl, Causals.jl, domain packages
   - Valuable additions that extend BetLang's capabilities

3. **Tier 2+:** Specialized packages
   - As needed for specific use cases

**This document covers both, but Tier 0 packages are the foundation that must be implemented first.**

## Architecture Overview

BetLang → Julia compiler that translates Racket betlang to high-performance Julia code, leveraging Julia's scientific computing ecosystem.

```
┌─────────────────┐
│ BetLang (Racket)│
│   Source Code   │
└────────┬────────┘
         │ Parse/Analyze
         ▼
┌─────────────────┐
│   AST/IR        │
│  Representation │
└────────┬────────┘
         │ Code Generation
         ▼
┌─────────────────┐
│  Julia Code     │
│  (Generated)    │
└────────┬────────┘
         │ Runtime
         ▼
┌─────────────────┐
│ Julia Packages  │
│  (Distributions,│
│   StatsBase,    │
│   etc.)         │
└─────────────────┘
```

## Priority Structure

**Tier 0 (Foundation):** Standard Julia ecosystem - MOST IMPORTANT
**Tier 1 (Extensions):** Hyperpolymath packages - valuable additions
**Tier 2+:** Specialized use cases

---

## Tier 0: Core Julia Ecosystem (FOUNDATION - MOST CRITICAL)

### Must-Have Integrations (Standard Julia Packages)

#### 1. **Distributions.jl** - Probability Distributions
**Why:** Authoritative Julia package for probability distributions
**Maps to:** All betlang distribution operations

**Integration points:**
- `bet` → Sample from `Categorical([1/3, 1/3, 1/3])`
- `bet/weighted` → Sample from `Categorical(normalize(weights))`
- `lib/distributions.rkt` → Direct Distributions.jl types
- Number systems (DistnumberNormal, DistnumberBeta) → `Normal()`, `Beta()`

**Distributions.jl types to use:**
```julia
# Discrete
Categorical, Bernoulli, Binomial, Geometric, Poisson, Multinomial

# Continuous
Normal, Exponential, Gamma, Beta, Uniform, ChiSquare, TDist,
Weibull, Pareto, Cauchy, Laplace, Dirichlet

# Functions
pdf(), cdf(), quantile(), mean(), var(), entropy(), fit()
```

#### 2. **StatsBase.jl** - Statistical Utilities
**Why:** Comprehensive statistical functions
**Maps to:** `lib/statistics.rkt` (180+ functions)

**Integration points:**
- Descriptive stats: `mean()`, `median()`, `mode()`, `var()`, `std()`
- Correlation: `cor()`, `cov()`, `corspearman()`
- Sampling: `sample()`, `wsample()` (weighted sampling)
- Histograms: `fit(Histogram, data)`
- Percentiles: `percentile()`, `quantile()`

#### 3. **Random.jl** (Stdlib) - RNG
**Why:** Deterministic seeding for reproducibility
**Maps to:** `bet-with-seed`, all randomness

**Integration points:**
```julia
using Random
rng = MersenneTwister(seed)
rand(rng, Categorical([0.3, 0.4, 0.3]))
```

### Tier 2: Number Systems Support

#### 4. **IntervalArithmetic.jl** - Interval Arithmetic
**Why:** Native interval types for AffineNumber
**Maps to:** `AffineNumber` in `lib/number-systems.rkt`

```julia
using IntervalArithmetic
x = 18..22  # [18, 22]°C
y = 20..25
z = x + y   # Automatic interval arithmetic
```

#### 5. **Measurements.jl** - Uncertainty Propagation
**Why:** Automatic uncertainty tracking (like DistNumber)
**Maps to:** `DistnumberNormal` operations

```julia
using Measurements
height = 170 ± 10  # 170cm ± 10cm
weight = 75 ± 5
bmi = weight / (height/100)^2  # Automatic error propagation
```

#### 6. **MonteCarloMeasurements.jl** - Particle-Based Uncertainty
**Why:** Alternative to analytic uncertainty propagation
**Maps to:** Advanced number system operations

```julia
using MonteCarloMeasurements
x = 5.0 ± 1.0  # Creates particle distribution
y = 3.0 ± 0.5
z = x * y  # Monte Carlo propagation
```

### Tier 3: Bayesian/MCMC Integration

#### 7. **Turing.jl** - Probabilistic Programming
**Why:** Sister language! Can interoperate or compete
**Maps to:** `lib/bayesian.rkt` advanced features

**Comparison:**
```julia
# Turing.jl model
@model coin_flip(y) = begin
    p ~ Beta(1, 1)
    y .~ Bernoulli(p)
end

# BetLang equivalent (compiled to Julia using Turing backend)
(define model
  (bayesian-model
    (prior 'p (beta-dist 1 1))
    (likelihood 'y (bernoulli-dist p))))
```

**Decision:** Turing.jl could be:
- **Option A:** Backend for betlang Bayesian inference (use Turing's MCMC)
- **Option B:** Alternative target (compile betlang → Turing.jl models)
- **Option C:** Competition (betlang implements own MCMC, benchmarks against Turing)

**Recommendation:** Option A - Use Turing.jl as backend for MCMC samplers

#### 8. **AdvancedMH.jl** - MCMC Samplers
**Why:** Metropolis-Hastings, Gibbs, HMC implementations
**Maps to:** `lib/bayesian.rkt` MCMC functions

#### 9. **Gen.jl** - Alternative Probabilistic Programming
**Why:** MIT's probabilistic programming framework
**Maps to:** Potential alternative backend

### Tier 4: Financial/Risk Applications

#### 10. **TimeSeries.jl** - Time Series Analysis
**Why:** Financial modeling with temporal data
**Maps to:** `examples/finance.rkt` time series functions

#### 11. **Optim.jl** - Optimization
**Why:** Portfolio optimization, parameter fitting
**Maps to:** `lib/optimization.rkt`

```julia
using Optim
# Kelly criterion optimization
f(x) = -expected_log_wealth(x, probabilities, payoffs)
result = optimize(f, 0.0, 1.0)  # Optimal bet fraction
```

#### 12. **JuMP.jl** - Mathematical Optimization
**Why:** Constrained optimization (risk limits, portfolio constraints)
**Maps to:** Advanced portfolio optimization

```julia
using JuMP, HiGHS
model = Model(HiGHS.Optimizer)
@variable(model, 0 <= stake <= max_stake)
@objective(model, Max, expected_value(stake))
@constraint(model, risk_of_ruin(stake) <= 0.05)
optimize!(model)
```

### Tier 5: Advanced Mathematics

#### 13. **SpecialFunctions.jl** - Special Functions
**Why:** Gamma, Beta, Bessel functions for distributions
**Maps to:** Distribution internals

#### 14. **LinearAlgebra.jl** (Stdlib) - Matrix Operations
**Why:** Multivariate distributions, covariance matrices
**Maps to:** Multivariate statistics

#### 15. **Symbolics.jl** - Symbolic Math
**Why:** Symbolic uncertainty propagation
**Maps to:** Advanced number systems (SurrealAdvanced, PAdicAdvanced)

## Compilation Strategy

### Phase 1: Direct Translation (Simple)

**Input (BetLang):**
```racket
(bet 'heads 'tails 'edge)
```

**Output (Julia):**
```julia
using Distributions, Random
sample(Categorical([1/3, 1/3, 1/3]), 1)[1]  # Returns :heads, :tails, or :edge
```

### Phase 2: Function Translation

**Input (BetLang):**
```racket
(define (coin-flip-game trials)
  (bet-parallel trials 'heads 'tails 'edge))
```

**Output (Julia):**
```julia
function coin_flip_game(trials::Int)
    dist = Categorical([1/3, 1/3, 1/3])
    [sample(dist) for _ in 1:trials]
end
```

### Phase 3: Number System Mapping

**Input (BetLang):**
```racket
(define height (make-distnumber-normal 170 10))
(define weight (make-distnumber-normal 75 5))
(define bmi (distnumber-div weight (distnumber-mul height height)))
```

**Output (Julia):**
```julia
using Measurements
height = 170.0 ± 10.0
weight = 75.0 ± 5.0
bmi = weight / height^2
```

### Phase 4: Safety Features Integration

**Dutch Book Prevention:**
```julia
function validate_probabilities(probs::Vector{Float64}; tol=1e-6)
    total = sum(probs)
    if abs(total - 1.0) > tol
        throw(DutchBookError("Probabilities sum to $total, not 1.0"))
    end
    return probs
end
```

**Risk-of-Ruin Protection:**
```julia
using Distributions
function kelly_fraction(p::Float64, b::Float64, kelly_fraction::Float64=0.25)
    edge = p * b - (1 - p)
    full_kelly = edge / b
    return full_kelly * kelly_fraction  # Fractional Kelly
end
```

**Cool-Off Mechanism:**
```julia
mutable struct CoolOffTracker
    last_bet_time::Float64
    cool_off_seconds::Float64
    violations::Int
end

function bet_with_cooloff!(tracker::CoolOffTracker, bet_fn::Function)
    now = time()
    elapsed = now - tracker.last_bet_time
    if elapsed < tracker.cool_off_seconds
        tracker.violations += 1
        error("Cool-off active. Wait $(tracker.cool_off_seconds - elapsed)s")
    end
    tracker.last_bet_time = now
    return bet_fn()
end
```

## Performance Benefits

### Why Julia Backend?

1. **Speed:** 10-100x faster than Racket for numerical code
2. **Ecosystem:** Mature scientific computing libraries
3. **Integration:** Easy FFI with C/Fortran/Python
4. **JIT:** Compiled to native code, not interpreted
5. **Parallelism:** Native multi-threading, GPU support
6. **Type Stability:** Optional static typing for performance

### Benchmarks (Expected)

| Operation | Racket | Julia | Speedup |
|-----------|--------|-------|---------|
| Monte Carlo (1M samples) | 15s | 0.2s | 75x |
| Matrix operations | 8s | 0.1s | 80x |
| MCMC sampling (10K) | 120s | 2s | 60x |
| Distribution sampling | 5s | 0.05s | 100x |

## Implementation Roadmap

### Step 1: Minimal Viable Backend (2 weeks)
- [ ] Parse betlang AST from Racket
- [ ] Generate Julia code for basic `bet` operations
- [ ] Map to Distributions.jl Categorical
- [ ] Write generated code to `.jl` file
- [ ] Execute via `julia generated.jl`

### Step 2: Core Language (4 weeks)
- [ ] Translate all bet primitives (bet, bet/weighted, bet/conditional)
- [ ] Function definitions and calls
- [ ] Variable bindings (let, define)
- [ ] Control flow (if, cond)
- [ ] List operations (map, filter, fold)

### Step 3: Standard Library (6 weeks)
- [ ] Map `lib/statistics.rkt` → StatsBase.jl
- [ ] Map `lib/distributions.rkt` → Distributions.jl
- [ ] Map `lib/bayesian.rkt` → Turing.jl/AdvancedMH.jl
- [ ] Map `lib/optimization.rkt` → Optim.jl

### Step 4: Number Systems (4 weeks)
- [ ] DistnumberNormal → Measurements.jl
- [ ] AffineNumber → IntervalArithmetic.jl
- [ ] BayesianNumber → Custom type
- [ ] RiskNumber → Custom type with Distributions.jl
- [ ] Other 10 systems → Custom types

### Step 5: Safety Features (2 weeks)
- [ ] Dutch book validation (Julia-side checks)
- [ ] Risk-of-ruin protection (Kelly criterion)
- [ ] Cool-off mechanism (timer struct)

### Step 6: Performance Optimization (4 weeks)
- [ ] Type stability analysis
- [ ] Precompilation of common operations
- [ ] GPU acceleration for Monte Carlo
- [ ] Multi-threading for parallel bets

**Total Estimated Time:** 22 weeks (~5 months)

## Build System Integration

### Justfile Recipes

```just
# Build Julia backend
build-julia:
    racket compiler/betlang-to-julia.rkt examples/basic.bet --output build/basic.jl
    julia build/basic.jl

# Run tests with Julia backend
test-julia:
    racket tests/julia-backend-tests.rkt
    julia --project tests/run_tests.jl

# Benchmark Julia vs Racket
benchmark:
    racket benchmarks/performance.rkt --backend racket
    racket compiler/betlang-to-julia.rkt benchmarks/performance.bet
    julia benchmarks/performance.jl
    julia benchmarks/compare.jl
```

## Package Structure

```
betlang-julia/
├── Project.toml          # Julia package metadata
├── src/
│   ├── BetLang.jl       # Main module
│   ├── core.jl          # Core bet primitives
│   ├── distributions.jl # Distribution mappings
│   ├── statistics.jl    # Statistical functions
│   ├── bayesian.jl      # Bayesian inference
│   ├── optimization.jl  # Optimization algorithms
│   ├── number_systems.jl # 14 number systems
│   └── safety.jl        # Safety features
├── test/
│   └── runtests.jl
└── examples/
    ├── basic.jl
    ├── finance.jl
    └── monte_carlo.jl
```

### Project.toml

```toml
name = "BetLang"
uuid = "..."
authors = ["Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>"]
version = "0.8.0"

[deps]
Distributions = "31c24e10-a181-5473-b8eb-7969acd0382f"
StatsBase = "2913bbd2-ae8a-5f71-8c99-4fb6c76f3a91"
Random = "9a3f8284-a2c9-5f02-9a11-845980a1fd5c"
IntervalArithmetic = "d1acc4aa-44c8-5952-acd4-ba5d80a2a253"
Measurements = "eff96d63-e80a-5855-80a2-b1b0885c5ab7"
Turing = "fce5fe82-541a-59a6-adf8-730c64b5f9a0"
Optim = "429524aa-4258-5aef-a3af-852621145aeb"

[compat]
julia = "1.9"
Distributions = "0.25"
StatsBase = "0.34"
```

## Integration with Existing Ecosystem

### Aggregate Library Bridge

**Goal:** BetLang number systems → Aggregate library (via Julia FFI)

```julia
# BetLang.jl provides C ABI for aggregate-library
function distnumber_to_c_array(dn::DistnumberNormal)
    samples = [rand(Normal(dn.mean, dn.std)) for _ in 1:1000]
    return pointer(samples), length(samples)
end
```

### R Integration (Future)

Via RCall.jl:
```julia
using RCall
R"""
library(betlang)  # R package wrapping BetLang.jl
result <- bet(c('heads', 'tails', 'edge'))
"""
```

### Python Integration (Future)

Via PyCall.jl or PythonCall.jl:
```python
import juliacall
betlang = juliacall.newmodule("BetLang")
result = betlang.bet(['heads', 'tails', 'edge'])
```

## Alternatives Considered

### 1. Compile to C/Rust instead?
**Pros:** Maximum performance
**Cons:** No ecosystem, must reimplement everything
**Verdict:** Julia is better - ecosystem + performance

### 2. Use Turing.jl directly instead of betlang?
**Pros:** Mature, well-tested
**Cons:** Doesn't have ternary philosophy, safety features, or number systems
**Verdict:** BetLang offers unique value, can use Turing as backend

### 3. Keep Racket-only?
**Pros:** Simplest, already works
**Cons:** Too slow for large-scale simulations (millions of samples)
**Verdict:** Julia backend is valuable for performance-critical users

## Success Metrics

### v0.8.0 Definition of Done

- [ ] Compile basic betlang programs to Julia
- [ ] All `examples/*.rkt` files compile and run
- [ ] Performance: 50x+ speedup on Monte Carlo benchmarks
- [ ] Integration: Distributions.jl, StatsBase.jl working
- [ ] Tests: 100% of Racket test suite passes on Julia backend
- [ ] Documentation: Tutorial for using Julia backend

### Performance Goals

- Monte Carlo (1M samples): < 1 second
- MCMC sampling (10K iterations): < 5 seconds
- Portfolio optimization: < 100ms
- Distribution operations: < 1μs per sample

## Questions for User

1. **Priority order:** Which tier of packages should we focus on first?
2. **Turing.jl relationship:** Backend, target, or competitor?
3. **Number systems:** Implement all 14 in Julia, or just core 5?
4. **Timeline:** Is 5 months acceptable, or should we compress?
5. **Use cases:** What specific use cases need Julia performance?

## References

- Distributions.jl: https://juliastats.org/Distributions.jl/stable/
- StatsBase.jl: https://juliastats.org/StatsBase.jl/stable/
- Turing.jl: https://turinglang.org/stable/
- Measurements.jl: https://juliaphysics.github.io/Measurements.jl/stable/
- IntervalArithmetic.jl: https://juliaintervals.github.io/IntervalArithmetic.jl/stable/

---

**Next Steps:**
1. Get feedback on architecture
2. Prototype minimal compiler (Step 1)
3. Benchmark proof-of-concept
4. Iterate based on performance data
