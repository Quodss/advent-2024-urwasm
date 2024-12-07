/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/7-2/wasm
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
;<  ptr0=@  try:m  (call-1 '__wbindgen_malloc' len0 1 ~)
;<  ~       try:m  (memwrite ptr0 len0 input-cord)
(call-1 'solve' ptr0 len0 ~)

