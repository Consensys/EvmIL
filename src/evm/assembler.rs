use std::fmt;
use std::collections::{HashMap};
use crate::evm::{Bytecode,LabelledInstruction,Instruction,Section};
use crate::util::{FromHexString,ToHexString};

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

pub struct Assembly {
    bytecode: Bytecode<LabelledInstruction>
}

impl Assembly {
    /// Construct a new parser from a given string slice.
    pub fn new() -> Self {
        let bytecode = Bytecode::new(Vec::new());
        //
        Assembly { bytecode }
    }

    pub fn to_assembly(self) -> Bytecode<LabelledInstruction> {
        self.bytecode
    }

    /// Parse assembly language to form an assembly
    pub fn from_str(input: &str) -> Result<Bytecode<LabelledInstruction>,AssemblyLanguageError> {
        // Holds the set of lines being parsed.
        let lines : Vec<&str> = input.lines().collect();
        let mut parser = Assembly::new();
        //
        for l in &lines {
            //parser.parse(l)?;
            todo!();
        }
        // Done
        Ok(parser.to_assembly())
    }

    // /// Parse a single line of assembly language.
    // pub fn parse(&mut self, line: &str) -> Result<(),AssemblyLanguageError> {
    //     let mut lexer = Lexer::new(line);
    //     //
    //     match lexer.next()? {
    //         Token::Section("code") => {
    //             self.bytecode.push(self.parse_code_section(&mut lexer)?);
    //         }
    //         // Token::Section("data") => {
    //         //     self.bytecode.push(self.parse_data_section()?);
    //         // }
    //         _ => {
    //             // Something went wrong
    //             return Err(AssemblyLanguageError::UnexpectedToken);
    //         }
    //     };
    //     // Sanity check what's left
    //     match lexer.next()? {
    //         Token::EOF => Ok(()),
    //         _ => Err(AssemblyLanguageError::UnexpectedToken)
    //     }
    // }

    fn parse_code_section(&mut self, lexer: &mut Lexer) -> Result<Section<LabelledInstruction>,AssemblyLanguageError> {
        let mut insns = Vec::new();
        //
        match lexer.next()? {
            // Token::Identifier("push"|"PUSH") => {
            //     self.parse_push(lexer.next()?)?
            // }
            // Token::Identifier("rjump"|"RJUMP") => {
            //     self.parse_rjump(lexer.next()?)?
            // }
            // Token::Identifier("rjumpi"|"RJUMPI") => {
            //     self.parse_rjumpi(lexer.next()?)?
            // }
            Token::Identifier(id) => {
                insns.push(parse_opcode(id)?);
            }
            Token::Label(s) => {
                // Mark label in bytecode sequence
                insns.push(LabelledInstruction::LABEL(s.to_string()));
            }
            _ => {
                // Something went wrong
                return Err(AssemblyLanguageError::UnexpectedToken);
            }
        };
        //
        // FIXME: zeros do not make sense here.
        Ok(Section::Code(insns,0,0,0))
    }

    // fn parse_data_section(&mut self, lexer: &mut Lexer) -> Section<LabelledInstruction> {
    //     let bytes = Vec::new();
    //     match lexer.next()? {
    //         Token::Hex(s) => {
    //             self.push(parse_hex(s)?)
    //         }
    //         _ => {
    //             // Something went wrong
    //             return Err(AssemblyLanguageError::UnexpectedToken);
    //         }
    //     };
    // }

    // /// Parse a push instruction with a given operand.
    // fn parse_push(&mut self, operand: Token) -> Result<(),AssemblyLanguageError> {
    //     // Push always expects an argument, though it could be a
    //     // label or a hexadecimal operand.
    //     match operand {
    //         Token::Hex(s) => {
    //             self.bytecode.push(Instruction::PUSH(parse_hex(s)?));
    //             Ok(())
    //         }
    //         Token::Identifier(s) => {
    //             // This indicates we have an incomplete push
    //             // instruction which requires a label to be resolved
    //             // before it can be fully instantiated.
    //             let insn = AssemblyInstruction::Partial(2,s.to_string(),|_,lab| Instruction::PUSH(lab.to_bytes()));
    //             self.bytecode.push(insn);
    //             Ok(())
    //         },
    //         Token::EOF => Err(AssemblyLanguageError::ExpectedOperand),
    //         _ => Err(AssemblyLanguageError::UnexpectedToken)
    //     }
    // }

