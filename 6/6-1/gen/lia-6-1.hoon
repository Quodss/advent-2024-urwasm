/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/bin-6-1/wasm
::
=/  cw  coin-wasm:wasm-sur:wasm
|=  input=wain
|^  ^-  @
=/  input-cord=cord  (of-wain:format input)
=/  len0=@  (met 3 input-cord)
%-  yield-need:wasm
%^  (run-once:wasm @)  [bin imports]  %$
=/  m  (script:lia-sur:wasm @)
^-  form:m
=,  wasm
;<  ptr0=@  try:m  (call-1 'malloc' len0 ~)
;<  ~       try:m  (memwrite ptr0 len0 input-cord)
(call-1 'solve' ptr0 len0 len0 ~)
::
++  imports
  %-  ~(gas by *import:lia-sur:wasm)
  :~
    :-  ['wasi_snapshot_preview1' 'fd_write']
    |=  args=(pole cw)
    =/  m  (script:lia-sur:wasm (list cw))
    ^-  form:m
    =,  wasm
    ?>  ?=([[%i32 fd=@] [%i32 iovs-ptr=@] [%i32 iovs-len=@] [%i32 nwritten-ptr=@] ~] args)
    ?>  =(fd.args 1)
    =/  nwritten=@  0
    =/  iovs-i=@  0
    |-  ^-  form:m
    =*  outer  $
    ?:  =(iovs-i iovs-len.args)
      ;<  ~  try:m  (memwrite nwritten-ptr.args 4 nwritten)
      (return:m i32+0 ~)
    =/  iov-ptr=@  (add iovs-ptr.args (mul 8 iovs-i))
    ;<  [@ ptr=@]  try:m  (memread iov-ptr 4)
    ;<  [@ len=@]  try:m  (memread (add 4 iov-ptr) 4)
    =.  nwritten  (add nwritten len)
    =/  i=@  0
    =|  log-line=tape
    |-  ^-  form:m
    =*  inner  $
    ?:  =(i len)  outer(iovs-i +(iovs-i))
    ;<  [@ c=@]  try:m  (memread (add ptr i) 1)
    ?:  =(c 13)  inner(i +(i))
    ?:  =(c 10)
      ~&  `tape`(flop log-line)
      =.  log-line  ""
      inner(i +(i))
    =.  log-line  [c log-line]
    inner(i +(i))
  ::
  ==
--