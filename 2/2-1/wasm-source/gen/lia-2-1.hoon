/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/2-1/wasm
::
=/  lv  lia-value:lia-sur:wasm
=/  cw  coin-wasm:wasm-sur:wasm
|=  input=wain
^-  @
=/  input-cord=cord  (of-wain:format input)
=/  len0=@  (met 3 input-cord)
=;  out=(list lv)
  ?>  ?=([[%i32 *] ~] out)
  +.i.out
%-  yield-need:wasm
|^
%^  run-once:wasm  [bin imports]  %$
=/  m  runnable:wasm
=,  wasm
;<  ptr0=@  try:m  (call-1 'get_arena' ~)
;<  ~       try:m  (memwrite ptr0 len0 input-cord)
;<  ret=@   try:m  (call-1 'solve' ptr0 len0 ~)
(return:m i32+ret ~)
::
++  imports
  %-  ~(gas by *import:lia-sur:wasm)
  :~
    :-  ['env' 'print']
    |=  args=(list cw)
    =/  m  (script:lia-sur:wasm (list cw))
    ?>  ?=([[%i32 *] ~] args)
    ~&  +.i.args
    (return:m ~)
  ::
  ==
--