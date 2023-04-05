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
use crate::evm::{Instruction,ToInstructions};

/// The EOF magic prefix as dictated in EIP3540.
pub const EOF_MAGIC : u16 = 0xEF00;

// ============================================================================
// Error
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

    /// Add a new section to this bytecode container
    pub fn add(&mut self, section: Section) {
        self.sections.push(section)
    }

    /// Convert this bytecode contract into a byte sequence correctly
    /// formatted according to the container version (e.g. legacy or
    /// EOF).
    pub fn to_legacy_bytes(self) -> Vec<u8> {
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
    pub fn from_bytes(bytes: &[u8]) -> Result<Bytecode,DecodingError> {
        // Check for EOF container
        if bytes.starts_with(&[opcode::EOF]) {
            from_eof_bytes(bytes)
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
            Section::Code{insns, inputs: _, outputs: _, max_stack: _} => {
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
fn from_eof_bytes(bytes: &[u8]) -> Result<Bytecode,DecodingError> {
    let mut iter = EofIterator::new(bytes);
    iter.match_u16(EOF_MAGIC,|w| DecodingError::InvalidMagicNumber(w))?;
    // Pull out static information
    let version = iter.next_u8()?;
    // Sanity check version information
    if version != 1 { return Err(DecodingError::UnsupportedEofVersion(version)); }
    iter.match_u8(0x01,|w| DecodingError::InvalidKindType(w))?;
    let type_len = iter.next_u16()?;
    iter.match_u8(0x02,|w| DecodingError::InvalidKindCode(w))?;
    let num_code_sections = iter.next_u16()? as usize;
    // Sanity check length of type section
    if (type_len as usize) != (num_code_sections * 4) {
        return Err(DecodingError::InvalidTypeSize(type_len));
    }
    let mut code_sizes : Vec<usize> = Vec::new();
    // Extract code sizes
    for _i in 0..num_code_sections {
        code_sizes.push(iter.next_u16()? as usize);
    }
    iter.match_u8(0x03,|w| DecodingError::InvalidKindData(w))?;
    let data_size = iter.next_u16()? as usize;
    iter.match_u8(0x00,|w| DecodingError::InvalidTerminator(w))?;
    // parse types section
    let mut types = Vec::new();
    for _i in 0..num_code_sections {
        let inputs = iter.next_u8()?;
        let outputs = iter.next_u8()?;
        let max_stack = iter.next_u16()?;
        types.push((inputs,outputs,max_stack));
    }
    let mut code = Bytecode::new(Vec::new());
    // parse code section(s)
    for i in 0..num_code_sections {
        let bytes = iter.next_bytes(code_sizes[i])?;
        // Recall type information
        let (inputs,outputs,max_stack) = types[i];
        // Convert byte sequence into an instruction sequence.
        let insns = bytes.to_insns();
        // Add code section
        code.add(Section::Code{insns,inputs,outputs,max_stack})
    }
    // parse data sectin (if present)
    let data = iter.next_bytes(data_size)?.to_vec();
    code.add(Section::Data(data));
    //
    iter.match_eof()?;
    // Done
    Ok(code)
}

/// A simple alias to make things a bit clearer.  In essence, this
/// generates a decoding error from a given byte or word in the stream
/// (depending on the kind of error being generated).
type DecodingErrorFn<T> = fn(T)->DecodingError;

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

    /// Attempt to match a given `u8` byte in the bytestream at the
    /// present position.  If the match fails, an error is generating
    /// using the provided decoding error generator.
    pub fn match_u8(&mut self, n: u8, ef: DecodingErrorFn<u8>) -> Result<(),DecodingError> {
        let m = self.next_u8()?;
        if m == n { Ok(()) }
        else { Err(ef(m)) }
    }

    /// Attempt to match a given `u16` word in the bytestream at the
    /// present position assuming a _big endian_ representation.  If
    /// the match fails, an error is generating using the provided
    /// decoding error generator.
    pub fn match_u16(&mut self, n: u16, ef: DecodingErrorFn<u16>) -> Result<(),DecodingError> {
        let m = self.next_u16()?;
        if m == n { Ok(()) }
        else { Err(ef(m)) }
    }

    /// Attempt to match the _end of file_.  That is, we are expected
    /// at this point that all bytes in original stream have been
    /// consumed.  If not, then we have some trailing garbage in the
    /// original stream and, if so, an error is generating using the
    /// provided decoding error generator.
    pub fn match_eof(&mut self) -> Result<(),DecodingError> {
        if self.index == self.bytes.len() {
            Ok(())
        } else {
            Err(DecodingError::ExpectedEndOfFile)
        }
    }

    /// Read the next byte from the sequence, and move our position to
    /// the next byte in the sequence.  If no such byte is available
    /// (i.e. we have reached the end of the byte sequence), then an
    /// error is reported.
    pub fn next_u8(&mut self) -> Result<u8,DecodingError> {
        if self.index < self.bytes.len() {
            let next = self.bytes[self.index];
            self.index += 1;
            Ok(next)
        } else {
            Err(DecodingError::UnexpectedEndOfFile)
        }
    }

    /// Read the next word from the sequence assuming a _big endian_
    /// representation, whilst moving our position to the next byte in
    /// the sequence.  If no such word is available (i.e. we have
    /// reached the end of the byte sequence), then an error is
    /// reported.
    pub fn next_u16(&mut self) -> Result<u16,DecodingError> {
        let msb = self.next_u8()?;
        let lsb = self.next_u8()?;
        Ok(u16::from_be_bytes([msb,lsb]))
    }

    /// Read the next `n` bytes from the sequence, whilst moving our
    /// position to the following byte.  If there are insufficient
    /// bytes remaining, then an error is reported.
    pub fn next_bytes(&mut self, length: usize) -> Result<&'a [u8],DecodingError> {
        let start = self.index;
        self.index += length;
        if self.index <= self.bytes.len() {
            Ok(&self.bytes[start..self.index])
        } else {
            Err(DecodingError::UnexpectedEndOfFile)
        }
    }
}
