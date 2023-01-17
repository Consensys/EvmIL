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
use crate::evm::opcode::*;
use crate::evm::ConcreteStack;
use crate::evm::{Evm, Stack, Stepable, Word};
use crate::util::w256;

// ===================================================================
// Concrete EVM
// ===================================================================

#[derive(Debug, PartialEq)]
pub enum ConcreteResult<'a> {
    Continue(ConcreteEvm<'a>),
    Return { data: Vec<u8> },
    Revert { data: Vec<u8> },
}

pub type ConcreteEvm<'a> = Evm<'a, ConcreteStack<w256>>;

impl<'a> ConcreteEvm<'a> {
    /// Execute the contract to completion.
    pub fn run(mut self) -> ConcreteResult<'a> {
        // Eventually, this needs a return type.
        loop {
            let r = self.step();

            match r {
                ConcreteResult::Continue(evm) => {
                    self = evm;
                }
                _ => {
                    return r;
                }
            }
        }
    }
}

impl<'a> Stepable for ConcreteEvm<'a> {
    type Result = ConcreteResult<'a>;

    /// Execute instruction at the current `pc`.
    fn step(self) -> Self::Result {
        let opcode = self.code[self.pc];
        //
        match opcode {
            STOP => Self::Result::Return { data: Vec::new() },
            //
            ADD => {
                let lhs = self.stack.peek(1);
                let rhs = self.stack.peek(0);
                Self::Result::Continue(self.pop(2).push(lhs + rhs).next(1))
            }
            PUSH1..=PUSH32 => {
                // Determine push size
                let n = ((opcode - PUSH1) + 1) as usize;
                let pc = self.pc + 1;
                // Extract bytes
                let bytes = &self.code[pc..pc + n];
                // Convert bytes into w256 word
                let w = w256::from_be_bytes(bytes);
                // Done
                Self::Result::Continue(self.push(w.into()).next(n + 1))
            }
            //
            _ => {
                panic!("unknown instruction encountered");
            }
        }
    }
}
