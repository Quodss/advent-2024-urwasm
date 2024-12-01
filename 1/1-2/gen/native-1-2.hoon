|=  input=wain
=|  left=(list @)
=|  right=(list @)
|-  ^-  @
?^  input
  =/  [l=@ r=@]  (rash i.input ;~(plug dem ;~(pfix ace ace ace dem)))
  $(left [l left], right [r right], input t.input)
=/  total-similarity=@  0
|-  ^-  @
?~  left  total-similarity
=.  total-similarity
  %+  add  total-similarity
  (mul i.left (lent (fand ~[i.left] right)))
$(left t.left)