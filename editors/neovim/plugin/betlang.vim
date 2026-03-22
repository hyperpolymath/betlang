" SPDX-License-Identifier: MIT OR Apache-2.0
" Betlang Neovim/Vim plugin entry point

if exists('g:loaded_betlang')
  finish
endif
let g:loaded_betlang = 1

" Check for Neovim with Lua support
if has('nvim')
  " Load Lua module
  lua require('betlang').setup()
endif

" Commands (available even without Lua)
command! BetlangVersion echo "Betlang plugin v0.1.0"