    // /// Parse a rjump instruction with a given operand label.
    // fn parse_rjump(&mut self, operand: Token) -> Result<(),AssemblyLanguageError> {
    //     match operand {
    //         Token::Identifier(s) => {
    //             let insn = AssemblyInstruction::Partial(3,s.to_string(),|pc,lab| Instruction::RJUMP(lab.relative_to(pc+3)));
    //             self.bytecode.push(insn);
    //             Ok(())
    //         },
    //         Token::EOF => Err(AssemblyLanguageError::ExpectedOperand),
    //         _ => Err(AssemblyLanguageError::UnexpectedToken)
    //     }
    // }

    // /// Parse a rjumpi instruction with a given operand label.
    // fn parse_rjumpi(&mut self, operand: Token) -> Result<(),AssemblyLanguageError> {
    //     match operand {
    //         Token::Identifier(s) => {
    //             let insn = AssemblyInstruction::Partial(3,s.to_string(),|pc,lab| Instruction::RJUMPI(lab.relative_to(pc+3)));
    //             self.bytecode.push(insn);
    //             Ok(())
    //         },
    //         Token::EOF => Err(AssemblyLanguageError::ExpectedOperand),
    //         _ => Err(AssemblyLanguageError::UnexpectedToken)
    //     }
    // }
}


// ===================================================================
// Helpers
// ===================================================================

/// Parse a hexadecimal string
fn parse_hex(hex: &str) -> Result<Vec<u8>,AssemblyLanguageError> {
    match hex.from_hex_string() {
        Ok(bytes) => { Ok(bytes) }
        Err(_e) => Err(AssemblyLanguageError::InvalidHexString(0))
    }
}

