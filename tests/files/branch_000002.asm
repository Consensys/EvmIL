.code
        push lab0
        jump
        push 0x00
        push lab1
        jumpi
        invalid
lab1:
        jumpdest
lab0:
        jumpdest
