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
use std::collections::HashMap;
use crate::util::{ByteEncoder,ByteDecoder};
use crate::bytecode::{Bytecode,Section};
use crate::instruction::{AssemblyInstruction,Instruction,ToInstructions};
use crate::instruction::AbstractInstruction::*;

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
// Decoding (EOF)
// ============================================================================

/// Construct a bytecode container from an EOF formatted byte
/// sequence.  See EIP 3540 "EOF - EVM Object Format v1" for more
/// details on the format being parsed here.  Since the EOF format is
/// quite prescriptive, its possible that the incoming bytes are
/// malformed in some way --- in which case an error will be
/// generated.
pub fn from_bytes(bytes: &[u8]) -> Result<Bytecode<AssemblyInstruction>,DecodingError> {
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
        let (_inputs,_outputs,_max_stack) = types[i];
        // Convert byte sequence into an instruction sequence.
        let insns = disassemble(bytes);
        // Add code section
        code.add(Section::Code(insns));
        // Validate types information?
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

pub fn to_bytes(bytecode: Bytecode<Instruction>) -> Result<Vec<u8>,EncodingError> {
    let mut code_sections = Vec::new();
    let mut data_section : Option<Vec<u8>> = None;
    // Count number of code contracts (to be deprecated?)
    for section in &bytecode {
        match section {
            Section::Code(_) => {
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
            Section::Code(_) => {
                // FIXME: infer necessary information.
                bytes.encode_u8(0);
                bytes.encode_u8(0);
                bytes.encode_u16(0);
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

// ============================================================================
// Disassembler
// ============================================================================

/// Disassembles a set of EOF instructions into assembly instructions.
/// This is must easier than for the legacy case, as inferring labels
/// is straightforward.
fn disassemble(bytes: &[u8]) -> Vec<AssemblyInstruction> {
    let bytecodes = bytes.to_insns();
    let mut count = 0;
    let mut pc = 0;
    // Identify and allocate all labels
    let mut labels = HashMap::new();
    for b in &bytecodes {
        //
        match b {
            RJUMP(r16)|RJUMPI(r16) => {
                let target = from_rel_offset(pc+3,*r16);
                // Allocate label (if not already)
                if !labels.contains_key(&target) {
                    labels.insert(target,format!("lab{count}"));
                    count += 1;
                }
            }
            _ => {}
        }
        //
        pc += b.length();
    }
    // Translate all instructions whilst inserting labels.
    pc = 0;
    let mut asm = Vec::new();
    //
    for b in &bytecodes {
        if labels.contains_key(&pc) {
            asm.push(LABEL(labels[&pc].clone()));
        }
        asm.push(translate_insn(pc,b,&labels));
        pc += b.length();
    }
    // Done
    asm
}

fn translate_insn(pc: usize, insn: &Instruction, labels: &HashMap<usize,String>) -> AssemblyInstruction {
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
        RJUMP(r16) => {
            let target = from_rel_offset(pc+3,*r16);
            RJUMP(labels[&target].clone())
        }
        RJUMPI(r16) => {
            let target = from_rel_offset(pc+3,*r16);
            RJUMPI(labels[&target].clone())
        }
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
        PUSHL(..)|LABEL(_) => unreachable!(),
    }
}

/// Calculate the absolute byte offset of a given relative jump target
/// from a given `pc` position (which is the `pc` after the
/// instruction in question).
fn from_rel_offset(pc: usize, rel: i16) -> usize {
    let mut r = pc as isize;
    r += rel as isize;
    r as usize
}
