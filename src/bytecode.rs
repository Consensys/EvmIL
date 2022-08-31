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
    /// A push instruction cannot push zero bytes and, likewise,
    /// cannot push more than 32 bytes.
    InvalidPush,
    /// A label cannot exceed the 24Kb limit imposed by the EVM.
    InvalidLabelOffset
}

// ============================================================================
// Label Offsets
// ============================================================================

/// Used to simplify calculation of label offsets.
#[derive(PartialEq,Copy,Clone)]
pub struct Offset(u16);

impl Offset {
    /// Determine the width of this offset (in bytes).
    pub fn width(&self) -> u16 {
        if self.0 > 255 { 2 } else { 1 }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if self.0 > 255 {
            vec![(self.0/256) as u8,(self.0%256) as u8]
        } else {
            vec![self.0 as u8]
        }
    }
}

// ============================================================================
// Bytecode Instructions
// ============================================================================

#[derive(Debug,PartialEq)]
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
    /// Determine the opcode for a given instruction.  In many cases,
    /// this is a straightforward mapping.  However, in other cases,
    /// its slightly more involved as a calculation involving the
    /// operands is required.
    pub fn opcode(&self, offsets: &[Offset]) -> Result<u8,CodeGenError> {
        let op = match self {
            // 0s: Stop and Arithmetic Operations
            Instruction::STOP => 0x00,
            Instruction::ADD => 0x01,
            Instruction::MUL => 0x02,
            Instruction::SUB => 0x03,
            Instruction::DIV => 0x04,
            // 50s: Stack, Memory, Storage and Flow Operations
            Instruction::JUMP => 0x56,
            Instruction::JUMPI => 0x57,
            // ...
            Instruction::JUMPDEST => 0x5b,
            //
            Instruction::PUSH(bs) => {
                if bs.len() == 0 || bs.len() > 32 {
                    return Err(CodeGenError::InvalidPush);
                } else {
                    (0x5f + bs.len()) as u8
                }
            }
            //
            Instruction::PUSHL(lab) => {
                let offset = &offsets[*lab];
                if offset.width() == 2 { 0x61 }
                else { 0x60 }
            }
            // f0s: System Operations
            Instruction::INVALID => 0xfe,
            //
            _ => {
                panic!("Invalid instruction ({:?})",self);
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

    /// Return the number of labels in the instruction sequence thus
    /// far.
    pub fn num_labels(&self) -> usize {
        self.labels
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
            bytes.push(b.opcode(&offsets)?);
            // Push operands (if applicable)
            match b {
                Instruction::PUSH(args) => {
                    bytes.extend(args);
                }
                Instruction::PUSHL(idx) => {
                    bytes.extend(offsets[*idx].to_bytes());
                }
                _ => {
                    // All other instructions have no operands.
                }
            }
        }
        // Done
        Ok(bytes)
    }

    /// Determine the offsets of all labels within the instruction
    /// sequence.  This is non-trivial because labels which are
    /// further away affect the overall size of the bytecode sequence
    /// (hence, a label can affect the offset of itself or other
    /// labels).
    fn determine_offsets(&self) -> Vec<Offset> {
        // Construct initial set of empty offsets
        let mut offsets = vec![Offset(0); self.labels];
        // Iterate to a fixpoint.
        while self.update_offsets(&mut offsets) {
            // Keep going until no more changes!
        }
        //
        offsets
    }

    /// Update the offset information, noting whether or not anything
    /// actually changed.  The key is that as we recalculate offsets
    /// we may find the width has changed.  If this happens, we have
    /// to recalculate all offsets again assuming the larger width(s).
    fn update_offsets(&self, offsets: &mut [Offset]) -> bool {
        let mut changed = false;
        let mut offset = 0u16;
        let mut lab = 0;
        // Calculate label offsets
        for b in &self.bytecodes {
            match b {
                Instruction::JUMPDEST => {
                    // Extract old offset
                    let oldoff = offsets[lab];
                    // Construct new offset
                    let newoff = Offset(offset);
                    // Check!
                    if oldoff != newoff {
                        // Offset has changed, but the key thing is
                        // whether or not its _width_ has changed.
                        changed |= oldoff.width() != newoff.width();
                        // Update new offset.
                        offsets[lab] = newoff;
                    }
                    lab = lab + 1;
                }
                Instruction::PUSH(bs) => offset = offset + (bs.len() as u16),
                Instruction::PUSHL(lab) => {
                    // This time calculate a more accurate figure.
                    offset = offset + offsets[*lab].width()
                }
                _ => {}
            }
            offset = offset + 1;
        }
        //
        changed
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
