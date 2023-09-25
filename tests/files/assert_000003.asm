.code
        push 0x00
        calldataload        
        push 0x01
        lt
        iszero
        push lab0
        jumpi
        push 0x03
        push 0x02
        lt
        push lab1
        jumpi
lab0:
        jumpdest
        push 0x00
        push 0x00
        revert
lab1:
        jumpdest
