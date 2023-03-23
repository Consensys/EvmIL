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
use crate::evm::{Stack, Stepper};
use crate::ll::{Instruction, Instruction::*};
use crate::util::{
    w256, Bottom, Concretizable, IsBottom, JoinInto, JoinLattice, JoinSemiLattice, Top, SortedVec
};

#[derive(Clone,PartialEq)]
pub struct AbstractEvm<'a, S>
where S: Stack + Clone + Ord + JoinSemiLattice,
      S::Word: Debug + JoinLattice + Concretizable<Item = w256> {
    /// Program Counter
    pub pc: usize,
    /// Bytecode being executed
    pub code: &'a [u8],
    /// The internal set of stacks
    stacks: SortedVec<S>
}

impl<'a, S: Stack + Clone + Ord + JoinSemiLattice> AbstractEvm<'a, S>
where S::Word: Debug + JoinLattice + Concretizable<Item = w256> {
    pub fn new(code: &'a [u8]) -> Self {
        let mut stacks = SortedVec::new();
        stacks.insert(S::default());
        Self{pc:0, code, stacks}
    }

    /// Peek 'n'th item on the stack.
    pub fn peek(&self, n: usize) -> S::Word {
        let mut w = S::Word::BOTTOM;
        self.stacks.iter().for_each(|s| { w.join_into(&s.peek(n)); });
        w
    }

    /// Pop `n` items of the stack.
    pub fn pop(mut self, n: usize) -> Self {
        self.stacks.iter_mut().for_each(|s| s.pop(n));
        self
    }

    /// Push a word onto the stack.
    pub fn push(mut self, word: S::Word) -> Self {
        self.stacks.iter_mut().for_each(|s| s.push(word.clone()));
        self
    }

    pub fn set(mut self, n: usize, word: S::Word) -> Self {
        self.stacks.iter_mut().for_each(|s| s.set(n,word.clone()));
        self
    }

    /// Shift the `pc` by `n` bytes.
    pub fn next(mut self, n: usize) -> Self {
        self.pc = self.pc + n;
        self
    }

    /// Update `pc` to a given location.
    pub fn goto(mut self, n: usize) -> Self {
        self.pc = n;
        self
    }
}

impl<'a, S: Stack + Clone + Ord + JoinSemiLattice> JoinInto for AbstractEvm<'a, S>
where S::Word: Debug + JoinLattice + Concretizable<Item = w256> {
    fn join_into(&mut self, other: &Self) -> bool {
        if other.is_bottom() {
            false
        } else if self.is_bottom() {
            self.pc = other.pc;
            self.code = other.code;
            self.stacks = other.stacks.clone();
            true // We've definitely changed
        } else {
            assert_eq!(self.pc, other.pc); // see #63
            // PROBLEM!!
            self.stacks.insert_all(&other.stacks)
        }
    }
}

impl<'a, S: Stack + Clone + Ord + JoinSemiLattice> Bottom for AbstractEvm<'a, S>
where S::Word: Debug + JoinLattice + Concretizable<Item = w256> {
    const BOTTOM: AbstractEvm<'a, S> = AbstractEvm{pc: 0, code: &[], stacks: SortedVec::new()};
}

