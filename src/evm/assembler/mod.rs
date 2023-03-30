mod lexer; // private
mod parser; // private (for now)

use std::fmt;
use std::collections::{HashMap};
use crate::evm::opcode;
use crate::evm::{Bytecode,Instruction,Section};
use crate::util::FromHexString;

use parser::Parser;
use lexer::{Token,Lexer};

// ===================================================================
// Error
// ===================================================================

#[derive(Debug)]
pub enum AsmError {
    ExpectedOperand,
    InvalidHexString(usize),
    InvalidInstruction,
    UnexpectedCharacter(usize),
    UnexpectedToken
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AsmError {

}

// ============================================================================
// Bytecode Assembly
// ===========================================================================

/// Represents a sequence of zero or more bytecodes which can be
/// turned, for example, into a hex string.  Likewise, they can be
/// decompiled or further optimised.
pub struct Assembly {
    /// The underlying bytecode sequence.
    bytecodes: Vec<PartialInstruction>,
    /// Marked labels in the sequence
    labels: Vec<(String,usize)>
}

impl Assembly {
    /// Create empty assembly
    pub fn new() -> Self {
        Assembly {
            bytecodes: Vec::new(),
            labels: Vec::new(),
        }
    }

    /// Parse assembly language to form an assembly
    pub fn from_str(input: &str) -> Result<Assembly,AsmError> {
        // Holds the set of lines being parsed.
        let lines : Vec<&str> = input.lines().collect();
        let mut parser = Parser::new();
        //
        for l in &lines {
            parser.parse(l)?;
        }
        // Done
        Ok(parser.to_assembly())
    }

    /// Add a concrete instruction onto this bytecode sequence.
    pub fn push(&mut self, insn: Instruction) {
        self.bytecodes.push(PartialInstruction::Done(insn));
    }

    /// Add a partial instruction onto this bytecode sequence which is
    /// parameterised by a given label.
    pub fn push_partial(&mut self, label: &str, f: fn(ByteOffset)->Instruction) {
        self.bytecodes.push(PartialInstruction::Unlinked(label.to_string(), f));
    }

    /// Mark a new label in this bytecode sequence
    pub fn label(&mut self, name: &str) {
        // Construct label record
        let lab = (name.to_string(),self.bytecodes.len());
        // Store it
        self.labels.push(lab);
    }

    /// Translate this sequence of bytecode instructions into a
    /// sequence of raw bytes.  This can still fail in a number of
    /// ways.  For example, the target for a `PUSHL` does not match
    /// any known `JUMPEST` label; Or, the stack size is exceeded,
    /// etc.
    pub fn to_bytecode(mut self) -> Result<Bytecode, AsmError> {
        // Translate all patches into concrete instructions.
        self.resolve_patches();
        // Translate concrete instructions into bytes.
        let mut insns = Vec::new();
        //
        for b in self.bytecodes {
            // Encode instruction
            insns.push(b.unwrap());
        }
        // FIXME: this fundamentally broken
        let mut bytecode = Bytecode::new();
        // Add code section
        bytecode.add(Section::Code{insns,inputs:0,outputs:0,max_stack:0});
        // Done
        Ok(bytecode)
    }

    /// Determine the offsets of all labels within the instruction
    /// sequence.  This is non-trivial because labels which are
    /// further away affect the overall size of the bytecode sequence
    /// (hence, a label can affect the offset of itself or other
    /// labels).
    fn resolve_patches(&mut self) {
        // Link all instructions
        for b in &mut self.bytecodes {
            b.link(&self.labels);
        }
        // Construct initial set of empty offsets
        let mut offsets = vec![ByteOffset(0); self.bytecodes.len()];
        // Iterate to a fixpoint.
        while self.update_offsets(&mut offsets) {
            // Keep going until no more changes!
        }
        // Finalise patches
        for b in &mut self.bytecodes {
            b.finalise(&offsets);
        }
    }

