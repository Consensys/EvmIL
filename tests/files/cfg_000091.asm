.code
        push 0xff
        push 0x00
        sload
        push 0x09
        jumpi
        pop
        jumpdest
        push 0x00
