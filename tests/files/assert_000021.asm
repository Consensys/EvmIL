.code
        push 0x02
        push 0x03
        sub
        push 0x01
        sub
        push lab0        
        jumpi
        push 0x00
        push 0x00
        revert
lab0:
        jumpdest
