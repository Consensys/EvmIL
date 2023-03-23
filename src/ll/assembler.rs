use std::fmt;
use crate::evm::opcode;
use crate::ll::{Bytecode,Instruction};
use crate::util::FromHexString;

// ===================================================================
// Error
// ===================================================================

#[derive(Debug)]
pub enum AsmError {
    INVALID
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AsmError {

}

// ===================================================================
// Assembler
// ===================================================================

pub struct Assembler<'a> {
    // Holds the set of lines being parsed.
    lines: Vec<&'a str>
}

impl<'a> Assembler<'a> {
    /// Construct a new parser from a given string slice.
    pub fn new(input: &'a str) -> Self {
        // Split lines up
        let lines = input.lines().collect();
        //
        Assembler { lines }
    }

    /// Determine how many lines there are.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Parse file into a bytecode
    pub fn parse(&self) -> Result<Bytecode,AsmError> {
        let mut code = Bytecode::new();
        //
        for l in &self.lines {
            let insn = parse_line(l)?;
            code.push(insn);
        }
        // Done
        Ok(code)
    }

    // ===============================================================
    // Internal
    // ===============================================================
}

fn parse_line(line: &str) -> Result<Instruction,AsmError> {
    let insn : Instruction;
    // Convert string slice into Vec<char>
    let chars : Vec<char> = line.chars().collect();
    // Skip any leading whitespace
    let insn_start = skip(&chars,0, |c| c.is_ascii_whitespace());
    // Parse the instruction
    let insn_end = skip(&chars,insn_start, |c| c.is_ascii_alphanumeric());
    // Skip any further whitespace
    let arg_start = skip(&chars,insn_end, |c| c.is_ascii_whitespace());
    // Skip over argument
    let arg_end = skip(&chars,arg_start, |c| c.is_ascii_alphanumeric());
    // Sanity check any remaining characters
    if arg_end != chars.len() {
        panic!("unknown trailing garbage");
    }
    // Parse argument (if present)
    let arg = parse_hex_string(&line[arg_start..arg_end])?;
    let arg_len = arg.clone().map(|v| v.len() as u8);
    // Parse the various bits
    let opcode = parse_opcode(&line[insn_start..insn_end], arg_len)?;
    // Pretty much done!
    let insn = match arg {
        None => {
            assert!(!requires_argument(opcode));
            Instruction::decode(0,&[opcode])
        }
        Some(mut bytes) => {
            assert!(requires_argument(opcode));
            bytes.insert(0,opcode);
            Instruction::decode(0,&bytes)
        }
    };
    Ok(insn)
}

/// Skip over any characters matching a given predicate.
fn skip<P>(input: &[char], index: usize, pred: P) -> usize
where P: Fn(char) -> bool {
    let mut i = index;
    // Continue matching
    while i < input.len() && pred(input[i]) {
        i = i + 1;
    }
    // Done
    i
}

/// Parse a given opcode from a string.
fn parse_opcode(insn: &str, arg: Option<u8>) -> Result<u8,AsmError> {
    let opcode = match insn {
        // 0s: Stop and Arithmetic Operations
        "stop"|"STOP" => opcode::STOP,
        "add"|"ADD" => opcode::ADD,
        "mul"|"MUL" => opcode::MUL,
        "sub"|"SUB" => opcode::SUB,
        "div"|"DIV" => opcode::DIV,
        "sdiv"|"SDIV" => opcode::SDIV,
        "mod"|"MOD" => opcode::MOD,
        "smod"|"SMOD" => opcode::SMOD,
        "addmod"|"ADDMOD" => opcode::ADDMOD,
        "mulmod"|"MULMOD" => opcode::MULMOD,
        "exp"|"EXP" => opcode::EXP,
        "signextend"|"SIGNEXTEND" => opcode::SIGNEXTEND,
        // 10s: Comparison & Bitwise Logic Operations
        // 20s: Keccak256
        // 30s: Environmental Information
        // 40s: Block Information
        // 50s: Stack, Memory, Storage and Flow Operations
        // 60s & 70s: Push Operations
        "push"|"PUSH" => opcode::PUSH1 + (arg.unwrap() - 1),
        // 80s: Duplication Operations
        // 90s: Swap Operations
        // a0s: Log Operations
        // f0s: System Operations
        //
        _ => {
            todo!()
        }
    };
    Ok(opcode)
}

/// Parse a hexadecimal string
fn parse_hex_string(hex: &str) -> Result<Option<Vec<u8>>,AsmError> {
    if hex == "" {
        Ok(None)
    } else {
        match hex.from_hex_string() {
            Ok(bytes) => { Ok(Some(bytes)) }
            Err(e) => Err(AsmError::INVALID)
        }
    }
}

/// Check whether a given argument requires an operand or not.
fn requires_argument(opcode: u8) -> bool {
    opcode >= 0x60 && opcode <= 0x7f
}
