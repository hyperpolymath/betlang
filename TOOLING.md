# Betlang Optional Tooling

This document identifies optional tooling components that are NOT part of the
authoritative betlang implementation.

## Authoritative Implementation

The **Racket implementation** in `core/betlang.rkt` is the sole authoritative
reference implementation. See `SPEC.core.scm` for the formal semantics.

**Authoritative components:**
- `core/betlang.rkt` - Core DSL primitives
- `lib/*.rkt` - Standard library modules
- `repl/shell.rkt` - Interactive REPL
- `tests/basics.rkt` - Test suite
- `conformance/` - Conformance test corpus
- `SPEC.core.scm` - Formal specification

## Optional Tooling (Non-Authoritative)

The following components are **optional tooling** that enhances the developer
experience but **does not define or affect betlang semantics**:

### Rust/Cargo Components

| Directory | Purpose | Status |
|-----------|---------|--------|
| `compiler/bet-syntax/` | AST definitions | Optional tooling |
| `compiler/bet-parse/` | Parser | Optional tooling |
| `compiler/bet-core/` | Core types | Optional tooling |
| `compiler/bet-check/` | Type checking | Optional tooling |
| `compiler/bet-eval/` | Evaluator | Optional tooling |
| `compiler/bet-codegen/` | Code generation | Optional tooling |
| `runtime/bet-rt/` | Runtime | Optional tooling |
| `runtime/bet-rand/` | Random number generation | Optional tooling |
| `runtime/bet-viz/` | Visualization | Optional tooling |
| `tools/bet-cli/` | CLI tool | Optional tooling |

**Note:** These Rust components may be developed as alternative implementations
or performance-optimized tooling, but they are NOT authoritative. Any
discrepancy between Rust behavior and Racket behavior means the Racket
implementation is correct.

### Language Bindings

| Directory | Purpose | Status |
|-----------|---------|--------|
| `bindings/chapel/` | Chapel language bindings | Optional tooling |
| `bindings/julia/` | Julia language bindings | Optional tooling |

**Note:** These bindings MUST conform to SPEC.core.scm semantics. They are
convenience wrappers, not authoritative implementations.

### Container Definitions

| File | Purpose | Status |
|------|---------|--------|
| `containers/Containerfile` | Production container | Optional tooling |
| `containers/Containerfile.dev` | Development container | Optional tooling |

## Semantic Authority Hierarchy

1. **SPEC.core.scm** - Formal specification (normative)
2. **core/betlang.rkt** - Reference implementation (authoritative)
3. **conformance/** - Conformance tests (verification)
4. **Everything else** - Optional tooling (non-authoritative)

## Guidelines for Optional Tooling

When developing optional tooling:

1. **MUST NOT** modify betlang semantics
2. **MUST** pass all conformance tests when applicable
3. **MUST** defer to Racket implementation on ambiguities
4. **SHOULD** document any intentional behavioral differences
5. **SHOULD** clearly mark as "optional" in documentation

## Forbidden in f0 (First Phase)

Per the anchor document:
- Replacing Racket as the authoritative implementation
- Adding Julia or other backends as primary implementations
- Unseeded randomness in tests

These restrictions may be relaxed in future phases (f1+).
