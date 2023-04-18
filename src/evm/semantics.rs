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
use crate::util::{w256,Top};
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
pub fn execute<T:EvmState>(insn: &Instruction, state: &mut T) -> Result<(),EvmException>
where T::Word : Top {
    match insn {
        // ===========================================================
        // 0s: Stop and Arithmetic Operations
        // ===========================================================
        STOP => Ok(()),
        ADD => execute_binary(state,|l,r| l+r),
        MUL => execute_binary(state, |_,_| T::Word::TOP),
        SUB => execute_binary(state, |_,_| T::Word::TOP),
        DIV => execute_binary(state,  |_,_| T::Word::TOP),
        SDIV => execute_binary(state,  |_,_| T::Word::TOP),
        MOD => execute_binary(state,  |_,_| T::Word::TOP),
        SMOD => execute_binary(state,  |_,_| T::Word::TOP),
        ADDMOD => execute_binary(state,  |_,_| T::Word::TOP),
        MULMOD => execute_binary(state, |_,_| T::Word::TOP),
        EXP => execute_binary(state,  |_,_| T::Word::TOP),
        SIGNEXTEND => execute_binary(state,  |_,_| T::Word::TOP),

        // ===========================================================
        // 10s: Comparison & Bitwise Logic Operations
        // ===========================================================
        LT => execute_binary(state, |_,_| T::Word::TOP),
        GT => execute_binary(state, |_,_| T::Word::TOP),
        SLT => execute_binary(state, |_,_| T::Word::TOP),
        SGT => execute_binary(state, |_,_| T::Word::TOP),
        EQ => execute_binary(state, |_,_| T::Word::TOP),
        ISZERO => execute_unary(state, |_| T::Word::TOP),
        AND => execute_binary(state, |_,_| T::Word::TOP),
        OR => execute_binary(state, |_,_| T::Word::TOP),
        XOR => execute_binary(state, |_,_| T::Word::TOP),
        NOT => execute_unary(state, |_| T::Word::TOP),
        BYTE => execute_binary(state, |_,_| T::Word::TOP),
        SHL => execute_binary(state, |_,_| T::Word::TOP),
        SHR => execute_binary(state, |_,_| T::Word::TOP),
        SAR => execute_binary(state, |_,_| T::Word::TOP),

        // ===========================================================
        // 20s: Keccak256
        // ===========================================================
        KECCAK256 => execute_binary(state, |_,_| T::Word::TOP),

        // ===========================================================
        // 30s: Environment Information
        // ===========================================================
        ADDRESS => execute_producer(state, &[T::Word::TOP]),
        BALANCE => execute_unary(state, |_| T::Word::TOP),
        ORIGIN => execute_producer(state, &[T::Word::TOP]),
        CALLER => execute_producer(state, &[T::Word::TOP]),
        CALLVALUE => execute_producer(state, &[T::Word::TOP]),
        CALLDATALOAD => execute_unary(state, |_| T::Word::TOP),
        CALLDATASIZE => execute_unary(state, |_| T::Word::TOP),
        CALLDATACOPY => execute_consumer(state, 3),
        CODESIZE => execute_producer(state, &[T::Word::TOP]),
        CODECOPY => execute_consumer(state, 3),
        GASPRICE => execute_producer(state, &[T::Word::TOP]),
        EXTCODESIZE => execute_unary(state, |_| T::Word::TOP),
        EXTCODECOPY => execute_consumer(state, 4),
        RETURNDATASIZE => execute_producer(state, &[T::Word::TOP]),
        RETURNDATACOPY => execute_consumer(state, 3),
        EXTCODEHASH => execute_unary(state, |_| T::Word::TOP),

        // ===========================================================
        // 40s: Block Information
        // ===========================================================
        BLOCKHASH => execute_unary(state, |_| T::Word::TOP),
        COINBASE => execute_producer(state, &[T::Word::TOP]),
        TIMESTAMP => execute_producer(state, &[T::Word::TOP]),
        NUMBER => execute_producer(state, &[T::Word::TOP]),
        DIFFICULTY => execute_producer(state, &[T::Word::TOP]),
        GASLIMIT => execute_producer(state, &[T::Word::TOP]),
        CHAINID => execute_producer(state, &[T::Word::TOP]),
        SELFBALANCE => execute_producer(state, &[T::Word::TOP]),

        // ===========================================================
        // 50s: Stack, Memory Storage and Flow Operations
        // ===========================================================
        POP => execute_consumer(state,1),
        MLOAD => execute_mload(state),
        MSTORE => execute_mstore(state),
        MSTORE8 => execute_mstore8(state),
        SLOAD => execute_sload(state),
        SSTORE => execute_sstore(state),
        PC => execute_producer(state, &[T::Word::TOP]),
        MSIZE => execute_producer(state, &[T::Word::TOP]),
        GAS => execute_producer(state, &[T::Word::TOP]),
        JUMPDEST => Ok(()), // nop
        JUMP => execute_jump(state),
        JUMPI => execute_jumpi(state),

        // ===========================================================
        // 60 & 70s: Push Operations
        // ===========================================================
        PUSH(bytes) => execute_push(state,bytes),

        // ===========================================================
        // 80s: Duplication Operations
        // ===========================================================
        DUP(k) => execute_dup(state,*k as usize),

        // ===========================================================
        // 90s: Exchange Operations
        // ===========================================================
        SWAP(k) => execute_swap(state,*k as usize),

        _ => {
            Err(EvmException::InvalidOpcode)
        }
    }
}

