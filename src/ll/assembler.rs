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
    // Extract insn string (if any)
    let insn = &line[insn_start..insn_end];
    // Check whether this is data or not.
    if insn.starts_with("0x") {
        assert_eq!(arg_start,arg_end);
        let hex = insn.from_hex_string().unwrap();
        Ok(Instruction::DATA(hex))
    } else {
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
        "push"|"PUSH" => opcode::PUSH1 + (arg.unwrap() - 1),
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
            panic!("unknown instruction encountered: {insn}");
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
