mod lexer; // private
mod parser; // private (for now)

use std::fmt;
use std::collections::{HashMap};
use crate::evm::opcode;
use crate::evm::{Bytecode,BytecodeVersion,Instruction,Section};
use crate::util::{ToHexString,FromHexString};

use parser::Parser;
use lexer::{Token,Lexer};

// ===================================================================
// Assembly Error
// ===================================================================

#[derive(Debug)]
pub enum AssemblyError {
    /// Indicates an instruction is given inside a data section.
    InvalidCodeSection,
    /// Indicates data is given inside a code section.
    InvalidDataSection,
    /// Indicates a partial instruction was encountered that targets a
    /// non-existent label.
    UnknownLabel(String)
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AssemblyError {

}

// ===================================================================
// Parse Error
// ===================================================================

/// Indicates an error occurred whilst parsing some assembly language
/// into an assembly (i.e. an error originating from the lexer or
/// parser).
#[derive(Debug)]
pub enum AssemblyLanguageError {
    ExpectedOperand,
    InvalidHexString(usize),
    InvalidInstruction,
    UnexpectedCharacter(usize),
    UnexpectedToken
}

impl fmt::Display for AssemblyLanguageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AssemblyLanguageError {

}

// ============================================================================
// Bytecode Assembly
// ===========================================================================

/// Represents a sequence of zero or more bytecodes which can be
/// turned, for example, into a hex string.  Likewise, they can be
/// decompiled or further optimised.
pub struct Assembly {
    version: BytecodeVersion,
    /// The underlying bytecode sequence.
    bytecodes: Vec<AssemblyInstruction>
}

impl Assembly {
    /// Create an empty assembly
    pub fn new(version: BytecodeVersion) -> Self {
        Assembly {
            version,
            bytecodes: Vec::new()
        }
    }

    /// Parse assembly language to form an assembly
    pub fn from_str(version: BytecodeVersion, input: &str) -> Result<Assembly,AssemblyLanguageError> {
        // Holds the set of lines being parsed.
        let lines : Vec<&str> = input.lines().collect();
        let mut parser = Parser::new(version);
        //
        for l in &lines {
            parser.parse(l)?;
        }
        // Done
        Ok(parser.to_assembly())
    }

    /// Add an assembly instruction onto this bytecode sequence.
    pub fn push<T:Into<AssemblyInstruction>>(&mut self, insn: T) {
        self.bytecodes.push(insn.into());
    }

    /// Translate this sequence of bytecode instructions into a
    /// sequence of raw bytes.  This can still fail in a number of
    /// ways.  For example, the target for a `PUSHL` does not match
    /// any known `JUMPEST` label; Or, the stack size is exceeded,
    /// etc.
    pub fn to_bytecode(mut self) -> Result<Bytecode, AssemblyError> {
        // Resolve all partial instructions into concrete instructions.
        resolve_labels(&mut self.bytecodes)?;
        // Translate concrete instructions into bytes.
        let mut sections = Vec::new();
        //
        for b in self.bytecodes {
            match b {
                AssemblyInstruction::Label(_) => {
                    // Since all labels have been resolved, we simply
                    // discard this as it has no semantic meaning.
                }
                AssemblyInstruction::CodeSection => {
                    // FIXME: need to figure out how to determine the inputs/outputs, etc.
                    sections.push(Section::Code{insns: Vec::new(), inputs: 0, outputs: 0, max_stack: 0});
                }
                AssemblyInstruction::DataSection => {
                    sections.push(Section::Data(Vec::new()));
                }
                AssemblyInstruction::Concrete(insn) => {
                    match sections.last_mut() {
                        Some(Section::Code{insns,inputs,outputs,max_stack}) => {
                            insns.push(insn);
                        }
                        _ => {
                            // For now, we cannot add an instruction
                            // into a data section, or when no initial
                            // code section was defined.  At some
                            // point, however, we will presumably want
                            // to support data sections which contain
                            // initcode.
                            return Err(AssemblyError::InvalidCodeSection);
                        }
                    }
                }
                AssemblyInstruction::Partial(_,_,_) => {
                    // This case is prevented by `resolve_labels()`
                    // above which ensures no partial instructions
                    // exist in the sequence.
                    unreachable!();
                }
                AssemblyInstruction::DataBytes(bytes) => {
                    match sections.last_mut() {
                        Some(Section::Data(section_bytes)) => {
                            section_bytes.extend(bytes);
                        }
                        _ => {
                            return Err(AssemblyError::InvalidDataSection);
                        }
                    }
                }
            }
        }
        //
        Ok(Bytecode::new(self.version,sections))
    }
}

// ============================================================================
// Traits
// ============================================================================

/// An iterator over the instructions making up an assembly.
pub type AssemblyIter<'a> = std::slice::Iter<'a,AssemblyInstruction>;

impl<'a> IntoIterator for &'a Assembly {
    type Item = &'a AssemblyInstruction;
    type IntoIter = AssemblyIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytecodes.iter()
    }
}

