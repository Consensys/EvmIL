.code
        push 0x00
        sload
        push 0x0d
        jumpi
        push 0x0b
        push 0x19
        jump
        jumpdest
        stop
        jumpdest
        push 0x13
        push 0x19
        jump
        jumpdest
        push 0x00
        push 0x00
        revert
        jumpdest
        push 0x00
        sload
        push 0x22
        jumpi
        jump
.data
        0x00
.code
        jumpdest
        push 0x00
        push 0x00
        revert
