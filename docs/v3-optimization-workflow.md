# Betlang v3 optimization workflow

## Purpose

This workflow keeps the Betlang core fast and stable while giving you a repeatable path to exercised "optimized" artifacts.

## Steps

1. **Configure Rust toolchain**
   - Run `rustup default stable` (or your preferred channel). The optimized build uses the Rust compiler, so a working toolchain is required before you run `just build-v3`.

2. **Run the optimization driver**
   - Execute `just build-v3`. That currently delegates to `cargo build --release` via `build-tooling` and is the entry point for v3 release artifacts.
   - When the toolchain is available, this command produces release-mode Rust binaries that you can ship or plug into other tools.

3. **Verify tests (optional but recommended)**
   - Run `cargo test`/`cargo test --release` in `compiler/bet-parse` to make sure your optimized binaries are still correct.
   - Continue running Racket smoke tests (`just test`, `racket lib/number-systems.rkt`, `examples/safety-features.rkt`) as part of your nightly verification.

4. **Document results**
   - After each optimized build, note the toolchain version and outcomes in your release notes or logs (e.g., `logs/v3-optimized-YYYYMMDD.md`).

## Notes

- The `build-v3` target is intentionally lightweight: it acknowledges Rust’s release build as the proxy for optimization while keeping the Racket core unchanged.
- If you add other optimized artifacts (e.g., precompiled Racket files or Julia caches), extend this workflow and `Justfile` recipe accordingly.
