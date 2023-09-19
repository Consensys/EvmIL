# Overview

This is a library and tool for working with [EVM
bytecode](https://ethereum.org/en/developers/docs/evm/).  The tool
allows you to disassemble contracts into assembly language, and
assemble them back again.  The tool also supports a primitive
intermediate language which can be compiled into bytecode.

## Assembler / Disassembler

To illustrate the tool, we will first disassemble the bytecode
contract `0x60006000511161000f5760016000525b`.  We can do this as
follows:

```
evmil disassemble --code 0x60006000511161000f5760016000525b
```

This should produce the following output:

```
.code
        push 0x00
        push 0x00
        mload
        gt
        push 0x000f
        jumpi
        push 0x01
        push 0x00
        mstore
_0x000f:
        jumpdest
```

If we store this into a file `test.asm`, we can then assemble it back
as follows:

```
evmil assemble test.asm
```

And we should see our original bytecode being output:

```
0x60006000511161000f5760016000525b
```

Finally, when writing assembly language we can use labels for
simplicity.  For example, the above could be rewritten as follows:

```
.code
        push 0x00
        push 0x00
        mload
        gt
        push lab
        jumpi
        push 0x01
        push 0x00
        mstore
lab:
        jumpdest
```

This just makes writing the assembly language a bit easier.
