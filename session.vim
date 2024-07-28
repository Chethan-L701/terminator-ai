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
badd +292 ~/Workspace/Rust/gemini-cli/src/main.rs
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
13,14fold
13,15fold
13,16fold
13,17fold
13,18fold
12,19fold
23,24fold
23,25fold
23,26fold
23,27fold
21,28fold
30,40fold
44,45fold
44,46fold
44,47fold
44,48fold
50,54fold
50,56fold
50,56fold
42,57fold
59,63fold
65,67fold
78,81fold
98,102fold
97,106fold
97,109fold
97,109fold
113,115fold
116,118fold
119,122fold
123,126fold
127,129fold
130,134fold
135,138fold
112,140fold
111,141fold
143,146fold
148,151fold
158,161fold
165,186fold
164,187fold
188,202fold
164,203fold
164,203fold
217,218fold
217,219fold
216,220fold
216,221fold
216,222fold
216,223fold
216,224fold
216,225fold
216,226fold
216,227fold
229,232fold
234,236fold
241,247fold
241,248fold
240,249fold
239,250fold
252,261fold
252,262fold
251,263fold
239,264fold
239,264fold
267,271fold
267,272fold
266,273fold
276,279fold
275,280fold
208,281fold
208,283fold
208,283fold
87,284fold
285,288fold
87,289fold
87,289fold
69,292fold
let &fdl = &fdl
let s:l = 292 - ((42 * winheight(0) + 21) / 43)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 292
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
