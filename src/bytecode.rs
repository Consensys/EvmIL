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
use crate::{Term};

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum CompileError {

}

#[derive(Debug)]
pub enum CodeGenError {
    PushEmpty,
    PushOverflow
}

// ============================================================================
// Bytecode Instructions
// ============================================================================

#[derive(PartialEq)]
pub enum Instruction {
    // 0s: Stop and Arithmetic Operations
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    // 10s: Comparison & Bitwise Logic Operations
    // 20s: Keccak256
    // 30s: Environmental Information
    // 40s: Block Information
    // 50s: Stack, Memory, Storage and Flow Operations
    JUMP,
    JUMPI,
    JUMPDEST,
    // 60 & 70s: Push Operations
    PUSH(Vec<u8>),
    PUSHL(usize), // Push label offset.
    // 80s: Duplicate Operations
    DUP(u8),
    // 90s: Exchange Operations
    SWAP(u8),
    // a0s: Logging Operations
    LOG(u8),
    // f0s: System Operations
    INVALID
}

impl Instruction {
    pub fn opcode(&self) -> Result<u8,CodeGenError> {
        let op = match self {
            Instruction::STOP => 0x00,
            Instruction::PUSH(bs) => {
                if bs.len() == 0 {
                    return Err(CodeGenError::PushEmpty);
                } else if bs.len() > 32 {
                    return Err(CodeGenError::PushOverflow);
                } else {
                    (0x5f + bs.len()) as u8
                }
            }
            _ => {
                panic!("Invalid instruction");
            }
        };
        //
        Ok(op)
    }
}

// ============================================================================
// Bytecode Programs
// ============================================================================

/// Represents a sequence of zero or more bytecodes which can be
/// turned, for example, into a hex string.  Likewise, they can be
/// decompiled or further optimised.
pub struct Bytecode {
    /// The underlying bytecode sequence.
    bytecodes: Vec<Instruction>,
    /// Counts the number of labels
    labels: usize
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode{bytecodes:Vec::new(), labels:0}
    }

    pub fn push(&mut self, insn: Instruction) {
        if insn == Instruction::JUMPDEST {
            self.labels = self.labels + 1;
        }
        self.bytecodes.push(insn);
    }

    /// Translate this sequence of bytecode instructions into a
    /// sequence of raw bytes.  This can still fail in a number of
    /// ways.  For example, the target for a `PUSHL` does not match
    /// any known `JUMPEST` label; Or, the stack size is exceeded,
    /// etc.
    pub fn to_bytes(&self) -> Result<Vec<u8>,CodeGenError> {
        let offsets = self.determine_offsets();
        let mut bytes = Vec::new();
        //
        for b in &self.bytecodes {
            // Push opcode
            bytes.push(b.opcode()?);
            // Push operands (if applicable)
            match b {
                Instruction::PUSH(args) => {
                    bytes.extend(args);
                }
                Instruction::PUSHL(idx) => {
                    let offset = offsets[*idx] as u8;
                    bytes.push(offset);
                }
                _ => {
                    // All other instructions have no operands.
                }
            }
        }
        // Done
        Ok(bytes)
    }

    fn determine_offsets(&self) -> Vec<usize> {
        let mut offsets = Vec::new();
        let mut offset = 0;
        // Calculate label offsets
        for b in &self.bytecodes {
            match b {
                Instruction::JUMPDEST => {
                    offsets.push(offset);
                }
                Instruction::PUSH(bs) => {
                    offset = offset + bs.len();
                }
                Instruction::PUSHL(_) => {
                    // FIXME: this is a false assumption once the
                    // instruction sequence gets sufficiently long.
                    offset = offset + 1;
                }
                _ => {}
            }
            offset = offset + 1;
        }
        //
        offsets
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn try_from(terms: &[Term]) -> Result<Bytecode,CompileError> {
    let mut bytecode = Bytecode::new();
    // Translate statements one-by-one
    for t in terms {
        t.translate(&mut bytecode)?;
    }
    // Done
    Ok(bytecode)
}

// ============================================================================
// Trait implementstions
// ============================================================================

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl TryFrom<&[Term]> for Bytecode {
    type Error = CompileError;

    fn try_from(terms: &[Term]) -> Result<Bytecode,Self::Error> {
        try_from(terms)
    }
}

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl<const N: usize> TryFrom<&[Term;N]> for Bytecode {
    type Error = CompileError;

    fn try_from(terms: &[Term;N]) -> Result<Bytecode,Self::Error> {
        try_from(terms)
    }
}

/// Try and translate a bytecode sequence into byte sequence.  This
/// can fail for a number of reasons (e.g. dangling branches, stack
/// depth exceeded, etc).
impl TryInto<Vec<u8>> for Bytecode {
    type Error = CodeGenError;

    fn try_into(self) -> Result<Vec<u8>,Self::Error> {
        self.to_bytes()
    }
}
