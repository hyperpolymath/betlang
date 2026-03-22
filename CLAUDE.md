# CLAUDE.md

This file provides context for Claude Code when working with the betlang repository.

## Project Overview

**betlang** is a ternary DSL (Domain-Specific Language) for probabilistic modeling and symbolic wagers, implemented in Racket. The core primitive is the `(bet A B C)` form, which randomly selects one of three values, inspired by musical ternary form (A–B–A).

## Project Structure

```
betlang/
├── core/
│   └── betlang.rkt          # DSL primitives and core functionality
├── lib/
│   └── ternary.rkt          # Conditional abstraction utilities
├── repl/
│   └── shell.rkt            # Interactive REPL with logging
├── tests/
│   └── basics.rkt           # Probabilistic test suite
├── docs/
│   ├── architecture.md      # Project architecture overview
│   └── semantics.md         # Language semantics documentation
└── homepage/                # Public web interface
```

## Language & Technology

- **Primary Language**: Racket (Scheme dialect)
- **File Extension**: `.rkt`
- **Paradigm**: Functional, probabilistic programming

## Development Guidelines

### Running the REPL

To start the interactive betlang REPL:
```bash
racket repl/shell.rkt
```

### Running Tests

To run the test suite:
```bash
racket tests/basics.rkt
```

### Code Style

- Follow standard Racket conventions
- Use meaningful names for bet expressions
- Document probabilistic behavior in comments
- Keep ternary forms (A, B, C) semantically related

### Core Concepts

1. **bet form**: `(bet A B C)` - The fundamental primitive that randomly selects one of three values
2. **Ternary logic**: Based on musical ternary form structure (A–B–A)
3. **Probabilistic modeling**: The language is designed for expressing probabilistic computations
4. **Symbolic wagers**: Represent uncertain choices symbolically

## Working with Claude Code

### When Adding Features

- Ensure new features align with the ternary (three-valued) philosophy
- Add corresponding tests in `tests/` directory
- Update documentation in `docs/` if semantics change
- Consider both probabilistic and deterministic use cases

### When Fixing Bugs

- Check probabilistic edge cases (what happens with different random seeds?)
- Verify REPL behavior after changes
- Ensure backward compatibility with existing bet forms

### When Refactoring

- Maintain the simplicity of the core `bet` primitive
- Keep the functional programming style
- Preserve REPL usability and interactivity

## Common Tasks

### Adding a New DSL Form
1. Define the form in `core/betlang.rkt`
2. Add helper utilities to `lib/ternary.rkt` if needed
3. Add tests to `tests/basics.rkt`
4. Document semantics in `docs/semantics.md`

### Modifying the REPL
1. Edit `repl/shell.rkt`
2. Test interactively to ensure logging works
3. Verify error handling for invalid inputs

### Updating Documentation
- `docs/architecture.md` - For structural changes
- `docs/semantics.md` - For language semantics
- `README.md` - For user-facing information

## Dependencies

Check the Racket version and any required packages:
```bash
racket --version
```

Ensure all necessary Racket modules are available before running the project.

## Testing Philosophy

Since betlang is probabilistic:
- Tests may have non-deterministic outcomes
- Use appropriate statistical assertions
- Consider edge cases with all three bet branches
- Test both random and deterministic scenarios

## Resources

- [Racket Documentation](https://docs.racket-lang.org/)
- Project docs in `docs/` directory
- Ternary logic and probabilistic programming concepts

## Notes for Claude

- The project is minimal and focused - keep contributions aligned with core philosophy
- Probabilistic behavior means some outputs are inherently non-deterministic
- The ternary (three-valued) structure is fundamental to the language design
- Functional programming principles should be preserved throughout
