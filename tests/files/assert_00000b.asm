.code
        push 0x00
        calldataload
        push lab0
        jumpi
        push 0x00
        push 0x00
        revert
lab0:
        jumpdest
