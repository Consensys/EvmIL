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
use crate::util::{w256};
use crate::evm::{EvmState,EvmStack,EvmMemory,EvmStorage,Instruction};
use crate::evm::AbstractInstruction::*;
use crate::evm::EvmException::*;

/// Represents the set of possible errors that can arise when
/// executing a given sequence of EVM bytecode.
pub enum EvmException {
    Revert,
    InsufficientGas,
    InsufficientFunds,
    InvalidOpcode,
    StackUnderflow,
    StackOverflow,
    BalanceOverflow,
    ReturnDataOverflow,
    InvalidJumpDest,
    InvalidPrecondition,
    CodeSizeExceeded,
    CallDepthExceeded,
    AccountCollision,
    WriteProtectionViolated
}

/// Execute an instruction from the given EVM state.
pub fn execute<T:EvmState>(insn: &Instruction, state: &mut T) -> Result<(),EvmException> {
    match insn {
        // ===========================================================
        // 0s: Stop and Arithmetic Operations
        // ===========================================================

        STOP => Ok(()),
        ADD => execute_add(state),
        MUL => execute_mul(state),
        SUB => execute_sub(state),
        DIV => execute_div(state),
        SDIV => execute_sdiv(state),
        ADDMOD => execute_addmod(state),
        MULMOD => execute_mulmod(state),
        ISZERO => execute_iszero(state),
        NOT => execute_not(state),

        // ===========================================================
        // 10s: Comparison & Bitwise Logic Operations
        // ===========================================================

        LT => execute_lt(state),
        GT => execute_gt(state),
        SLT => execute_slt(state),
        SGT => execute_sgt(state),

        // ===========================================================
        // 60 & 70s: Push Operations
        // ===========================================================

        PUSH(bytes) => execute_push(state,bytes),

        _ => {
            Err(EvmException::InvalidOpcode)
        }
    }
}

// ===================================================================
// 0s: Stop and Arithmetic Operations
// ===================================================================

fn execute_add<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack_mut();
    //
    if stack.has_operands(2) {
        let lhs = stack.peek(1);
        let rhs = stack.peek(0);
        stack.pop(2);
        stack.push(lhs + rhs);
        state.skip(1);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

fn execute_mul<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack_mut();
    //
    if stack.has_operands(2) {
        let lhs = stack.peek(1);
        let rhs = stack.peek(0);
        stack.pop(2);
        stack.push(lhs * rhs);
        state.skip(1);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

fn execute_sub<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack_mut();
    //
    if stack.has_operands(2) {
        let lhs = stack.peek(1);
        let rhs = stack.peek(0);
        stack.pop(2);
        stack.push(lhs - rhs);
        state.skip(1);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

fn execute_div<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!();  // Unsure how to implement this.
}

fn execute_sdiv<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!();  // Unsure how to implement this.
}

fn execute_addmod<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!();  // Unsure how to implement this.
}

fn execute_mulmod<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!();  // Unsure how to implement this.
}

// ===================================================================
// 10s: Comparison & Bitwise Logic Operations
// ===================================================================

fn execute_iszero<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!();  // Unsure how to implement this.
}

fn execute_not<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack_mut();
    //
    if stack.has_operands(1) {
        let val = stack.peek(0);
        stack.pop(1);
        stack.push(!val);
        state.skip(1);
        Ok(())
    } else {
        Err(StackUnderflow)
    }
}

fn execute_lt<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!()
}

fn execute_gt<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!()
}

fn execute_slt<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!()
}

fn execute_sgt<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!()
}

// ===================================================================
// 60 & 70s: Push Operations
// ===================================================================

fn execute_push<T:EvmState>(state: &mut T, bytes: &[u8]) -> Result<(),EvmException> {
    let stack = state.stack_mut();
    //
    if stack.has_capacity(1) {
        // Extract word from bytes
        let n = w256::from_be_bytes(&bytes);
        // Push word on stack, and advance pc.
        stack.push(T::Word::from(n));
        // Advance program counter
        state.skip(1 + bytes.len());
        //
        Ok(())
    } else {
        Err(StackOverflow)
    }
}
