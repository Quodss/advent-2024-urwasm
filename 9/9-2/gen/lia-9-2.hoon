/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/9-2/wasm
::
|=  input=wain
^-  @
=/  input-cord=cord  (of-wain:format input)
=/  len0=@  (met 3 input-cord)
%-  yield-need:wasm
%^  (run-once:wasm @)  [bin ~]  %$
=/  m  (script:lia-sur:wasm @)
^-  form:m
=,  wasm
;<  ptr0=@    try:m  (call-1 'malloc' len0 ~)
;<  ptr1=@    try:m  (call-1 'malloc' 8 ~)
;<  ~         try:m  (memwrite ptr0 len0 input-cord)
;<  *         try:m  (call 'solve' ptr0 len0 ptr1 ~)
;<  res=octs  try:m  (memread ptr1 8)
(return:m q.res)

