# BetLang Integration with Hyperpolymath Julia Packages

**Status:** Planning Phase (2026-02-07)
**Purpose:** Connect betlang to existing hyperpolymath Julia ecosystem

## IMPORTANT: Priority Clarification

**This document covers Tier 1 (Extensions) - these are additions to the core Julia ecosystem, not replacements.**

### Priority Order (Most → Least Critical):

**Tier 0 (Foundation) - MOST IMPORTANT:**
- Distributions.jl, StatsBase.jl, Turing.jl, Optim.jl, etc.
- See `julia-backend-design.md` for comprehensive Tier 0 coverage
- **These must be implemented first** - they're mature, essential packages

**Tier 1 (This Document) - Valuable Extensions:**
- BowtieRisk.jl, ZeroProb.jl, Causals.jl, domain packages
- Built **on top of** Tier 0 foundation
- Add domain-specific capabilities

**Tier 2+ - Specialized:**
- As needed for specific use cases

## Overview

Once BetLang has solid integration with standard Julia packages (Tier 0), it can *also* leverage your existing hyperpolymath packages to create a comprehensive probabilistic programming + risk analysis + causal inference ecosystem.

## Your Julia Packages (Discovered)

### Tier 1: Direct BetLang Integration (High Synergy)

#### 1. **BowtieRisk.jl** - Bowtie Risk Modeling ⭐⭐⭐
**Purpose:** Hazard analysis with threats, barriers, and consequences
**Current deps:** Distributions.jl, JSON3

**Integration with BetLang:**

**Synergy Points:**
- **Risk-of-Ruin Protection** → Bowtie consequence analysis
- **Monte Carlo Simulation** → Bowtie barrier effectiveness
- **Probability Validation** → Dutch book prevention for threat probabilities

**Example Integration:**
```julia
using BetLang, BowtieRisk

# Define a gambling bowtie model
model = BowtieModel(
    hazard = "Financial Ruin",
    threats = [
        Threat("High Stakes", probability=0.3),
        Threat("Rapid Betting", probability=0.4),
        Threat("Chasing Losses", probability=0.5)
    ],
    barriers = [
        # BetLang safety features as barriers!
        Barrier("Dutch Book Prevention", effectiveness=0.95),
        Barrier("Risk-of-Ruin Check", effectiveness=0.90),
        Barrier("Cool-Off Mechanism", effectiveness=0.85),
        Barrier("Kelly Criterion", effectiveness=0.92)
    ],
    consequences = [
        Consequence("Bankruptcy", severity=10.0, probability=0.8)
    ]
)

# Run BetLang-powered Monte Carlo simulation
results = simulate_bowtie(model, n=100_000)
# Uses BetLang's bet primitives for sampling

# Validate no Dutch books in threat probabilities
threats_probs = [t.probability for t in model.threats]
validate_dutch_book(threats_probs)  # BetLang function
```

**New Features to Add to BowtieRisk:**
- `BetLangBarrier` type - barriers implemented as betlang functions
- `gambling_harm_reduction_template()` - pre-built model for gambling risks
- `monte_carlo_with_betlang()` - use betlang's uncertainty numbers

#### 2. **ZeroProb.jl** - Zero-Probability Event Handling ⭐⭐⭐
**Purpose:** Handle continuous probability spaces with measure-zero events
**Current deps:** Distributions.jl, StatsBase, Plots, Makie

**Integration with BetLang:**

**Synergy Points:**
- **Continuous Distributions** → betlang's DistnumberNormal, DistnumberBeta
- **Theoretical Rigor** → Formal foundations for betlang's probability theory
- **Edge Cases** → Handling degenerate cases in number systems

