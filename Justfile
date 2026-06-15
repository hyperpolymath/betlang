# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
# betlang - Development Tasks
# AUTHORITY: AUTHORITY_STACK.mustfile-nickel.scm
# All operations MUST be invoked via `just <recipe>`.
# If a recipe does not exist, ADD it here (and document it).

set shell := ["bash", "-uc"]
set dotenv-load := true

project := "betlang"

# ============================================================================
# DEFAULT
# ============================================================================

# Show all available recipes
default:
    @just --list --unsorted

# ============================================================================
# CORE OPERATIONS (Racket - Authoritative)
# ============================================================================

# Run the betlang test suite
test:
    @echo "Running betlang test suite..."
    racket tests/basics.rkt

# Run conformance tests (deterministic)
test-conformance-deterministic:
    @echo "Running deterministic conformance tests..."
    racket conformance/deterministic.rkt

# Run conformance tests (stochastic, seeded)
test-conformance-stochastic:
    @echo "Running seeded stochastic conformance tests..."
    racket conformance/stochastic-seeded.rkt

# Run all conformance tests
test-conformance: test-conformance-deterministic test-conformance-stochastic

# Run full test suite (unit + conformance)
test-all: test test-conformance

# Run smoke test (golden path verification)
smoke:
    @echo "Running smoke test..."
    racket repl/shell.rkt < conformance/smoke.bet

# Start the interactive REPL
repl:
    racket repl/shell.rkt

# Demo: show basic betlang capabilities
demo:
    @echo "=== betlang Demo ==="
    @echo ""
    @echo "Running examples..."
    racket examples/basic-tutorial.rkt
    @echo ""
    @echo "Demo complete. Run 'just repl' for interactive mode."

# ============================================================================
# TOOLING (Rust - Optional, Non-Authoritative)
# ============================================================================

# Build Rust tooling (optional, non-authoritative)
build-tooling:
    @echo "Building optional Rust tooling..."
    @echo "NOTE: These are non-authoritative. See TOOLING.md"
    cargo build --release

# Optimized release build for Betlang v3
build-v3: build-tooling
    @echo "Betlang v3 optimized build: release tooling complete."

# Run Rust tooling tests (optional)
test-tooling:
    @echo "Testing optional Rust tooling..."
    cargo test

# Clean Rust build artifacts
clean-tooling:
    cargo clean

# --- Rust toolchain recipes (the working compiler/interpreter pipeline) ------
# These drive the real multi-crate Rust implementation under compiler/, runtime/
# and tools/. (The Racket recipes above target the historical core/*.rkt tree.)

# Build the entire Rust workspace (all crates)
build-rust:
    cargo build --workspace

# Test the entire Rust workspace
test-rust:
    cargo test --workspace

# Type-check a betlang source file with the Rust checker
check FILE:
    cargo run -q -p bet -- check {{FILE}}

# Run a betlang source file with the Rust interpreter
run FILE:
    cargo run -q -p bet -- run {{FILE}}

# Compile a betlang source file to a backend (TARGET = js | llvm | beam)
compile FILE TARGET="js":
    cargo run -q -p bet -- compile {{FILE}} --target {{TARGET}}

# Start the Rust REPL
repl-rust:
    cargo run -q -p bet -- repl

# ============================================================================
# PROOFS (formal verification — see docs/AFFINESCRIPT-ALIGNMENT.adoc)
# ============================================================================

# Machine-check the Lean 4 formalisation (proofs/BetLang.lean)
proof-check-lean4:
    @echo "Building Lean 4 proofs (lake build)..."
    lake build

# Scan proof sources for banned soundness escape hatches
proof-scan:
    @echo "Scanning proofs for banned patterns..."
    bash tools/proof-scan.sh .

# Run all available proof checks (scan + machine-check)
proof-check-all: proof-scan proof-check-lean4
    @echo "All proof checks complete."

# ============================================================================
# CODE QUALITY
# ============================================================================

# Format Racket code (if raco fmt available)
fmt:
    @echo "Formatting not yet configured for Racket"
    @echo "Consider: raco fmt (if available)"

# Lint Racket code (if raco lint available)
lint:
    @echo "Linting not yet configured for Racket"
    @echo "Consider: raco review (if available)"

# Format Rust tooling code
fmt-tooling:
    cargo fmt

# Lint Rust tooling code
lint-tooling:
    cargo clippy -- -D warnings

# ============================================================================
# BENCHMARKS
# ============================================================================

# Run performance benchmarks
bench:
    @echo "Running performance benchmarks..."
    racket benchmarks/performance.rkt

# ============================================================================
# DOCUMENTATION
# ============================================================================

# Show project status
status:
    @echo "=== Project Status ==="
    @echo ""
    @echo "Authoritative implementation: Racket (core/betlang.rkt)"
    @echo "Formal spec: SPEC.core.scm"
    @echo "Conformance tests: conformance/"
    @echo ""
    @echo "See ANCHOR.scope-arrest.2026-01-01.Jewell.scm for semantic anchor"
    @echo "See AUTHORITY_STACK.mustfile-nickel.scm for operational authority"
    @echo "See TOOLING.md for optional tooling documentation"

# ============================================================================
# CLEAN
# ============================================================================

# Clean all build artifacts and logs
clean:
    @echo "Cleaning build artifacts..."
    rm -rf logs/*.log logs/*.txt
    @echo "Clean complete"

# Clean everything including Rust artifacts
clean-all: clean clean-tooling

# ============================================================================
# CONTAINER (podman-first per authority stack)
# ============================================================================

# Build container image
container-build:
    podman build -f containers/Containerfile -t betlang:latest .

# Build development container
container-build-dev:
    podman build -f containers/Containerfile.dev -t betlang:dev .

# Run container interactively
container-run:
    podman run -it --rm betlang:latest

# ============================================================================
# HELP
# ============================================================================

# Show first-run workflow
first-run:
    @echo "=== First Run Workflow ==="
    @echo ""
    @echo "1. Read ANCHOR*.scm and STATE.scm"
    @echo "2. Run: just --list"
    @echo "3. Run: just test"
    @echo "4. Run: just demo"
    @echo ""
    @echo "For adding new capabilities:"
    @echo "1. Update SPEC/ROADMAP first"
    @echo "2. Add a just recipe (and tests)"
    @echo "3. Only then edit code"

# Run panic-attacker pre-commit scan
assail:
    @command -v panic-attack >/dev/null 2>&1 && panic-attack assail . || echo "panic-attack not found — install from https://github.com/hyperpolymath/panic-attacker"

secret-scan-trufflehog:
    @command -v trufflehog >/dev/null && trufflehog filesystem . --only-verified || true
