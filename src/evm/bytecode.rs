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
use std::slice::{Iter};
use crate::evm::{assembler};
use crate::evm::{AssembleError,AssemblyError};
use crate::evm::instruction::{AssemblyInstruction,Instruction};

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
#[derive(Clone,Debug,PartialEq)]
pub struct Bytecode<T:PartialEq> {
    sections: Vec<Section<T>>
}

impl<T:PartialEq> Bytecode<T> {
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

    pub fn iter<'a>(&'a self) -> Iter<'a,Section<T>> {
        self.sections.iter()
    }

    /// Add a new section to this bytecode container
    pub fn add(&mut self, section: Section<T>) {
        self.sections.push(section)
    }
}

// ===================================================================
// Traits
// ===================================================================

impl<'a,T:PartialEq> IntoIterator for &'a Bytecode<T> {
    type Item = &'a Section<T>;
    type IntoIter = Iter<'a,Section<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.sections.iter()
    }
}

// ============================================================================
// Section
// ============================================================================

#[derive(Clone,Debug,PartialEq)]
pub enum Section<T> {
    /// A data section is simply a sequence of zero or more bytes.
    Data(Vec<u8>),
    /// A code section is a sequence of zero or more instructions
    /// along with appropriate _metadata_.
    Code(Vec<T>)
}

impl Section<Instruction> {
    /// Flattern this section into an appropriately formated byte
    /// sequence for the enclosing container type.
    pub fn encode(&self, bytes: &mut Vec<u8>) {
        match self {
            Section::Data(bs) => {
                bytes.extend(bs);
            }
            Section::Code(insns) => {
                for b in insns {
                    // NOTE: unwrap safe as instructions validated on
                    // entry to the container.
                    b.encode(bytes).unwrap();
                }
            }
        }
    }
}


// ============================================================================
// Assembly
// ============================================================================

/// An assembly represents one or more sections contained assembly
/// instructions (that is, instructions which uses labels instead of
/// explicit jump targets).
pub type Assembly = Bytecode<AssemblyInstruction>;

/// An assembly section represents a section as found within an
/// `Assembly`.
pub type AssemblySection = Section<AssemblyInstruction>;

impl Assembly {
    /// Assemble an assembly into a `Bytecode` object containing
    /// concrete EVM instructions.  This requires resolving any labels
    /// contained within the assembly into known jump destinations.
    /// As such, this can fail if an instruction attempts to branch to
    /// a label which does not exist.
    pub fn assemble(&self) -> Result<Bytecode<Instruction>,AssembleError> {
        let mut sections = Vec::new();
        // Map each assemply section to a compiled section.
        for s in &self.sections {
            match s {
                Section::Code(insns) => {
                    let ninsns = assembler::assemble(insns)?;
                    sections.push(Section::Code(ninsns));
                }
                Section::Data(bytes) => {
                    sections.push(Section::Data(bytes.clone()));
                }
            }
        }
        // Done
        Ok(Bytecode::new(sections))
    }

    /// Parse some assembly language into an `Assembly`.  This can
    /// fail for a variety of reasons, such as an unknown instruction
    /// is used or there is some unexpected junk in the file.
    pub fn from_str(input: &str) -> Result<Assembly,AssemblyError> {
        assembler::parse(input)
    }
}
