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
use crate::evm::{Evm, Stack, Stepper};
use crate::ll::{Instruction, Instruction::*};
use crate::util::{
    w256, Bottom, Concretizable, IsBottom, JoinInto, JoinLattice, JoinSemiLattice, Top,
};

impl<'a, S: Stack + Clone + JoinSemiLattice> JoinInto for Evm<'a, S> {
    fn join_into(&mut self, other: &Self) -> bool {
        if other.is_bottom() {
            false
        } else if self.is_bottom() {
            *self = (*other).clone();
            true // We've definitely changed
        } else {
            assert_eq!(self.pc, other.pc); // see #63
            self.stack.join_into(&other.stack)
        }
    }
}

impl<'a, S: Stack + JoinSemiLattice> Bottom for Evm<'a, S> {
    const BOTTOM: Evm<'a, S> = Evm::new_const(&[], S::BOTTOM);
}

impl<'a, S: Stack + Clone + JoinSemiLattice> Stepper for Evm<'a, S>
where
    S::Word: Debug + JoinLattice + Concretizable<Item = w256>,
{
    type Result = (Evm<'a, S>, Evm<'a, S>);

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
            JUMPDEST(_) => self, // nop
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
                // Extract jump address
                let target: usize = self.peek(0).constant().into();
                // Branch!
                return (Evm::BOTTOM, self.pop(1).goto(target));
            }
            JUMPI => {
                // Extract jump address
                let target: usize = self.peek(0).constant().into();
                // Pop jump address & value
                self = self.pop(2);
                let other = self.clone();
                // Branch!
                return (self, other.goto(target));
            }
            INVALID | RETURN | REVERT => Evm::BOTTOM,
            SELFDESTRUCT => self.pop(1),
            _ => {
                // This is a catch all to ensure no instructions are
                // missed above.
                panic!("S::Word::TOP instruction ({:?})", insn);
            }
        };
        //
        (st, Evm::BOTTOM)
    }
}
