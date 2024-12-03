|=  input=wain
|^  ^-  @
=/  input-cord=cord  (of-wain:format input)
=<  -
%-  roll  :_  add-commands
^-  (list command)
%+  rash  input-cord
commands-noise
::
+$  command
  $%  [%do ~]
      [%dont ~]
      [%mul p=@ q=@]
  ==
::
++  commands-noise
  %+  knee  *(list command)
  |.  ~+
  ;~  pose
    (full (easy ~))
    ;~(plug command-rule commands-noise)
    ;~(pfix next commands-noise)
  ==
++  command-rule
  %+  cook  |=(command +<)
  ;~  pose
    mul-rule
    (cold do+~ (jest 'do()'))
    (cold dont+~ (jest 'don\'t()'))
  ==
::
++  mul-rule
  %+  cook  |=(command +<)
  ;~  pfix
    (jest 'mul(')
    %+  stag  %mul
    ;~  plug
      dem
      ;~  pfix
        com
        ;~(sfix dem par)
      ==
    ==
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