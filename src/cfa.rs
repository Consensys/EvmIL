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
use crate::{Instruction,Instruction::*};
use crate::{AbstractState};
use crate::util;

const MAX_CODE_SIZE : u128 = 24576;

/// Bottom represents an _unvisited_ state.
const BOTTOM : CfaState = CfaState{stack: None};

// ============================================================================
// Abstract Value
// ============================================================================

/// An abstract value is either a known constant, or an unknown
/// (i.e. arbitrary value).
#[derive(Clone,Debug,PartialEq)]
enum Value {
    Known(usize),
    Unknown
}

// ============================================================================
// Disassembly Context
// ============================================================================

#[derive(Debug,PartialEq)]
pub struct CfaState {
    stack: Option<Vec<Value>>
}

impl CfaState {
    pub fn is_bottom(&self) -> bool {
        self.stack.is_none()
    }
    /// Pop an item of this stack, producing an updated state.
    fn pop(self) -> Self {
        match self.stack {
            Some(mut stack) => {
                // Pop target address off the stack.
                stack.pop();
                // Done
                CfaState{stack:Some(stack)}
            }
            None => {
                panic!("stack underflow");
            }
        }
    }
}

impl Default for CfaState {
    fn default() -> Self { BOTTOM }
}

impl Clone for CfaState {
    fn clone(&self) -> Self {
        CfaState{stack:self.stack.clone()}
    }
}

impl AbstractState for CfaState {
    fn is_reachable(&self) -> bool { self.stack.is_some() }

    fn transfer(self, insn: &Instruction) -> Self {
        let state = if self.is_bottom() {
            Vec::new()
        } else {
            self.stack.unwrap()
        };
        //
        update(insn,state)
    }

    fn branch(&self, pc: usize) -> Self {
        self.clone().pop()
    }

    fn merge(&mut self, other: Self) -> bool {
        if self.is_bottom() {
            if !other.is_bottom() {
                *self = other;
                return true;
            }
        } else if !other.is_bottom() {
            if self.stack != other.stack {
                // In principle, we could do better here.
                panic!("Cannot handle conflicting states ({:?} vs {:?})",self,other);
            }
        }
        //
        false
    }

    fn top(&self) -> usize {
        // Extract the stack.  We assume for now we are not bottom.
        let stack = self.stack.as_ref().unwrap();
        // Inspect last element.  Again, we assume for now this
        // exists.
        match stack.last().unwrap() {
            Value::Known(n) => *n,
            Value::Unknown => {
                // At some point, this will need to be fixed.
                panic!("Unknown value encountered");
            }
        }
    }
}
// ============================================================================
// Abstract Instruction Semantics (stack)
// ============================================================================

/// Update an abstract stack with the effects of a given instruction.
fn update(insn: &Instruction, mut stack: Vec<Value>) -> CfaState {
    match insn {
        STOP => {
            return BOTTOM;
        }
        // 0s: Stop and Arithmetic Operations
        ADD|MUL|SUB|DIV|SDIV|MOD|SMOD|EXP|SIGNEXTEND => {
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        ADDMOD|MULMOD => {
            stack.pop();
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        // 0s: Stop and Arithmetic Operations
        ISZERO|NOT => {
            stack.pop();
            stack.push(Value::Unknown);
        }
        // Binary Comparators
        LT|GT|SLT|SGT|EQ => {
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        // Binary bitwise operators
        AND|OR|XOR|BYTE|SHL|SHR|SAR => {
            stack.pop();
            stack.pop();
            stack.push(Value::Unknown);
        }
        // 20s: Keccak256
        // 30s: Environmental Information
        // 40s: Block Information
        // 50s: Stack, Memory, Storage and Flow Operations
        JUMPI => {
            stack.pop();
        }
        JUMPDEST(_) => {
            // Nop
        }
        // 60 & 70s: Push Operations
        PUSH(bytes) => {
            let n = util::from_be_bytes(&bytes);
            if n <= MAX_CODE_SIZE {
                stack.push(Value::Known(n as usize));
            } else {
                stack.push(Value::Unknown);
            }
        }
        // 80s: Duplicate Operations
        // 90s: Exchange Operations
        // a0s: Logging Operations
        // f0s: System Operations
        INVALID|JUMP|RETURN|REVERT|STOP => {
            return BOTTOM;
        }
        _ => {
            // This is a catch all to ensure no instructions are
            // missed above.
            panic!("unknown instruction ({:?})",insn);
        }
    }
    // Done
    CfaState{stack:Some(stack)}
}
