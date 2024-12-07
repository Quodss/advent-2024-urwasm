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
?:  =(perm ^~((pow 2 n-operators)))  sum
=;  folded=@
  ?:  =(result folded)  (add sum result)
  $(perm +(perm))
=<  -
%+  roll  `(list @)`+.operands
|:  [operand=*@ folded=`@`-.operands perm=perm]
^-  [@ @]
?:  =(0 (mod perm 2))  [(add folded operand) (div perm 2)]
[(mul folded operand) (div perm 2)]