**Example Integration:**
```julia
using BetLang, ZeroProb

# BetLang distribution as continuous space
height_dist = DistnumberNormal(170, 10)

# Query exact point (has zero probability but can occur)
point = 175.5
density = density_ratio(height_dist, point)  # ZeroProb relevance measure

# ε-neighborhood probability (BetLang + ZeroProb)
prob_near_175 = epsilon_neighborhood_prob(height_dist, 175.5, ε=0.1)
# P(175.4 < height < 175.6) ≈ 0.008

# Hausdorff measure for zero-probability sets
fractal_support = hausdorff_measure(
    betlang_distribution_support(height_dist),
    dimension=1.5
)
```

**New Features to Add to ZeroProb:**
- `DistnumberAdapter` - wrap betlang number systems
- `zero_prob_bet()` - betting on measure-zero events
- `continuous_game_theory()` - combine with betlang game theory examples

#### 3. **Causals.jl** - Causal Inference ⭐⭐
**Purpose:** Dempster-Shafer, DAGs, do-calculus, counterfactuals
**Current deps:** (need to check)

**Integration with BetLang:**

**Synergy Points:**
- **Dempster-Shafer** → betlang already has DempsterShafer number system!
- **Counterfactuals** → "What if I had bet differently?" analysis
- **Causal DAGs** → Model causal relationships in probabilistic systems
- **Do-Calculus** → Interventions in gambling scenarios

**Example Integration:**
```julia
using BetLang, Causals

# Model causal structure of gambling outcomes
dag = CausalDAG()
add_nodes!(dag, [:stake, :skill, :luck, :outcome, :bankroll])
add_edges!(dag, [
    :stake => :outcome,
    :skill => :outcome,
    :luck => :outcome,
    :outcome => :bankroll
])

# Counterfactual: "What if I had bet less?"
actual_outcome = simulate_bet(stake=1000, skill=0.6, luck=0.4)
counterfactual = intervene(dag, :stake => 500)
cf_outcome = simulate_bet(counterfactual)

# Compare using betlang's number systems
diff = imprecise_probability(
    lower = min(actual_outcome, cf_outcome),
    upper = max(actual_outcome, cf_outcome)
)
```

**New Features to Add to Causals:**
- `gambling_dag_template()` - standard causal models for gambling
- `betlang_counterfactual()` - counterfactual reasoning with bets
- `causal_number_systems()` - propagate uncertainty through DAGs

### Tier 2: Domain-Specific Extensions

#### 4. **HackenbushGames.jl** - Hackenbush Game Theory ⭐
**Purpose:** Combinatorial game theory (surreal numbers!)
**Current deps:** None

**Integration with BetLang:**

**Synergy Points:**
- **Surreal Numbers** → betlang has SurrealAdvanced number system!
- **Game Theory** → betlang has `examples/game-theory.rkt`
- **Ternary Games** → Hackenbush with 3-player variants

**Example Integration:**
```julia
using BetLang, HackenbushGames

# Hackenbush position as surreal number
position = analyze_hackenbush(blue_edges, red_edges)
surreal_value = to_surreal(position)  # e.g., {0,1|2,3}

# Convert to BetLang SurrealAdvanced
betlang_surreal = SurrealAdvanced(
    left_set = surreal_value.left,
    right_set = surreal_value.right
)

# Use in betlang game theory
game = ternary_game(
    payoffs = betlang_surreal,
    players = 3
)
nash_equilibrium(game)
```

**New Features to Add to HackenbushGames:**
- `ternary_hackenbush()` - 3-player variant (blue/red/green)
- `surreal_to_betlang()` - convert to BetLang number system
- `probabilistic_hackenbush()` - edges fail with probability

#### 5. **Cliometrics.jl** - Quantitative Economic History ⭐
**Purpose:** Growth analysis, convergence, institutional quality
**Current deps:** (recently implemented)

**Integration with BetLang:**

**Synergy Points:**
- **Uncertainty in Historical Data** → betlang number systems
- **Monte Carlo Historical Simulation** → counterfactual histories
- **Economic Risk Analysis** → combine with BowtieRisk

