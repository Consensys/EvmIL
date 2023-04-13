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
use std::fmt;
use crate::evm::opcode;
use crate::evm::eof;
use crate::evm::{LabelledInstruction,Instruction};

// ============================================================================
// Bytecode Contract
// ============================================================================

/// A structured representation of an EVM bytecode contract which is
/// either a _legacy contract_, or an EVM Object Format (EOF)
/// compatiable contract.  Regardless of whether it is legacy or not,
/// a contract is divided into one or more _sections_.  A section is
/// either a _code section_ or a _data section_.  For EOF contracts,
/// the _data section_ should also come last.  However, for legacy
/// contracts, they can be interleaved.
pub struct Bytecode<T> {
    sections: Vec<Section<T>>
}

impl<T> Bytecode<T> {
    pub fn empty() -> Self {
        Bytecode {
            sections: Vec::new()
        }
    }

    pub fn new(sections: Vec<Section<T>>) -> Self {
        Bytecode { sections }
    }

    /// Return the number of sections in the code.
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    pub fn iter<'a>(&'a self) -> BytecodeIter<'a,Section<T>> {
        self.sections.iter()
    }

    /// Add a new section to this bytecode container
    pub fn add(&mut self, section: Section<T>) {
        self.sections.push(section)
    }
}

// ============================================================================
// Assemble
// ============================================================================

impl Bytecode<LabelledInstruction> {
    /// Convert assembly instructions into concrete EVM instructions.
    pub fn assemble(&self) -> Bytecode<Instruction> {
        todo!()
    }
}

// ===================================================================
// Traits
// ===================================================================

/// An iterator over the sections of a bytecode contract (e.g. code or
/// data).
pub type BytecodeIter<'a,T> = std::slice::Iter<'a,T>;

impl<'a,T> IntoIterator for &'a Bytecode<T> {
    type Item = &'a Section<T>;
    type IntoIter = BytecodeIter<'a,Section<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.sections.iter()
    }
}


// ============================================================================
// Section
// ============================================================================

pub enum Section<T> {
    /// A data section is simply a sequence of zero or more bytes.
    Data(Vec<u8>),
    /// A code section is a sequence of zero or more instructions
    /// along with appropriate _metadata_.
    Code(Vec<T>, u8, u8, u16)
}

impl Section<Instruction> {
    /// Flattern this section into an appropriately formated byte
    /// sequence for the enclosing container type.
    pub fn encode(&self, bytes: &mut Vec<u8>) {
        match self {
            Section::Data(bs) => {
                bytes.extend(bs);
            }
            Section::Code(insns, _, _, _) => {
                for b in insns {
                    // NOTE: unwrap safe as instructions validated on
                    // entry to the container.
                    b.encode(bytes).unwrap();
                }
            }
        }
    }
}