/// Parse a given opcode from a string, and a given number of operand
/// bytes.
fn parse_opcode(insn: &str) -> Result<LabelledInstruction,AssemblyLanguageError> {
    let insn = match insn {
        // 0s: Stop and Arithmetic Operations
        "stop"|"STOP" => LabelledInstruction::STOP,
        "add"|"ADD" => LabelledInstruction::ADD,
        "mul"|"MUL" => LabelledInstruction::MUL,
        "sub"|"SUB" => LabelledInstruction::SUB,
        "div"|"DIV" => LabelledInstruction::DIV,
        "sdiv"|"SDIV" => LabelledInstruction::SDIV,
        "mod"|"MOD" => LabelledInstruction::MOD,
        "smod"|"SMOD" => LabelledInstruction::SMOD,
        "addmod"|"ADDMOD" => LabelledInstruction::ADDMOD,
        "mulmod"|"MULMOD" => LabelledInstruction::MULMOD,
        "exp"|"EXP" => LabelledInstruction::EXP,
        "signextend"|"SIGNEXTEND" => LabelledInstruction::SIGNEXTEND,
        // 10s: Comparison & Bitwise Logic Operations
        "lt"|"LT" => LabelledInstruction::LT,
        "gt"|"GT" => LabelledInstruction::GT,
        "slt"|"SLT" => LabelledInstruction::SLT,
        "sgt"|"SGT" => LabelledInstruction::SGT,
        "eq"|"EQ" => LabelledInstruction::EQ,
        "iszero"|"ISZERO" => LabelledInstruction::ISZERO,
        "and"|"AND" => LabelledInstruction::AND,
        "or"|"OR" => LabelledInstruction::OR,
        "xor"|"XOR" => LabelledInstruction::XOR,
        "not"|"NOT" => LabelledInstruction::NOT,
        "byte"|"BYTE" => LabelledInstruction::BYTE,
        "shl"|"SHL" => LabelledInstruction::SHL,
        "shr"|"SHR" => LabelledInstruction::SHR,
        "sar"|"SAR" => LabelledInstruction::SAR,
        // 20s: Keccak256
        "keccak256"|"KECCAK256" => LabelledInstruction::KECCAK256,
        // 30s: Environmental Information
        "address"|"ADDRESS" => LabelledInstruction::ADDRESS,
        "balance"|"BALANCE" => LabelledInstruction::BALANCE,
        "origin"|"ORIGIN" => LabelledInstruction::ORIGIN,
        "caller"|"CALLER" => LabelledInstruction::CALLER,
        "callvalue"|"CALLVALUE" => LabelledInstruction::CALLVALUE,
        "calldataload"|"CALLDATALOAD" => LabelledInstruction::CALLDATALOAD,
        "calldatasize"|"CALLDATASIZE" => LabelledInstruction::CALLDATASIZE,
        "calldatacopy"|"CALLDATACOPY" => LabelledInstruction::CALLDATACOPY,
        "codesize"|"CODESIZE" => LabelledInstruction::CODESIZE,
        "codecopy"|"CODECOPY" => LabelledInstruction::CODECOPY,
        "gasprice"|"GASPRICE" => LabelledInstruction::GASPRICE,
        "extcodesize"|"EXTCODESIZE" => LabelledInstruction::EXTCODESIZE,
        "extcodecopy"|"EXTCODECOPY" => LabelledInstruction::EXTCODECOPY,
        "returndatasize"|"RETURNDATASIZE" => LabelledInstruction::RETURNDATASIZE,
        "returndatacopy"|"RETURNDATACOPY" => LabelledInstruction::RETURNDATACOPY,
        "extcodehash"|"EXTCODEHASH" => LabelledInstruction::EXTCODEHASH,
        // 40s: Block Information
        "blockhash"|"BLOCKHASH" => LabelledInstruction::BLOCKHASH,
        "coinbase"|"COINBASE" => LabelledInstruction::COINBASE,
        "timestamp"|"TIMESTAMP" => LabelledInstruction::TIMESTAMP,
        "number"|"NUMBER" => LabelledInstruction::NUMBER,
        "difficulty"|"DIFFICULTY" => LabelledInstruction::DIFFICULTY,
        "gaslimit"|"GASLIMIT" => LabelledInstruction::GASLIMIT,
        "chainid"|"CHAINID" => LabelledInstruction::CHAINID,
        "selfbalance"|"SELFBALANCE" => LabelledInstruction::SELFBALANCE,
        // 50s: Stack, Memory, Storage and Flow Operations
        "pop"|"POP" => LabelledInstruction::POP,
        "mload"|"MLOAD" => LabelledInstruction::MLOAD,
        "mstore"|"MSTORE" => LabelledInstruction::MSTORE,
        "mstore8"|"MSTORE8" => LabelledInstruction::MSTORE8,
        "sload"|"SLOAD" => LabelledInstruction::SLOAD,
        "sstore"|"SSTORE" => LabelledInstruction::SSTORE,
        "jump"|"JUMP" => LabelledInstruction::JUMP,
        "jumpi"|"JUMPI" => LabelledInstruction::JUMPI,
        "pc"|"PC" => LabelledInstruction::PC,
        "msize"|"MSIZE" => LabelledInstruction::MSIZE,
        "gas"|"GAS" => LabelledInstruction::GAS,
        "jumpdest"|"JUMPDEST" => LabelledInstruction::JUMPDEST,
        // 60s & 70s: Push Operations
        "push"|"PUSH" => {
            // Should be impossible to get here!
            unreachable!();
        }
        // 80s: Duplication Operations
        "dup1"|"DUP1" => LabelledInstruction::DUP(1),
        "dup2"|"DUP2" => LabelledInstruction::DUP(2),
        "dup3"|"DUP3" => LabelledInstruction::DUP(3),
        "dup4"|"DUP4" => LabelledInstruction::DUP(4),
        "dup5"|"DUP5" => LabelledInstruction::DUP(5),
        "dup6"|"DUP6" => LabelledInstruction::DUP(6),
        "dup7"|"DUP7" => LabelledInstruction::DUP(7),
        "dup8"|"DUP8" => LabelledInstruction::DUP(8),
        "dup9"|"DUP9" => LabelledInstruction::DUP(9),
        "dup10"|"DUP10" => LabelledInstruction::DUP(10),
        "dup11"|"DUP11" => LabelledInstruction::DUP(11),
        "dup12"|"DUP12" => LabelledInstruction::DUP(12),
        "dup13"|"DUP13" => LabelledInstruction::DUP(13),
        "dup14"|"DUP14" => LabelledInstruction::DUP(14),
        "dup15"|"DUP15" => LabelledInstruction::DUP(15),
        "dup16"|"DUP16" => LabelledInstruction::DUP(16),
        // 90s: Swap Operations
        "swap1"|"SWAP1" => LabelledInstruction::SWAP(1),
        "swap2"|"SWAP2" => LabelledInstruction::SWAP(2),
        "swap3"|"SWAP3" => LabelledInstruction::SWAP(3),
        "swap4"|"SWAP4" => LabelledInstruction::SWAP(4),
        "swap5"|"SWAP5" => LabelledInstruction::SWAP(5),
        "swap6"|"SWAP6" => LabelledInstruction::SWAP(6),
        "swap7"|"SWAP7" => LabelledInstruction::SWAP(7),
        "swap8"|"SWAP8" => LabelledInstruction::SWAP(8),
        "swap9"|"SWAP9" => LabelledInstruction::SWAP(9),
        "swap10"|"SWAP10" => LabelledInstruction::SWAP(10),
        "swap11"|"SWAP11" => LabelledInstruction::SWAP(11),
        "swap12"|"SWAP12" => LabelledInstruction::SWAP(12),
        "swap13"|"SWAP13" => LabelledInstruction::SWAP(13),
        "swap14"|"SWAP14" => LabelledInstruction::SWAP(14),
        "swap15"|"SWAP15" => LabelledInstruction::SWAP(15),
        "swap16"|"SWAP16" => LabelledInstruction::SWAP(16),
        // a0s: Log Operations
        "log0"|"LOG0" => LabelledInstruction::LOG(0),
        "log1"|"LOG1" => LabelledInstruction::LOG(1),
        "log2"|"LOG2" => LabelledInstruction::LOG(2),
        "log3"|"LOG3" => LabelledInstruction::LOG(3),
        "log4"|"LOG4" => LabelledInstruction::LOG(4),
        // f0s: System Operations
        "create"|"CREATE" => LabelledInstruction::CREATE,
        "call"|"CALL" => LabelledInstruction::CALL,
        "callcode"|"CALLCODE" => LabelledInstruction::CALLCODE,
        "return"|"RETURN" => LabelledInstruction::RETURN,
        "delegatecall"|"DELEGATECALL" => LabelledInstruction::DELEGATECALL,
        "create2"|"CREATE2" => LabelledInstruction::CREATE2,
        "staticcall"|"STATICCALL" => LabelledInstruction::STATICCALL,
        "revert"|"REVERT" => LabelledInstruction::REVERT,
        "invalid"|"INVALID" => LabelledInstruction::INVALID,
        "selfdestruct"|"SELFDESTRUCT" => LabelledInstruction::SELFDESTRUCT,
        //
        _ => {
            return Err(AssemblyLanguageError::InvalidInstruction);
        }
    };
    //
    Ok(insn)
}

