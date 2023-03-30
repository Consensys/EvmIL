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
use crate::evm::opcode;
use crate::evm::{Error,Instruction,ToInstructions};

/// The EOF magic prefix as dictated in EIP3540.
pub const EOF_MAGIC : [u8;2] = [0xEF,0x00];

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

    /// Construct a bytecode contract from a given set of bytes.  The
    /// exact interpretation of these bytes depends on the fork.  For
    /// example, on some forks, certain instructions are permitted
    /// whilst on others they are not.  Likewise, EOF containers are
    /// supported on some forks but not others.
    pub fn from_bytes(bytes: &[u8]) -> Bytecode {
        // Check for EOF container
        if bytes.starts_with(&EOF_MAGIC) {
            from_eof_bytes(bytes)
        } else {
            todo!()
        }
    }
}

// ===================================================================
// Traits
// ===================================================================

pub type BytecodeIter<'a,T> = std::slice::Iter<'a,T>;

impl<'a> IntoIterator for &'a Bytecode {
    type Item = &'a Section;
    type IntoIter = BytecodeIter<'a,Section>;

    fn into_iter(self) -> Self::IntoIter {
        todo!();
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

// ============================================================================
// Disassembly
// ============================================================================

/// Construct a bytecode container from an EOF formatted byte
/// sequence.
fn from_eof_bytes(bytes: &[u8]) -> Bytecode {
    let mut iter = EofIterator::new(bytes);
    iter.match_u8(0xEF,"magic");
    iter.match_u8(0x00,"magic");
    // Pull out static information
    let version = iter.next_u8();
    iter.match_u8(0x01,"kind_type");
    let type_size = iter.next_u16();
    iter.match_u8(0x02,"kind_code");
    let num_code_sections = iter.next_u16() as usize;
    let mut code_sizes : Vec<usize> = Vec::new();
    // Extract code sizes
    for i in 0..num_code_sections {
        code_sizes.push(iter.next_u16() as usize);
    }
    iter.match_u8(0x03,"kind_data");
    let data_size = iter.next_u16() as usize;
    iter.match_u8(0x00,"terminator");
    // parse types section
    let mut types = Vec::new();
    for i in 0..type_size {
        let inputs = iter.next_u8();
        let outputs = iter.next_u8();
        let max_stack = iter.next_u16();
        types.push((inputs,outputs,max_stack));
    }
    let mut code = Bytecode::new();
    // parse code section(s)
    for i in 0..num_code_sections {
        let bytes = iter.next_bytes(code_sizes[i]);
        // Recall type information
        let (inputs,outputs,max_stack) = types[i];
        // Convert byte sequence into an instruction sequence.
        let insns = bytes.to_insns();
        // Add code section
        code.add(Section::Code{insns,inputs,outputs,max_stack})
    }
    // parse data sectin (if present)
    let data = iter.next_bytes(data_size).to_vec();
    code.add(Section::Data(data));
    // Done
    code
}

/// Helper for pulling information out of an EOF formatted byte
/// stream.
struct EofIterator<'a> {
    bytes: &'a [u8],
    index: usize
}

impl<'a> EofIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self{bytes,index:0}
    }

    pub fn match_u8(&mut self, n: u8, _msg: &str) {
        let m = self.next_u8();
        if m != n { panic!(); }
    }

    pub fn next_u8(&mut self) -> u8 {
        let next = self.bytes[self.index];
        self.index += 1;
        next
    }

    pub fn next_u16(&mut self) -> u16 {
        let msb = self.next_u8();
        let lsb = self.next_u8();
        u16::from_be_bytes([msb,lsb])
    }

    pub fn next_bytes(&mut self, length: usize) -> &'a [u8] {
        let start = self.index;
        self.index += length;
        &self.bytes[start..self.index]
    }
}
