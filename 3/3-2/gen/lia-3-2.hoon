/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/3-2/wasm
::
=/  lv  lia-value:lia-sur:wasm
|=  input=wain
^-  @
=/  input-cord=cord  (of-wain:format input)
=/  len0=@  (met 3 input-cord)
=;  out=(list lv)
  ?>  ?=([[%i32 *] ~] out)
  +.i.out
%-  yield-need:wasm
%^  run-once:wasm  [bin ~]  %$
=/  m  runnable:wasm
=,  wasm
;<  ptr0=@  try:m  (call-1 '__wbindgen_malloc' len0 1 ~)
;<  ~       try:m  (memwrite ptr0 len0 input-cord)
;<  ret=@   try:m  (call-1 'solve' ptr0 len0 ~)
(return:m i32+ret ~)
