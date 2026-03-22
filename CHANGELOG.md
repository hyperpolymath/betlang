# Changelog

All notable changes to betlang will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- RSR (Rhodium Standard Repository) compliance improvements
- `.gitignore` for Racket projects
- `CONTRIBUTING.md` with comprehensive contribution guidelines
- `SECURITY.md` with security policy and vulnerability reporting
- `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1)
- `MAINTAINERS.md` with project maintainer information
- `.github/workflows/test.yml` for automated CI/CD testing
- `info.rkt` for Racket package metadata
- `.well-known/` directory with RFC 9116 compliant files
- `TPCF.md` for Tri-Perimeter Contribution Framework

## [0.1.0] - 2025-11-22

### Added

#### Core Language (`core/betlang.rkt`)
- `bet` - Basic ternary bet primitive
- `bet/weighted` - Weighted probability distributions
- `bet/conditional` - Conditional betting
- `bet/lazy` - Lazy evaluation for bets
- `bet-chain` - Chain bets together
- `bet-compose` - Compose functions into bets
- `bet-map`, `bet-fold`, `bet-filter` - List operations with bets
- `bet-parallel` - Run multiple parallel trials
- `bet-repeat`, `bet-until` - Iteration primitives
- `bet-with-seed` - Deterministic randomness
- `bet-probability`, `bet-entropy`, `bet-expect` - Statistical utilities
- `all-bets`, `make-bet-generator` - Utility functions

#### Libraries

##### Statistics (`lib/statistics.rkt`)
- Descriptive statistics: `mean`, `median`, `mode`, `variance`, `stddev`
- Correlation: `covariance`, `correlation`
- Distributions: `percentile`, `z-score`, `normalize`, `histogram`
- Statistical tests: `chi-square-test`, `kolmogorov-smirnov`
- Resampling: `bootstrap`, `jackknife`
- Time series: `moving-average`, `exponential-smoothing`
- Monte Carlo: `run-simulation`, `monte-carlo-pi`

##### Distributions (`lib/distributions.rkt`)
- Discrete: `uniform`, `bernoulli`, `binomial`, `geometric`, `poisson`
- Continuous: `normal`, `exponential`, `gamma`, `beta`, `chi-square`
- Advanced: `student-t`, `f-distribution`, `weibull`, `pareto`, `cauchy`, `laplace`
- Multivariate: `multinomial`, `dirichlet`, `categorical`, `zipf`
- Stochastic processes: `random-walk`, `brownian-motion`, `levy-flight`

##### Bayesian Inference (`lib/bayesian.rkt`)
- Bayes' theorem utilities
- Conjugate priors: `conjugate-beta-binomial`, `conjugate-normal`
- MCMC: `metropolis-hastings`, `gibbs-sampler`
- Sampling: `importance-sampling`, `rejection-sampling`
- ABC: `abc-algorithm`
- Inference: `bayesian-update`, `credible-interval`, `bayes-factor`
- Posterior predictive: `posterior-predictive`, `empirical-bayes`

##### Optimization (`lib/optimization.rkt`)
- `simulated-annealing` - Simulated annealing algorithm
- `genetic-algorithm` - Genetic algorithm with ternary operations
- `particle-swarm` - Particle swarm optimization
- `hill-climbing` - Hill climbing with ternary moves
- `random-search` - Random search baseline
- `evolutionary-strategy` - Evolution strategies
- `cross-entropy-method` - Cross-entropy optimization
- `ternary-search` - Ternary search for unimodal functions

##### Advanced Sampling (`lib/sampling.rkt`)
- `stratified-sampling` - Stratified sampling
- `latin-hypercube-sampling` - LHS for high dimensions
- `sobol-sequence`, `halton-sequence` - Quasi-random sequences
- `importance-resampling` - Importance sampling with resampling
- `sequential-monte-carlo` - Particle filters
- `slice-sampling` - Slice sampling for MCMC
- `hamiltonian-monte-carlo` - HMC (simplified)
- `ternary-sampling` - betlang-specific sampling
- Variance reduction: `antithetic-variates`, `control-variates`

##### Markov Chains (`lib/markov.rkt`)
- `make-markov-chain`, `markov-step`, `markov-simulate`
- `markov-stationary` - Stationary distribution estimation
- `estimate-transitions` - Learn transition matrix from data
- `markov-classify` - Sequence classification
- `hmm-viterbi` - Hidden Markov Model Viterbi algorithm
- `generate-text-markov` - Text generation
- `ternary-markov` - Ternary-specific Markov chains

##### Combinators (`lib/combinators.rkt`)
- Monadic: `bet-pure`, `bet-bind`, `bet-join`, `bet-lift`, `bet-ap`
- Logical: `bet-or`, `bet-and`, `bet-xor`, `bet-not`
- Error handling: `bet-try`, `bet-guard`, `bet-retry`, `bet-fallback`
- Conditional: `bet-when`, `bet-unless`, `bet-cond`, `bet-case`
- Performance: `bet-memoize`, `bet-cache`, `bet-throttle`, `bet-debounce`
- Composition: `bet-pipeline`, `bet-fork-join`, `bet-race`

##### Ternary Logic (`lib/ternary.rkt`)
- `ternary-and`, `ternary-or`, `ternary-not`, `ternary-xor`
- `ternary-implies`, `ternary-equiv`
- `ternary-majority`, `ternary-consensus`
- `ternary-min`, `ternary-max`, `ternary-median`
- `ternary-fold`, `ternary-map`, `ternary-filter`, `ternary-reduce`
- `make-ternary-table` - Truth table generation
- Constants: `TRUE`, `FALSE`, `UNKNOWN`

#### Examples

- `examples/basic-tutorial.rkt` - Step-by-step tutorial with 10 sections
- `examples/monte-carlo.rkt` - 7 Monte Carlo simulations
- `examples/game-theory.rkt` - 6 game theory applications
- `examples/finance.rkt` - 10 financial modeling examples
- `examples/probabilistic-structures.rkt` - 7 probabilistic data structures

#### Tools

- `tools/analyzer.rkt` - Bet analysis and visualization
  - `analyze-bet`, `compare-bets`, `convergence-analysis`
  - `text-histogram`, `probability-report`, `entropy-analysis`
- `benchmarks/performance.rkt` - 15 performance benchmarks

#### REPL (`repl/shell.rkt`)
- Interactive shell with help system
- Commands: `:help`, `:stats`, `:reset-stats`, `:history`, `:examples`, `:quit`
- Session statistics tracking
- Command history
- Error handling and pretty-printing
- Persistent logging

#### Documentation

- `README.md` - Comprehensive project overview (285 lines)
- `docs/tutorial.md` - Complete tutorial (500+ lines)
- `docs/semantics.md` - Formal language semantics (340+ lines)
- `docs/api-reference.md` - API documentation (600+ lines)
- `docs/architecture.md` - Project structure
- `CLAUDE.md` - AI assistant context

#### Tests

- `tests/basics.rkt` - 25 comprehensive test cases
  - Basic functionality tests
  - Probability distribution tests
  - Statistical function tests
  - Edge case coverage

## [0.0.1] - 2025-07-28

### Added
- Initial prototype
- Basic `bet` primitive
- Simple REPL
- Minimal documentation

---

## Version History

- **0.1.0** (2025-11-22): Major expansion - Full-featured probabilistic programming language
- **0.0.1** (2025-07-28): Initial prototype

## Migration Guide

### From 0.0.1 to 0.1.0

**Breaking Changes:** None (0.0.1 was minimal prototype)

**New Features:**
- All features listed above are new in 0.1.0
- Core `bet` primitive remains unchanged
- Fully backward compatible

**Recommended Updates:**
1. Explore new libraries (`lib/*.rkt`)
2. Review examples for usage patterns
3. Read tutorial for comprehensive guide
4. Check API reference for function signatures

---

**Note:** betlang follows Semantic Versioning:
- **MAJOR**: Incompatible API changes
- **MINOR**: Backward-compatible functionality
- **PATCH**: Backward-compatible bug fixes

[unreleased]: https://github.com/yourusername/betlang/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/betlang/releases/tag/v0.1.0
[0.0.1]: https://github.com/yourusername/betlang/releases/tag/v0.0.1
