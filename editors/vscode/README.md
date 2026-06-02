<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Betlang for Visual Studio Code

Language support for [Betlang](https://github.com/hyperpolymath/betlang) — a
ternary probabilistic programming language.

## Features

- Syntax highlighting (TextMate grammar, `syntaxes/betlang.tmLanguage.json`)
- Language Server Protocol client — spawns `bet-lsp` for diagnostics,
  completion, and hover
- Commands: Start/Stop REPL, Evaluate Selection, Restart Language Server

## Implementation

The extension is written in **AffineScript** (`src/extension.affine`), per the
hyperpolymath language policy under which AffineScript replaces
ReScript/TypeScript for editor tooling. It compiles to `out/extension.cjs`,
which `package.json`'s `main` field points to:

```bash
affinescript compile src/extension.affine -o out/extension.cjs --vscode-extension
# or
npm run compile
```

Requires the [AffineScript compiler](https://github.com/hyperpolymath/affinescript).

> History: the previous ReScript implementation (`Extension.res` / `VSCode.res`)
> was removed 2026-06-02 when the extension was migrated to AffineScript.

## Settings

- `betlang.lspPath` — path to the `bet-lsp` executable (default: `bet-lsp`)
- `betlang.enableDiagnostics`, `betlang.enableCompletion`, `betlang.enableHover`
