# Betlang Conformance Test Corpus

This directory contains the canonical conformance test suite for betlang.
Any implementation claiming conformance to betlang semantics MUST pass all
tests in this corpus.

## Test Files

### smoke.bet
Basic smoke test for REPL verification.

```bash
racket repl/shell.rkt < conformance/smoke.bet
```

**Success criteria:**
- Program runs without errors
- Output is produced for each expression
- REPL exits cleanly on `:quit`

### deterministic.rkt
Tests for deterministic behavior that must be identical across implementations.

```bash
racket conformance/deterministic.rkt
```

**Covers:**
- Idempotent bets: `(bet X X X) = X`
- Conditional true-branch: `(bet/conditional #t A B C) = A`
- `all-bets` utility
- Seeded bet reproducibility
- Lazy evaluation (single thunk execution)
- Bet chain and repeat operations
- Entropy calculation edge cases

### stochastic-seeded.rkt
Statistical tests using seeded randomness for reproducibility.

```bash
racket conformance/stochastic-seeded.rkt
```

**Covers:**
- Uniform distribution verification
- Weighted distribution verification
- Probability estimation accuracy
- Expected value calculation
- Composed function distribution
- Entropy convergence to theoretical values
- Full reproducibility across runs

## Running All Tests

```bash
# Smoke test
racket repl/shell.rkt < conformance/smoke.bet

# Deterministic tests
racket conformance/deterministic.rkt

# Stochastic tests
racket conformance/stochastic-seeded.rkt
```

## Conformance Requirements

Per SPEC.core.scm, implementations MUST:

1. **Seedability**: All stochastic tests use `bet-with-seed` for reproducibility
2. **Determinism**: Identical seeds produce identical results
3. **Distribution accuracy**: Statistical properties match formal semantics
4. **Error handling**: Invalid inputs produce deterministic diagnostics

## Adding New Tests

When adding conformance tests:

1. Use `bet-with-seed` for ALL stochastic operations
2. Document expected output/behavior
3. Include in appropriate file (deterministic vs stochastic)
4. Verify test passes on reference implementation (Racket)
