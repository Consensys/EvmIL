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
use crate::util::{ByteEncoder,ByteDecoder};
use crate::evm::opcode;
use crate::evm::{Instruction,ToInstructions};

/// The EOF magic prefix as dictated in EIP3540.
pub const EOF_MAGIC : u16 = 0xEF00;

// ============================================================================
// Encoding Error
// ============================================================================

/// An error which arises when attempting to encode an EOF bytecode
/// structure.  This indicates the bytecode structure is malformed in
/// some way.
pub enum EncodingError {
    /// Indicates there are too many code sections than can be encoded
    /// in the EOF format.
    TooManyCodeSections(usize),
    /// Indiciates a code section is too long
    CodeSectionTooLong(usize),
    /// Indicates the data section is too long
    DataSectionTooLong(usize),
    /// Indicates the data section is not last (which it is required
    /// to be for EOF)
    DataSectionNotLast,
    /// Indicates more than one data section
    MultipleDataSections
}


impl fmt::Debug for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodingError::TooManyCodeSections(w) => write!(f,"too many code sections ({:#x})",w),
            EncodingError::CodeSectionTooLong(w) => write!(f,"code section too long ({:#x})",w),
            EncodingError::DataSectionTooLong(w) => write!(f,"data section too long ({:#x})",w),
            EncodingError::DataSectionNotLast => write!(f,"data section is not last"),
            EncodingError::MultipleDataSections => write!(f,"multiple data sections")
        }
    }
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just reuse debug formatting.
        write!(f,"{:?}",self)
    }
}

// ============================================================================
// Decoding Error
// ============================================================================

/// An error which arises when attempting to decode a sequence of
/// bytes into a `Bytecode` structure.  In essence, this indicates the
/// bytecode sequence is malformed in some way.  These errors
/// generally apply when decoding EOF containers, since these have
/// essential and required structure.
pub enum DecodingError {
    /// Indicates the expected magic number for an EOF container was
    /// not `0xEF00` as expected.
    InvalidMagicNumber(u16),
    /// Indicates the EOF container has a version number which is not
    /// supported.
    UnsupportedEofVersion(u8),
    /// Indicates an invalid `kind_type` field was present for the
    /// given EOF version.
    InvalidKindType(u8),
    /// Indicates an invalid `kind_code` was present for the given EOF
    /// version.
    InvalidKindCode(u8),
    /// Indicates an invalid `kind_data` was present for the given EOF
    /// version.
    InvalidKindData(u8),
    /// A zero byte terminator is expected after the header before the
    /// main contents.
    InvalidTerminator(u8),
    /// Indicates the given `type_size` field is not consistent with
    /// the number of code sections (it should be multiple of four).
    InvalidTypeSize(u16),
    /// Indicates there were not enough bytes provide to complete
    /// decoding (i.e. the byte sequence is truncated in some way).
    UnexpectedEndOfFile,
    /// Indicates, having read the EOF container entirely, there are
    /// some unexpected trailing bytes.
    ExpectedEndOfFile
}

impl Default for DecodingError {
    fn default() -> Self {
        DecodingError::UnexpectedEndOfFile
    }
}

impl fmt::Debug for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodingError::InvalidMagicNumber(w) => write!(f,"invalid magic number ({:#x})",w),
            DecodingError::UnsupportedEofVersion(w) => write!(f,"unsupported EOF version ({:#x})",w),
            DecodingError::InvalidKindType(w) => write!(f,"invalid kind marker for type section ({:#x})",w),
            DecodingError::InvalidKindCode(w) => write!(f,"invalid kind marker for code section ({:#x})",w),
            DecodingError::InvalidKindData(w) => write!(f,"invalid kind marker for data section ({:#x})",w),
            DecodingError::InvalidTerminator(w) => write!(f,"invalid terminator for header ({:#x})",w),
            DecodingError::InvalidTypeSize(w) => write!(f,"invalid type section length ({:#x})",w),
            DecodingError::UnexpectedEndOfFile => write!(f,"unexpected end-of-bytes"),
            DecodingError::ExpectedEndOfFile => write!(f,"unexpected trailing bytes")
        }
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Just reuse debug formatting.
        write!(f,"{:?}",self)
    }
}

