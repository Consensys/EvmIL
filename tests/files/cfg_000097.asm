.code
        push lab1
        jump
lab0:
        jumpdest
        stop
lab1:
        jumpdest
        push lab0
        jump
