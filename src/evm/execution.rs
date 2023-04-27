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
use std::ops;
use crate::util::{Bottom,Top};
use super::{EvmState,Instruction,Section};
use super::semantics::{execute,Outcome};

/// Simple alias since we're always dealing with concrete executions
/// here.
type Bytecode = super::Bytecode<Instruction>;

pub struct Execution<'a,T:EvmState+Clone+PartialEq> {
    /// The bytecode being executed by this execution.
    bytecode: &'a Bytecode,
    /// The set of states observed at each `pc` location.
    states: Vec<ExecutionSection<T>>
}

impl<'a,T:EvmState+Clone+PartialEq> Execution<'a,T>
where T::Word : Top {
    pub fn new(bytecode: &'a Bytecode) -> Self {
        let mut states = Vec::new();
        for s in bytecode {
            match s {
                Section::Code(insns) => {
                    states.push(ExecutionSection::new(insns));
                }
                Section::Data(_) => {
                    // Essentially a dummy
                    states.push(ExecutionSection::new(&[]));
                }
            }
        }
        Self{bytecode, states}
    }

    pub fn get(&self, index: usize) -> &ExecutionSection<T> {
        &self.states[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut ExecutionSection<T> {
        &mut self.states[index]
    }

    pub fn execute(&mut self, state: T) {
        let root : &Section<Instruction> = self.bytecode.iter().next().unwrap();
        // Access first section and begin execution from there.
        match root {
            Section::Code(_) => {
                self.states[0].execute(state);
            }
            Section::Data(_) => {
                // Data only contract?
            }
        }
    }
}

impl<'a,T:EvmState+Clone+PartialEq> ops::Index<usize> for Execution<'a,T>
where T::Word : Top {
    type Output = ExecutionSection<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<'a,T:EvmState+Clone+PartialEq> ops::IndexMut<usize> for Execution<'a,T>
where T::Word : Top {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

// ===================================================================
// Execution Section
// ===================================================================

pub struct ExecutionSection<T:EvmState+Clone+PartialEq> {
    bytecode: Vec<u8>,
    states: Vec<EvmSuperState<T>>
}

impl<T:EvmState+Clone+PartialEq> ExecutionSection<T>
where T::Word : Top {
    pub fn new(insns: &[Instruction]) -> Self {
        // Convert instructions into bytecodes.  Not really clear to
        // me how best to handle this, to be honest.
        let mut bytecode = Vec::new();
        for i in insns {
            // Just assuming this is safe for now.  The only way it
            // could not be would be if one of the instructions is
            // malformed.  That probably cannot arise by construction.
            i.encode(&mut bytecode).unwrap();
        }
        // Construct initially empty super states
        let states = vec![EvmSuperState::new(); bytecode.len()];
        //
        Self{bytecode,states}
    }

    pub fn get(&self, index: usize) -> &EvmSuperState<T> {
        &self.states[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut EvmSuperState<T> {
        &mut self.states[index]
    }

    pub fn execute(&mut self, state: T) {
        // FIXME: this is a very minimal initial implementation to
        // bootstrap the system and get things up and running.  It
        // certainly has problems for certain contracts, such as those
        // including loops.
        let mut worklist : Vec<T> = vec![state];
        //
        while worklist.len() > 0 {
            let st = worklist.pop().unwrap();
            // PC value is known for all states
            let pc = st.pc();
            // Record observed state
            if pc < self.states.len() {
                // Falling off the the end of a (legacy) bytecode
                // sequence indicates immediately that the machine has
                // stopped.
                if self.states[pc].join(st.clone()) {
                    // Decode the instruction
                    let insn = Instruction::decode(pc,&self.bytecode);
                    // Execute for next state
                    match execute(&insn,st) {
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
            }
        }
    }
}


impl<T:EvmState+Clone+PartialEq> ops::Index<usize> for
ExecutionSection<T> where T::Word : Top { type Output =
EvmSuperState<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T:EvmState+Clone+PartialEq> ops::IndexMut<usize> for ExecutionSection<T>
where T::Word : Top {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

// ===================================================================
// Execution State
// ===================================================================
type EvmSuperIter<'a,T> = std::slice::Iter<'a,T>;

#[derive(Clone,PartialEq)]
pub struct EvmSuperState<T:EvmState+Clone+PartialEq> {
    substates: Vec<T>
}

impl<T:EvmState+Clone+PartialEq> EvmSuperState<T> {
    pub fn new() -> Self {
        Self{substates: Vec::new()}
    }
    pub fn join(&mut self, state: T) -> bool {
        let n = self.substates.len();
        // Simplest possible join operator (for now)
        self.substates.push(state);
        // Deduplicate
        self.substates.dedup();
        // Check whether anything actually changed.
        n != self.substates.len()
    }
    pub fn iter<'a>(&'a self) -> EvmSuperIter<'a,T> {
        self.substates.iter()
    }
}

impl<T:EvmState+Clone+PartialEq> Bottom for EvmSuperState<T> {
    const BOTTOM : Self = EvmSuperState{substates:Vec::new()};
}
