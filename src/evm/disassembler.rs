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
use std::fmt::{Debug};
use crate::evm::{AbstractEvm, Stack, Stepper};
use crate::ll::{Instruction, Instruction::*};
use crate::util::{
    w256, Bottom, Concretizable, IsBottom, JoinInto, JoinLattice, JoinSemiLattice,
};

// ============================================================================
// Disassembly
// ============================================================================

/// Identifies a sequential block of instructions within the original
/// bytecode sequence.  That is, a sequence does not contain a jump
/// destination (other than at the very start), and ends either with a
/// terminating instruction (e.g. `RETURN`, `REVERT`, etc) or an
/// unconditional branch (to another block).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Block {
    /// Starting offset (in bytes) of this block.
    pub start: usize,
    /// End offset (in bytes) of this block.  That is the first byte
    /// which is not part of this block.
    pub end: usize,
}

impl Block {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end);
        //
        Block { start, end }
    }

    /// Check whether this block encloses (i.e. includes) the given
    /// bytecode address.
    pub fn encloses(&self, pc: usize) -> bool {
        self.start <= pc && pc < self.end
    }
}

// ============================================================================
// Disassembly
// ============================================================================

/// Identifies all contiguous code blocks within the bytecode program.
/// Here, a block is a sequence of bytecodes terminated by either
/// `STOP`, `REVERT`, `RETURN` or `JUMP`.  Observe that a `JUMPDEST`
/// can only appear as the first instruction of a block.  In fact,
/// every reachable block (except the root block) begins with a
/// `JUMPDEST`.
pub struct Disassembly<'a, S: Stack>
where
    S: Clone + Ord + Stack + JoinSemiLattice,
    S::Word: JoinLattice + Concretizable<Item = w256> + Debug
{
    /// The bytes we are disassembling.
    bytes: &'a [u8],
    /// The set of known blocks (in order).
    blocks: Vec<Block>,
    /// The (incoming) contexts for each block.
    contexts: Vec<AbstractEvm<'a, S>>,
}

