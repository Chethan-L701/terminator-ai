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
9,11fold
14,20fold
24,30fold
23,31fold
22,32fold
35,43fold
46,56fold
61,64fold
65,69fold
61,72fold
61,72fold
76,78fold
79,81fold
82,84fold
85,87fold
88,91fold
92,95fold
96,99fold
75,102fold
74,103fold
58,105fold
45,106fold
117,118fold
117,119fold
117,120fold
117,121fold
117,122fold
116,123fold
127,128fold
127,129fold
127,130fold
127,131fold
125,132fold
134,144fold
148,149fold
148,150fold
148,151fold
148,152fold
154,158fold
154,160fold
154,160fold
146,161fold
163,167fold
108,168fold
171,174fold
185,188fold
192,214fold
191,215fold
216,230fold
191,231fold
191,231fold
170,237fold
247,248fold
247,249fold
246,250fold
246,251fold
246,252fold
246,253fold
246,254fold
246,255fold
246,256fold
246,257fold
263,272fold
263,273fold
262,274fold
261,275fold
277,283fold
277,284fold
276,285fold
261,286fold
261,286fold
289,293fold
289,294fold
288,295fold
239,298fold
305,307fold
305,308fold
305,309fold
304,310fold
303,312fold
316,318fold
316,319fold
316,320fold
315,321fold
314,323fold
302,324fold
301,325fold
300,326fold
331,334fold
340,343fold
340,345fold
340,345fold
338,346fold
347,350fold
338,351fold
338,351fold
328,354fold
let &fdl = &fdl
let s:l = 39 - ((38 * winheight(0) + 21) / 43)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 39
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
