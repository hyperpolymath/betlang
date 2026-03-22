# Tri-Perimeter Contribution Framework (TPCF)

## Introduction

betlang adopts the **Tri-Perimeter Contribution Framework (TPCF)** to define clear boundaries for different types of contributions. This framework ensures that:

- **Core stability** is maintained
- **Community innovation** is encouraged
- **Contribution paths** are clear and welcoming

## The Three Perimeters

### Perimeter 1: Inner Sanctum (Core Language)

**Purpose:** Preserve language semantics and ternary philosophy

**Scope:**
- `core/betlang.rkt` - Core bet primitives
- Ternary abstraction (A-B-C structure)
- Fundamental language semantics

**Contribution Policy:**
- ✅ **Bug fixes**: Always welcome
- ⚠️ **New primitives**: Requires RFC and maintainer approval
- ❌ **Breaking changes**: Extremely rare, require community consensus
- ❌ **Non-ternary primitives**: Not acceptable

**Rationale:**
The core `bet` primitive defines betlang's identity. Changes here affect all users and must be carefully considered.

**Examples:**

✅ **Acceptable:**
```racket
;; Bug fix: Fix edge case in bet/weighted
(define (bet/weighted . weighted-choices)
  (unless (= (length weighted-choices) 3)  ;; Add validation
    (error 'bet/weighted "expected exactly 3 weighted choices"))
  ...)
```

❌ **Not Acceptable:**
```racket
;; Adding binary or quaternary primitives
(define (binary-bet a b) ...)  ;; Breaks ternary philosophy
(define (quad-bet a b c d) ...)  ;; Not ternary
```

**Contribution Process:**
1. Open an issue with `[RFC: Core]` tag
2. Community discussion (minimum 2 weeks)
3. Maintainer review and decision
4. If approved: PR with comprehensive tests
5. Documentation update required

**Approval Requirement:** Unanimous maintainer approval

---

### Perimeter 2: Middle Ring (Standard Library)

**Purpose:** Provide high-quality, well-tested utilities

**Scope:**
- `lib/*.rkt` - All library modules
- `tools/*.rkt` - Analysis and utility tools
- `repl/shell.rkt` - REPL implementation

**Contribution Policy:**
- ✅ **New functions**: Welcome with tests and docs
- ✅ **Optimizations**: Welcome with benchmarks
- ✅ **Bug fixes**: Always welcome
- ⚠️ **Breaking API changes**: Require deprecation period
- ✅ **New libraries**: Welcome with justification

**Rationale:**
Libraries extend betlang's capabilities without changing core semantics. We encourage innovation here while maintaining quality standards.

**Examples:**

✅ **Acceptable:**
```racket
;; New distribution in lib/distributions.rkt
(define (generalized-pareto alpha beta)
  "Generalized Pareto distribution"
  ...)

;; New statistical test
(define (anderson-darling-test samples)
  "Anderson-Darling normality test"
  ...)

;; Performance optimization
(define (fast-mean samples)
  "Optimized mean calculation"
  (/ (apply + samples) (length samples)))
```

⚠️ **Requires Discussion:**
```racket
;; Changing existing API (needs deprecation)
(define (mean samples #:weighted weights)  ;; Adding parameter
  ...)
```

**Contribution Process:**
1. Open an issue or skip to PR for small additions
2. Submit PR with:
   - Implementation
   - Tests (>70% coverage preferred)
   - Documentation (docstrings + API reference update)
   - Example usage
3. Maintainer review (1-2 weeks)
4. Address feedback
5. Merge

**Approval Requirement:** One maintainer approval

**Quality Standards:**
- All exported functions must have docstrings
- Tests required for non-trivial functions
- No breaking changes without deprecation
- Follow Racket style guide

---

### Perimeter 3: Outer Circle (Examples & Community)

**Purpose:** Share knowledge, explore applications, teach concepts

**Scope:**
- `examples/*.rkt` - Example programs
- `benchmarks/*.rkt` - Performance tests
- `docs/*.md` - Documentation
- `homepage/` - Website assets
- Community contributions (blog posts, papers, talks)

**Contribution Policy:**
- ✅ **New examples**: Always welcome
- ✅ **Documentation improvements**: Highly encouraged
- ✅ **Benchmarks**: Welcome
- ✅ **Tutorials**: Very welcome
- ✅ **Translations**: Welcome
- ✅ **Bug fixes**: Always welcome

**Rationale:**
The outer circle is where creativity flourishes. We want this to be an open, welcoming space for all skill levels.

**Examples:**

✅ **Highly Encouraged:**
```racket
;; New domain-specific example
;; examples/epidemiology.rkt
(require "../core/betlang.rkt")
(require "../lib/distributions.rkt")

;; SIR model simulation with betlang
(define (sir-model population infected ...)
  ...)
```

```markdown
<!-- New tutorial section -->
# Tutorial: Using betlang for A/B Testing

Learn how to model A/B tests with probabilistic programming...
```

**Contribution Process:**
1. Submit PR directly (no issue needed)
2. Brief review for quality and accuracy
3. Quick merge (usually <3 days)

**Approval Requirement:** Any contributor or maintainer can merge

