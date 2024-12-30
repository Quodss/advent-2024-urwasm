/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/10-1/wasm
::
|=  input=wain
^-  @
=/  input-cord=cord  (cat 3 (of-wain:format input) '\0a')
=/  len0=@  (met 3 input-cord)
%-  yield-need:wasm  =<  -
%^  (run-once:wasm @ *)  [bin `~]  %$
=/  m  (script:lia-sur:wasm @ *)
^-  form:m
=/  arr  (arrows:wasm *)
=,  arr
;<  ptr0=@    try:m  (call-1 'malloc' len0 ~)
;<  ~         try:m  (memwrite ptr0 len0 input-cord)
(call-1 'solve' ptr0 len0 ~)
