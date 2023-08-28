// Responsible for code generation.
mod codegen;
// Defines AssemblyInstruction
mod instruction;
// Support for parsing.
mod lexer;
// Responsible for parsing.
mod parser;

pub use instruction::{AssemblyInstruction};
pub use parser::{AssemblyError};
pub use codegen::{AssembleError};

use crate::bytecode::{Bytecode,Section};
use crate::instruction::Instruction;

// ============================================================================
// Assembly
// ============================================================================

/// An assembly represents one or more sections contained assembly
/// instructions (that is, instructions which uses labels instead of
/// explicit jump targets).
pub type Assembly = Bytecode<AssemblyInstruction>;

/// An assembly section represents a section as found within an
/// `Assembly`.
pub type AssemblySection = Section<AssemblyInstruction>;

impl Assembly {
    /// Assemble an assembly into a `Bytecode` object containing
    /// concrete EVM instructions.  This requires resolving any labels
    /// contained within the assembly into known jump destinations.
    /// As such, this can fail if an instruction attempts to branch to
    /// a label which does not exist.
    pub fn assemble(&self) -> Result<Bytecode<Instruction>,AssembleError> {
        let mut sections = Vec::new();
        // Map each assemply section to a compiled section.
        for s in self {
            match s {
                Section::Code(insns) => {
                    let ninsns = codegen::assemble(insns)?;
                    sections.push(Section::Code(ninsns));
                }
                Section::Data(bytes) => {
                    sections.push(Section::Data(bytes.clone()));
                }
            }
        }
        // Done
        Ok(Bytecode::new(sections))
    }

    /// Parse some assembly language into an `Assembly`.  This can
    /// fail for a variety of reasons, such as an unknown instruction
    /// is used or there is some unexpected junk in the file.
    pub fn from_str(input: &str) -> Result<Assembly,AssemblyError> {
        parser::parse(input)
    }
}
