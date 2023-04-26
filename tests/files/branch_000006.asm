.code
        push 0x01
        iszero
        push lab0
        jumpi
        push 0x00
        push lab1
        jumpi
lab0:
        jumpdest
lab1:
        jumpdest
