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
use crate::{Instruction,Instruction::*};
use crate::hex::FromHexString;
use crate::block::Block;
use crate::util;

const MAX_CODE_SIZE : u128 = 24576;

// ============================================================================
// Disassembly
// ============================================================================

/// A partially disassembled chunk of code.  This contains additional
/// meta-data which is useful for understanding the bytecode in
/// question.
pub struct Disassembly<'a> {
    /// The bytes we are disassembling.
    bytes: &'a [u8],
    /// The set of known blocks maintained in sorted order.
    blocks: Vec<Block>
}

impl<'a> Disassembly<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Disassembly{blocks: Vec::new(), bytes}
    }

    /// Flattern the disassembly into a sequence of instructions.
    pub fn to_vec(&self) -> Vec<Instruction> {
        let mut insns = Vec::new();
        let mut last = 0;
        // Iterate blocks in order
        for blk in &self.blocks {
            // Check for a gap
            if blk.start != last {
                // Extract all bytes in the gap
                let gap = self.bytes[last..blk.start].to_vec();
                // Register as data
                insns.push(Instruction::DATA(gap));
            }
            // Disassemble block
            self.disassemble_into(blk,&mut insns);
            // Update gap information
            last = blk.end;
        }
        // Check for gap
        if last != self.bytes.len() {
            // Extract all bytes in the gap
            let gap = self.bytes[last..].to_vec();
            // Register as data
            insns.push(Instruction::DATA(gap));
        }
        //
        insns
    }

    fn disassemble_into(&self, blk: &Block, insns: &mut Vec<Instruction>) {
        let mut pc = blk.start;
        // Parse the block
        while pc < blk.end {
            // Decode instruction at the current position
            let insn = Instruction::decode(pc,&self.bytes);
            // Increment PC for next instruction
            pc = pc + insn.length(&[]);
            //
            insns.push(insn);
        }
    }

    /// Insert a new block into this disassembly.
    fn insert(&mut self, blk: Block) {
        match self.blocks.binary_search(&blk) {
            Err(pos) => self.blocks.insert(pos, blk),
            _ => {}
        }
    }
}

// ============================================================================
// Disassembler
// ============================================================================

/// Responsible for turning a byte sequence into a `Disassembly`.
pub struct Disassembler<'a> {
    /// Access to the raw sequence of bytes.
    bytes: &'a [u8],
}

impl<'a> Disassembler<'a> {

    pub fn new<T:AsRef<[u8]> + ?Sized>(bytes: &'a T) -> Self {
        Self{bytes:bytes.as_ref()}
    }

    pub fn disassemble(&self) -> Disassembly<'a> {
        let mut blocks = Disassembly::new(self.bytes);
        let mut worklist = Vec::new();
        // Initialise worklist with root context
        worklist.push(Context::new(0,Vec::new()));
        // Continue disassembling until worklist empty
        while !worklist.is_empty() {
            // Get next context to work on
            let ctx = worklist.pop().unwrap();
            // Extract block (+follow ons)
            let (blk,mut follows) = ctx.apply(self.bytes);
            // Add "follow ons" to worklist
            worklist.append(&mut follows);
            blocks.insert(blk);
        }
        // Done
        blocks
    }
}

// ============================================================================
// Disassemble Trait
// ============================================================================

/// Provides a default disassembly pipeline for standard types
/// (e.g. string slices, byte slices, etc).
pub trait Disassemble {
    fn disassemble<'a>(&'a self) -> Disassembly<'a>;
}

impl<T:AsRef<[u8]>> Disassemble for T {
    fn disassemble<'a>(&'a self) -> Disassembly<'a> {
        Disassembler::new(self.as_ref()).disassemble()
    }
}

// ============================================================================
// Abstract Value
// ============================================================================

/// An abstract value is either a known constant, or an unknown
/// (i.e. arbitrary value).
#[derive(Clone)]
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

    /// Apply this context to a given sequence of bytes to identify
    /// the block at this point, along with any "follow on" contexts.
    /// This is done using a symbolic analysis of the stack as it
    /// holds at this point.  Observe that the context is destroyed
    /// during this process, since it is updated in place.
    pub fn apply(self, bytes: &[u8]) -> (Block,Vec<Context>) {
        let mut pc = self.pc;
        let mut stack = self.stack;
        let mut follows = Vec::new();
        // Parse the block
        while pc < bytes.len() {
            // Decode instruction at the current position
            let insn = Instruction::decode(pc,&bytes);
            // Increment PC for next instruction
            pc = pc + insn.length(&[]);
            // Check whether terminating instruction
            match insn {
                JUMPDEST(n) => {
                    // Determine whether this signals the start of this
                    // block, or the next block.
                    if (pc - 1) != self.pc {
                        // Backtrack so its included in this block
                        pc = pc - 1;
                        break;
                    }
                }
                JUMPI => {
                    match stack.last().unwrap() {
                        Value::Known(n) => {
                            // Push follow on context
                            follows.push(Context::new(*n,stack.clone()));
                        }
                        _ => { panic!("Unknown jumpi destination"); }
                    }
                }
                JUMP => {
                    match stack.last().unwrap() {
                        Value::Known(n) => {
                            // Push follow on context
                            follows.push(Context::new(*n,stack));
                            // Terminate control flow
                            break;
                        }
                        _ => { panic!("Unknown jump destination"); }
                    }
                }
                RETURN|REVERT|STOP => {
                    // End of this block
                    break;
                }
                _ => {}
            }
            // Apply semantics
            stack = update(insn,stack)
        }
        //
        (Block::new(self.pc,pc),follows)
    }
}


// ============================================================================
// Instruction Semantics (stack)
// ============================================================================

/// Update an abstract stack with the effects of a given instruction.
fn update(insn: Instruction, mut stack: Vec<Value>) -> Vec<Value> {
    match insn {
        // Binary arithmetic
        ADD|MUL|SUB|DIV|SDIV|MOD|SMOD|EXP|SIGNEXTEND => {
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        // Ternary arithmetic
        ADDMOD|MULMOD => {
            stack.pop();
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        // Unary operators
        ISZERO|NOT => {
            stack.pop();
            stack.push(Value::Unknown);
        }
        // Binary Comparators
        LT|GT|SLT|SGT|EQ => {
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        // Binary bitwise operators
        AND|OR|XOR|BYTE|SHL|SHR|SAR => {
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        // ...
        PUSH(bytes) => {
            let n = util::from_be_bytes(&bytes);
            if n <= MAX_CODE_SIZE {
                stack.push(Value::Known(n as usize));
            } else {
                stack.push(Value::Unknown);
            }
        }
        _ => {
            // no change
        }
    }
    stack
}
