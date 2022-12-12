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
use crate::{BinOp,Bytecode,Instruction,Region,Term};
use crate::instruction;
use crate::instruction::Instruction::*;

// ============================================================================
// Instruction Block
// ============================================================================

pub struct Block {
    /// Starting offset (in bytes) of this block.
    start: usize,
    /// End offset (in bytes) of this block.  That is the first byte
    /// which is not part of this block.
    end: usize
}

impl Block {
    pub fn new(start: usize, end: usize) -> Self {
        if start >= end {
            panic!("invalid block");
        }
        Block{start,end}
    }
}

// ============================================================================
// Disassembler
// ============================================================================

/// Responsible for turning a byte sequence into a (preliminary)
/// instruction sequence.
pub struct Disassembler<'a> {
    /// Access to the raw sequence of bytes.
    bytes: &'a [u8],
}

impl<'a> Disassembler<'a> {
    pub fn new(bytes: &'a[u8]) -> Self {
        Self{bytes}
    }

    pub fn disassemble(&mut self) -> Vec<Instruction> {
        todo!("got here");
    }
}

// ============================================================================
// Abstract Value
// ============================================================================

/// An abstract value is either a known constant, or an unknown
/// (i.e. arbitrary value).
enum Value {
    Known(usize),
    Unknown
}

// ============================================================================
// Disassembly Context
// ============================================================================

struct Context {
    pc: usize,
    stack: Vec<Value>
}

impl Context {
    pub fn new(pc: usize, stack: Vec<Value>) -> Self {
        Context{pc,stack}
    }

    /// Disassemble bytecode from a given context.  This produces a
    /// block of instructions, along with zero or more "follow on"
    /// contexts.  Observe that the context is destroyed during this
    /// process, since it is updated in place.
    pub fn disassemble(self, bytes: &[u8]) -> (Block,Vec<Context>) {
        let mut pc = self.pc;
        let mut stack = self.stack;
        let mut follows = Vec::new();
        // Parse the block
        while pc < bytes.len() {
            // Decode instruction at the current position
            let insn = Instruction::decode(&bytes[pc..]);
            // Increment PC for next instruction
            pc = pc + insn.length(&[]);
            // Check whether terminating instruction
            match insn {
                JUMPDEST(n) => {
                    // Determine whether this signals the start of this
                    // block, or the next block.
                    if pc != self.pc {
                        // Backtrack so its included in this block
                        pc = pc - 1;
                        break;
                    }
                }
                JUMP => {
                    // Construct new context
                    match stack.pop().unwrap() {
                        Value::Unknown => {
                            panic!("Unknown jump destination encountered");
                        }
                        Value::Known(n) => {
                            // Push follow on context
                            follows.push(Context::new(n,stack));
                            // Done
                            break;
                        }
                    }
                }
                RETURN|REVERT|STOP => {
                    // End of this block
                    break;
                }
                _ => {}
            }
        }
        //
        (Block::new(self.pc,pc),follows)
    }
}