impl<'a, S> Disassembly<'a, S>
where
    S: Clone + Stack + Ord + JoinSemiLattice,
    S::Word: JoinLattice + Concretizable<Item = w256> + Debug
{
    pub fn new(bytes: &'a [u8]) -> Self {
        // Perform linear scan of blocks
        let blocks = Self::scan_blocks(bytes);
        // Construct default contexts
        let mut contexts = vec![AbstractEvm::BOTTOM; blocks.len()];
        // Update origin context
        contexts[0] = AbstractEvm::new(bytes);
        // Done
        Disassembly {
            bytes,
            blocks,
            contexts,
        }
    }

    /// Get the state at a given program location.
    pub fn get_state(&self, loc: usize) -> AbstractEvm<'a, S> {
        // Determine enclosing block
        let bid = self.get_enclosing_block_id(loc);
        let mut st = self.contexts[bid].clone();
        // Reconstruct state
        while st.pc < loc {
            // Apply the transfer function!
            (st, _) = st.step();
        }
        // Done
        st
    }

    /// Get the enclosing block for a given bytecode location.
    pub fn get_enclosing_block(&self, pc: usize) -> &Block {
        for i in 0..self.blocks.len() {
            if self.blocks[i].encloses(pc) {
                return &self.blocks[i];
            }
        }
        panic!("invalid bytecode address");
    }

    /// Determine whether a given block is currently considered
    /// reachable or not.  Observe the root block (`id=0`) is _always_
    /// considered reachable.
    pub fn is_block_reachable(&self, id: usize) -> bool {
        id == 0 || self.contexts[id] != AbstractEvm::BOTTOM
    }

    /// Read a slice of bytes from the bytecode program, padding with
    /// zeros as necessary.
    pub fn read_bytes(&self, start: usize, end: usize) -> Vec<u8> {
        let n = self.bytes.len();

        if start >= n {
            vec![0; end - start]
        } else if end > n {
            // Determine lower potion
            let mut slice = self.bytes[start..n].to_vec();
            // Probably a more idiomatic way to do this?
            for _i in end..n {
                slice.push(0);
            }
            //
            slice
        } else {
            // Easy case
            self.bytes[start..end].to_vec()
        }
    }

    /// Flattern the disassembly into a sequence of instructions.
    pub fn to_vec(&self) -> Vec<Instruction> {
        let mut insns = Vec::new();
        // Iterate blocks in order
        for i in 0..self.blocks.len() {
            let blk = &self.blocks[i];
            let ctx = &self.contexts[i];
            // Check for reachability
            if i == 0 || ctx != &AbstractEvm::BOTTOM {
                // Disassemble block
                self.disassemble_into(blk, &mut insns);
            } else {
                // Not reachable, so must be data.
                let data = self.read_bytes(blk.start, blk.end);
                //
                insns.push(DATA(data));
            }
        }
        //
        insns
    }

    // ================================================================
    // Helpers
    // ================================================================

    /// Disassemble a given block into a sequence of instructions.
    fn disassemble_into(&self, blk: &Block, insns: &mut Vec<Instruction>) {
        let mut pc = blk.start;
        // Parse the block
        while pc < blk.end {
            // Decode instruction at the current position
            let insn = Instruction::decode(pc, &self.bytes);
            // Increment PC for next instruction
            pc = pc + insn.length();
            //
            insns.push(insn);
        }
    }

    /// Perform a linear scan splitting out the blocks.  This is an
    /// over approximation of the truth, as some blocks may turn out
    /// to be unreachable (e.g. they are data).
    fn scan_blocks(bytes: &[u8]) -> Vec<Block> {
        let mut blocks = Vec::new();
        // Current position in bytecodes
        let mut pc = 0;
        // Identifies start of current block.
        let mut start = 0;
        // Parse the block
        while pc < bytes.len() {
            // Decode instruction at the current position
            let insn = Instruction::decode(pc, &bytes);
            // Increment PC for next instruction
            pc = pc + insn.length();
            // Check whether terminating instruction
            match insn {
                JUMPDEST => {
                    // Determine whether start of this block, or next
                    // block.
                    if (pc - 1) != start {
                        // Start of next block
                        blocks.push(Block::new(start, pc - 1));
                        start = pc - 1;
                    }
                }
                INVALID | JUMP | RJUMP(_) | RETURN | REVERT | SELFDESTRUCT | STOP => {
                    blocks.push(Block::new(start, pc));
                    start = pc;
                }
                _ => {}
            }
        }
        // Append last block (if necessary)
        if start != pc {
            blocks.push(Block::new(start, pc));
        }
        // Done
        blocks
    }

    /// Determine the enclosing block number for a given bytecode
    /// address.
    fn get_enclosing_block_id(&self, pc: usize) -> usize {
        for i in 0..self.blocks.len() {
            if self.blocks[i as usize].encloses(pc) {
                return i;
            }
        }
        panic!("invalid bytecode address");
    }
}

impl<'a, S> Disassembly<'a, S>
where
    S: Clone + Debug + Ord + Stack + JoinSemiLattice,
    S::Word: Debug + Concretizable<Item = w256> + JoinLattice
{
    /// Apply flow analysis to refine the results of this disassembly.
    pub fn build(mut self) -> Self {
        let mut changed = true;
        //
        while changed {
            // Reset indicator
            changed = false;
            // Iterate blocks in order
            for i in 0..self.blocks.len() {
                // Sanity check whether block unreachable.
                if !self.is_block_reachable(i) {
                    continue;
                }
                // Yes, is reachable so continue.
                let blk = &self.blocks[i];
                let mut st = self.contexts[i].clone();
                // println!("CONTEXT (pc={}): {}", pc, ctx);
                // Parse the block
                while !st.is_bottom() && st.pc < blk.end {
                    // Execute next instruction
                    let (nst, branches) = st.step();
                    // Merge any branches
                    for b in &branches {
                        // Convert target into block ID.
                        let block_id = self.get_enclosing_block_id(b.pc);
                        // Merge in updated state
                        changed |= self.contexts[block_id].join_into(&b);
                    }
                    st = nst;
                }
                // Merge state into following block.
                if (i + 1) < self.blocks.len() {
                    changed |= self.contexts[i + 1].join_into(&st);
                }
            }
        }
        self
    }
}

// ============================================================================
// Disassemble Trait
// ============================================================================

// Provides a default disassembly pipeline for standard types
// (e.g. string slices, byte slices, etc).
// pub trait Disassemble {
//     fn disassemble<'a>(&'a self) -> Disassembly<'a,()>;
// }

// impl<T:AsRef<[u8]>> Disassemble for T {
//     fn disassemble<'a>(&'a self) -> Disassembly<'a,()> {
//         Disassembly::new(self.as_ref())
//     }
// }
