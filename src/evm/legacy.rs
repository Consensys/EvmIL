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
use std::collections::HashMap;
use crate::util::{Concretizable,w256,IsBottom,Top};
use crate::evm::{AssemblyInstruction,Bytecode,Execution,ExecutionSection,Instruction,Section,ToInstructions};
use crate::evm::{EvmState,EvmMemory,EvmStack,EvmStorage,EvmWord};
use crate::evm::AbstractInstruction::*;

pub fn from_bytes(bytes: &[u8]) -> Bytecode<AssemblyInstruction> {
    /// NOTE: currently, we begin by converting bytes into
    /// instructions.  Personally, I think this is a bad choice and
    /// that we would be better of working directly with bytes.  It
    /// certainly makes for some ugly repetition here.
    let insns = bytes.to_insns();
    let mut bytecode = Bytecode::new(vec![Section::Code(insns)]);
    let mut execution : Execution<LegacyEvmState> = Execution::new(&bytecode);
    // Run execution (and for now hope it succeeds!)
    execution.execute(LegacyEvmState::new());
    // Extract analysis results for first section;
    let analysis = &execution[0];
    // Identify position where the data section starts.
    let pivot = identify_data(analysis,bytes);
    // Split instructions from data
    let insns = (&bytes[..pivot]).to_insns();
    // Translate instructions into assembly instructions.
    let asm = disassemble(analysis,&insns);
    // Done
    if pivot == bytes.len() {
        // No data section
        Bytecode::new(vec![Section::Code(asm)])
    } else {
        let databytes = bytes[pivot..].to_vec();
        Bytecode::new(vec![Section::Code(asm),Section::Data(databytes)])
    }
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
// Helpers
// ===================================================================

/// Identify the data section within the instruction array.  That is,
/// all contiguous unreachable bytes from the end of the instruction
/// sequence.
fn identify_data(analysis: &ExecutionSection<LegacyEvmState>, bytes: &[u8]) -> usize {
    if bytes.len() == 0 {
        0
    } else {
        // Seach backwards through the data for the first reachable
        // instruction.  Everything after this instruction is
        // considered to be part of the data section.
        let mut pc = bytes.len() - 1;
        loop {
            // First instruction is always considered to be reachable,
            // no matter what.
            if pc == 0 || !analysis[pc].is_bottom() {
                // Decode last reachable instruction so that we can
                // figure out its length.
                let insn = Instruction::decode(pc,bytes);
                // Done!
                return pc + insn.length();
            }
            pc -= 1;
        }
    }
}

/// Perform initial translation.  This just translates each concrete
/// instruction into its corresponding assembly instruction, whilst
/// identifying unreachable data bytes.
fn disassemble(analysis: &ExecutionSection<LegacyEvmState>, insns: &[Instruction]) -> Vec<AssemblyInstruction> {
    let mut pc = 0;
    let mut asm = Vec::new();
    // Initialise translation
    for (i,insn) in insns.iter().enumerate() {
        if pc != 0 && analysis[pc].is_bottom() {
            let mut bytes = Vec::new();
            insn.encode(&mut bytes);
            asm.push(AssemblyInstruction::DATA(bytes));
        } else {
            // Check whether this is a databyte or not.
            asm.push(translate_insn(insn));
        }
        pc += insn.length();
    }
    // Refine translation by identifying instructions which push
    // labels.
    refine_instructions(analysis,insns,&mut asm);
    // Done
    asm
}

/// Refine instructions by converting concrete `PUSH` instructions
/// into labelled `PUSHL` instructions, whilst inserting labels as
/// appropriate.
fn refine_instructions(analysis: &ExecutionSection<LegacyEvmState>, insns: &[Instruction], asm: &mut Vec<AssemblyInstruction>) {
    // Construct labels
    let mut labels = determine_labels(analysis,insns);
    // Initialise instruction offsets
    let offsets = determine_insn_offsets(analysis,insns);
    //
    let mut pc = 0;
    //
    for (i,insn) in insns.iter().enumerate() {
        let info = &analysis[pc];
        // Check whether instruction is reachable.
        if pc == 0 || !info.is_bottom() {
            match insn {
                Instruction::JUMP|Instruction::JUMPI => {
                    for s in info.iter() {
                        // Identify byte offset for instruction (the
                        // "dependency") which gave rise to the item
                        // on top of the stack.
                        let dep_pc = s.stack.source(0);
                        // Convert our byte offset into an instruction
                        // offset.
                        let dep_i = offsets[dep_pc];
                        // See what that instruction is.
                        match &insns[dep_i] {
                            Instruction::PUSH(bytes) => {
                                // Extract destination.
                                let n = w256::from_be_bytes(&bytes);
                                // Sanity check.
                                assert_eq!(n,s.stack.peek(0).constant());
                                // Construct label
                                let label = labels.get(&n.into()).unwrap().clone();
                                // Convert concrete instruction to
                                // labelled instruction.
                                asm[dep_i] = AssemblyInstruction::PUSHL(label);
                            }
                            _ => {
                                /// This indicates an usual case,
                                /// where the jump destination has
                                /// been constructed in an unusual
                                /// manner.  For example, it might
                                /// have been constructed by adding
                                /// two numbers together, or it might
                                /// have been stored in memory.  For
                                /// now, I just don't handle this
                                /// case.
                                panic!("Complex dependency encountered!");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        pc += insn.length();
    }
    // Finally, insert labels at all reachable JUMPDEST instructions.
    let mut delta = 0;
    for i in 0..pc {
        let j = i as u16;
        match labels.get(&j) {
            Some(lab) => {
                let index = offsets[i] + delta;
                asm.insert(index,AssemblyInstruction::LABEL(lab.clone()));
                delta += 1;
            }
            None => {}
        }
    }
}

/// Map every reachable `JUMPDEST` instruction to a fresh label.
fn determine_labels(analysis: &ExecutionSection<LegacyEvmState>, insns: &[Instruction]) -> HashMap<u16,String> {
    let mut pc = 0;
    let mut labels = HashMap::new();
    for insn in insns {
        let info = &analysis[pc];
        if pc == 0 || !info.is_bottom() {
            match insn {
                Instruction::JUMPDEST => {
                    // Construct label
                    let label = format!("lab{:#}",labels.len());
                    labels.insert(pc as u16,label);
                }
                _ => {}
            }
        }
        pc += insn.length();
    }
    labels
}

/// Construct a map from byte offsets to instruction offsets.
fn determine_insn_offsets(analysis: &ExecutionSection<LegacyEvmState>, insns: &[Instruction]) -> Vec<usize> {
    let mut pc = 0;
    let mut offsets = Vec::new();
    for (i,insn) in insns.iter().enumerate() {
        offsets.resize(pc+1,0);
        offsets[pc] = i;
        pc += insn.length();
    }
    offsets
}

// ===================================================================
// Legacy State
// ===================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct LegacyEvmState {
    stack: LegacyEvmStack,
    memory: LegacyEvmMemory,
    storage: LegacyEvmStorage}

impl LegacyEvmState {
    pub fn new() -> Self {
        let stack = LegacyEvmStack::new();
        let memory = LegacyEvmMemory{};
        let storage = LegacyEvmStorage{};
        Self{stack,memory,storage}
    }
}

impl EvmState for LegacyEvmState {
    type Word = aw256;
    type Stack = LegacyEvmStack;
    type Memory = LegacyEvmMemory;
    type Storage = LegacyEvmStorage;

    fn pc(&self) -> usize {
        self.stack.pc
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
        self.stack.pc += n
    }

    /// Move _program counter_ to a given (byte) offset within the
    /// code section.
    fn goto(&mut self, pc: usize) {
        self.stack.pc = pc;
    }
}

// ===================================================================
// Legacy Stack
// ===================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct LegacyEvmStack {
    pc: usize,
    items: Vec<(usize,aw256)>
}

impl LegacyEvmStack {
    pub fn new() -> Self {
        Self{pc: 0, items: Vec::new()}
    }
    fn source(&self, n: usize) -> usize {
        assert!(self.has_operands(n));
        self.items[self.items.len() - (n+1)].0
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

    fn size(&self) -> usize {
        self.items.len()
    }

    fn peek(&self, n: usize) -> &Self::Word {
        assert!(self.has_operands(n));
        let (_,word) = &self.items[self.items.len() - (n+1)];
        word
    }

    fn push(&mut self, item: Self::Word) {
        self.items.push((self.pc,item));
    }

    fn pop(&mut self) -> aw256 {
        assert!(self.has_operands(1));
        self.items.pop().unwrap().1
    }

    fn dup(&mut self, n: usize) {
        assert!(n >= 0);
        assert!(self.has_operands(n+1));
        let i = self.items.len() - (n+1);
        self.items.push(self.items[i]);
    }

    fn swap(&mut self, n: usize) {
        assert!(n > 0);
        assert!(self.has_operands(n+1));
        let i = self.items.len() - (n+1);
        let j = self.items.len() - 1;
        // Use slice swap to avoid cloning.
        self.items.swap(i,j);
    }
}

// ===================================================================
// Legacy Memory
// ===================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct LegacyEvmMemory { }

impl EvmMemory for LegacyEvmMemory {
    type Word = aw256;

    fn read(&mut self, address: Self::Word) -> Self::Word {
        aw256::Unknown
    }

    fn write(&mut self, address: Self::Word, item: Self::Word) {
        // no op (for now)
    }

    fn write8(&mut self, address: Self::Word, item: Self::Word) {
        // no op (for now)
    }
}

// ===================================================================
// Legacy Storage
// ===================================================================

#[derive(Clone,Debug,PartialEq)]
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

#[derive(Copy,Clone,Debug,PartialEq)]
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
