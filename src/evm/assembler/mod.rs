// Responsible for code generation.
mod codegen;
// Support for parsing.
mod lexer;
// Responsible for parsing.
mod parser;

pub use parser::{parse,AssemblyError};
pub use codegen::{assemble,AssembleError};
