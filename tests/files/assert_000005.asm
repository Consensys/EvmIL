   push 0x00
   push 0x01
   sload
   lt
   iszero
   push lab0
   jumpi
   invalid
lab0:
   jumpdest