impl From<Bytecode> for Assembly {
    /// Construct an assembly from a bytecode structure.  The key
    /// challenge here lies in identifying labels, and converting full
    /// instructions into partial instructions.
    fn from(bytecode: Bytecode) -> Self {
        //
        let mut asm = Assembly::new(bytecode.version());
        //
        for section in &bytecode {
            match section {
                Section::Code{insns,inputs,outputs,max_stack} => {
                    // Mark start of code section
                    asm.push(AssemblyInstruction::CodeSection);
                    // Push all instructions
                    for insn in insns {
                        // FIXME: should be possible to drop this clone!
                        asm.push((*insn).clone());
                    }
                }
                Section::Data(bytes) => {
                    // Mark start of data section
                    asm.push(AssemblyInstruction::DataSection);
                    // Push data bytes
                    // FIXME: should be possible to drop this clone!
                    asm.push(AssemblyInstruction::DataBytes(bytes.clone()));
                }
            }
        }
        //
        asm
    }
}

// ============================================================================
// Assembly Instructions
// ============================================================================

/// Represents an instruction within an assembly which may be
/// incomplete (i.e. because it requires a label to be resolved before
/// it can be fully instantiated).  Furthermore, it may also be a
/// label to mark a given point in the sequence, or the start of a
/// section (code or data).
#[derive(Clone,Debug,PartialEq)]
pub enum AssemblyInstruction {
    /// Marks a position within the instruction sequence.
    Label(String),
    /// Indicates the start of a code section.
    CodeSection,
    /// Indicates the start of a data section.
    DataSection,
    /// Indicates a concrete instruction.
    Concrete(Instruction),
    /// Indicates an instruction parameterised by a given label.  The
    /// instruction can be instantiated using the provided generator
    /// once the concrete byteoffset of the label is known.  A partial
    /// instruction also requires a _minimum length_ to aid the offset
    /// resolution algorithm.
    Partial(usize,String,fn(ByteOffset)->Instruction),
    /// Indicates a sequence of zero or more _data bytes_.
    DataBytes(Vec<u8>)
}

impl From<Instruction> for AssemblyInstruction {
    fn from(insn: Instruction) -> Self {
        AssemblyInstruction::Concrete(insn)
    }
}

impl fmt::Display for AssemblyInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssemblyInstruction::Label(s) => write!(f,"{s}"),
            AssemblyInstruction::CodeSection => write!(f,".code"),
            AssemblyInstruction::DataSection => write!(f,".data"),
            AssemblyInstruction::Concrete(insn) => write!(f,"{insn}"),
            AssemblyInstruction::Partial(_,_,_) => write!(f,"???"),
            AssemblyInstruction::DataBytes(bytes) => write!(f,"{}",bytes.to_hex_string())
        }
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

// ===================================================================
// Label Resolution
// ===================================================================

/// Convert all `Partial` instructions into `Concrete` instructions by
/// first determining the byteoffset of each label, and then using
/// this to instantiate any partial instructions.  This can lead to an
/// error if there is a partial instruction which refers to a label
/// that does not exist; or, if a label is declared more than once.
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
fn resolve_labels(instructions: &mut [AssemblyInstruction]) -> Result<(),AssemblyError> {
    // Identify all labels contained within the sequence of assembly
    // instructions.  For each, we record their _instruction offset_.
    let mut labels = init_labels(instructions)?;
    // Construct initial set of empty offsets based on the minimum
    // length of each partial instruction;
    let mut offsets = init_offsets(instructions);
    // Iterate to a fixpoint.
    while update_offsets(instructions, &labels, &mut offsets) {
        // Keep going until no more changes!
    }
    // Instantiate any partial instructions
    for mut insn in instructions {
        insn_instantiate(&mut insn, &labels, &offsets);
    }
    Ok(())
}

