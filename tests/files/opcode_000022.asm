.code
        push lab0
        push 0x01
        dup1
        dup1
        calldatacopy
        jump
lab0:
        jumpdest
