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
use crate::util::{Concretizable,w256,Top};
use crate::evm::{AssemblyInstruction,Bytecode,Execution,Instruction,Section,ToInstructions};
use crate::evm::{EvmState,EvmMemory,EvmStack,EvmStorage,EvmWord};
use crate::evm::AbstractInstruction::*;

pub fn from_bytes(bytes: &[u8]) -> Bytecode<AssemblyInstruction> {
    /// Convert bytes into instructions
    let insns = bytes.to_insns();
    let mut bytecode = Bytecode::new(vec![Section::Code(insns)]);
    let mut execution : Execution<LegacyEvmState> = Execution::new(&bytecode);
    // Run execution (and for now hope it succeeds!)
    execution.execute(LegacyEvmState::new());
    // Construct assembly language
    let mut asm = Vec::new();
    let mut pc = 0;
    let Section::Code(insns) = bytecode.iter().next().unwrap() else { unreachable!(); };
    //
    for insn in insns {
        // Check whether this is a databyte or not.
        asm.push(translate_insn(insn));
        pc += insn.length()
    }
    //
    Bytecode::new(vec![Section::Code(asm)])
}

/// Convert this bytecode contract into a byte sequence correctly
/// formatted for legacy code.
pub fn to_bytes(bytecode: &Bytecode<Instruction>) -> Vec<u8> {
    let mut bytes = Vec::new();
    //
    for s in bytecode { s.encode(&mut bytes); }
    // Done
    bytes
}

// ===================================================================
// Legacy State
// ===================================================================

#[derive(Clone)]
pub struct LegacyEvmState {
    stack: LegacyEvmStack,
    memory: LegacyEvmMemory,
    storage: LegacyEvmStorage,
    pc: usize
}

impl LegacyEvmState {
    pub fn new() -> Self {
        let stack = LegacyEvmStack::new();
        let memory = LegacyEvmMemory{};
        let storage = LegacyEvmStorage{};
        Self{pc:0,stack,memory,storage}
    }
}

impl EvmState for LegacyEvmState {
    type Word = aw256;
    type Stack = LegacyEvmStack;
    type Memory = LegacyEvmMemory;
    type Storage = LegacyEvmStorage;

    fn pc(&self) -> usize {
        self.pc
    }

    fn stack(&mut self) -> &mut Self::Stack {
        &mut self.stack
    }

    fn memory(&mut self) -> &mut Self::Memory {
        &mut self.memory
    }

    fn storage(&mut self) -> &mut Self::Storage {
        &mut self.storage
    }

    fn skip(&mut self, n: usize) {
        self.pc += n
    }

    /// Move _program counter_ to a given (byte) offset within the
    /// code section.
    fn goto(&mut self, pc: usize) {
        self.pc = pc;
    }
}

// ===================================================================
// Legacy Stack
// ===================================================================

#[derive(Clone)]
pub struct LegacyEvmStack {
    items: Vec<aw256>
}

impl LegacyEvmStack {
    pub fn new() -> Self {
        Self{items: Vec::new()}
    }
}

impl EvmStack for LegacyEvmStack {
    type Word = aw256;

    fn has_capacity(&self, n: usize) -> bool {
        (1024 - self.items.len()) >= n
    }

    fn has_operands(&self, n: usize) -> bool {
        self.items.len() >= n
    }

    fn peek(&self, n: usize) -> &Self::Word {
        assert!(self.has_operands(n));
        &self.items[self.items.len() - n]
    }

    fn push(&mut self, item: Self::Word) {
        self.items.push(item);
    }

    fn pop(&mut self) -> aw256 {
        assert!(self.has_operands(1));
        self.items.pop().unwrap()
    }

    fn set(&mut self, n: usize, item: Self::Word) {
        assert!(self.has_operands(n));
        let m = self.items.len() - n;
        self.items[m] = item;
    }
}

// ===================================================================
// Legacy Memory
// ===================================================================

#[derive(Clone)]
pub struct LegacyEvmMemory { }

impl EvmMemory for LegacyEvmMemory {
    type Word = aw256;

    fn read(&mut self, address: Self::Word) -> Self::Word {
        aw256::Unknown
    }

    fn write(&mut self, address: Self::Word, item: Self::Word) {
        // no op (for now)
    }
}

// ===================================================================
// Legacy Storage
// ===================================================================

#[derive(Clone)]
pub struct LegacyEvmStorage { }

impl EvmStorage for LegacyEvmStorage {
    type Word = aw256;

    fn get(&mut self, address: Self::Word) -> Self::Word {
        aw256::Unknown
    }

