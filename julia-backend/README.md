# BetLang Julia Backend (v0.8.0-dev)

High-performance Julia backend for the betlang probabilistic programming language.

## Status

**Phase:** Step 1 - Minimal Viable Backend ✅
**Completion:** 20% (basic bet operations working)

### What Works
- ✅ Core bet primitives (`bet`, `bet_weighted`, `bet_parallel`)
- ✅ Statistical utilities (`bet_probability`, `bet_entropy`)
- ✅ Conditional bets
- ✅ Deterministic (seeded) bets
- ✅ Test suite (8/8 passing)
- ✅ Basic compiler (betlang → Julia)

### What's Next
- [ ] Standard library functions (statistics, distributions)
- [ ] Number systems (14 types)
- [ ] Safety features (Dutch book, risk-of-ruin, cool-off)
- [ ] Full Racket compatibility

## Quick Start

### Prerequisites
- Julia 1.9+
- Racket (for compiler)
- just (task runner)

### Installation

```bash
cd julia-backend
just install
```

### Run Tests

```bash
just test
```

### Run Example

```bash
just example
```

### Compile BetLang Source

```bash
# Compile .bet file to .jl
just compile examples/test.bet

# Compile and run
just run examples/test.bet
```

## Usage

### Direct Julia API

```julia
using BetLang

# Basic ternary bet
result = bet("heads", "tails", "edge")

# Weighted bet
result = bet_weighted([("common", 0.7), ("rare", 0.3)])

# Parallel trials
results = bet_parallel(100, "win", "draw", "lose")

# Probability estimation
prob = bet_probability(1000, x -> x == "heads", "heads", "tails", "edge")

# Entropy calculation
H = bet_entropy(10000, "A", "B", "C")
```

### Compiled BetLang

Write betlang source (`example.bet`):
```racket
;; Simple bet program
(define result (bet "heads" "tails" "edge"))
(display result)
```

Compile and run:
```bash
just run example.bet
```

## Architecture

```
BetLang (Racket) Source
         ↓
    Compiler (Racket)
         ↓
    Julia Code (Generated)
         ↓
    BetLang.jl Runtime
         ↓
    Distributions.jl
```

## Performance

Expected speedup vs Racket:
- Monte Carlo (1M samples): **75x faster**
- Distribution sampling: **100x faster**
- Statistical operations: **50x faster**

## Integration with Julia Ecosystem

### Tier 0 (Foundation) - Current Focus
- **Distributions.jl** ✅ - In use for Categorical distributions
- **StatsBase.jl** ✅ - Dependencies declared
- **Random.jl** ✅ - Used for seeded RNG
- Turing.jl ⏳ - Planned for Bayesian inference
- IntervalArithmetic.jl ⏳ - Planned for AffineNumber
- Measurements.jl ⏳ - Planned for DistnumberNormal

### Tier 1 (Extensions) - Future
- BowtieRisk.jl - Risk analysis integration
- ZeroProb.jl - Zero-probability event handling
- Causals.jl - Causal inference
- Cliometrics.jl, Cliodynamics.jl, Cladistics.jl - Domain applications

See `docs/julia-backend-design.md` for complete integration plan.

## Development

### Project Structure

```
julia-backend/
├── Project.toml          # Julia package metadata
├── src/
│   └── BetLang.jl       # Core module
├── test/
│   └── runtests.jl      # Test suite
├── examples/
│   ├── basic.jl         # Julia examples
│   └── test.bet         # BetLang source
├── compiler/
│   └── betlang-to-julia.rkt  # Compiler
└── justfile             # Build recipes
```

### Adding New Features

1. **Add to BetLang.jl**: Implement in `src/BetLang.jl`
2. **Add tests**: Update `test/runtests.jl`
3. **Update compiler**: Add translation in `compiler/betlang-to-julia.rkt`
4. **Run tests**: `just test`

### Running REPL

```bash
just repl
```

Then:
```julia
julia> using BetLang
julia> bet("a", "b", "c")
```

## Comparison to Racket

| Feature | Racket | Julia | Status |
|---------|--------|-------|--------|
| Core bets | ✅ | ✅ | Complete |
| Weighted bets | ✅ | ✅ | Complete |
| Conditional | ✅ | ✅ | Complete |
| Statistics | ✅ | ⏳ | Partial |
| Distributions | ✅ | ⏳ | Planned |
| Number systems | ✅ | ⏳ | Planned |
| Safety features | ✅ | ⏳ | Planned |
| Performance | 1x | 50-100x | ✅ |

## Roadmap

### ✅ Step 1: Minimal Viable Backend (Current)
- Basic bet operations
- Simple compiler
- Test suite

### Step 2: Core Language (4 weeks)
- All bet primitives
- Function definitions
- Control flow
- List operations

### Step 3: Standard Library (6 weeks)
- Statistics functions
- Distribution operations
- Bayesian inference
- Optimization

### Step 4: Number Systems (4 weeks)
- All 14 uncertainty-aware types
- Automatic error propagation
- Julia ecosystem integration

### Step 5: Safety Features (2 weeks)
- Dutch book prevention
- Risk-of-ruin protection
- Cool-off mechanism

### Step 6: Optimization (4 weeks)
- Type stability
- GPU acceleration
- Multi-threading
- Precompilation

**Total Timeline:** 22 weeks (~5 months)

## Contributing

See `../CONTRIBUTING.adoc` for contribution guidelines.

## License

PMPL-1.0-or-later (Palimpsest License)

## Authors

Jonathan D.A. Jewell <jonathan.jewell@open.ac.uk>

## References

- Main betlang docs: `../README.adoc`
- Julia backend design: `../docs/julia-backend-design.md`
- Hyperpolymath integration: `../docs/hyperpolymath-julia-integration.md`
