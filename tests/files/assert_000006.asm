   push 0x00
   push 0x01
   calldataload
   lt
   iszero
   push lab0
   jumpi
   invalid
lab0:
   jumpdest
