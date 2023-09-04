// Responsible for code generation.
mod codegen;
// Defines AssemblyInstruction
mod instruction;
// Support for parsing.
mod lexer;
// Responsible for parsing.
mod parser;

pub use instruction::{AssemblyInstruction};

use std::fmt;
use crate::bytecode::{StructuredContract,Instruction,StructuredSection};

// ============================================================================
// Errors
// ============================================================================

/// Errors which can arise when parsing assembly language and/or
/// assembling it.
#[derive(Debug)]
pub enum AssemblyError {
    /// When parsing some assembly language, mnemonic was encountered
    /// that requires an operand (e.g. `push`) but none was found.
    ExpectedOperand,
    /// When parsing some assembly language, an invalid comment was
    /// encountered.
    InvalidComment(usize),
    /// When parsing some assembly language, an invalid hex literal
    /// was encountered.
    InvalidHexString(usize),
    /// When parsing some assembly language, an unexpected mnemonic
    /// was encountered.
    InvalidInstruction,
    /// When parsing some assembly language, an unexpected character
    /// was encountered.
    UnexpectedCharacter(usize),
    /// When parsing some assembly language, an unexpected token was
    /// encountered.
    UnexpectedToken,
    /// When assembling a given assembly, a labelled instruction was
    /// encountered that targets a non-existent label.
    UnknownLabel(String),
    /// When assembling a given assembly, the distance of a calculated
    /// relative offset was found to exceed 16bits.
    InvalidRelativeOffset,
    /// When assembling a given assembly, the distance of a calculated
    /// offset exceeds the maximum permitted code size.
    OffsetTooLarge
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AssemblyError {

}

// ============================================================================
// Assembly
// ============================================================================

/// An assembly represents a contract containing sections of assembly
/// language instructions (that is, instructions which uses labels
/// instead of explicit jump targets).  The intuition is that an
/// _assembly_ can be _assembled_ into a bytecode contract.
pub type Assembly = StructuredContract<AssemblyInstruction>;

/// An assembly section represents a section as found within an
/// `Assembly`.
pub type AssemblySection = StructuredSection<AssemblyInstruction>;

impl Assembly {
    /// Assemble an assembly into a contract containing concrete EVM
    /// instructions.  This requires resolving any labels contained
    /// within the assembly into known jump destinations.  As such,
    /// this can fail if an instruction attempts to branch to a label
    /// which does not exist.
    pub fn assemble(&self) -> Result<StructuredContract<Instruction>,AssemblyError> {
        let mut sections = Vec::new();
        // Map each assemply section to a compiled section.
        for s in self {
            match s {
                StructuredSection::Code(insns) => {
                    let ninsns = codegen::assemble(insns)?;
                    sections.push(StructuredSection::Code(ninsns));
                }
                StructuredSection::Data(bytes) => {
                    sections.push(StructuredSection::Data(bytes.clone()));
                }
            }
        }
        // Done
        Ok(StructuredContract::new(sections))
    }

    /// Parse some assembly language into an `Assembly`.  This can
    /// fail for a variety of reasons, such as an unknown instruction
    /// is used or there is some unexpected junk in the file.
    pub fn from_str(input: &str) -> Result<Assembly,AssemblyError> {
        parser::parse(input)
    }
}