impl std::error::Error for DecodingError {}

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
    sections: Vec<Section>
}

impl Bytecode {
    pub fn empty() -> Self {
        Bytecode {
            sections: Vec::new()
        }
    }

    pub fn new(sections: Vec<Section>) -> Self {
        Bytecode { sections }
    }

    /// Return the number of sections in the code.
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    pub fn iter<'a>(&'a self) -> BytecodeIter<'a,Section> {
        self.sections.iter()
    }

    /// Add a new section to this bytecode container
    pub fn add(&mut self, section: Section) {
        self.sections.push(section)
    }

    /// Convert this bytecode contract into a byte sequence correctly
    /// formatted for legacy code.
    pub fn to_legacy_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        //
        for s in self.sections { s.encode(&mut bytes); }
        // Done
        bytes
    }

    /// Convert this bytecode contract into a byte sequence correctly
    /// formatted according to the EOF format.
    pub fn to_eof_bytes(self) -> Vec<u8> {
        internal_to_eof_bytes(self).unwrap()
    }

    /// Construct a bytecode contract from a given set of bytes.  The
    /// exact interpretation of these bytes depends on the fork.  For
    /// example, on some forks, certain instructions are permitted
    /// whilst on others they are not.  Likewise, EOF containers are
    /// supported on some forks but not others.
    pub fn from_bytes(bytes: &[u8]) -> Result<Bytecode,DecodingError> {
        // Check for EOF container
        if bytes.starts_with(&[opcode::EOF]) {
            internal_from_eof_bytes(bytes)
        } else {
            todo!()
        }
    }
}

// ===================================================================
// Traits
// ===================================================================

/// An iterator over the sections of a bytecode contract (e.g. code or
/// data).
pub type BytecodeIter<'a,T> = std::slice::Iter<'a,T>;

impl<'a> IntoIterator for &'a Bytecode {
    type Item = &'a Section;
    type IntoIter = BytecodeIter<'a,Section>;

    fn into_iter(self) -> Self::IntoIter {
        self.sections.iter()
    }
}

// ============================================================================
// Section
// ============================================================================

pub enum Section {
    /// A data section is simply a sequence of zero or more bytes.
    Data(Vec<u8>),
    /// A code section is a sequence of zero or more instructions
    /// along with appropriate _metadata_.
    Code(Vec<Instruction>, u8, u8, u16)
}

