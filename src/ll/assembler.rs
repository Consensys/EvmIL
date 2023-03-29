use std::fmt;
use std::collections::{HashMap};
use crate::evm::opcode;
use crate::ll::{Bytecode,Instruction};
use crate::util::FromHexString;

// ===================================================================
// Error
// ===================================================================

#[derive(Debug)]
pub enum AsmError {
    ExpectedOperand,
    InvalidHexString,
    InvalidInstruction,
    UnexpectedCharacter,
    UnexpectedToken
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
    input: &'a str,
    // Maps labels to identifiers
    labels: HashMap<String,usize>,
    // Bytecode package being constructed
    bytecode: Bytecode
}

impl<'a> Assembler<'a> {
    /// Construct a new parser from a given string slice.
    pub fn new(input: &'a str) -> Self {
        let labels = HashMap::new();
        let bytecode = Bytecode::new();
        //
        Assembler { input, labels, bytecode }
    }

    /// Parse file into a bytecode
    pub fn parse(mut self) -> Result<Bytecode,AsmError> {
        // Holds the set of lines being parsed.
        let lines : Vec<&str> = self.input.lines().collect();
        //
        for l in &lines {
            self.parse_line(l)?;
        }
        // Done
        Ok(self.bytecode)
    }

    fn parse_line(&mut self, line: &'a str) -> Result<(),AsmError> {
        let mut lexer = Lexer::new(line);
        //
        match lexer.next()? {
            Token::Hex(s) => {
                self.bytecode.push(Instruction::DATA(parse_hex(s)?))
            }
            Token::Identifier("push"|"PUSH") => {
                self.parse_push(lexer.next()?)?
            }
            Token::Identifier(id) => {
                self.bytecode.push(parse_opcode(id)?);
            }
            Token::Label(s) => {
                // Mark label in bytecode sequence
                self.bytecode.label(s);
            }
            _ => {
                // Something went wrong
                return Err(AsmError::UnexpectedToken);
            }
        };
        // Sanity check what's left
        match lexer.next()? {
            Token::EOF => Ok(()),
            _ => Err(AsmError::UnexpectedToken)
        }
    }

    /// Parse a push instruction with a given operand.
    fn parse_push(&mut self, operand: Token) -> Result<(),AsmError> {
        // Push always expects an argument, though it could be a
        // label or a hexadecimal operand.
        match operand {
            Token::Hex(s) => {
                self.bytecode.push(Instruction::PUSH(parse_hex(s)?));
                Ok(())
            }
            Token::Identifier(s) => {
                self.bytecode.push_partial(s,|target| Instruction::PUSH(target.to_bytes()));
                Ok(())
            },
            Token::EOF => Err(AsmError::ExpectedOperand),
            _ => Err(AsmError::UnexpectedToken)
        }
    }
}

// ===================================================================
// Lexer
// ===================================================================

enum Token<'a> {
    EOF,
    Hex(&'a str),
    Identifier(&'a str),
    Label(&'a str)
}

impl<'a> Token<'a> {
    // Return the "length" of a token.  That is, the number of
    // characters it represents.
    pub fn len(&self) -> usize {
        match self {
            Token::EOF => 0,
            Token::Hex(s) => s.len(),
            Token::Identifier(s) => s.len(),
            Token::Label(s) => s.len() + 1
        }
    }
}

/// A very simple lexer
struct Lexer<'a> {
    input: &'a str,
    chars: Vec<char>,
    index: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        // FIXME: this could be made more efficient by using an
        // iterator instead of allocating a new vector.
        let chars : Vec<char> = input.chars().collect();
        //
        Self{input, chars, index: 0}
    }

    pub fn lookahead(&self) -> Result<Token<'a>,AsmError> {
        // Skip any whitespace
        let start = skip(&self.chars, self.index, |c| c.is_ascii_whitespace());
        // Sanity check for end-of-file
        if start >= self.chars.len() {
            Ok(Token::EOF)
        } else {
            // Determine what kind of token we have.
            match self.chars[start] {
                '0'..='9' => self.scan_hex_literal(start),
                'a'..='z'|'A'..='Z'|'_' => self.scan_id_or_label(start),
                _ => Err(AsmError::UnexpectedCharacter)
            }
        }
    }

    pub fn next(&mut self) -> Result<Token<'a>,AsmError> {
        // Skip any whitespace
        self.index = skip(&self.chars, self.index, |c| c.is_ascii_whitespace());
        // Determine next token
        let tok = self.lookahead()?;
        // Account for next token
        self.index += tok.len();
        //
        Ok(tok)
    }

    fn scan_hex_literal(&self, start: usize) -> Result<Token<'a>,AsmError> {
        // Sanity check literal starts with "0x"
        if self.chars[start..].starts_with(&['0','x']) {
            // Scan all digits of this hex literal
            let end = skip(&self.chars,start + 2,|c| c.is_ascii_alphanumeric());
            // Construct token
            Ok(Token::Hex(&self.input[start..end]))
        } else {
            Err(AsmError::InvalidHexString)
        }
    }

    fn scan_id_or_label(&self, start: usize) -> Result<Token<'a>,AsmError> {
        // Scan all characters of this identifier or label
        let end = skip(&self.chars,start,|c| c.is_ascii_alphanumeric());
        // Distinguish label versus identifier.
        if end < self.chars.len() && self.chars[end] == ':' {
            Ok(Token::Label(&self.input[start..end]))
        } else {
            Ok(Token::Identifier(&self.input[start..end]))
        }
    }
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

