!.  |^
|=  input=wain
~>  %bout
^-  @
%+  roll  input
|=  [line=cord sum=@]
^-  @
=/  [result=@ operands=(list @)]
  (rash line ;~((glue (jest ': ')) dem (more ace dem)))
=/  n-operators=@  (dec (lent operands))
=/  perm=@  0
|-  ^-  @
?:  =(perm ^~((pow 3 n-operators)))  sum
=;  folded=@
  ?:  =(result folded)  (add sum result)
  $(perm +(perm))
=<  -
%+  roll  `(list @)`+.operands
|:  [operand=*@ folded=`@`-.operands perm=perm]
^-  [@ @]
?:  =(0 (mod perm 3))  [(add folded operand) (div perm 3)]
?:  =(1 (mod perm 3))  [(mul folded operand) (div perm 3)]
[(concat folded operand) (div perm 3)]
::
++  concat
  |=  [a=@ b=@]
  ~+  ^-  @
  =;  b-digits=@
    (add (mul a (pow 10 b-digits)) b)
  =/  b-digits=@  1
  |-  ^-  @
  ?:  (lth b 10)  b-digits
  $(b (div b 10), b-digits +(b-digits))
--