// ===================================================================
// Unary Operations
// ===================================================================

fn execute_unary<T:EvmState,F>(state: &mut T, op: F) -> Result<(),EvmException>
where F:Fn(T::Word)->T::Word {
    let stack = state.stack();
    //
    if !stack.has_operands(1) {
        Err(StackUnderflow)
    } else {
        // Read word on top of stack
        let word = stack.peek(0);
        // Pop it off
        stack.pop(1);
        // Push back result of operation
        stack.push(op(word));
        // Move to next instruction
        state.skip(1);
        // Done
        Ok(())
    }
}

// ===================================================================
// Binary Operations
// ===================================================================

fn execute_binary<T:EvmState,F>(state: &mut T, op: F) -> Result<(),EvmException>
where F:Fn(T::Word,T::Word)->T::Word {
    let stack = state.stack();
    //
    if !stack.has_operands(2) {
        Err(StackUnderflow)
    } else {
        let lhs = stack.peek(1);
        let rhs = stack.peek(0);
        stack.pop(2);
        stack.push(op(lhs,rhs));
        state.skip(1);
        Ok(())
    }
}

// ===================================================================
// Producers / Consumers
// ===================================================================

fn execute_producer<T:EvmState>(state: &mut T, items: &[T::Word]) -> Result<(),EvmException> {
    let stack = state.stack();
    //
    if !stack.has_capacity(items.len()) {
        Err(StackOverflow)
    } else {
        for i in items {
            stack.push(i.clone());
        }
        state.skip(1);
        Ok(())
    }
}

fn execute_consumer<T:EvmState>(state: &mut T, n: usize) -> Result<(),EvmException> {
    let stack = state.stack();
    //
    if !stack.has_operands(n) {
        Err(StackUnderflow)
    } else {
        stack.pop(n);
        state.skip(1);
        Ok(())
    }
}

// ===================================================================
// Memory / Storage
// ===================================================================

fn execute_mload<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack();
    //
    if !stack.has_operands(1) {
        Err(StackUnderflow)
    } else {
        // Determine address to load from
        let address = stack.peek(0);
        // Pop address from stack
        stack.pop(1);
        // Read word from memory
        let word = state.memory().read(address);
        // Push value at address
        state.stack().push(word);
        // Move to next instruction
        state.skip(1);
        //
        Ok(())
    }
}

fn execute_mstore<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack();
    //
    if !stack.has_operands(2) {
        Err(StackUnderflow)
    } else {
        // Determine address to load from
        let address = stack.peek(0);
        let word = stack.peek(1);
        // Pop both from stack
        stack.pop(2);
        // Write word into memory
        state.memory().write(address, word);
        // Move to next instruction
        state.skip(1);
        //
        Ok(())
    }
}

fn execute_mstore8<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!()
}

fn execute_sload<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack();
    //
    if !stack.has_operands(1) {
        Err(StackUnderflow)
    } else {
        // Determine address to load from
        let address = stack.peek(0);
        // Pop address from stack
        stack.pop(1);
        // Read word from memory
        let word = state.storage().get(address);
        // Push value at address
        state.stack().push(word);
        // Move to next instruction
        state.skip(1);
        //
        Ok(())
    }
}

fn execute_sstore<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    let stack = state.stack();
    //
    if !stack.has_operands(2) {
        Err(StackUnderflow)
    } else {
        // Determine address to load from
        let address = stack.peek(0);
        let word = stack.peek(1);
        // Pop both from stack
        stack.pop(2);
        // Write word into memory
        state.storage().put(address, word);
        // Move to next instruction
        state.skip(1);
        //
        Ok(())
    }
}

// ===================================================================
// Jump
// ===================================================================

fn execute_jump<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!()
}

fn execute_jumpi<T:EvmState>(state: &mut T) -> Result<(),EvmException> {
    todo!()
}

// ===================================================================
// Push
// ===================================================================

fn execute_push<T:EvmState>(state: &mut T, bytes: &[u8]) -> Result<(),EvmException> {
    let stack = state.stack();
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

// ===================================================================
// Dup
// ===================================================================

fn execute_dup<T:EvmState>(state: &mut T, k: usize) -> Result<(),EvmException> {
    assert!(1 <= k && k <= 16);
    let stack = state.stack();
    //
    if !stack.has_operands(k) {
        Err(StackUnderflow)
    } else if !stack.has_capacity(1) {
        Err(StackOverflow)
    } else {
        let val = stack.peek(k-1);
        stack.push(val);
        state.skip(1);
        Ok(())
    }
}

// ===================================================================
// Swap
// ===================================================================

fn execute_swap<T:EvmState>(state: &mut T, k: usize) -> Result<(),EvmException> {
    assert!(1 <= k && k <= 16);
    let stack = state.stack();
    //
    if !stack.has_operands(k) {
        Err(StackUnderflow)
    } else {
        let kth = stack.peek(k-1);
        let top = stack.peek(0);
        stack.set(k-1,top);
        stack.set(0,kth);
        state.skip(1);
        Ok(())
    }
}
