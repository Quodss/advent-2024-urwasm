|=  input=wain
|^  ^-  @
=/  input-cord=cord  (of-wain:format input)
=<  -
%-  roll  :_  add-commands
^-  (list command)
%+  rash  input-cord
%+  ifix  [. .]:(star ;~(less command-rule next))
(more (star ;~(less command-rule next)) command-rule)
::
+$  command
  $%  [%do ~]
      [%dont ~]
      [%mul p=@ q=@]
  ==
::
++  command-rule
  %+  cook  |=(command +<)
  ;~  pose
    mul-rule
    (cold do+~ (jest 'do()'))
    (cold dont+~ (jest 'don\'t()'))
  ==
::
++  mul-rule
  %+  stag  %mul
  ;~  pfix
    (jest 'mul(')
    ;~(sfix ;~((glue com) dem dem) par)
  ==
::
++  add-commands
  |=  [com=command sum=@ flag=_&]
  ^-  [@ ?]
  ?-  -.com
    %do    [sum &]
    %dont  [sum |]
    %mul   [?:(flag (add sum (mul +.com)) sum) flag]
  ==

--