/// Initialise the labels map which maps each label to its
/// _instruction offset_.  Note that this may differ from an
/// instruction's _byte offset_ (i.e. since not all instructions are
/// one byte long).  Finally, this also checks that every partial
/// instruction targets a known label.
fn init_labels(instructions: &[AssemblyInstruction]) -> Result<HashMap<String,usize>,AssemblyError> {
    let mut labels : HashMap<String, usize> = HashMap::new();
    // Compute labels
    for (i,b) in instructions.iter().enumerate() {
        match b {
            AssemblyInstruction::Label(lab) => {
                // NOTE: how to avoid this allocation?  It seems like
                // we should be able to use a `HashMap<&str,usize>`
                // here but I was unable to get it to work fully.
                labels.insert(lab.to_string(),i);
            }
            _ => {} // ignore
        }
    }
    // Sanity check partial instructions target known labels.
    for (i,b) in instructions.iter().enumerate() {
        match b {
            AssemblyInstruction::Partial(_,lab,_) => {
                if !labels.contains_key(lab) {
                    return Err(AssemblyError::UnknownLabel(lab.to_string()))
                }
            }
            _ => {} // ignore
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
fn init_offsets(instructions: &[AssemblyInstruction]) -> Vec<ByteOffset> {
    let mut offsets = Vec::new();
    let mut offset = 0;
    for (i,b) in instructions.iter().enumerate() {
        // Update instruction offset
        offsets.push(ByteOffset(offset as u16));
        // Calculate next offset
        offset = offset + insn_min_length(b);
    }
    offsets
}

/// Update the offset information, noting whether or not anything
/// actually changed.  The key is that as we recalculate offsets
/// we may find the width has changed.  If this happens, we have
/// to recalculate all offsets again assuming the larger width(s).
fn update_offsets(instructions: &[AssemblyInstruction], labels: &HashMap<String,usize>, offsets: &mut [ByteOffset]) -> bool {
    let mut changed = false;
    let mut offset = 0;
    // Calculate label offsets
    for (i,b) in instructions.iter().enumerate() {
        let old = offsets[i].0 as usize;
        // Update instruction offset
        offsets[i] = ByteOffset(offset as u16);
        // Determine whether this changed (or not)
        changed |= (offset != old);
        // Calculate next offset
        offset = offset + insn_length(b,labels,offsets);
    }
    //
    changed
}

/// Determine the _minimum length_ of an assembly instruction.
/// Observe that this may not be the same as the final length
/// determined for this instruction, but it provides us a safe
/// initial guess.
fn insn_min_length(insn: &AssemblyInstruction) -> usize {
    match insn {
        // Minimum length of concrete instruction is its actual
        // length!
        AssemblyInstruction::Concrete(insn) => insn.length(),
        // Minimum length of a partial instruction is the provided
        // minimum length.
        AssemblyInstruction::Partial(min_length,_,_) => *min_length,
        // Length of data bytes given by the bytes!
        AssemblyInstruction::DataBytes(bytes) => bytes.len(),
        // Everything else (e.g. labels) as no length
        AssemblyInstruction::CodeSection => 0,
        AssemblyInstruction::DataSection => 0,
        AssemblyInstruction::Label(_) => 0
    }
}

/// Determine the _actual length_ of an assembly instruction based on
/// the current estimate of all bytecode offsets.
fn insn_length(insn: &AssemblyInstruction, labels: &HashMap<String,usize>, offsets: &[ByteOffset]) -> usize {
    match insn {
        // Minimum length of concrete instruction is its actual
        // length!
        AssemblyInstruction::Concrete(insn) => insn.length(),
        // Minimum length of a partial instruction is the provided
        // minimum length.
        AssemblyInstruction::Partial(_,lab,insn_fn) => {
            // Get the instruction offset of the given label.
            let lab_insn_offset = labels.get(lab).unwrap();
            // Convert the instruction offset into an (estimated) byte
            // offset for the given label.
            let lab_byte_offset = offsets[*lab_insn_offset];
            // NOTE: we are determining the length of the instruction
            // here by instantiating it based on available
            // information.  That's actually suboptimal since it may
            // force memory allocation which is unnecessary.
            let insn = insn_fn(lab_byte_offset);
            // Finally, just return the instantiated instructions
            // length
            insn.length()
        }
        // Length of data bytes given by the bytes!
        AssemblyInstruction::DataBytes(bytes) => bytes.len(),
        // Everything else (e.g. labels) as no length
        AssemblyInstruction::CodeSection => 0,
        AssemblyInstruction::DataSection => 0,
        AssemblyInstruction::Label(_) => 0
    }
}

/// Instantiate an assembly instruction using the computed byte offset
/// for each label.  This only has an effect in the case of a partial
/// instruction.
fn insn_instantiate(insn: &mut AssemblyInstruction, labels: &HashMap<String,usize>, offsets: &[ByteOffset]) {
   match insn {
        // Minimum length of a partial instruction is the provided
        // minimum length.
        AssemblyInstruction::Partial(_,lab,insn_fn) => {
            // Get the instruction offset of the given label.
            let lab_insn_offset = labels.get(lab).unwrap();
            // Convert the instruction offset into its computed byte
            // offset for the given label.
            let lab_byte_offset = offsets[*lab_insn_offset];
            // Finally, instantiate the instruction
            *insn = AssemblyInstruction::Concrete(insn_fn(lab_byte_offset));
        }
        // No need to do anything for other instruction types
       _ => {}
    }
}