    fn put(&mut self, address: Self::Word, item: Self::Word) {
        // no op (for now)
    }
}

// ===================================================================
// Abstract Word
// ===================================================================

#[derive(Clone)]
pub enum aw256 {
    Word(w256),
    Unknown
}

impl From<w256> for aw256 {
    fn from(word: w256) -> aw256 {
        aw256::Word(word)
    }
}

impl Top for aw256 {
    const TOP : aw256 = aw256::Unknown;
}

impl Concretizable for aw256 {
    type Item = w256;

    fn is_constant(&self) -> bool {
        match self {
            aw256::Word(_) => true,
            aw256::Unknown => false
        }
    }

    fn constant(&self) -> w256 {
        match self {
            aw256::Word(w) => *w,
            aw256::Unknown => {
                panic!();
            }
        }
    }
}

impl EvmWord for aw256 {

}

// ===================================================================
// Disassembler
// ===================================================================

fn translate_insn(insn: &Instruction) -> AssemblyInstruction {
    match insn {
        // 0s: Stop and Arithmetic Operations
        STOP => STOP,
        ADD => ADD,
        MUL => MUL,
        SUB => SUB,
        DIV => DIV,
        SDIV => SDIV,
        MOD => MOD,
        SMOD => SMOD,
        ADDMOD => ADDMOD,
        MULMOD => MULMOD,
        EXP => EXP,
        SIGNEXTEND => SIGNEXTEND,
        // 10s: Comparison & Bitwise Logic Operations
        LT => LT,
        GT => GT,
        SLT => SLT,
        SGT => SGT,
        EQ => EQ,
        ISZERO => ISZERO,
        AND => AND,
        OR => OR,
        XOR => XOR,
        NOT => NOT,
        BYTE => BYTE,
        SHL => SHL,
        SHR => SHR,
        SAR => SAR,
        // 20s: Keccak256
        KECCAK256 => KECCAK256,
        // 30s: Environmental Information
        ADDRESS => ADDRESS,
        BALANCE => BALANCE,
        ORIGIN => ORIGIN,
        CALLER => CALLER,
        CALLVALUE => CALLVALUE,
        CALLDATALOAD => CALLDATALOAD,
        CALLDATASIZE => CALLDATASIZE,
        CALLDATACOPY => CALLDATACOPY,
        CODESIZE => CODESIZE,
        CODECOPY => CODECOPY,
        GASPRICE => GASPRICE,
        EXTCODESIZE => EXTCODESIZE,
        EXTCODECOPY => EXTCODECOPY,
        RETURNDATASIZE => RETURNDATASIZE,
        RETURNDATACOPY => RETURNDATACOPY,
        EXTCODEHASH => EXTCODEHASH,
        // 40s: Block Information
        BLOCKHASH => BLOCKHASH,
        COINBASE => COINBASE,
        TIMESTAMP => TIMESTAMP,
        NUMBER => NUMBER,
        DIFFICULTY => DIFFICULTY,
        GASLIMIT => GASLIMIT,
        CHAINID => CHAINID,
        SELFBALANCE => SELFBALANCE,
        // 50s: Stack, Memory, Storage and Flow Operations
        POP => POP,
        MLOAD => MLOAD,
        MSTORE => MSTORE,
        MSTORE8 => MSTORE8,
        SLOAD => SLOAD,
        SSTORE => SSTORE,
        JUMP => JUMP,
        JUMPI => JUMPI,
        PC => PC,
        MSIZE => MSIZE,
        GAS => GAS,
        JUMPDEST => JUMPDEST,
        // 60s & 70s: Push Operations
        PUSH(bs) => PUSH(bs.clone()),
        // 80s: Duplication Operations
        DUP(n) => DUP(*n),
        // 90s: Swap Operations
        SWAP(n) => SWAP(*n),
        // a0s: Log Operations
        LOG(n) => LOG(*n),
        // f0s: System Operations
        CREATE => CREATE,
        CALL => CALL,
        CALLCODE => CALLCODE,
        RETURN => RETURN,
        DELEGATECALL => DELEGATECALL,
        CREATE2 => CREATE2,
        STATICCALL => STATICCALL,
        REVERT => REVERT,
        INVALID => INVALID,
        SELFDESTRUCT => SELFDESTRUCT,
        DATA(bs) => DATA(bs.clone()),
        //
        PUSHL(_)|LABEL(_) => unreachable!(),
        RJUMP(_)|RJUMPI(_) => unreachable!()
    }
}
