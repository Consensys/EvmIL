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
use std::{fmt};
use crate::{Instruction,Instruction::*};
use crate::{AbstractState};
use crate::dfa::{AbstractValue,AbstractStack,BOTTOM_STACK,EMPTY_STACK};
use crate::util;
use crate::util::Interval;

const MAX_CODE_SIZE : u128 = 24576;
const UNKNOWN : AbstractValue = AbstractValue::Unknown;

// ============================================================================
// Disassembly Context
// ============================================================================

#[derive(Debug,PartialEq)]
pub struct CfaState {
    stack: AbstractStack
}

impl CfaState {
    pub fn new(stack: AbstractStack) -> Self {
        // Done
        Self{stack}
    }
    pub fn is_bottom(&self) -> bool {
        self.stack.is_bottom()
    }
    pub fn len(&self) -> Interval {
        self.stack.len()
    }
    pub fn push(self, val: AbstractValue) -> Self {
        CfaState::new(self.stack.push(val))
    }
    pub fn pop(self) -> Self {
        CfaState::new(self.stack.pop())
    }
    pub fn set(self, n:usize, val: AbstractValue) -> Self {
        CfaState::new(self.stack.set(n,val))
    }
}

impl Clone for CfaState {
    fn clone(&self) -> Self {
        CfaState::new(self.stack.clone())
    }
}

impl fmt::Display for CfaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.stack)
    }
}

impl AbstractState for CfaState {
    fn is_reachable(&self) -> bool { !self.stack.is_bottom() }

    fn branch(&self, _pc: usize, insn: &Instruction) -> Self {
        match insn {
            JUMPI => self.clone().pop().pop(),
            JUMP => self.clone().pop(),
            _ => {
                unreachable!()
            }
        }
    }

    fn peek(&self, n: usize) -> AbstractValue {
        self.stack.peek(n)
    }

    fn merge(&mut self, other: Self) -> bool {
        if *self != other {
            if !other.is_bottom() {
                if self.is_bottom() {
                    *self = other;
                    return true;
                } else {
                    return self.stack.merge_into(&other.stack);
                }
            }
        }
        false
    }

    fn bottom() -> Self { CfaState::new(BOTTOM_STACK) }

    fn origin() -> Self {
        CfaState::new(EMPTY_STACK)
    }

    // ============================================================================
    // Abstract Instruction Semantics (stack)
    // ============================================================================

    /// Update an abstract state with the effects of a given instruction.
    fn transfer(self, insn: &Instruction) -> CfaState {
        match insn {
            STOP => CfaState::bottom(),
            // 0s: Stop and Arithmetic Operations
            ADD|MUL|SUB|DIV|SDIV|MOD|SMOD|EXP|SIGNEXTEND => {
                self.pop().pop().push(UNKNOWN)
            }
            ADDMOD|MULMOD => {
                self.pop().pop().pop().push(UNKNOWN)
            }
            // 0s: Stop and Arithmetic Operations
            ISZERO|NOT => {
                self.pop().push(UNKNOWN)
            }
            // Binary Comparators
            LT|GT|SLT|SGT|EQ => {
                self.pop().pop().push(UNKNOWN)
            }
            // Binary bitwise operators
            AND|OR|XOR|BYTE|SHL|SHR|SAR => {
                self.pop().pop().push(UNKNOWN)
            }
            // 30s: Environmental Information
            ADDRESS => self.push(UNKNOWN),
            BALANCE => self.pop().push(UNKNOWN),
            ORIGIN => self.push(UNKNOWN),
            CALLER => self.push(UNKNOWN),
            CALLVALUE => self.push(UNKNOWN),
            CALLDATALOAD => self.pop().push(UNKNOWN),
            CALLDATASIZE => self.push(UNKNOWN),
            CALLDATACOPY => self.pop().pop().pop(),
            CODESIZE => self.push(UNKNOWN),
            CODECOPY => self.pop().pop().pop(),
            GASPRICE => self.push(UNKNOWN),
            EXTCODESIZE => self.pop().push(UNKNOWN),
            EXTCODECOPY => self.pop().pop().pop().pop(),
            RETURNDATASIZE => self.push(UNKNOWN),
            RETURNDATACOPY => self.pop().pop().pop(),
            EXTCODEHASH => self.pop().push(UNKNOWN),
            // 40s: Block Information
            // 50s: Stack, Memory, Storage and Flow Operations
            POP => self.pop(),
            MLOAD => self.pop().push(UNKNOWN),
            MSTORE => self.pop().pop(),
            SLOAD => self.pop().push(UNKNOWN),
            SSTORE => self.pop().pop(),
            JUMPI => self.pop().pop(),
            JUMPDEST(_) => self, // nop
            // 60 & 70s: Push Operations
            PUSH(bytes) => {
                let n = util::from_be_bytes(&bytes);
                if n <= MAX_CODE_SIZE {
                    self.push(AbstractValue::Known(n as usize))
                } else {
                    self.push(UNKNOWN)
                }
            }
            // 80s: Duplicate Operations
            DUP(n) => {
                let m = (*n - 1) as usize;
                let nth = self.peek(m);
                self.push(nth)
            }
            // 90s: Swap Operations
            SWAP(n) => {
                let m = (*n - 1) as usize;
                let x = self.peek(m);
                let y = self.peek(0);
                self.set(0,x).set(m,y)
            }
            // 90s: Exchange Operations
            // a0s: Logging Operations
            // f0s: System Operations
            INVALID|JUMP|RETURN|REVERT => {
                CfaState::bottom()
            }
            // 20s: Keccak256
            KECCACK256 => {
                // NOTE: there is some kind of compiler bug which is
                // preventing me from putting this case in the
                // expected position.
                self.pop().pop().push(UNKNOWN)
            }
            _ => {
                // This is a catch all to ensure no instructions are
                // missed above.
                panic!("unknown instruction ({:?})",insn);
            }
        }
    }
}