**Example Integration:**
```julia
using BetLang, Cliometrics

# Historical GDP with uncertainty
gdp_1900 = DistnumberNormal(1000, 100)  # BetLang uncertainty
gdp_2000 = DistnumberNormal(50000, 5000)

# Growth rate with error propagation
growth_rate = calculate_growth_rates(
    [gdp_1900, gdp_2000],
    years = [1900, 2000]
)
# Result: DistnumberNormal(3.9%, 0.2%) - uncertainty propagated!

# Counterfactual: "What if industrial revolution started earlier?"
counterfactual_trajectory = bet(
    scenario_a = early_industrialization_path(),
    scenario_b = actual_path(),
    scenario_c = late_industrialization_path()
)
```

#### 6. **Cliodynamics.jl** - Mathematical Historical Dynamics ⭐
**Purpose:** Turchin's models, elite overproduction, secular cycles
**Current deps:** (recently implemented)

**Integration with BetLang:**

**Synergy Points:**
- **Stochastic Historical Models** → betlang Monte Carlo
- **Uncertainty in Parameters** → betlang number systems
- **Probabilistic Forecasting** → crisis prediction with confidence

**Example Integration:**
```julia
using BetLang, Cliodynamics

# Model parameters with uncertainty
elite_size = DistnumberBeta(8, 2)  # High elite concentration
state_capacity = DistnumberNormal(0.7, 0.1)  # Weakening state

# Run probabilistic cliodynamic simulation
trajectories = bet_repeat(10_000) do
    simulate_secular_cycle(
        elite_size = sample(elite_size),
        state_capacity = sample(state_capacity),
        years = 100
    )
end

# Crisis probability distribution
crisis_probs = [trajectory.crisis_probability for trajectory in trajectories]
mean_crisis_prob = mean(crisis_probs)
ci_95 = percentile(crisis_probs, [2.5, 97.5])
```

#### 7. **Cladistics.jl** - Phylogenetic Analysis
**Purpose:** UPGMA, neighbor-joining, maximum parsimony, bootstrap
**Current deps:** (recently implemented)

**Integration with BetLang:**

**Synergy Points:**
- **Bootstrap Support** → Monte Carlo uncertainty quantification
- **Uncertain Branch Lengths** → betlang number systems
- **Probabilistic Trees** → distribution over tree topologies

**Example Integration:**
```julia
using BetLang, Cladistics

# Distance matrix with measurement uncertainty
distances = [
    DistnumberNormal(0, 0) DistnumberNormal(3, 0.5) DistnumberNormal(5, 0.7);
    DistnumberNormal(3, 0.5) DistnumberNormal(0, 0) DistnumberNormal(4, 0.6);
    DistnumberNormal(5, 0.7) DistnumberNormal(4, 0.6) DistnumberNormal(0, 0)
]

# Build tree with uncertainty propagation
trees = bet_repeat(1000) do
    D = sample.(distances)  # Sample from each distance uncertainty
    upgma(D)
end

# Consensus tree with confidence intervals
consensus = consensus_tree(trees, threshold=0.7)
```

### Tier 3: Theoretical Foundations

#### 8. **KnotTheory.jl** - Knot Theory
**Purpose:** Knot invariants, Jones polynomial, etc.
**Integration:** Possibly via surreal numbers? (Hackenbush-style)

#### 9. **Axiology.jl** - Value Theory
**Purpose:** Formal value systems
**Integration:** Decision theory under uncertainty (utility + betlang)

#### 10. **SMTLib.jl** - SMT Solver Interface
**Purpose:** Satisfiability Modulo Theories
**Integration:** Verify betlang safety properties formally

#### 11. **ProvenCrypto.jl** - Proven Cryptography
**Purpose:** Formally verified crypto
**Integration:** Secure random number generation for betlang

## Proposed Architecture

### Unified Ecosystem: BetLang + Hyperpolymath.jl

