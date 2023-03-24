   push 0x02
   push 0x01
   lt
   iszero
   push lab1
   jumpi
   push 0x03
   push 0x02
   lt
   push lab0
   jumpi
lab1:
   jumpdest
   invalid
lab0:
   jumpdest
