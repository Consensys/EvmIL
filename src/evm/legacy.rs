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
use crate::evm::{Bytecode,Execution,Instruction,Section,ToInstructions};
use crate::evm::{EvmState,EvmMemory,EvmStack,EvmStorage,EvmWord};

pub fn from_bytes(bytes: &[u8]) -> Bytecode<Instruction> {
    /// Convert bytes into instructions
    let insns = bytes.to_insns();
    let mut bytecode = Bytecode::new(vec![Section::Code(insns)]);
    let mut execution : Execution<LegacyEvmState> = Execution::new(&bytecode);
    // run the execution (and hope it succeeds!)
    execution.execute(LegacyEvmState::new());
    //
    bytecode
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
