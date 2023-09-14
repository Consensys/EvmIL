.code
        push lab0
        jump
        ;; Unreachable code
        db 0x6000
        db 0x61000f
        db 0x57
        db 0x6000
        db 0x6000
        db 0xfd
        db 0x5b
lab0:
        jumpdest