impl Section {
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

// ============================================================================
// Decoding (EOF)
// ============================================================================

/// Construct a bytecode container from an EOF formatted byte
/// sequence.  See EIP 3540 "EOF - EVM Object Format v1" for more
/// details on the format being parsed here.  Since the EOF format is
/// quite prescriptive, its possible that the incoming bytes are
/// malformed in some way --- in which case an error will be
/// generated.
fn internal_from_eof_bytes(bytes: &[u8]) -> Result<Bytecode,DecodingError> {
    let mut iter = ByteDecoder::new(bytes);
    iter.match_u16(EOF_MAGIC,|w| DecodingError::InvalidMagicNumber(w))?;
    // Pull out static information
    let version = iter.decode_u8()?;
    // Sanity check version information
    if version != 1 { return Err(DecodingError::UnsupportedEofVersion(version)); }
    iter.match_u8(0x01,|w| DecodingError::InvalidKindType(w))?;
    let type_len = iter.decode_u16()?;
    iter.match_u8(0x02,|w| DecodingError::InvalidKindCode(w))?;
    let num_code_sections = iter.decode_u16()? as usize;
    // Sanity check length of type section
    if (type_len as usize) != (num_code_sections * 4) {
        return Err(DecodingError::InvalidTypeSize(type_len));
    }
    let mut code_sizes : Vec<usize> = Vec::new();
    // Extract code sizes
    for _i in 0..num_code_sections {
        code_sizes.push(iter.decode_u16()? as usize);
    }
    iter.match_u8(0x03,|w| DecodingError::InvalidKindData(w))?;
    let data_size = iter.decode_u16()? as usize;
    iter.match_u8(0x00,|w| DecodingError::InvalidTerminator(w))?;
    // parse types section
    let mut types = Vec::new();
    for _i in 0..num_code_sections {
        let inputs = iter.decode_u8()?;
        let outputs = iter.decode_u8()?;
        let max_stack = iter.decode_u16()?;
        types.push((inputs,outputs,max_stack));
    }
    let mut code = Bytecode::new(Vec::new());
    // parse code section(s)
    for i in 0..num_code_sections {
        let bytes = iter.decode_bytes(code_sizes[i])?;
        // Recall type information
        let (inputs,outputs,max_stack) = types[i];
        // Convert byte sequence into an instruction sequence.
        let insns = bytes.to_insns();
        // Add code section
        code.add(Section::Code(insns,inputs,outputs,max_stack))
    }
    // parse data sectin (if present)
    let data = iter.decode_bytes(data_size)?.to_vec();
    code.add(Section::Data(data));
    //
    iter.match_eof(DecodingError::ExpectedEndOfFile)?;
    // Done
    Ok(code)
}

// ============================================================================
// Encoding (EOF)
// ============================================================================

fn internal_to_eof_bytes(bytecode: Bytecode) -> Result<Vec<u8>,EncodingError> {
    let mut code_sections = Vec::new();
    let mut data_section : Option<Vec<u8>> = None;
    // Count number of code contracts (to be deprecated?)
    for section in &bytecode {
        match section {
            Section::Code(_,_,_,_) => {
                if data_section != None {
                    return Err(EncodingError::DataSectionNotLast)
                }
                let mut code_bytes = Vec::new();
                section.encode(&mut code_bytes);
                code_sections.push(code_bytes);
            }
            Section::Data(data_bytes) => {
                if data_section != None {
                    return Err(EncodingError::MultipleDataSections)
                } else {
                    data_section = Some(data_bytes.clone())
                }
            }
        }
    }
    let data_len :usize = data_section.as_ref().map_or(0,|s| s.len());
    let mut bytes = ByteEncoder::new();
    // Magic
    bytes.encode_u16(EOF_MAGIC);
    // Version
    bytes.encode_u8(1);
    // Kind type
    bytes.encode_u8(0x1);
    // Type length
    bytes.encode_checked_u16(code_sections.len() * 4, |c| {
        EncodingError::TooManyCodeSections(c/4)
    })?;
    // Kind code
    bytes.encode_u8(0x2);
    // Num code sections
    bytes.encode_checked_u16(code_sections.len(), |_| unreachable!())?;
    // Code section lengths
    for code_bytes in &code_sections {
        bytes.encode_checked_u16(code_bytes.len(), |n| {
            EncodingError::CodeSectionTooLong(n)
        })?;
    }
    // Kind data
    bytes.encode_u8(0x3);
    // Data length
    bytes.encode_checked_u16(data_len, |n| {
        EncodingError::DataSectionTooLong(n)
    })?;
    // Header terminator
    bytes.encode_u8(0x00);
    // Write types data
    for section in &bytecode {
        match section {
            Section::Code(_,inputs,outputs,max_stack) => {
                bytes.encode_u8(*inputs);
                bytes.encode_u8(*outputs);
                bytes.encode_u16(*max_stack);
            }
            _ => {}
        }
    }
    // Write code bytes
    for code_bytes in code_sections {
        bytes.encode_bytes(code_bytes);
    }
    // Write data bytes
    bytes.encode_bytes(data_section.unwrap_or(Vec::new()));
    // Done
    Ok(bytes.to_vec())
}
