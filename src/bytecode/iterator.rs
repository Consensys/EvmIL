// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::util::SubsliceOffset;
use crate::bytecode::Instruction;
use Instruction::*;

/// An iterator over the _byte offsets_ for a given instruction
/// sequence.  The byte offset for an instruction its its byte
/// position within the original byte sequence.  For example:
///
/// ```txt
///    push 0x80 ;; 0
///    push 0x60 ;; 2
///    mstore    ;; 3
/// ```
///
/// Here, we see the instructions with their byte offsets shown to the
/// right.
pub struct ByteOffsetIterator<'a> {
    insns: &'a [Instruction],
    pc: usize
}

impl<'a> ByteOffsetIterator<'a> {
    pub fn new(insns: &'a [Instruction]) -> Self {
        Self{insns, pc:0}
    }
}

impl<'a> Iterator for ByteOffsetIterator<'a> {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.insns.len() == 0 {
            None
        } else {
            let mpc = self.pc;
            // Update pc position
            self.pc += self.insns[0].length();
            // Strip off head instruction
            self.insns = &self.insns[1..];
            // Done
            Some(mpc)
        }
    }                
}

/// An iterator over the basic blocks of an instruction sequence.  A
/// basic block can only have a single entry point, and may have zero
/// or more exits.  For example, consider the following:
///
/// ```txt
///    .code
///    push 0x80    -+
///    push 0x60     |
///    mstore        | 
///    calldatasize  |
///    push lab1     |
///    jumpi         |
///    push lab2     |
///    jump         -+
/// lab1:
///    push 0x00    -+
///    push 0x00     |
///    revert       -+
/// lab2
///    stop         -+
/// ```
///
/// This example consists of three basic blocks.  The first has two
/// exits (given by `jumpi` and `jump` respectively), whilst the
/// latter two blocks have none.
pub struct BlockIterator<'a> {
    insns: &'a [Instruction]    
}

impl<'a> BlockIterator<'a> {
    pub fn new(insns: &'a [Instruction]) -> Self {
        Self{insns}
    }
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = &'a [Instruction];

    fn next(&mut self) -> Option<Self::Item> {
        if self.insns.len() == 0 {
            None
        } else {
            let mut i = 0;
            //
            while i < self.insns.len() {
                match &self.insns[i] {
                    JUMPDEST => {
                        // Can only be the start of a basic block.
                        if i != 0 { break; }
                    }
                    INVALID|JUMP|RETURN|REVERT|SELFDESTRUCT|STOP => {
                        // Instructions which always terminate a basic
                        // block.
                        i += 1;
                        break;
                    }
                    _ => {
                        // Everything else.
                    }
                }
                i += 1;
            }
            // Extract the block
            let tmp = self.insns;
            let block = &self.insns[..i];
            // Update position within instruction sequence
            self.insns = &self.insns[i..];
            // Done
            Some(block)
        }                
    }
}


