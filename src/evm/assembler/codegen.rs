use std::fmt;
use std::collections::{HashMap};
use crate::evm::instruction::{AssemblyInstruction,Instruction};

// =============================================================================
// Assemble Error
// =============================================================================

#[derive(Debug)]
pub enum AssembleError {
    /// Indicates a labelled instruction was encountered that targets a
    /// non-existent label.
    UnknownLabel(String),
    /// Indicates that the distance of a calculated relative offset
    /// exceeds 16bits.
    InvalidRelativeOffset,
    /// Indicates that the distance of a calculated offset exceeds the
    /// maximum permitted code size.
    OffsetTooLarge,
}

impl fmt::Display for AssembleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AssembleError {

}

/// Convert _labelled_ instructions into _concrete_ instructions
/// by first determining the byteoffset of each label, and then
/// using this to instantiate any partial instructions.  This can
/// lead to an error if there is a partial instruction which
/// refers to a label that does not exist; or, if a label is
/// declared more than once.
///
/// This algorithm computes the byte offsets of each instruction
/// iteratively until a fixed-point is reached.  This is necessary
/// because instructions are variable length.  In particular, legacy
/// bytecode uses labelled `PUSH` instructions for almost all control
/// flow.  Whilst, for EOF bytecode, this may be less of a problem it
/// can still presumably arise.  Since we want to choose the smallest
/// `PUSH` instructions possible, we have a chicken-and-egg problem:
/// to choose the smallest instructions we need to know the actual
/// byte offset of all instructions; to know the actual byte offset of
/// all instructions means we need to have determined what `PUSH`
/// instruction to use, etc.
///
/// Computing byte offsets for variable length instructions is a
/// classic problem (e.g. as found in Java bytecode), which we resolve
/// iteratively.  We assume all `PUSH` instructions have a one byte
/// operand to determine an initial set of offsets.  Based on this, we
/// then refine our choices of `PUSH` instruction (always increasing
/// monotonically in size) until we have a solution.
pub fn assemble(asm: &[AssemblyInstruction]) -> Result<Vec<Instruction>,AssembleError> {
    // Identify all labels contained within the sequence of assembly
    // instructions.  For each, we record their _instruction offset_.
    let labels = init_labels(asm)?;
    // Construct initial set of empty offsets based on the minimum
    // length of each partial instruction;
    let mut offsets = init_offsets(asm);
    // Iterate to a fixpoint.
    while update_offsets(asm, &labels, &mut offsets) {
        // Keep going until no more changes!
    }
    let mut insns = Vec::new();
    //
    // Instantiate all instructions
    for (i,b) in asm.iter().enumerate() {
        // Calculate byte offset of instruction, which is needed
        // to compute relative addresses.
        let insn_byte_offset = offsets[i];
        // Instantiate the instruction
        let insn = b.instantiate(insn_byte_offset,|lab| {
            // For given label compute its byte offset using the
            // label offsets and instruction offsets.
            get_label_byte_offset(lab,&labels,&offsets)
        }).unwrap();
        // Check whether we got anything
        match insn {
            Some(i) => { insns.push(i); }
            None => {}
        }
    }
    Ok(insns)
}

/// Initialise the labels map which maps each label to its
/// _instruction offset_.  Note that this may differ from an
/// instruction's _byte offset_ (i.e. since not all instructions are
/// one byte long).  Finally, this also checks that every partial
/// instruction targets a known label.
fn init_labels(instructions: &[AssemblyInstruction]) -> Result<HashMap<String,usize>,AssembleError> {
    let mut labels : HashMap<String, usize> = HashMap::new();
    // Compute labels
    for (i,b) in instructions.iter().enumerate() {
        match b {
            AssemblyInstruction::LABEL(lab) => {
                // NOTE: how to avoid this allocation?  It seems like
                // we should be able to use a `HashMap<&str,usize>`
                // here but I was unable to get it to work fully.
                labels.insert(lab.to_string(),i);
            }
            _ => {} // ignore
        }
    }
    // Sanity check partial instructions target known labels.
    for insn in instructions {
        match insn.target() {
            Some(lab) => {
                if !labels.contains_key(lab) {
                    return Err(AssembleError::UnknownLabel(lab.to_string()))
                }
            }
            None => {} // ignore
        }
    }
    // Done
    Ok(labels)
}

/// Compute the initial set of instruction offsets based on the
/// _minimum length_ of each instruction.  For single byte
/// instructions, the minimum length is always `1`.  However, for a
/// variable length instruction (e.g. `PUSH`), its minimum length is
/// `2`, etc.  Finally, artificial instructions (e.g. labels) have no
/// length since they do not correspond to actual instructions in the
/// final sequence.
fn init_offsets(instructions: &[AssemblyInstruction]) -> Vec<usize> {
    let mut offsets = Vec::new();
    let mut offset = 0;
    for insn in instructions {
        // Update instruction offset
        offsets.push(offset);
        // Calculate next offset
        offset = offset + insn.min_len();
    }
    offsets
}

/// Update the offset information, noting whether or not anything
/// actually changed.  The key is that as we recalculate offsets
/// we may find the width has changed.  If this happens, we have
/// to recalculate all offsets again assuming the larger width(s).
fn update_offsets(instructions: &[AssemblyInstruction], labels: &HashMap<String,usize>, offsets: &mut [usize]) -> bool {
    let mut changed = false;
    let mut offset = 0;
    // Calculate label offsets
    for (i,b) in instructions.iter().enumerate() {
        let old = offsets[i];
        // Update instruction offset
        offsets[i] = offset;
        // Determine whether this changed (or not)
        changed |= offset != old;
        // Calculate next offset
        offset = offset + insn_length(i,b,labels,offsets);
    }
    //
    changed
}

/// Determine the _actual length_ of an assembly instruction based on
/// the current estimate of all bytecode offsets.
fn insn_length(index: usize, insn: &AssemblyInstruction, labels: &HashMap<String,usize>, offsets: &[usize]) -> usize {
    // Calculate byte offset of instruction, which is needed
    // to compute relative addresses.
    let insn_byte_offset = offsets[index];
    // NOTE: we are determining the length of the instruction
    // here by instantiating it based on available
    // information.  That's actually suboptimal since it may
    // force memory allocation which is unnecessary.
    let insn = insn.instantiate(insn_byte_offset,|lab| get_label_byte_offset(lab,labels,offsets)).unwrap();
    // Finally, just return the instantiated instructions
    // length
    match insn {
        Some(i) => i.length(),
        None => 0
    }
}

/// Get the the byte offset for a given label.
fn get_label_byte_offset(lab: &str, labels: &HashMap<String,usize>, offsets: &[usize]) -> Option<usize> {
    // Get the instruction offset of the given label.  Observe
    // that we can assume here that this is available as,
    // otherwise, an error would have occurred upstream.
    let lab_insn_offset = labels.get(lab).unwrap();
    // Convert the instruction offset into an (estimated) byte
    // offset for the given label.
    Some(offsets[*lab_insn_offset])
}
