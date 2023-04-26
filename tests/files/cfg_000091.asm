.code
        push 0xff
        push 0x00
        sload
        push lab0
        jumpi
        pop
lab0:
        jumpdest
        push 0x00