```
                    ┌──────────────┐
                    │   BetLang    │
                    │   (Racket)   │
                    └──────┬───────┘
                           │ Compiler
                           ▼
            ┌──────────────────────────┐
            │  BetLang.jl (Julia Core) │
            └──────┬───────────────────┘
                   │
        ┌──────────┼──────────┬──────────┐
        ▼          ▼          ▼          ▼
  ┌─────────┐ ┌──────────┐ ┌────────┐ ┌────────┐
  │BowtieRisk│ │ ZeroProb │ │Causals │ │Clio-   │
  │   .jl    │ │   .jl    │ │  .jl   │ │metrics │
  └─────────┘ └──────────┘ └────────┘ └────────┘
       │            │           │           │
       └────────────┴───────────┴───────────┘
                      │
              ┌───────▼────────┐
              │ Hyperpolymath  │
              │   Julia Stack  │
              └────────────────┘
```

## Implementation Phases

### Phase 1: Core BetLang.jl (v0.8.0)
- Compile betlang → Julia
- Integrate Distributions.jl, StatsBase.jl
- Basic number systems

### Phase 2: Risk Integration (v0.9.0)
- **BowtieRisk.jl** integration
  - BetLang safety features as barriers
  - Monte Carlo with betlang uncertainty
  - Gambling harm reduction templates

- **ZeroProb.jl** integration
  - Handle continuous betlang distributions
  - Measure-zero event reasoning
  - Theoretical rigor for edge cases

### Phase 3: Causal + Domain Extensions (v1.0.0)
- **Causals.jl** integration
  - Dempster-Shafer ↔ betlang number system
  - Counterfactual gambling analysis
  - Causal DAGs for probabilistic models

- **Domain packages** (Cliometrics, Cliodynamics, Cladistics)
  - Use betlang for uncertainty quantification
  - Monte Carlo historical simulations
  - Probabilistic phylogenies

- **Game theory** (HackenbushGames)
  - Surreal number interop
  - Ternary game variants
  - Combinatorial game theory + probability

### Phase 4: Theoretical Foundations (v1.1.0+)
- **SMTLib.jl** - Verify safety properties
- **ProvenCrypto.jl** - Secure RNG
- **KnotTheory.jl** - Advanced surreal arithmetic

## Concrete Use Cases

### Use Case 1: Gambling Risk Management System
**Components:** BetLang + BowtieRisk + Causals

```julia
using BetLang, BowtieRisk, Causals

# 1. Causal model of gambling addiction
addiction_dag = gambling_addiction_dag()  # From Causals.jl

# 2. Bowtie risk model
risk_model = gambling_bowtie(
    barriers = betlang_safety_barriers()  # Dutch book, risk-of-ruin, cool-off
)

# 3. Run counterfactual analysis
actual = simulate_gambling_session(
    bets_per_hour = 60,
    stake_size = 100
)

counterfactual_with_cooloff = intervene(
    addiction_dag,
    :cool_off_seconds => 30  # BetLang cool-off
)

# 4. Compare outcomes
risk_reduction = bowtie_probability_reduction(
    risk_model,
    actual vs counterfactual_with_cooloff
)
# => "Cool-off reduces ruin risk by 73%"
```

### Use Case 2: Historical Counterfactual with Uncertainty
**Components:** BetLang + Cliometrics + Cliodynamics

```julia
using BetLang, Cliometrics, Cliodynamics

# Historical GDP with measurement uncertainty (BetLang)
gdp_1850_britain = DistnumberNormal(2100, 200)  # Million pounds
gdp_1850_france = DistnumberNormal(1900, 250)

# Growth model with uncertainty propagation
growth_trajectory = bet(
    industrial_revolution_earlier = cliometrics_model(start=1750),
    actual_history = cliometrics_model(start=1800),
    industrial_revolution_later = cliometrics_model(start=1850)
)

# Cliodynamic crisis prediction with uncertainty
crisis_prob = cliodynamic_crisis_probability(
    elite_overproduction = DistnumberBeta(7, 3),  # High uncertainty
    state_capacity = DistnumberNormal(0.6, 0.15)
)
# => ImpreciseProbability([0.35, 0.65]) - wide uncertainty band
```

