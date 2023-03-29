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
use crate::il::Term;
use crate::il::{Compiler, CompilerError};
use crate::ll::instruction;
use crate::ll::{Instruction};

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
    pub fn unwrap(&self) -> &Instruction {
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
// Bytecode Programs
// ============================================================================


/// Represents a sequence of zero or more bytecodes which can be
/// turned, for example, into a hex string.  Likewise, they can be
/// decompiled or further optimised.
pub struct Bytecode {
    /// The underlying bytecode sequence.
    bytecodes: Vec<PartialInstruction>,
    /// Marked labels in the sequence
    labels: Vec<(String,usize)>
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode {
            bytecodes: Vec::new(),
            labels: Vec::new(),
        }
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

    /// Get access to the raw sequence of instructions.
    pub fn instructions(&self) -> &[Instruction] {
        todo!("eliminate me");
    }

    /// Translate this sequence of bytecode instructions into a
    /// sequence of raw bytes.  This can still fail in a number of
    /// ways.  For example, the target for a `PUSHL` does not match
    /// any known `JUMPEST` label; Or, the stack size is exceeded,
    /// etc.
    pub fn to_bytes(mut self) -> Result<Vec<u8>, instruction::Error> {
        // Translate all patches into concrete instructions.
        self.resolve_patches();
        // Translate concrete instructions into bytes.
        let mut bytes = Vec::new();
        //
        for b in &self.bytecodes {
            // Encode instruction
            b.unwrap().encode(&mut bytes)?;
        }
        // Done
        Ok(bytes)
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
// Helpers
// ============================================================================

fn try_from(terms: &[Term]) -> Result<Bytecode, CompilerError> {
    let mut bytecode = Bytecode::new();
    let mut compiler = Compiler::new(&mut bytecode);
    // Translate statements one-by-one
    for t in terms {
        compiler.translate(t)?;
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
    type Error = CompilerError;

    fn try_from(terms: &[Term]) -> Result<Bytecode, Self::Error> {
        try_from(terms)
    }
}

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl<const N: usize> TryFrom<&[Term; N]> for Bytecode {
    type Error = crate::il::CompilerError;

    fn try_from(terms: &[Term; N]) -> Result<Bytecode, Self::Error> {
        try_from(terms)
    }
}

/// Try and translate a bytecode sequence into byte sequence.  This
/// can fail for a number of reasons (e.g. dangling branches, stack
/// depth exceeded, etc).
impl TryInto<Vec<u8>> for Bytecode {
    type Error = instruction::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        self.to_bytes()
    }
}
