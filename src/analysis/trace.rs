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
use std::fmt::Debug;
use crate::util::{Bottom,Top};
use crate::bytecode::Instruction;
use super::{EvmState,EvmStateSet};
use super::semantics::{execute,Outcome};

pub fn trace<T>(insns: &[Instruction], init: T::State, limit: usize) -> Result<Vec<T>,Vec<T>>
where T:EvmStateSet+Bottom+PartialEq+Debug,
      T::State: Clone, <T::State as EvmState>::Word: Top 
{
    // initialise state data
    let mut states = Vec::new();
    for _ in insns { states.push(T::BOTTOM); }
    // calculate byte offsets
    let offsets = determine_byte_offsets(insns);
    // Initialise worklist
    let mut worklist = vec![init];
    // Terminator
    let mut count = 0usize;
    // Iterate to a fixed point
    while !worklist.is_empty() && count != limit {
        let mut st = worklist.pop().unwrap();
        // Sanity check bytecode position
        if st.pc() >= offsets.len() {
            // Execution has fallen "of the end" of the bytecode
            // sequence.  When this happens the EVM immediately
            // executes a STOP instruction.
            continue;
        }
        // Determine instruction position
        let mut pc = st.pc();
        let mut ipc = offsets[pc];
        //
        while ipc < states.len() && states[ipc].join_into(&st) {
            let insn = &insns[ipc];
            // Update pc value (for next instruction)
            pc += insn.length();
            // Debug info
            // println!("[{ipc}:{}] {:?}",insns[ipc],states[ipc]);
            //
            match execute(insn,st) {
                Outcome::Return|Outcome::Exception(_) => {
                    // For now, we don't do anything specicial with
                    // accumulated returns.  However, at some point,
                    // it probably makes sense.
                    break;
                }
                Outcome::Continue(nst) => {
                    // Check for branch
                    if nst.pc() != pc {
                        worklist.push(nst);
                        break;
                    } else {
                        // Execution continues.
                        st = nst;
                    }
                }
                Outcome::Split(nst,bst) => {
                    // Execution splits
                    st = nst;
                    // Add branch
                    worklist.push(bst);
                }                
            }
            ipc += 1;
	    count+=1;		    
        }
    }
    // Sanity check whether hit the limit
    if count == limit {
	return Err(states)
    }
    // Done
    Ok(states)
}

fn determine_byte_offsets(insns: &[Instruction]) -> Vec<usize> {
    let mut offsets = Vec::new();

    for (i,insn) in insns.iter().enumerate() {
        let len = insn.length();
        for _ in 0..len { offsets.push(i); }       
    }
    // Done
    offsets
}
