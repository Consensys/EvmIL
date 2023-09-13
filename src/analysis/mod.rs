mod memory;
mod semantics;
mod state;
mod stack;
mod storage;
mod word;

pub use memory::*;
pub use state::*;
pub use stack::*;
pub use storage::*;
pub use word::*;

use crate::util::{Bottom,JoinInto,Top};
use crate::bytecode::Instruction;
use semantics::{execute,Outcome};

pub fn trace<T>(insns: &[Instruction], init: T) -> Vec<T>
where T:EvmState+Clone+PartialEq+JoinInto+Bottom,
      T::Word : Top {
    // initialise data
    let mut states = vec![T::BOTTOM; insns.len()];
    // Initialise entry state
    states[0] = init;
    // calculate byte offsets
    let offsets = determine_byte_offsets(insns);
    // Iterate until a fixed point is reached
    while update(insns, &offsets, &mut states) {}
    //
    states
}

fn update<T>(insns: &[Instruction], offsets: &[usize], states: &mut [T]) -> bool
where T:EvmState+Clone+PartialEq+JoinInto+Bottom,
      T::Word : Top 
{
    let mut changed = false;
    //
    for i in 0..insns.len() {
        let insn = &insns[i];
        let st = states[i].clone();
        // FIXME: this is currently necessary because (amongst others)
        // EvmStack.has_capacity() cannot operate on an bottom state.
        if st == T::BOTTOM { continue; }
        //
        match execute(insn,st) {
            Outcome::Return|Outcome::Exception(_) => {
                // For now, we don't do anything specicial with
                // accumulated returns.  However, at some point,
                // it probably makes sense.
            }
            Outcome::Continue(nst) => {
                // Execution continues.
                let pc = offsets[nst.pc()];
                changed |= states[pc].join_into(&nst);
            }
            Outcome::Split(st1,st2) => {
                // Execution splits
                let pc1 = offsets[st1.pc()];
                let pc2 = offsets[st2.pc()];                
                changed |= states[pc1].join_into(&st1);
                changed |= states[pc2].join_into(&st2);
            }
        }        
    }
    //
    changed
}

fn determine_byte_offsets(insns: &[Instruction]) -> Vec<usize> {
    let mut offsets = Vec::new();

    for insn in insns {
        let len = insn.length();
        for i in 0..len { offsets.push(i); }       
    }
    // Done
    offsets
}
