# Betlang experimental ideas

## Purpose

This note collects the "next question" ideas we discussed so we can experiment safely without letting scope creep derail the stable heart of Betlang. Keep the core grammar/number systems working, then prototype here.

## Current focus

- `compiler/bet-parse` now accepts `end`-terminated blocks for `bet`, `let … in`, `if … then … else`, `match`, `do`, `parallel`.  
- The brace/`{…}` syntax remains supported for backward compatibility, so nothing breaks downstream.  
- Julia bindings already surface the same features via the C FFI.
- Tests should be run (`cargo test`), but the rust toolchain needs a default (see below).

## Experiment ideas backlog

### 1. Aspect-oriented grammar injections
- Investigate how `julia-the-viper` handles `adder`/`havard` block injection.
- Define hooks or directives so Betlang can optionally parse new blocks that desugar to current AST.
- Include instructions for turning these hooks on/off for experiments before landing them in docs.

### 2. Quantum / multi-log layers
- Prototype as libraries (e.g., `lib/quantum-preview.rkt`) so they can be used today without touching grammar.
- If the pattern matures, define a keyword extension with optional grammar rules (ad-hoc `quantum … end`) that maps to standard semantics.
- Keep proofs/test coverage separated from the core to minimize safety risk.

### 3. Base/radix/representation experiments
- Continue using real arithmetic but wrap p-adic probability base parameterization to explore digit expansion ideas.
- Document conversion helpers in `lib/number-systems.rkt` or new helper module rather than altering core numeric types.

### 4. Safety / social controls
- Use modules (like `lib/cool-off`) to surface new responsible-gambling layers, and extend docs with checklists for trial runs.
- Keep verified guarantees (Dutch book, risk-of-ruin) central when experimenting with new primitives.

## Notes

- `cargo test` currently fails because the Rust toolchain is not configured (`rustup default stable` needed). The compiler tests still need to be run once the default toolchain is installed.
- Anything that touches grammar should remain optional until the library-level experimentation proves consistent.
