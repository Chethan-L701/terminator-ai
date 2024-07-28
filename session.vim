let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Workspace/Rust/gemini-cli
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 ~/Workspace/Rust/gemini-cli/src/main.rs
argglobal
%argdel
edit ~/Workspace/Rust/gemini-cli/src/main.rs
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=100
setlocal fml=1
setlocal fdn=0
setlocal fen
silent! normal! zE
11,12fold
11,13fold
11,14fold
11,15fold
11,16fold
10,17fold
21,22fold
21,23fold
21,24fold
21,25fold
19,26fold
28,38fold
40,42fold
52,55fold
68,72fold
67,76fold
67,79fold
67,79fold
82,84fold
82,86fold
82,89fold
82,92fold
82,94fold
82,94fold
82,94fold
82,94fold
82,94fold
81,95fold
97,100fold
102,105fold
112,115fold
131,132fold
131,133fold
130,134fold
130,135fold
130,136fold
130,137fold
130,138fold
130,139fold
130,140fold
130,141fold
143,146fold
148,150fold
154,160fold
154,161fold
153,162fold
165,169fold
165,170fold
164,171fold
174,177fold
173,178fold
122,179fold
122,181fold
122,181fold
61,182fold
183,186fold
61,187fold
61,187fold
44,190fold
let &fdl = &fdl
let s:l = 2 - ((1 * winheight(0) + 21) / 43)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 2
normal! 0
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
