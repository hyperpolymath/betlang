-- SPDX-License-Identifier: MIT OR Apache-2.0
-- Betlang Neovim plugin

local M = {}

-- Default configuration
M.config = {
  -- LSP settings
  lsp = {
    enabled = true,
    cmd = nil,  -- Auto-detect if nil
    settings = {},
  },
  -- REPL settings
  repl = {
    cmd = nil,  -- Auto-detect if nil
    split = "horizontal",  -- "horizontal", "vertical", or "float"
    size = 15,
  },
  -- Highlighting
  highlight = {
    enabled = true,
  },
}

-- Find bet-lsp executable
local function find_lsp()
  local paths = {
    vim.fn.getcwd() .. "/lsp/bet-lsp/build/erlang-shipment/entrypoint.sh",
    vim.fn.expand("~/.bet/bin/bet-lsp"),
    vim.fn.expand("~/.local/bin/bet-lsp"),
    "/usr/local/bin/bet-lsp",
    "/usr/bin/bet-lsp",
  }

  for _, path in ipairs(paths) do
    if vim.fn.executable(path) == 1 then
      return path
    end
  end

  -- Check if bet-lsp is in PATH
  if vim.fn.executable("bet-lsp") == 1 then
    return "bet-lsp"
  end

  return nil
end

-- Find bet-cli for REPL
local function find_repl()
  local paths = {
    vim.fn.getcwd() .. "/target/release/bet-cli",
    vim.fn.getcwd() .. "/target/debug/bet-cli",
    vim.fn.expand("~/.cargo/bin/bet-cli"),
    vim.fn.expand("~/.bet/bin/bet-cli"),
    vim.fn.expand("~/.local/bin/bet-cli"),
    "/usr/local/bin/bet-cli",
    "/usr/bin/bet-cli",
  }

  for _, path in ipairs(paths) do
    if vim.fn.executable(path) == 1 then
      return path
    end
  end

  -- Check PATH
  if vim.fn.executable("bet-cli") == 1 then
    return "bet-cli"
  end

  -- Fallback to Racket REPL
  local racket_repl = vim.fn.getcwd() .. "/repl/shell.rkt"
  if vim.fn.filereadable(racket_repl) == 1 and vim.fn.executable("racket") == 1 then
    return { "racket", racket_repl }
  end

  return nil
end

-- Setup LSP
function M.setup_lsp()
  if not M.config.lsp.enabled then
    return
  end

  local lspconfig_ok, lspconfig = pcall(require, "lspconfig")
  if not lspconfig_ok then
    vim.notify("nvim-lspconfig not found, LSP support disabled", vim.log.levels.WARN)
    return
  end

  local configs = require("lspconfig.configs")

  if not configs.betlang then
    configs.betlang = {
      default_config = {
        cmd = M.config.lsp.cmd or { find_lsp(), "--stdio" },
        filetypes = { "betlang", "bet" },
        root_dir = lspconfig.util.root_pattern("betlang.toml", ".bet", ".git"),
        settings = M.config.lsp.settings,
      },
    }
  end

  lspconfig.betlang.setup({
    on_attach = function(client, bufnr)
      -- Enable completion
      vim.bo[bufnr].omnifunc = "v:lua.vim.lsp.omnifunc"

      -- Key mappings
      local opts = { buffer = bufnr, noremap = true, silent = true }
      vim.keymap.set("n", "gd", vim.lsp.buf.definition, opts)
      vim.keymap.set("n", "K", vim.lsp.buf.hover, opts)
      vim.keymap.set("n", "gi", vim.lsp.buf.implementation, opts)
      vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, opts)
      vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, opts)
      vim.keymap.set("n", "gr", vim.lsp.buf.references, opts)
      vim.keymap.set("n", "[d", vim.diagnostic.goto_prev, opts)
      vim.keymap.set("n", "]d", vim.diagnostic.goto_next, opts)
    end,
    capabilities = vim.lsp.protocol.make_client_capabilities(),
  })
end

-- REPL state
local repl_bufnr = nil
local repl_jobid = nil
local repl_winnr = nil

-- Open REPL
function M.open_repl()
  if repl_bufnr and vim.api.nvim_buf_is_valid(repl_bufnr) then
    -- REPL already exists, show it
    if not repl_winnr or not vim.api.nvim_win_is_valid(repl_winnr) then
      M.show_repl_window()
    else
      vim.api.nvim_set_current_win(repl_winnr)
    end
    return
  end

  local cmd = M.config.repl.cmd or find_repl()
  if not cmd then
    vim.notify("Could not find bet-cli or Racket REPL", vim.log.levels.ERROR)
    return
  end

  -- Create buffer
  repl_bufnr = vim.api.nvim_create_buf(false, true)
  vim.api.nvim_buf_set_name(repl_bufnr, "Betlang REPL")
  vim.api.nvim_buf_set_option(repl_bufnr, "buftype", "terminal")

  -- Show window
  M.show_repl_window()

  -- Start terminal
  local full_cmd = type(cmd) == "table" and cmd or { cmd, "repl" }
  repl_jobid = vim.fn.termopen(full_cmd, {
    on_exit = function(_, code)
      vim.notify("Betlang REPL exited with code " .. code)
      repl_bufnr = nil
      repl_jobid = nil
    end,
  })

  -- Enter insert mode
  vim.cmd("startinsert")
