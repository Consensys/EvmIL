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
use crate::util::SortedVec;
use super::Instruction;
use super::BlockVec;

type EdgeSet = SortedVec<usize>;

/// A _bidirectional_ directed graph which stores edges in an
/// _adjacency list_ format.
pub struct BlockGraph<'a> {
    blocks: BlockVec<'a>,
    /// For each block, the corresponding set of incoming edges
    /// (i.e. edges which transfer control into this block).
    incoming: Vec<EdgeSet>,
    /// For each block, the corresponding set of outgoing edges
    /// (i.e. edges which transfer control out of this block).
    outgoing: Vec<EdgeSet>    
}

impl<'a> BlockGraph<'a> {
    pub fn new(blocks: BlockVec<'a>) -> Self {
        let incoming = Vec::new();
        let outgoing = Vec::new();
        Self{blocks,incoming,outgoing}
    }

    /// Returns the number of basic blocks stored within this graph.    
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    
    /// Returns the ith block within this graph.    
    pub fn get(&self, blk: usize) -> &'a [Instruction] {
        self.blocks.get(blk)
    }

    /// Determine which block encloses the given `pc` position
    /// (i.e. byte offset within original byte sequence).
    pub fn lookup_pc(&self, pc: usize) -> usize {
        self.blocks.lookup_pc(pc)
    }
    
    /// Returns the set of blocks which can transfer control _into_ a
    /// given block (`blk`).    
    pub fn incoming(&self, blk: usize) -> &[usize] {
        &self.incoming[blk]        
    }

    /// Returns the set of blocks to which a given block (`blk`) can
    /// transfer control.    
    pub fn outgoing(&self, blk: usize) -> &[usize] {
        &self.outgoing[blk]
    }

    /// Connect one basic block to another which forms a directed edge
    /// in the graph.  Returns `true` if a connection was added.
    pub fn connect(&mut self, from: usize, to: usize) -> bool {
        todo!();
    }

    /// Remove a connection between two basic blocks from the graph.
    /// Returns `true` if a connection was removed.
    pub fn disconnect(&mut self, from: usize, to: usize) -> bool {
        todo!();
    }
}
