<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
## Machine-Readable Artefacts

The following files in `.machine_readable/` contain structured project metadata:

- `STATE.a2ml` - Current project state and progress
- `META.a2ml` - Architecture decisions and development practices
- `ECOSYSTEM.a2ml` - Position in the ecosystem and related projects
- `AGENTIC.a2ml` - AI agent interaction patterns
- `NEUROSYM.a2ml` - Neurosymbolic integration config
- `PLAYBOOK.a2ml` - Operational runbook

---

# CLAUDE.md - AI Assistant Instructions

## Language Policy (Hyperpolymath Standard)

### ALLOWED Languages & Tools

| Language/Tool | Use Case | Notes |
|---------------|----------|-------|
| **Deno** | Runtime & package management | Replaces Node/npm/bun |
| **Rust** | Performance-critical, systems, WASM | Preferred for CLI tools |
| **Gossamer** | Mobile apps (iOS/Android) | Rust backend + web UI |
| **Dioxus** | Mobile apps (native UI) | Pure Rust, React-like |
| **Gleam** | Backend services | Runs on BEAM or compiles to JS |
| **Bash/POSIX Shell** | Scripts, automation | Keep minimal |
| **JavaScript** | Only where AffineScript cannot | MCP protocol glue, Deno APIs |
| **Nickel** | Configuration language | For complex configs |
| **6a2 Scheme** | State/meta files | STATE.a2ml, META.a2ml, ECOSYSTEM.a2ml, PLAYBOOK.a2ml, AGENTIC.a2ml, NEUROSYM.a2ml |
| **Julia** | Batch scripts, data processing | Per RSR |
| **OCaml** | AffineScript compiler | Language-specific |
| **Ada** | Safety-critical systems | Where required |

### BANNED - Do Not Use

| Banned | Replacement |
|--------|-------------|
| TypeScript | AffineScript |
| Node.js | Deno |
| npm | Deno |
| Bun | Deno |
| pnpm/yarn | Deno |
| Go | Rust |
| Python | Julia/Rust/AffineScript |
| Java/Kotlin | Rust/Gossamer/Dioxus |
| Swift | Gossamer/Dioxus |
| React Native | Gossamer/Dioxus |
| Flutter/Dart | Gossamer/Dioxus |

### Mobile Development

**No exceptions for Kotlin/Swift** - use Ephapax or Rust-first approach:

1. **Gossamer** - Web UI (AffineScript) +/- Gossamer panel, MPL-2.0
2. **Dioxus** - Pure Rust native UI, MIT/Apache-2.0

Both are FOSS with independent governance (no Big Tech).

### Enforcement Rules

1. **No new TypeScript files** - Convert existing TS to AffineScript
2. **No package.json - use deno.json deps** - Use deno.json imports
3. **No node_modules in production** - Deno caches deps automatically
4. **No Go code** - Use Rust instead
5. **No Python anywhere** - Use Julia for data/batch, Rust for systems, AffineScript for apps
6. **No Kotlin/Swift for mobile** - Use Gossamer or Dioxus

### Package Management

- **Primary**: Guix (guix.scm)
- **Fallback**: Nix (flake.nix)
- **JS deps**: Deno (deno.json imports)

### TypeScript Exemptions (Approved)

The hyperpolymath "no new TypeScript" policy has the following approved exemptions in this repo. These are *not* policy violations — they are documented carve-outs.

| Path | Files | Rationale | Unblock condition |
|---|---|---|---|
| `playground/**` | 6 (`src/probability.ts`, `src/ternary.ts`, `src/main.ts`, `test/ternary_test.ts`, `test/probability_test.ts`, `examples/uncertainty.ts`) | Per `playground/README.adoc`, the directory is an **intentional experimental sandbox** "decoupled from the main compiler to allow rapid experimentation". TypeScript was chosen as one of several languages explored alongside Deno/Nickel/Idris2/Zig in the sandbox. The primary Betlang implementation in `core/`, `lib/`, and `tests/` remains Racket-only. | Owner decision to either (a) migrate the playground sample to AffineScript, or (b) delete the TypeScript files once the experimental questions they answer are settled. No scheduled issue. |

Adding to this list requires explicit user approval and an unblock condition. New TypeScript files outside this list are blocked by the RSR antipattern check (`governance / Language / package anti-pattern policy`).

### Security Requirements

- No MD5/SHA1 for security (use SHA256+)
- HTTPS only (no HTTP URLs)
- No hardcoded secrets
- SHA-pinned dependencies
- SPDX license headers on all files

