.code
        push 0x01
        iszero
        push lab1
        jumpi
        push 0x00
        push lab0
        jumpi
lab1:
        jumpdest
lab0:
        jumpdest
