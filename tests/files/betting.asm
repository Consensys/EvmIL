.code
	push 0x80
	push 0x40
	mstore
	push 0x04
	calldatasize
	lt
	push lab0
	jumpi
	push 0x00
	calldataload
	push 0xe0
	shr
	dup1
	push 0x11610c25
	eq
	push lab1
	jumpi
	dup1
	push 0x310bd74b
	eq
	push lab3
	jumpi
	dup1
	push 0xd4b83992
	eq
	push lab6
	jumpi
lab0:
	jumpdest
	push 0x00
	dup1
	revert
lab1:
	jumpdest
	push lab2
	push lab9
	jump
lab2:
	jumpdest
	stop
lab3:
	jumpdest
	callvalue
	dup1
	iszero
	push lab4
	jumpi
	push 0x00
	dup1
	revert
lab4:
	jumpdest
	pop
	push lab2
	push lab5
	calldatasize
	push 0x04
	push lab16
	jump
lab5:
	jumpdest
	push lab13
	jump
lab6:
	jumpdest
	callvalue
	dup1
	iszero
	push lab7
	jumpi
	push 0x00
	dup1
	revert
lab7:
	jumpdest
	pop
	push lab8
	push 0x00
	sload
	dup2
	jump
lab8:
	jumpdest
	push 0x40
	mload
	swap1
	dup2
	mstore
	push 0x20
	add
	push 0x40
	mload
	dup1
	swap2
	sub
	swap1
	return
lab9:
	jumpdest
	push 0x00
	sload
	callvalue
	gt
	iszero
	push lab10
	jumpi
	push 0x00
	dup1
	revert
lab10:
	jumpdest
	push 0x00
	dup1
	sload
	callvalue
	swap1
	sub
	dup1
	dup3
	sstore
	swap1
	sub
	push lab12
	jumpi
	push 0x40
	mload
	caller
	swap1
	selfbalance
	dup1
	iszero
	push 0x08fc
	mul
	swap2
	push 0x00
	dup2
	dup2
	dup2
	dup6
	dup9
	dup9
	call
	swap4
	pop
	pop
	pop
	pop
	iszero
	dup1
	iszero
	push lab11
	jumpi
	returndatasize
	push 0x00
	dup1
	returndatacopy
	returndatasize
	push 0x00
	revert
lab11:
	jumpdest
	pop
lab12:
	jumpdest
	jump
lab13:
	jumpdest
	push 0x0de0b6b3a7640000
	dup2
	gt
	iszero
	push lab14
	jumpi
	push 0x00
	dup1
	revert
lab14:
	jumpdest
	push 0x00
	sload
	iszero
	push lab15
	jumpi
	push 0x00
	dup1
	revert
lab15:
	jumpdest
	push 0x00
	sstore
	jump
lab16:
	jumpdest
	push 0x00
	push 0x20
	dup3
	dup5
	sub
	slt
	iszero
	push lab17
	jumpi
	push 0x00
	dup1
	revert
lab17:
	jumpdest
	pop
	calldataload
	swap2
	swap1
	pop
	jump
.data
	0xa26469706673582212207b57e8eca1da96d5c0491373554b699c55b7dfcbe6779ae889f253ae5f8366cd64736f6c63430008110033