**Quality Standards:**
- Examples should run without errors
- Code should be readable and well-commented
- Documentation should be clear and accurate
- No offensive or inappropriate content

---

## Perimeter Matrix

| Aspect | Inner (Core) | Middle (Library) | Outer (Examples) |
|--------|-------------|------------------|------------------|
| **Scope** | Core primitives | Standard library | Examples, docs |
| **Stability** | Very High | High | Medium |
| **Innovation** | Low | Medium | High |
| **Approval** | Unanimous | 1 maintainer | Any contributor |
| **RFC Required** | Yes | Sometimes | No |
| **Tests Required** | Comprehensive | Yes | Recommended |
| **Docs Required** | Yes | Yes | Helpful |
| **Breaking Changes** | Rare | Deprecated | Allowed |
| **Contribution Speed** | Slow | Medium | Fast |

## Special Cases

### Cross-Perimeter Changes

Some contributions affect multiple perimeters:

**Example:** Adding a new bet primitive + library functions
- Core primitive → Inner perimeter process
- Helper functions → Middle perimeter process
- Examples → Outer perimeter process

**Process:**
1. Follow Inner perimeter process for core change
2. Once core is approved, add library functions (Middle process)
3. Add examples freely (Outer process)

### Security Fixes

Security vulnerabilities bypass normal processes:
- **All perimeters:** Immediate fix, coordinate with SECURITY.md
- **Disclosure:** Follow responsible disclosure timeline
- **Process:** Fast-track review and merge

### Documentation-Only Changes

Pure documentation changes (typos, clarifications):
- **All perimeters:** Fast-track approval
- **Process:** Direct PR, quick merge

## Contribution Levels

### Beginner-Friendly (Outer Circle)
- Fix typos in documentation
- Add code comments
- Create new examples
- Improve tutorials
- Translate documentation

### Intermediate (Middle Ring)
- Add new library functions
- Optimize existing code
- Expand test coverage
- Create new tools
- Write comprehensive examples

### Advanced (Inner Sanctum)
- Propose new core primitives
- Refactor core architecture
- Design new language features
- Formal verification
- Performance optimization of core

## Migration Between Perimeters

Code can graduate from outer to middle to inner:

1. **Outer → Middle:**
   - Example becomes useful enough for library inclusion
   - Extract, test, document, move to `lib/`

2. **Middle → Inner:**
   - Rarely needed
   - Only if functionality is truly fundamental
   - Requires community consensus

## Contribution Rights

### All Contributors Have:
- Right to propose ideas in any perimeter
- Right to fair review of contributions
- Right to appeal decisions
- Right to fork under CC0 license

### Maintainers Commit To:
- Timely review of contributions
- Clear feedback on changes needed
- Transparent decision-making
- Respecting contributor time

## Frequently Asked Questions

**Q: I want to add a new probability distribution. Which perimeter?**
**A:** Middle ring (`lib/distributions.rkt`). Submit PR with tests and docs.

**Q: Can I add a binary bet primitive?**
**A:** No. Core is ternary-only. However, you can build binary on top:
```racket
(define (binary-bet a b) (bet a b a))  ;; In user code, not core
```

**Q: I found a typo in the tutorial.**
**A:** Outer circle. Fix it and submit a PR directly!

**Q: I want to change how `bet` works fundamentally.**
**A:** Inner sanctum. Open an [RFC] issue first. Expect long discussion.

**Q: Can I add a new example file?**
**A:** Outer circle. Yes! Add it to `examples/` and submit a PR.

**Q: I want to add type annotations.**
**A:** Middle ring for libraries, Inner for core. Open an issue to discuss approach first.

## Versioning and Compatibility

### Inner Perimeter (Core)
- Changes trigger **MAJOR** version bump
- Backward compatibility is critical
- Breaking changes require 6+ months deprecation

### Middle Perimeter (Library)
- New functions trigger **MINOR** version bump
- Breaking changes trigger **MAJOR** version bump
- Deprecation period: 3 months minimum

### Outer Perimeter (Examples)
- Changes don't affect version number
- Can change freely
- Keep aligned with current API

## Governance

**Decision Authority:**
- **Inner:** All maintainers (consensus)
- **Middle:** Any maintainer (1 approval needed)
- **Outer:** Any contributor (self-merge after 24 hours)

**Appeals Process:**
1. Contributor can appeal to all maintainers
2. Discussion in public issue
3. Final decision by lead maintainer if no consensus

## Evolution of This Framework

This TPCF declaration is itself subject to the framework:

- **Framework changes:** Middle perimeter process
- **Open to discussion:** Community input welcome
- **Versioned:** Track changes in CHANGELOG.md

## Contact

Questions about TPCF:
- Open an issue: [Questions about contribution perimeters]
- Email: maintainers@betlang.org
- See: CONTRIBUTING.md for detailed guidelines

---

**TPCF Version:** 1.0
**Last Updated:** 2025-11-22
**Applies to:** betlang v0.1.0+

---

## Summary

**Inner Perimeter:** Core language - stable, carefully guarded
**Middle Perimeter:** Standard library - quality-focused, innovation-friendly
**Outer Perimeter:** Examples & docs - open, welcoming, fast iteration

All contributions valued. Different perimeters = different processes. Choose your contribution level, and we'll guide you through! 🎲
