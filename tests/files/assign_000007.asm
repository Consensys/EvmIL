.code
        push 0x00
        calldataload
        dup1
        push lab0
        jumpi
        pop
        push 0x00
lab0:
        jumpdest
        push 0x00
        mstore
