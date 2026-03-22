" SPDX-License-Identifier: MIT OR Apache-2.0
" Vim syntax file for Betlang

if exists("b:current_syntax")
  finish
endif

" Keywords
syntax keyword betlangKeyword bet let fn if else match for in while return break continue
syntax keyword betlangBoolean true false
syntax keyword betlangConstant nil

" Built-in functions
syntax keyword betlangBuiltin print println sample observe uniform bernoulli
syntax keyword betlangBuiltin normal exponential poisson categorical
syntax keyword betlangBuiltin map filter fold reduce zip range

" Operators
syntax match betlangOperator /[+\-*/%<>=!&|^~]/
syntax match betlangOperator /==/
syntax match betlangOperator /!=/
syntax match betlangOperator />=/
syntax match betlangOperator /<=/
syntax match betlangOperator /&&/
syntax match betlangOperator /||/
syntax match betlangOperator /=>/
syntax match betlangOperator /->/

" Numbers
syntax match betlangNumber /\<\d\+\>/
syntax match betlangNumber /\<\d\+\.\d*\>/
syntax match betlangNumber /\<\d\+[eE][+-]\?\d\+\>/
syntax match betlangNumber /\<0x[0-9a-fA-F]\+\>/
syntax match betlangNumber /\<0b[01]\+\>/

" Strings
syntax region betlangString start=/"/ skip=/\\"/ end=/"/
syntax region betlangString start=/'/ skip=/\\'/ end=/'/

" Comments
syntax match betlangComment /\/\/.*$/
syntax region betlangComment start=/\/\*/ end=/\*\//

" Braces and brackets (for bet expressions)
syntax match betlangBrace /[{}]/
syntax match betlangBracket /[\[\]]/
syntax match betlangParen /[()]/
syntax match betlangDelimiter /[,;:]/

" Function definition
syntax match betlangFunction /\<fn\s\+\zs\w\+/

" Type annotations (if any)
syntax match betlangType /:\s*\zs\u\w*/

" Probability weights in bet
syntax match betlangWeight /\[\s*\d\+\.\?\d*\s*,\s*\d\+\.\?\d*\s*,\s*\d\+\.\?\d*\s*\]/

" Highlighting
highlight default link betlangKeyword Keyword
highlight default link betlangBoolean Boolean
highlight default link betlangConstant Constant
highlight default link betlangBuiltin Function
highlight default link betlangOperator Operator
highlight default link betlangNumber Number
highlight default link betlangString String
highlight default link betlangComment Comment
highlight default link betlangBrace Delimiter
highlight default link betlangBracket Delimiter
highlight default link betlangParen Delimiter
highlight default link betlangDelimiter Delimiter
highlight default link betlangFunction Function
highlight default link betlangType Type
highlight default link betlangWeight Special

let b:current_syntax = "betlang"
