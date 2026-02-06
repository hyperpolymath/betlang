;; SPDX-License-Identifier: PMPL-1.0-or-later
;; STATE.scm - Current project state

(define state
  '((metadata
     (version "1.0")
     (schema-version "1.0")
     (created "2026-02-04")
     (updated "2026-02-06")
     (project "betlang")
     (repo "hyperpolymath/betlang"))

    (project-context
     (name "betlang")
     (tagline "Probabilistic and Bayesian programming language for uncertainty-aware computation")
     (tech-stack ("rust" "rescript")))

    (current-position
     (phase "development")
     (overall-completion 35)
     (components
       (("bet-parse" "Parser" 80)
        ("bet-eval" "Type checker and evaluator" 70)
        ("bet-check" "Semantic checker" 60)
        ("bet-codegen" "Code generator" 50)
        ("bet-syntax" "Syntax definitions" 90)
        ("bet-core" "Core types and environment" 80)
        ("bet-cli" "Command-line interface with REPL" 80)
        ("bet-lsp" "LSP server with 5 handler modules" 70)
        ("vscode-extension" "Editor extension (ReScript, converted from TypeScript)" 60)
        ("stdlib" "Statistics, distributions, Bayesian inference" 50)
        ("fuzzing" "ClusterFuzzLite configuration" 80)
        ("chapel-bindings" "Chapel language bindings" 20)
        ("julia-bindings" "Julia language bindings" 20)
        ("debugger" "Not started" 0)
        ("package-manager" "Not started" 0)
        ("documentation" "Examples and benchmarks" 30)))
     (working-features
       ("6-crate Rust workspace: bet-core, bet-parse, bet-eval, bet-check, bet-codegen, bet-syntax"
        "Parser for probabilistic bet expressions"
        "Type checker with probabilistic types"
        "Evaluator for bet computations"
        "Semantic checker"
        "Code generator"
        "CLI with REPL"
        "LSP server with 5 handler modules: completion, hover, definition, diagnostics, formatting"
        "VS Code extension (converted from TypeScript to ReScript per language policy)"
        "Standard library: statistics, probability distributions, Bayesian inference"
        "Uncertainty-aware number system"
        "ClusterFuzzLite fuzzing support"
        "Chapel and Julia language bindings (partial)"
        "Example programs and benchmarks"
        "53 Rust source files, ~51,000 LOC total")))

    (route-to-mvp
     (milestones
      ((milestone-id "m1")
       (name "Core Language")
       (status "in-progress")
       (completion 75)
       (items ("Parser for probabilistic expressions (done)"
               "Type checker with probability types (mostly done)"
               "Evaluator for bet computations (mostly done)"
               "Semantic checker (mostly done)"
               "Core types and environment (done)")))

      ((milestone-id "m2")
       (name "Standard Library")
       (status "in-progress")
       (completion 50)
       (items ("Statistics functions (done)"
               "Probability distributions (done)"
               "Bayesian inference primitives (done)"
               "Uncertainty-aware numbers (done)"
               "Monte Carlo simulation (TODO)"
               "Markov chain support (TODO)")))

      ((milestone-id "m3")
       (name "Code Generation")
       (status "in-progress")
       (completion 50)
       (items ("Bytecode generation (partial)"
               "Chapel bindings (partial)"
               "Julia bindings (partial)"
               "Optimization for probabilistic operations (TODO)")))

      ((milestone-id "m4")
       (name "Developer Tooling")
       (status "in-progress")
       (completion 55)
       (items ("LSP server with 5 handlers (done)"
               "VS Code extension in ReScript (done)"
               "CLI with REPL (done)"
               "ClusterFuzzLite fuzzing (done)"
               "Debugger (TODO)"
               "Package manager (TODO)"
               "Documentation generator (TODO)")))))

    (blockers-and-issues
     (critical ())
     (high
       ("End-to-end pipeline needs validation for complex probabilistic programs"
        "Code generation maturity unclear"))
     (medium
       ("No debugger"
        "No package manager"
        "Chapel and Julia bindings are partial"
        "Documentation sparse"))
     (low
       ("Large codebase (51K LOC) - some may be generated or scaffolded")))

    (critical-next-actions
     (immediate
       ("Validate end-to-end pipeline with non-trivial probabilistic programs"
        "Document current language capabilities and limitations"))
     (this-week
       ("Complete Chapel and Julia bindings"
        "Add Monte Carlo simulation to stdlib"))
     (this-month
       ("Implement debugger"
        "Begin package manager"
        "Write comprehensive documentation with examples")))

    (session-history
     ((date "2026-02-06")
      (accomplishments
        ("Updated STATE.scm with accurate project status from code audit"))))))
