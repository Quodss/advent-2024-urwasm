/+  wasm=wasm-lia
/*  bin  %wasm  /aoc-files/4/wasm
::
|=  input=wain
|^  ^-  @
=/  input-cord=cord  (of-wain:format input)
%-  yield-need:wasm
%^  (run-once:wasm @)  [bin ~]  %$
=/  m  (script:lia-sur:wasm @)
^-  form:m
=,  wasm
;<  input=@  try:m  (lower-array 4 input-cord)
?<  =(input 0)
(call-1 'part1' input ~)
::
++  lower-array
  =/  m  (script:lia-sur:wasm @)
  =,  wasm
  |=  [id=@ values=cord]  ::  align = 0, __setU8 inlined
  ^-  form:m
  =/  length=@  (met 3 values)
  ;<  new=@     try:m  (call-1 '__new' length 1 ~)
  ;<  buffer=@  try:m  (call-1 '__pin' new ~)
  ;<  new=@     try:m  (call-1 '__new' 16 id ~)
  ;<  header=@  try:m  (call-1 '__pin' new ~)
  ::
  ;<  ~         try:m  (memwrite header 4 buffer)
  ;<  ~         try:m  (memwrite (add header 4) 4 buffer)
  ;<  ~         try:m  (memwrite (add header 8) 4 length)
  ;<  ~         try:m  (memwrite (add header 12) 4 length)
  ;<  ~         try:m  (memwrite buffer length values)
  ::
  ;<  *         try:m  (call '__unpin' buffer ~)
  ;<  *         try:m  (call '__unpin' header ~)
  (return:m header)
--