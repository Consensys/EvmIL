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

pub fn trace<T>(insns: &[Instruction], init: T::State) -> Vec<T>
where T:EvmStateSet+Bottom+PartialEq+Debug,
      T::State: Clone, <T::State as EvmState>::Word: Top 
{
    // initialise state data
    let mut states = Vec::new();
    for i in insns { states.push(T::BOTTOM); }
    // calculate byte offsets
    let offsets = determine_byte_offsets(insns);
    // Initialise worklist
    let mut worklist = vec![init];
    // Iterate to a fixed point
    while !worklist.is_empty() {
        let st = worklist.pop().unwrap();
        // Sanity check bytecode position
        if st.pc() >= offsets.len() {
            // Execution has fallen "of the end" of the bytecode
            // sequence.  When this happens the EVM immediately
            // executes a STOP instruction.
            continue;
        }
        // Determine instruction position
        let ipc = offsets[st.pc()];
        // Join state into set
        if states[ipc].join_into(&st) {
            // Something changed, therefore execute this state to
            // produce an updated state.
            match execute(&insns[ipc],st) {
                Outcome::Return|Outcome::Exception(_) => {
                    // For now, we don't do anything specicial with
                    // accumulated returns.  However, at some point,
                    // it probably makes sense.
                }
                Outcome::Continue(nst) => {
                    // Execution continues.
                    worklist.push(nst);
                }
                Outcome::Split(st1,st2) => {
                    // Execution splits
                    worklist.push(st1);
                    worklist.push(st2);
                }                
            }
        }
        // Debug info
        //println!("{:?}",states[ipc]);        
    }
    // Done
    states
}

fn determine_byte_offsets(insns: &[Instruction]) -> Vec<usize> {
    let mut offsets = Vec::new();

    for (i,insn) in insns.iter().enumerate() {
        let len = insn.length();
        for j in 0..len { offsets.push(i); }       
    }
    // Done
    offsets
}
