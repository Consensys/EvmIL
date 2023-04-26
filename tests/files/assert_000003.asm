.code
        push 0x02
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
        invalid
lab1:
        jumpdest