impl<'a, S: Stack + Clone + Ord + JoinSemiLattice> Stepper for AbstractEvm<'a, S>
where
    S::Word: Debug + JoinLattice + Concretizable<Item = w256>,
{
    type Result = (AbstractEvm<'a, S>, Vec<AbstractEvm<'a, S>>);

    fn step(mut self) -> Self::Result {
        // Decode instruction at the current position
        let insn = Instruction::decode(self.pc, &self.code);
        // Increment Program Counter
        self = self.next(1);
        //
        let st = match insn {
            STOP => Self::BOTTOM,
            // 0s: Stop and Arithmetic Operations
            ADD | MUL | SUB | DIV | SDIV | MOD | SMOD | EXP | SIGNEXTEND => {
                self.pop(2).push(S::Word::TOP)
            }
            ADDMOD | MULMOD => self.pop(3).push(S::Word::TOP),
            // 0s: Stop and Arithmetic Operations
            ISZERO | NOT => self.pop(1).push(S::Word::TOP),
            // Binary Comparators
            LT | GT | SLT | SGT | EQ => self.pop(2).push(S::Word::TOP),
            // Binary bitwise operators
            AND | OR | XOR | BYTE | SHL | SHR | SAR => self.pop(2).push(S::Word::TOP),
            // 20s: Keccak256
            KECCAK256 => {
                // NOTE: there is some kind of compiler bug which is
                // preventing me from putting this case in the
                // expected position.
                self.pop(2).push(S::Word::TOP)
            }
            // 30s: Environmental Information
            ADDRESS => self.push(S::Word::TOP),
            BALANCE => self.pop(1).push(S::Word::TOP),
            ORIGIN => self.push(S::Word::TOP),
            CALLER => self.push(S::Word::TOP),
            CALLVALUE => self.push(S::Word::TOP),
            CALLDATALOAD => self.pop(1).push(S::Word::TOP),
            CALLDATASIZE => self.push(S::Word::TOP),
            CALLDATACOPY => self.pop(3),
            CODESIZE => self.push(S::Word::TOP),
            CODECOPY => self.pop(3),
            GASPRICE => self.push(S::Word::TOP),
            EXTCODESIZE => self.pop(1).push(S::Word::TOP),
            EXTCODECOPY => self.pop(4),
            RETURNDATASIZE => self.push(S::Word::TOP),
            RETURNDATACOPY => self.pop(3),
            EXTCODEHASH => self.pop(1).push(S::Word::TOP),
            // 40s: Block Information
            BLOCKHASH => self.pop(1).push(S::Word::TOP),
            COINBASE => self.push(S::Word::TOP),
            TIMESTAMP => self.push(S::Word::TOP),
            NUMBER => self.push(S::Word::TOP),
            DIFFICULTY => self.push(S::Word::TOP),
            GASLIMIT => self.push(S::Word::TOP),
            CHAINID => self.push(S::Word::TOP),
            SELFBALANCE => self.push(S::Word::TOP),
            // 50s: Stack, Memory, Storage and Flow Operations
            POP => self.pop(1),
            MLOAD => self.pop(1).push(S::Word::TOP),
            MSTORE | MSTORE8 => self.pop(2),
            SLOAD => self.pop(1).push(S::Word::TOP),
            SSTORE => self.pop(2),
            PC | MSIZE | GAS => self.push(S::Word::TOP),
            JUMPDEST => self, // nop
            // 60 & 70s: Push Operations
            PUSH(bytes) => {
                // Extract word from bytes
                let n = w256::from_be_bytes(&bytes);
                // Push word on stack, and advance pc.
                self.push(S::Word::from(n)).next(bytes.len())
            }
            // 80s: Duplicate Operations
            DUP(n) => {
                let m = (n - 1) as usize;
                let nth = self.peek(m).clone();
                self.push(nth)
            }
            // 90s: Exchange Operations
            SWAP(n) => {
                let m = n as usize;
                let x = self.peek(m).clone();
                let y = self.peek(0).clone();
                // FIXME: supporting swap would avoid cloning.
                self.set(0, x).set(m, y)
            }
            // a0s: Logging Operations
            LOG(n) => self.pop((n + 2) as usize),
            // f0s: System Operations
            CREATE => self.pop(3).push(S::Word::TOP),
            CALL | CALLCODE => self.pop(7).push(S::Word::TOP),
            DELEGATECALL | STATICCALL => self.pop(6).push(S::Word::TOP),
            CREATE2 => self.pop(4).push(S::Word::TOP),
            JUMP => {
                let mut branches = Vec::new();
                // NOTE: performance could be improved here my
                // coalescing states which have the same target pc.
                // Unclear whether this is useful or not?
                for s in &self.stacks {
                    // Extract jump address
                    let target: usize = s.peek(0).constant().into();
                    let mut stack = s.clone();
                    stack.pop(1);
                    let mut stacks = SortedVec::new();
                    stacks.insert(stack);
                    // Create new EVM
                    let nevm = Self{pc:target, code: self.code, stacks};
                    //
                    branches.push(nevm);
                }
                // Branch!
                return (AbstractEvm::BOTTOM, branches);
            }
            JUMPI => {
                let mut branches = Vec::new();
                // NOTE: performance could be improved here my
                // coalescing states which have the same target pc.
                // Unclear whether this is useful or not?
                for s in &self.stacks {
                    // Extract jump address
                    let target: usize = s.peek(0).constant().into();
                    let mut stack = s.clone();
                    // Pop jump address & value
                    stack.pop(2);
                    let mut stacks = SortedVec::new();
                    stacks.insert(stack);
                    // Create new EVM
                    let nevm = Self{pc:target, code: self.code, stacks};
                    //
                    branches.push(nevm);
                }
                // Pop jump address & value
                self = self.pop(2);
                // Branch!
                return (self, branches);
            }
            INVALID | RETURN | REVERT => AbstractEvm::BOTTOM,
            SELFDESTRUCT => { self.pop(1); AbstractEvm::BOTTOM },
            _ => {
                // This is a catch all to ensure no instructions are
                // missed above.
                panic!("S::Word::TOP instruction ({:?})", insn);
            }
        };
        //
        (st, Vec::new())
    }
}
