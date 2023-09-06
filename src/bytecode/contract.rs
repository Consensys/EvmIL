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
use crate::asm;
use crate::asm::{AssemblyInstruction,AssemblyError};
use crate::bytecode::{Instruction};
use super::{eof,legacy};
pub use super::eof::DecodingError;    

// ============================================================================
// Structured Contract
// ============================================================================

/// A structured representation of an EVM bytecode contract which is
/// either a _legacy contract_, or an EVM Object Format (EOF)
/// compatiable contract.  Regardless of whether it is legacy or not,
/// a contract is divided into one or more _sections_.  A section is
/// either a _code section_ or a _data section_.  For EOF contracts,
/// the _data section_ should also come last.  However, for legacy
/// contracts, they can be interleaved.
#[derive(Clone,Debug,PartialEq)]
pub struct StructuredContract<T:PartialEq> {
    sections: Vec<StructuredSection<T>>
}

impl<T:PartialEq> StructuredContract<T> {

    pub fn from_legacy_bytes(bytes: &[u8]) -> StructuredContract<AssemblyInstruction> {
        legacy::from_bytes(bytes)
    }

    /// A decoded EOF byte sequence (see
    /// [EIP3540](https://eips.ethereum.org/EIPS/eip-3540)).  This
    /// provides a gateway for disassembling EOF contracts into assembly
    /// language and back again.
    ///
    /// # Examples
    /// ```
    /// use evmil::asm::Assembly;
    /// use evmil::util::FromHexString;
    ///
    /// // EOF bytecode contract
    /// let hex = "0xef00010100040200010001030000000000000000";
    /// // Conversion into bytes
    /// let bytes = hex.from_hex_string().unwrap();
    /// // Decode EOF bytecode (assuming no errors)
    /// let eof = Assembly::from_eof_bytes(&bytes).unwrap();
    /// // Check that section contains one instruction
    /// // assert_eq!(eof.sections.len(),1);
    /// ```    
    pub fn from_eof_bytes(bytes: &[u8]) -> Result<StructuredContract<AssemblyInstruction>,DecodingError> {
        eof::from_bytes(bytes)
    }
    
    pub fn empty() -> Self {
        Self {
            sections: Vec::new()
        }
    }

    pub fn new(sections: Vec<StructuredSection<T>>) -> Self {
        Self { sections }
    }

    /// Return the number of sections in the code.
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a,StructuredSection<T>> {
        self.sections.iter()
    }

    /// Add a new section to this bytecode container
    pub fn add(&mut self, section: StructuredSection<T>) {
        self.sections.push(section)
    }
}

impl StructuredContract<Instruction> {
    pub fn to_legacy_bytes(&self) -> Vec<u8> {
        legacy::to_bytes(self)
    }

    pub fn to_eof_bytes(&self) -> Vec<u8> {
        // FIXME: need to figure this out :)
        eof::to_bytes(self).unwrap()
    }    
}    

// ===================================================================
// Traits
// ===================================================================

impl<'a,T:PartialEq> IntoIterator for &'a StructuredContract<T> {
    type Item = &'a StructuredSection<T>;
    type IntoIter = Iter<'a,StructuredSection<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.sections.iter()
    }
}

// ============================================================================
// Section
// ============================================================================

#[derive(Clone,Debug,PartialEq)]
pub enum StructuredSection<T> {
    /// A data section is simply a sequence of zero or more bytes.
    Data(Vec<u8>),
    /// A code section is a sequence of zero or more instructions
    /// along with appropriate _metadata_.
    Code(Vec<T>)
}

impl StructuredSection<Instruction> {
    /// Flattern this section into an appropriately formated byte
    /// sequence for the enclosing container type.
    pub fn encode(&self, bytes: &mut Vec<u8>) {
        match self {
            StructuredSection::Data(bs) => {
                bytes.extend(bs);
            }
            StructuredSection::Code(insns) => {
                for b in insns { b.encode(bytes); }
            }
        }
    }
}
