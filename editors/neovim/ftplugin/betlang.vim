" SPDX-License-Identifier: MIT OR Apache-2.0
" Filetype plugin for Betlang

if exists("b:did_ftplugin")
  finish
endif
let b:did_ftplugin = 1

" Set options
setlocal tabstop=2
setlocal shiftwidth=2
setlocal softtabstop=2
setlocal expandtab
setlocal autoindent
setlocal smartindent

" Comments
setlocal commentstring=//\ %s
setlocal comments=://

" File patterns
setlocal suffixesadd=.bet,.betlang
setlocal include=import

" Folding
setlocal foldmethod=syntax

" Key mappings (buffer-local)
nnoremap <buffer> <leader>br :BetlangRepl<CR>
nnoremap <buffer> <leader>bt :BetlangReplToggle<CR>
nnoremap <buffer> <leader>bl :BetlangSendLine<CR>
vnoremap <buffer> <leader>bs :BetlangSendSelection<CR>
nnoremap <buffer> <leader>bx :BetlangRun<CR>

" Undo settings when leaving buffer
let b:undo_ftplugin = "setlocal tabstop< shiftwidth< softtabstop< expandtab<"
      \ . " autoindent< smartindent< commentstring< comments<"
      \ . " suffixesadd< include< foldmethod<"