end

function M.show_repl_window()
  local split = M.config.repl.split
  local size = M.config.repl.size

  if split == "float" then
    local width = math.floor(vim.o.columns * 0.8)
    local height = math.floor(vim.o.lines * 0.8)
    local row = math.floor((vim.o.lines - height) / 2)
    local col = math.floor((vim.o.columns - width) / 2)

    repl_winnr = vim.api.nvim_open_win(repl_bufnr, true, {
      relative = "editor",
      width = width,
      height = height,
      row = row,
      col = col,
      style = "minimal",
      border = "rounded",
      title = " Betlang REPL ",
      title_pos = "center",
    })
  elseif split == "vertical" then
    vim.cmd("vsplit")
    repl_winnr = vim.api.nvim_get_current_win()
    vim.api.nvim_win_set_buf(repl_winnr, repl_bufnr)
    vim.cmd("vertical resize " .. size)
  else
    vim.cmd("split")
    repl_winnr = vim.api.nvim_get_current_win()
    vim.api.nvim_win_set_buf(repl_winnr, repl_bufnr)
    vim.cmd("resize " .. size)
  end
end

-- Close REPL
function M.close_repl()
  if repl_jobid then
    vim.fn.jobstop(repl_jobid)
  end
  if repl_bufnr and vim.api.nvim_buf_is_valid(repl_bufnr) then
    vim.api.nvim_buf_delete(repl_bufnr, { force = true })
  end
  repl_bufnr = nil
  repl_jobid = nil
  repl_winnr = nil
end

-- Toggle REPL
function M.toggle_repl()
  if repl_winnr and vim.api.nvim_win_is_valid(repl_winnr) then
    vim.api.nvim_win_close(repl_winnr, false)
    repl_winnr = nil
  elseif repl_bufnr and vim.api.nvim_buf_is_valid(repl_bufnr) then
    M.show_repl_window()
  else
    M.open_repl()
  end
end

-- Send text to REPL
function M.send_to_repl(text)
  if not repl_jobid then
    M.open_repl()
    -- Wait for REPL to start
    vim.defer_fn(function()
      M.send_to_repl(text)
    end, 500)
    return
  end

  vim.fn.chansend(repl_jobid, text .. "\n")
end

-- Send current line to REPL
function M.send_line()
  local line = vim.api.nvim_get_current_line()
  M.send_to_repl(line)
end

-- Send visual selection to REPL
function M.send_selection()
  local start_pos = vim.fn.getpos("'<")
  local end_pos = vim.fn.getpos("'>")
  local lines = vim.fn.getline(start_pos[2], end_pos[2])

  if #lines == 0 then
    return
  end

  -- Adjust for partial line selection
  if #lines == 1 then
    lines[1] = string.sub(lines[1], start_pos[3], end_pos[3])
  else
    lines[1] = string.sub(lines[1], start_pos[3])
    lines[#lines] = string.sub(lines[#lines], 1, end_pos[3])
  end

  M.send_to_repl(table.concat(lines, "\n"))
end

-- Run current file
function M.run_file()
  local file = vim.fn.expand("%:p")
  if file == "" then
    vim.notify("No file to run", vim.log.levels.WARN)
    return
  end

  local cmd = M.config.repl.cmd or find_repl()
  if not cmd then
    vim.notify("Could not find bet-cli", vim.log.levels.ERROR)
    return
  end

  local run_cmd = type(cmd) == "table" and cmd[1] or cmd
  vim.cmd("!" .. run_cmd .. " run " .. vim.fn.shellescape(file))
end

-- Setup function
function M.setup(opts)
  M.config = vim.tbl_deep_extend("force", M.config, opts or {})

  -- Set up filetype detection
  vim.filetype.add({
    extension = {
      bet = "betlang",
      betlang = "betlang",
    },
  })

  -- Create user commands
  vim.api.nvim_create_user_command("BetlangRepl", M.open_repl, {})
  vim.api.nvim_create_user_command("BetlangReplClose", M.close_repl, {})
  vim.api.nvim_create_user_command("BetlangReplToggle", M.toggle_repl, {})
  vim.api.nvim_create_user_command("BetlangSendLine", M.send_line, {})
  vim.api.nvim_create_user_command("BetlangRun", M.run_file, {})

  -- Visual mode command
  vim.api.nvim_create_user_command("BetlangSendSelection", M.send_selection, {
    range = true,
  })

  -- Setup LSP if enabled
  if M.config.lsp.enabled then
    M.setup_lsp()
  end
end

return M
