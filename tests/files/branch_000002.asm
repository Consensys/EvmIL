.code
        push lab0
        jump
        push 0x00
        push lab1
        jumpi
        push 0x00
        push 0x00
        revert
lab1:   
        jumpdest
lab0:
        jumpdest
