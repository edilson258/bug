" Vim syntax file
" Language: bug

" Usage Instructions
" Put this file in .vim/syntax/bug.vim
" and add in your .vimrc file the next line:
" autocmd BufRead,BufNewFile *.bug set filetype=bug

if exists("b:current_syntax")
  finish
endif

syn keyword bugConditional if
syn keyword bugKeyword return
syn keyword bugType str int bool
syn keyword bugBool true false
syn keyword bugKeyword f nextgroup=bugFuncName skipwhite skipempty
syn match bugFuncName "\%(r#\)\=\%([^[:cntrl:][:space:][:punct:][:digit:]]\|_\)\%([^[:cntrl:][:punct:][:space:]]\|_\)*" display contained
syn match bugFuncCall "\.\w\+"
syn match bugOperator display "\%(+\|-\|/\|*\|=\|\^\|&\||\|!\|>\|<\|%\)=\?"
syntax region bugString start=/\v"/ skip=/\v\\./ end=/\v"/
syn region bugNumber start=/\d/ skip=/\d/ end=/\s/
syn match bugArrow display "->"


hi def link bugKeyword Keyword
hi def link bugKeyword Keyword
hi def link bugFuncName Function
hi def link bugFuncCall Function
hi def link bugConditional Conditional
hi def link bugType Type
hi def link bugOperator Operator
hi def link bugNumber Number
hi def link bugString String
hi def link bugBool Boolean

let b:current_syntax = "bug"
