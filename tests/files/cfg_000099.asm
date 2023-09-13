.code
        push 0x00
        sload
        push lab1
        jumpi
        push lab0
        push lab3
        jump
lab0:
        jumpdest
        stop
lab1:
        jumpdest
        push lab2
        push lab3
        jump
lab2:
        jumpdest
        push 0x00
        push 0x00
        revert
lab3:
        jumpdest
        jump
        stop
