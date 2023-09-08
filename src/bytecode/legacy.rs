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
use crate::bytecode::{Assembly,Assemble,Disassemble,Instruction,StructuredSection};
use crate::bytecode::Instruction::*;
use crate::analysis::{aw256,trace,AbstractState,ConcreteState,ConcreteStack,UnknownMemory,UnknownStorage};
use crate::analysis::{EvmState,EvmMemory,EvmStack,EvmStorage,EvmWord};

type LegacyConcreteState = ConcreteState<ConcreteStack<aw256>,UnknownMemory<aw256>,UnknownStorage<aw256>>;
type LegacyState = AbstractState<LegacyConcreteState>;

pub fn from_bytes(bytes: &[u8]) -> Assembly {
    let asm = bytes.disassemble();
    // Run analysis (and for now hope it succeeds!)    
    let mut analysis : Vec<LegacyState> = trace(&asm,AbstractState::new());
    // ???
    Assembly::new(vec![StructuredSection::Code(asm)])
}

/// Convert this bytecode contract into a byte sequence correctly
/// formatted for legacy code.
pub fn to_bytes(bytecode: &Assembly) -> Vec<u8> {
    let mut bytes = Vec::new();
    //
    for s in bytecode {
        match s {
            StructuredSection::Data(bs) => {
                bytes.extend(bs);
            }
            StructuredSection::Code(insns) => {
                let is : &[Instruction] = &insns;
                bytes.extend(is.assemble())
            }
        }        
    }
    // Done
    bytes
}    

// ===================================================================
// Legacy State
// ===================================================================

#[derive(Clone,Debug,PartialEq)]
pub struct LegacyEvmState {
    stack: LegacyEvmStack,
    memory: UnknownMemory<aw256>,
    storage: UnknownStorage<aw256>
}

impl LegacyEvmState {
    pub fn new() -> Self {
        let stack = LegacyEvmStack::new();
        let memory = UnknownMemory::new();
        let storage = UnknownStorage::new();
        Self{stack,memory,storage}
    }
}

impl EvmState for LegacyEvmState {
    type Word = aw256;
    type Stack = LegacyEvmStack;
    type Memory = UnknownMemory<aw256>;
    type Storage = UnknownStorage<aw256>;

    fn pc(&self) -> usize {
        self.stack.pc
    }

    fn stack(&self) -> &Self::Stack {
        &self.stack
    }

    fn memory(&self) -> &Self::Memory {
        &self.memory
    }

    fn storage(&self) -> &Self::Storage {
        &self.storage
    }

    fn stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.stack
    }

    fn memory_mut(&mut self) -> &mut Self::Memory {
        &mut self.memory
    }

    fn storage_mut(&mut self) -> &mut Self::Storage {
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