    /// Update the offset information, noting whether or not anything
    /// actually changed.  The key is that as we recalculate offsets
    /// we may find the width has changed.  If this happens, we have
    /// to recalculate all offsets again assuming the larger width(s).
    fn update_offsets(&self, offsets: &mut [ByteOffset]) -> bool {
        let mut changed = false;
        let mut offset = 0;
        // Calculate label offsets
        for i in 0..self.bytecodes.len() {
            let old = offsets[i].0 as usize;
            // Update instruction offset
            offsets[i] = ByteOffset(offset as u16);
            // Determine whether this changed (or not)
            changed |= (offset != old);
            // Calculate next offset
            offset = offset + self.bytecodes[i].length(offsets);
        }
        //
        changed
    }
}


// ============================================================================
// Partial Instruction
// ============================================================================

/// A partial instruction is one which may be incomplete due to it
/// containing an unresolved label.  The process of turning a partial
/// instruction into a complete works in three steps.  Initially, an
/// instruction containing a label is "unlinked", meaning we just have
/// the label as a string.  An unlinked instruction is "linked" by
/// replacing the label by the _instruction offset_ it represents.
/// Observe we are not yet finished because the EVM uses _byte
/// offsets_ rather than _instruction offsets_.  Therefore, to
/// complete the process, we require the byte offset of each
/// instruction in the enclosing sequence.
enum PartialInstruction {
    // Instruction which does not require patching.
    Done(Instruction),
    // Linked instruction which requires patching.  First operand is
    // the _instruction offset_ into the byte stream.
    Linked(usize,fn(ByteOffset)->Instruction),
    // Unlinked instruction which requires patching.  First operand
    // identifies the label to be patched within the byte stream.
    Unlinked(String,fn(ByteOffset)->Instruction)
}

impl PartialInstruction {
    /// Extract a reference to the completed instruction.  This will
    /// panic if this partial instruction is not in the `Done` state.
    pub fn unwrap(self) -> Instruction {
        match self {
            PartialInstruction::Done(insn) => insn,
            _ => { unreachable!(); }
        }
    }
    /// Link an unlinked instruction using a mapping from labels to
    /// their _instruction offsets_.  Note that instructions which are
    /// either already linked or are complete do not need further
    /// processing here.
    pub fn link(&mut self, labels: &[(String,usize)]) {
        let (i,f) = match self {
            PartialInstruction::Unlinked(l,f) => {
                let offset = Self::get_insn_offset(l,labels);
                (offset,*f)
            }
            _ => { return; } // nothing to do
        };
        // Link it!
        *self = PartialInstruction::Linked(i,f);
    }

    /// Finalise an instruction once the _byte offset_ of all
    /// instructions in the enclosing sequence is known.
    pub fn finalise(&mut self, offsets: &[ByteOffset]) {
        let insn = match self {
            PartialInstruction::Done(insn) => { return; }
            PartialInstruction::Linked(l,f) => {
                f(offsets[*l])
            }
            PartialInstruction::Unlinked(_,_) => {
                unreachable!()
            }
        };
        // Link it!
        *self = PartialInstruction::Done(insn);
    }

    /// Determine the length of this partial instruction using the
    /// available offset information.  Note that we cannot determine
    /// the offset of an unlinked instruction.
    pub fn length(&self, offsets: &[ByteOffset]) -> usize {
        match self {
            PartialInstruction::Done(insn) => insn.length(),
            PartialInstruction::Linked(i,f) => {
                // NOTE: this is not very efficient as it forces the
                // allocation of an entire instruction which is then
                // discarded immediately.
                f(offsets[*i]).length()
            }
            PartialInstruction::Unlinked(_,_) => {
                unreachable!();
            }
        }
    }

    /// Match the label against the set of labels and their offsets to
    /// get the offset.  A map would be easier to use here :)
    fn get_insn_offset(label: &str, labels: &[(String,usize)]) -> usize {
        for (lab,index) in labels {
            if label == lab {
                return *index;
            }
        }
        // Note: we could report a useful error here, as this
        // indicates a sequence where an instruction
        // (e.g. `JUMP`) refers to a label which does not
        // exist.
        unreachable!()
    }
}

// ============================================================================
// Label Offsets
// ============================================================================

/// Used to simplify calculation of label offsets.
#[derive(PartialEq, Copy, Clone)]
pub struct ByteOffset(pub u16);

impl ByteOffset {
    /// Determine the width of this offset (in bytes).
    pub fn width(&self) -> u16 {
        if self.0 > 255 {
            2
        } else {
            1
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if self.0 > 255 {
            vec![(self.0 / 256) as u8, (self.0 % 256) as u8]
        } else {
            vec![self.0 as u8]
        }
    }
}