// ===================================================================
// Token
// ===================================================================

pub enum Token<'a> {
    EOF, // End-Of-File (not EVM Object Format)
    Section(&'a str),
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
            Token::Section(s) => s.len() + 1,
            Token::Hex(s) => s.len(),
            Token::Identifier(s) => s.len(),
            Token::Label(s) => s.len() + 1
        }
    }
}

// ===================================================================
// Lexer
// ===================================================================

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

    pub fn lookahead(&self) -> Result<Token<'a>,AssemblyLanguageError> {
        // Skip any whitespace
        let start = skip(&self.chars, self.index, |c| c.is_ascii_whitespace());
        // Sanity check for end-of-file
        if start >= self.chars.len() {
            Ok(Token::EOF)
        } else {
            // Determine what kind of token we have.
            match self.chars[start] {
                '.' => self.scan_section_header(start),
                '0'..='9' => self.scan_hex_literal(start),
                'a'..='z'|'A'..='Z'|'_' => self.scan_id_or_label(start),
                _ => Err(AssemblyLanguageError::UnexpectedCharacter(start))
            }
        }
    }

    pub fn next(&mut self) -> Result<Token<'a>,AssemblyLanguageError> {
        // Skip any whitespace
        self.index = skip(&self.chars, self.index, |c| c.is_ascii_whitespace());
        // Determine next token
        let tok = self.lookahead()?;
        // Account for next token
        self.index += tok.len();
        //
        Ok(tok)
    }

    fn scan_hex_literal(&self, start: usize) -> Result<Token<'a>,AssemblyLanguageError> {
        // Sanity check literal starts with "0x"
        if self.chars[start..].starts_with(&['0','x']) {
            // Scan all digits of this hex literal
            let end = skip(&self.chars,start + 2,|c| c.is_ascii_alphanumeric());
            // Construct token
            Ok(Token::Hex(&self.input[start..end]))
        } else {
            Err(AssemblyLanguageError::InvalidHexString(start))
        }
    }

    fn scan_id_or_label(&self, start: usize) -> Result<Token<'a>,AssemblyLanguageError> {
        // Scan all characters of this identifier or label
        let end = skip(&self.chars,start,|c| c.is_ascii_alphanumeric());
        // Distinguish label versus identifier.
        if end < self.chars.len() && self.chars[end] == ':' {
            Ok(Token::Label(&self.input[start..end]))
        } else {
            Ok(Token::Identifier(&self.input[start..end]))
        }
    }

    fn scan_section_header(&self, mut start: usize) -> Result<Token<'a>,AssemblyLanguageError> {
        // Move passed "."
        start = start + 1;
        // Scan all characters of this identifier or label
        let end = skip(&self.chars,start,|c| c.is_ascii_alphabetic());
        // Done
        Ok(Token::Section(&self.input[start..end]))
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
