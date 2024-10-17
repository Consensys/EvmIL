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
use crate::bytecode::{BlockVec,Instruction};
use crate::util::{Digraph,Concretizable,SubsliceOffset};
use super::{EvmState,EvmStack};
use super::{aw256,ConcreteStack,ConcreteState,trace,ConcreteMemory,UnknownStorage};

use Instruction::*;

type DefaultState = ConcreteState<ConcreteStack<aw256>,ConcreteMemory<aw256>,UnknownStorage<aw256>>;

/// A block graph is a directed graph over the basic blocks of a
/// bytecode sequence.
pub type BlockGraph<'a> = Digraph<BlockVec<'a>>;

impl<'a> BlockGraph<'a> {
    pub fn from_blocks(blocks: BlockVec<'a>, limit: usize) -> Result<Self,Self> {
	let insns = blocks.insns();
        // Construct block graph
        let mut graph = BlockGraph::new(blocks.len()+1,blocks);
        // Compute analysis results
        let init = DefaultState::new();
        // Run the abstract trace
	let mut err = false;
        let trace : Vec<Vec<DefaultState>> = match trace(insns,init,limit) {
	    Ok(states) => states,
	    Err(states) => { err = true; states} 
	};
        // Connect edges!
        for b in 0..graph.len() {
            let blk = graph.get(b);
            let start = insns.subslice_offset(blk);
            let end = start + blk.len();
            //
            for i in start..end {
                let insn = &insns[i];
                match insn {
                    JUMP|JUMPI => {
                        for st in &trace[i] {
                            let target : usize = st.stack().peek(0).constant().to();
                            // Convert the branch target (which is a
                            // byte offset) into the corresponding
                            // block offset.
                            let bid = graph.nodes().lookup_pc(target);
                            // Connect edge
                            graph.connect(b,bid);
                        }
                        if insn == &JUMP {
                            // Jump instruction doesn't fall through.
                            // Observe its safe to break here, since
                            // we know this instruction terminate the
                            // enclosinc basic block.
                            break;
                        }
                    }
                    INVALID|RETURN|REVERT|SELFDESTRUCT|STOP|DATA(_) => {
                        // Instructions which don't fall through.
                        // Observe its safe to break here, since we
                        // know these instructions terminate the
                        // enclosinc basic block.
                        break;
                    }
                    _ => {
                        // Instructions which do not branch, but do
                        // fall through to the following instruction.
                    }
                }
                // If we get here, then we have an instruction which
                // falls through to the next.  If this is the last
                // instruction in the block, then add an edge
                // accordingly in the graph.
                if (i+1) == end { graph.connect(b,b+1); }
            }
        }
        // Done
	if err {
	    Err(graph)
	} else {
            Ok(graph)
	}
    }
}

impl<'a> From<&'a [Instruction]> for BlockGraph<'a>
{
    /// Construct a graph of the basic blocks for a given instruction
    /// sequence.
    fn from(insns: &'a [Instruction]) -> Self {
        // Construct block graph
        BlockGraph::from(BlockVec::new(insns))
    }
}

impl<'a> From<BlockVec<'a>> for BlockGraph<'a>
{
    /// Construct a graph of the basic blocks for a given instruction
    /// sequence.
    fn from(blocks: BlockVec<'a>) -> Self {
	Self::from_blocks(blocks, usize::MAX).map_err(|_| ()).unwrap()
    }
}