// ===================================================================
// Helpers
// ===================================================================

/// Parse a hexadecimal string
fn parse_hex(hex: &str) -> Result<Vec<u8>,AsmError> {
    match hex.from_hex_string() {
        Ok(bytes) => { Ok(bytes) }
        Err(e) => Err(AsmError::InvalidHexString)
    }
}

/// Parse a given opcode from a string, and a given number of operand
/// bytes.
fn parse_opcode(insn: &str) -> Result<Instruction,AsmError> {
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
        "lt"|"LT" => opcode::LT,
        "gt"|"GT" => opcode::GT,
        "slt"|"SLT" => opcode::SLT,
        "sgt"|"SGT" => opcode::SGT,
        "eq"|"EQ" => opcode::EQ,
        "iszero"|"ISZERO" => opcode::ISZERO,
        "and"|"AND" => opcode::AND,
        "or"|"OR" => opcode::OR,
        "xor"|"XOR" => opcode::XOR,
        "not"|"NOT" => opcode::NOT,
        "byte"|"BYTE" => opcode::BYTE,
        "shl"|"SHL" => opcode::SHL,
        "shr"|"SHR" => opcode::SHR,
        "sar"|"SAR" => opcode::SAR,
        // 20s: Keccak256
        "keccak256"|"KECCAK256" => opcode::KECCAK256,
        // 30s: Environmental Information
        "address"|"ADDRESS" => opcode::ADDRESS,
        "balance"|"BALANCE" => opcode::BALANCE,
        "origin"|"ORIGIN" => opcode::ORIGIN,
        "caller"|"CALLER" => opcode::CALLER,
        "callvalue"|"CALLVALUE" => opcode::CALLVALUE,
        "calldataload"|"CALLDATALOAD" => opcode::CALLDATALOAD,
        "calldatasize"|"CALLDATASIZE" => opcode::CALLDATASIZE,
        "calldatacopy"|"CALLDATACOPY" => opcode::CALLDATACOPY,
        "codesize"|"CODESIZE" => opcode::CODESIZE,
        "codecopy"|"CODECOPY" => opcode::CODECOPY,
        "gasprice"|"GASPRICE" => opcode::GASPRICE,
        "extcodesize"|"EXTCODESIZE" => opcode::EXTCODESIZE,
        "extcodecopy"|"EXTCODECOPY" => opcode::EXTCODECOPY,
        "returndatasize"|"RETURNDATASIZE" => opcode::RETURNDATASIZE,
        "returndatacopy"|"RETURNDATACOPY" => opcode::RETURNDATACOPY,
        "extcodehash"|"EXTCODEHASH" => opcode::EXTCODEHASH,
        // 40s: Block Information
        "blockhash"|"BLOCKHASH" => opcode::BLOCKHASH,
        "coinbase"|"COINBASE" => opcode::COINBASE,
        "timestamp"|"TIMESTAMP" => opcode::TIMESTAMP,
        "number"|"NUMBER" => opcode::NUMBER,
        "difficulty"|"DIFFICULTY" => opcode::DIFFICULTY,
        "gaslimit"|"GASLIMIT" => opcode::GASLIMIT,
        "chainid"|"CHAINID" => opcode::CHAINID,
        "selfbalance"|"SELFBALANCE" => opcode::SELFBALANCE,
        // 50s: Stack, Memory, Storage and Flow Operations
        "pop"|"POP" => opcode::POP,
        "mload"|"MLOAD" => opcode::MLOAD,
        "mstore"|"MSTORE" => opcode::MSTORE,
        "mstore8"|"MSTORE8" => opcode::MSTORE8,
        "sload"|"SLOAD" => opcode::SLOAD,
        "sstore"|"SSTORE" => opcode::SSTORE,
        "jump"|"JUMP" => opcode::JUMP,
        "jumpi"|"JUMPI" => opcode::JUMPI,
        "pc"|"PC" => opcode::PC,
        "msize"|"MSIZE" => opcode::MSIZE,
        "gas"|"GAS" => opcode::GAS,
        "jumpdest"|"JUMPDEST" => opcode::JUMPDEST,
        // 60s & 70s: Push Operations
        "push"|"PUSH" => {
            // Should be impossible to get here!
            unreachable!();
        }
        // 80s: Duplication Operations
        "dup1"|"DUP1" => opcode::DUP1,
        "dup2"|"DUP2" => opcode::DUP2,
        "dup3"|"DUP3" => opcode::DUP3,
        "dup4"|"DUP4" => opcode::DUP4,
        "dup5"|"DUP5" => opcode::DUP5,
        "dup6"|"DUP6" => opcode::DUP6,
        "dup7"|"DUP7" => opcode::DUP7,
        "dup8"|"DUP8" => opcode::DUP8,
        "dup9"|"DUP9" => opcode::DUP9,
        "dup10"|"DUP10" => opcode::DUP10,
        "dup11"|"DUP11" => opcode::DUP11,
        "dup12"|"DUP12" => opcode::DUP12,
        "dup13"|"DUP13" => opcode::DUP13,
        "dup14"|"DUP14" => opcode::DUP14,
        "dup15"|"DUP15" => opcode::DUP15,
        "dup16"|"DUP16" => opcode::DUP16,
        // 90s: Swap Operations
        "swap1"|"SWAP1" => opcode::SWAP1,
        "swap2"|"SWAP2" => opcode::SWAP2,
        "swap3"|"SWAP3" => opcode::SWAP3,
        "swap4"|"SWAP4" => opcode::SWAP4,
        "swap5"|"SWAP5" => opcode::SWAP5,
        "swap6"|"SWAP6" => opcode::SWAP6,
        "swap7"|"SWAP7" => opcode::SWAP7,
        "swap8"|"SWAP8" => opcode::SWAP8,
        "swap9"|"SWAP9" => opcode::SWAP9,
        "swap10"|"SWAP10" => opcode::SWAP10,
        "swap11"|"SWAP11" => opcode::SWAP11,
        "swap12"|"SWAP12" => opcode::SWAP12,
        "swap13"|"SWAP13" => opcode::SWAP13,
        "swap14"|"SWAP14" => opcode::SWAP14,
        "swap15"|"SWAP15" => opcode::SWAP15,
        "swap16"|"SWAP16" => opcode::SWAP16,
        // a0s: Log Operations
        "log0"|"LOG0" => opcode::LOG0,
        "log1"|"LOG1" => opcode::LOG1,
        "log2"|"LOG2" => opcode::LOG2,
        "log3"|"LOG3" => opcode::LOG3,
        "log4"|"LOG4" => opcode::LOG4,
        // f0s: System Operations
        "create"|"CREATE" => opcode::CREATE,
        "call"|"CALL" => opcode::CALL,
        "callcode"|"CALLCODE" => opcode::CALLCODE,
        "return"|"RETURN" => opcode::RETURN,
        "delegatecall"|"DELEGATECALL" => opcode::DELEGATECALL,
        "create2"|"CREATE2" => opcode::CREATE2,
        "staticcall"|"STATICCALL" => opcode::STATICCALL,
        "revert"|"REVERT" => opcode::REVERT,
        "invalid"|"INVALID" => opcode::INVALID,
        "selfdestruct"|"SELFDESTRUCT" => opcode::SELFDESTRUCT,
        //
        _ => {
            return Err(AsmError::InvalidInstruction);
        }
    };
    //
    Ok(Instruction::decode(0, &[opcode]))
}