### Use Case 3: Phylogenetic Uncertainty Quantification
**Components:** BetLang + Cladistics + ZeroProb

```julia
using BetLang, Cladistics, ZeroProb

# Genetic distances with measurement error
distances = measure_genetic_distances_with_uncertainty()  # BetLang DistnumberNormal

# Build distribution over tree topologies
tree_distribution = bet_repeat(10_000) do
    D = sample.(distances)
    neighbor_joining(D)
end

# Zero-probability analysis: "What if this exact tree?"
exact_tree = consensus_tree(tree_distribution)
relevance = zero_prob_relevance(tree_distribution, exact_tree)  # ZeroProb measure
```

## Package Interdependencies

```
BetLang.jl (core)
├── Distributions.jl (required)
├── StatsBase.jl (required)
├── BowtieRisk.jl (optional, recommended)
│   └── Uses BetLang uncertainty types
├── ZeroProb.jl (optional, theoretical)
│   └── Extends BetLang continuous distributions
├── Causals.jl (optional)
│   └── DempsterShafer ↔ BetLang.DempsterShafer
├── Cliometrics.jl (optional)
│   └── Uses BetLang number systems for historical data
├── Cliodynamics.jl (optional)
│   └── Uses BetLang Monte Carlo for stochastic models
├── Cladistics.jl (optional)
│   └── Uses BetLang for bootstrap/uncertainty
└── HackenbushGames.jl (optional)
    └── SurrealAdvanced ↔ Hackenbush positions
```

## Next Steps

### Immediate (v0.8 Planning)
1. **Survey your packages** - Read full docs for each
2. **Identify API surface** - What functions should BetLang call?
3. **Design adapters** - BetLang types ↔ your package types
4. **Prototype** - Small proof-of-concept integrations

### Questions to Resolve
1. **BowtieRisk:** Should BetLang safety features be first-class barrier types?
2. **ZeroProb:** Should this be bundled with BetLang.jl or separate?
3. **Causals:** Should Dempster-Shafer be unified between both packages?
4. **Domain packages:** Generic uncertainty interface vs custom integration?
5. **Package naming:** `BetLang.jl` or `BetLangCore.jl` + `BetLangRisk.jl` etc?

### Benefits of Integration

**For BetLang:**
- Instant domain-specific applications (risk, history, phylogenetics)
- Theoretical rigor (ZeroProb, formal verification)
- Real-world use cases for all 14 number systems

**For Your Packages:**
- Uncertainty quantification via BetLang number systems
- Monte Carlo simulation capabilities
- Probabilistic extensions to deterministic models
- Cross-package interoperability

**For Ecosystem:**
- **Unified hyperpolymath Julia stack**
- Probabilistic + Causal + Risk + Domain expertise
- Competitive with Turing.jl but more specialized
- Unique ternary philosophy + safety features

## Repository Organization

### Option A: Monorepo
```
hyperpolymath-julia/
├── BetLang.jl/
├── BowtieRisk.jl/
├── ZeroProb.jl/
├── Causals.jl/
└── ...
```

### Option B: Separate Repos with Registry
```
HyperpolymathJulia Registry
├── BetLang.jl → github.com/hyperpolymath/BetLang.jl
├── BowtieRisk.jl → github.com/hyperpolymath/BowtieRisk.jl
└── ... (current structure)
```

**Recommendation:** Keep separate repos, create `HyperpolymathJulia` metapackage

## Conclusion

Your existing Julia ecosystem is **highly synergistic** with BetLang! Priority integrations:

1. **BowtieRisk.jl** (⭐⭐⭐) - Perfect fit for safety features
2. **ZeroProb.jl** (⭐⭐⭐) - Theoretical foundations
3. **Causals.jl** (⭐⭐) - Dempster-Shafer unification

With these integrations, BetLang becomes the **first probabilistic programming language with integrated risk analysis and causal inference**.
