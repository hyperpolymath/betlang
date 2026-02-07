# Rust Compiler Status

**Last Updated:** 2026-02-07
**Status:** Blocked - LALRPOP Parser Conflicts

## Overview

The Rust compiler is **optional, non-authoritative tooling** for betlang. The **authoritative implementation is Racket** (see `core/betlang.rkt` and `lib/*.rkt`), which is **100% complete and production-ready**.

## Current Blockers

### LALRPOP Parser Generation Failures

The parser generator fails with **3 shift/reduce conflicts** in `compiler/bet-parse/src/grammar.lalrpop`:

1. **Type parsing conflict (line 157):**
   ```
   TypeAtom: Type = {
       "(" <t:Type> ")" => t,
   }
   ```
   Parser cannot decide when to reduce `Type` vs shift `)` token.

2. **Pattern list parsing conflict (line 410):**
   ```
   PatternAtom: Pattern = {
       "[" <elems:Comma<Spanned<Pattern>>> "]" => Pattern::List(elems, None),
   }
   ```
   The `Comma` macro creates ambiguity in list pattern parsing.

3. **Let expression conflict (line 378):**
   ```
   DoStatement vs ExprAtom (let-in)
   ```
   Ambiguity between do-block let statements and let-in expressions.

### Root Cause

These are **LR(1) grammar conflicts** caused by:
- Recursive productions with parentheses
- The `Comma<T>` and `Sep<T, S>` helper macros not being LR(1) compatible
- Overlapping syntax between do-blocks and let-in expressions

### Partial Fixes Completed (2026-02-07)

The following compilation issues were resolved:

✅ **Serde serialization:**
- Enabled `im` crate serde feature for HashMap/Vector
- Implemented custom Arc<T> serde helpers
- Manual PartialEq implementation for Value enum

✅ **Type system:**
- Added lifetime specifiers to reference-returning functions
- Fixed return type mismatches

✅ **API differences:**
- Fixed `im::HashSet` API usage (is_superset → is_subset)
- Fixed `im::HashSet` intersection method

### Blocked Components

Because the parser fails to generate, the entire Rust build is blocked:

- ❌ Parser (generation fails)
- ❌ Type checker (stubbed, cannot test)
- ❌ Interpreter (stubbed, cannot test)
- ❌ Code generator (stubbed, cannot test)
- ❌ CLI tool (depends on parser)
- ❌ LSP server (depends on parser)

## Solutions

### Option 1: Use Racket (Recommended)

**The Racket implementation is complete and production-ready.**

```bash
# Run betlang programs
racket examples/safety-features.rkt

# Interactive REPL
racket repl/shell.rkt

# Run tests
racket tests/basics.rkt
```

All 4 safety features and all 14 number systems work perfectly.

### Option 2: Fix LALRPOP Grammar (Future Work)

Estimated effort: **4-8 hours** for experienced LALRPOP developer

Required changes:
1. Replace `Sep<T, S>` and `Comma<T>` macros with LALRPOP built-ins (`<T+>`, `<T*>`)
2. Refactor Type/Pattern productions to eliminate parenthesis ambiguity
3. Add precedence declarations for let-in vs do-let
4. Possibly introduce intermediate non-terminals to break LR(1) cycles

### Option 3: Alternative Parser (Major Rewrite)

Switch to:
- **nom** (parser combinator library)
- **pest** (PEG parser generator)
- **Hand-written recursive descent parser**

Estimated effort: **8-16 hours**

## Current Rust Completion Status

| Component | Status | Completion | Notes |
|-----------|--------|------------|-------|
| Lexer | ✅ Working | 100% | Logos-based, all tokens |
| Parser | ❌ Blocked | 0% | LALRPOP conflicts |
| AST | ✅ Complete | 100% | All node types defined |
| Type Checker | ⚠️ Stubbed | 10% | Framework exists, logic needed |
| Interpreter | ⚠️ Stubbed | 10% | Framework exists, logic needed |
| Code Generator | ⚠️ Stubbed | 5% | Placeholder only |
| Runtime Library | ✅ Working | 90% | Some compilation issues fixed |
| CLI | ❌ Blocked | 0% | Depends on parser |
| LSP | ❌ Blocked | 0% | Depends on parser |

**Overall Rust Completion:** ~30%

## Recommendation

**Use the Racket implementation.** It is:
- ✅ Complete and tested
- ✅ Authoritative per project design
- ✅ Production-ready
- ✅ Fully documented
- ✅ Includes REPL, examples, tests, benchmarks

The Rust compiler is **optional future work** and not required for using betlang.

## Contributing

If you'd like to fix the Rust compiler:

1. Familiarity with LALRPOP parser generators required
2. See `compiler/bet-parse/src/grammar.lalrpop`
3. Focus on resolving the 3 shift/reduce conflicts
4. Test with `cargo build` in repository root
5. Submit a pull request

For questions: jonathan.jewell@open.ac.uk
