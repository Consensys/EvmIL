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
use crate::evm::{Error,Instruction};

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
pub struct Bytecode {
    version: BytecodeVersion,
    sections: Vec<Section>
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode {
	    version: BytecodeVersion::Legacy,
            sections: Vec::new()
        }
    }

    /// Add a new section to this bytecode container
    pub fn add(&mut self, section: Section) {
        self.sections.push(section)
    }

    /// Convert this bytecode contract into a byte sequence correctly
    /// formatted according to the container version (e.g. legacy or
    /// EOF).
    pub fn to_bytes(self) -> Vec<u8> {
        // Assumption for now
        assert!(self.version == BytecodeVersion::Legacy);
        //
        let mut bytes = Vec::new();
        //
        for s in self.sections { s.encode(&mut bytes); }
        // Done
        bytes
    }
}

// ============================================================================
// Versioning
// ============================================================================

#[derive(Debug,PartialEq)]
pub enum BytecodeVersion {
    /// Indicates a legacy (i.e. pre-EOF) contract.
    Legacy,
    /// Represents an EOF contract with a given versioning byte.
    EOF(u8)
}

// ============================================================================
// Section
// ============================================================================

pub enum Section {
    /// A data section is simply a sequence of zero or more bytes.
    Data(Vec<u8>),
    /// A code section is a sequence of zero or more instructions
    /// along with appropriate _metadata_.
    Code{insns: Vec<Instruction>, inputs: u8, outputs: u8, max_stack: u16}
}

impl Section {
    /// Flattern this section into an appropriately formated byte
    /// sequence for the enclosing container type.
    pub fn encode(&self, bytes: &mut Vec<u8>) {
        match self {
            Section::Data(bs) => {
                bytes.extend(bs);
            }
            Section::Code{insns, inputs, outputs, max_stack} => {
                for b in insns {
                    // NOTE: unwrap safe as instructions validated on
                    // entry to the container.
                    b.encode(bytes).unwrap();
                }
            }
        }
    }
}
