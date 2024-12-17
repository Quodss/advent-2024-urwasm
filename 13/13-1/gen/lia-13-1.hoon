/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/13-1/wasm
::
|=  input=wain
^-  @
=/  input-cord=cord  (cat 3 (of-wain:format input) '\0a')
=/  len0=@  (met 3 input-cord)
%-  yield-need:wasm  =<  -
%^  (run-once:wasm @ *)  [bin `~]  %$
=/  m  (script:lia-sur:wasm @ *)
=/  arr  (arrows:wasm *)
^-  form:m
=,  arr
;<  ptr0=@  try:m  (call-1 '__wbindgen_malloc' len0 1 ~)
;<  ~       try:m  (memwrite ptr0 len0 input-cord)
(call-1 'solve' ptr0 len0 ~)

