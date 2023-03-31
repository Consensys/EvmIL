.code
        push 0x00
        push 0x00
        mload
        gt
        push lab0
        jumpi
        push 0x01
        push 0x00
        mstore
lab0:
        jumpdest
