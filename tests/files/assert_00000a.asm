.code
        push 0x02
        push 0x01
        lt
        iszero
        push lab0
        jumpi
        invalid
lab0:
        jumpdest
