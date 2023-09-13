.code
        push 0x02
        push 0x01
        lt
        push lab0
        jumpi
        push 0x03
        push 0x02
        lt
        push lab0
        jumpi
        push 0x00
        push 0x00
        revert
lab0:
        jumpdest
