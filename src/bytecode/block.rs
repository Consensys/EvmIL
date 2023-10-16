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
use std::fmt;
use crate::util::SubsliceOffset;
use super::Instruction;
use super::{BlockIterator,ByteOffsetIterator};

/// A _block decomposition_ of a given bytecode sequence.  This is a
/// non-overlapping decomponsition of single instructions into
/// instruction blocks.
#[derive(Debug,Clone,PartialEq)]
pub struct BlockVec<'a> {
    /// Instruction sequence from which all blocks are extracted.
    insns: &'a [Instruction],
    /// Blocks as given by their ending (instruction) offset within
    /// the sequence.  Observe that this sequence is maintained in
    /// sorted order at all times.
    insn_offsets: Vec<usize>,
    /// Starting byte offsets for the first instruction in each block.
    /// Observe that this sequence is maintained in sorted order at
    /// all times.
    pc_offsets: Vec<usize>        
}

impl<'a> BlockVec<'a> {
    pub fn new(insns: &'a [Instruction]) -> Self {
        let b_iter = BlockIterator::new(insns);
        let bo_iter = ByteOffsetIterator::new(insns);
        // Identify block boundaries        
        let insn_offsets: Vec<_> = b_iter.map(|b| insns.subslice_offset(b)+b.len()).collect();
        // Identify PC offsets
        let pc_offsets: Vec<_> = bo_iter.collect();
        // Done
        Self{insns,insn_offsets,pc_offsets}
    }

    /// Returns the number of blocks within this decomposition.
    pub fn len(&self) -> usize {
        self.insn_offsets.len()
    }

    /// Get the _ith_ block within this decomposition.
    pub fn get(&self, index: usize) -> &'a [Instruction] {
        let m = if index == 0 { 0 } else { self.insn_offsets[index-1] };
        let n = self.insn_offsets[index];
        &self.insns[m..n]
    }

    /// Determine which block encloses a given instruction (as
    /// determined by its _instruction offset_).
    pub fn lookup_insn(&self, index: usize) -> usize {
        match self.insn_offsets.binary_search(&index) {
            Ok(i) => {
                // In this case, we matched the target index within
                // the instruction offsets.  This means the
                // instruction is part of the _next_ block.
                i+1
            }
            Err(i) => {
                // In this case, the target offset is not directly
                // expressed wtihin the instruction offsets.  This
                // means the instruction is determined by this block.
                i
            }
        }
    }

    /// Determine which block encloses a given instruction (as
    /// determined by its _byte offset_ within the original byte
    /// sequence).
    pub fn lookup_pc(&self, pc: usize) -> usize {
        // Use binary search for efficiency!  Observe that it doesn't
        // actually matter whether or not the given pc value is
        // contained within the list of pc_offsets.  What matters is
        // *where* we would insert it.
        match self.pc_offsets.binary_search(&pc) {
            Ok(i) => self.lookup_insn(i),
            Err(i) => self.lookup_insn(i)
        }
    }    
}

impl<'a> fmt::Display for BlockVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(")?;
        for i in 0..self.len() {
            let ith = self.get(i);            
            if i!=0 { write!(f,";"); }
            for j in 0..ith.len() {
                if j != 0 { write!(f,","); }
                write!(f,"{}",ith[j]);
            }
        }
        write!(f,")")
    }
}
