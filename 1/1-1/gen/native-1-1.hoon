|=  input=wain
=|  left=(list @)
=|  right=(list @)
|-  ^-  @
?^  input
  =/  [l=@ r=@]  (rash i.input ;~(plug dem ;~(pfix ace ace ace dem)))
  $(left [l left], right [r right], input t.input)
=.  left  (sort left lth)
=.  right  (sort right lth)
=/  total-distance=@  0
|-  ^-  @
?~  left  total-distance
?>  ?=(^ right)
=.  total-distance
  %+  add  total-distance
  ?:  (gth i.left i.right)
    (sub i.left i.right)
  (sub i.right i.left)
$(left t.left, right t.right)