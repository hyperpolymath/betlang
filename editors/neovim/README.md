# Betlang Neovim/Vim Plugin

Language support for Betlang in Neovim (with Lua) and Vim.

## Features

- Syntax highlighting
- LSP integration (via bet-lsp)
- Integrated REPL
- Run files directly
- Send code to REPL

## Installation

### Using lazy.nvim

```lua
{
  "hyperpolymath/betlang",
  config = function()
    require("betlang").setup({
      lsp = {
        enabled = true,
      },
      repl = {
        split = "horizontal",
        size = 15,
      },
    })
  end,
  ft = { "betlang", "bet" },
}
```

### Using packer.nvim

```lua
use {
  "hyperpolymath/betlang",
  config = function()
    require("betlang").setup()
  end,
  ft = { "betlang", "bet" },
}
```

### Using vim-plug

```vim
Plug 'hyperpolymath/betlang'
```

### Manual Installation

Copy the contents of this directory to:
- Neovim: `~/.config/nvim/pack/plugins/start/betlang/`
- Vim: `~/.vim/pack/plugins/start/betlang/`

## Configuration

```lua
require("betlang").setup({
  -- LSP settings
  lsp = {
    enabled = true,           -- Enable LSP
    cmd = nil,                -- Custom LSP command (auto-detect if nil)
    settings = {},            -- LSP settings
  },
  -- REPL settings
  repl = {
    cmd = nil,                -- Custom REPL command (auto-detect if nil)
    split = "horizontal",     -- "horizontal", "vertical", or "float"
    size = 15,                -- Split size
  },
  -- Highlighting
  highlight = {
    enabled = true,
  },
})
```

## Commands

| Command | Description |
|---------|-------------|
| `:BetlangRepl` | Open REPL |
| `:BetlangReplClose` | Close REPL |
| `:BetlangReplToggle` | Toggle REPL |
| `:BetlangSendLine` | Send current line to REPL |
| `:BetlangSendSelection` | Send selection to REPL |
| `:BetlangRun` | Run current file |

## Key Mappings

Default mappings (buffer-local for .bet files):

| Mapping | Description |
|---------|-------------|
| `<leader>br` | Open REPL |
| `<leader>bt` | Toggle REPL |
| `<leader>bl` | Send line to REPL |
| `<leader>bs` | Send selection to REPL (visual mode) |
| `<leader>bx` | Run current file |

## Requirements

- Neovim 0.9+ (for full Lua support) or Vim 8+
- [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig) (for LSP support)
- bet-lsp (for language server features)
- bet-cli or Racket (for REPL)

## License

MIT OR Apache-2.0
