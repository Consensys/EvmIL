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

/// A generic mechanism for decomposing an instruction sequence into a
/// collection of _basic blocks_, along with the _edges_ connecting
/// them together.  In essence, a block graph is a form of
/// _control-flow graph_.
trait BlockGraph {
    /// The type of _block identifiers_ used within this graph.
    type Bid;
    
    /// Returns the number of basic blocks stored within this graph.
    fn len(&self) -> usize;

    /// Returns a given `BasicBlock` within this graph.
    fn get(&self, ith: Bid) -> BasicBlock;

    /// Returns the set of blocks which can transfer control _into_ a
    /// given block.
    fn incoming(&self, ith: Bid) -> &[Bid];

    /// Returns the set of blocks to which a given block can transfer
    /// control.
    fn outgoing(&self, ith: Bid) -> &[Bid];

    /// Add a new basic block into this graph, and return its block
    /// identifier.
    fn add(&mut self, blk: BasicBlock) -> Bid;
}

/// Identifies a set of consecutive instructions within the original
/// instruction sequence.
struct BasicBlock {
    /// Byte offset (i.e. pc value) of the first instruction in this
    /// block.
    pub pc: usize,
    /// Instruction offset of first instruction within this block.
    pub start: usize,
    /// Instruction offset of first instruction _not_ within this
    /// block.  Thus, when `start == end` we have a _empty block_.
    pub end: usize
}
