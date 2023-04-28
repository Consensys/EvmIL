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
use std::fmt;
use crate::evm::AbstractInstruction::*;
use crate::evm::{Assembly,AssemblyInstruction,Section};
use crate::util::{FromHexString};
use super::lexer::{Lexer,Token};

// ===================================================================
// Parse Error
// ===================================================================

/// Indicates an error occurred whilst parsing some assembly language
/// into an assembly (i.e. an error originating from the lexer or
/// parser).
#[derive(Debug)]
pub enum AssemblyError {
    ExpectedOperand,
    InvalidHexString(usize),
    InvalidInstruction,
    UnexpectedCharacter(usize),
    UnexpectedToken
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AssemblyError {

}

// ===================================================================
// Parser
// ===================================================================

/// Parse assembly language to form an assembly
pub fn parse(input: &str) -> Result<Assembly,AssemblyError> {
    // Holds the set of lines being parsed.
    let mut lexer = Lexer::new(input);
    let mut sections = Vec::new();
    // Keep going until we reach the end.
    while lexer.lookahead()? != Token::EOF {
        sections.push(parse_section(&mut lexer)?);
    }
    // Done
    Ok(Assembly::new(sections))
}

/// Parse a single line of assembly language.
fn parse_section(lexer: &mut Lexer) -> Result<Section<AssemblyInstruction>,AssemblyError> {
    //
    match lexer.next()? {
        Token::Section("code") => {
            parse_code_section(lexer)
        }
        Token::Section("data") => {
            parse_data_section(lexer)
        }
        _ => {
            // Something went wrong
            Err(AssemblyError::UnexpectedToken)
        }
    }
}

fn parse_code_section(lexer: &mut Lexer) -> Result<Section<AssemblyInstruction>,AssemblyError> {
    let mut insns = Vec::new();
    //
    loop {
        match lexer.lookahead()? {
            Token::Identifier("push"|"PUSH") => {
                _ = lexer.next();
                insns.push(parse_push(false,lexer.next()?)?)
            }
            Token::Identifier("pushl"|"PUSHL") => {
                _ = lexer.next();
                insns.push(parse_push(true,lexer.next()?)?)
            }
            Token::Identifier("rjump"|"RJUMP") => {
                _ = lexer.next();
                insns.push(parse_rjump(lexer.next()?)?);
            }
            Token::Identifier("rjumpi"|"RJUMPI") => {
                _ = lexer.next();
                insns.push(parse_rjumpi(lexer.next()?)?);
            }
            Token::Identifier("db"|"DB") => {
                _ = lexer.next();
                insns.push(parse_data(lexer.next()?)?);
            }
            Token::Identifier(id) => {
                _ = lexer.next();
                insns.push(parse_opcode(id)?);
            }
            Token::Label(s) => {
                _ = lexer.next();
                // Mark label in bytecode sequence
                insns.push(LABEL(s.to_string()));
            }
            Token::EOF|Token::Section(_) => {
                return Ok(Section::Code(insns));
            }
            _ => {
                // Something went wrong
                return Err(AssemblyError::UnexpectedToken);
            }
        };
    }
    //
    // FIXME: zeros do not make sense here.

}

fn parse_data_section(lexer: &mut Lexer) -> Result<Section<AssemblyInstruction>,AssemblyError> {
    let mut bytes = Vec::new();
    loop {
        match lexer.lookahead()? {
            Token::Hex(s) => {
                _ = lexer.next();
                bytes.extend(parse_hex(s)?)
            }
            Token::EOF|Token::Section(_) => {
                return Ok(Section::Data(bytes));
            }
            _ => {
                // Something went wrong
                return Err(AssemblyError::UnexpectedToken);
            }
        }
    };
}

/// Parse a push instruction with a given operand.
fn parse_push(large: bool, operand: Token) -> Result<AssemblyInstruction,AssemblyError> {
    // Push always expects an argument, though it could be a
    // label or a hexadecimal operand.
    match operand {
        Token::Hex(s) => Ok(PUSH(parse_hex(s)?)),
        Token::Identifier(s) => Ok(PUSHL(large,s.to_string())),
        Token::EOF => Err(AssemblyError::ExpectedOperand),
        _ => Err(AssemblyError::UnexpectedToken)
    }
}

/// Parse a rjump instruction with a given operand label.
fn parse_rjump(operand: Token) -> Result<AssemblyInstruction,AssemblyError> {
    match operand {
        Token::Identifier(s) => Ok(RJUMP(s.to_string())),
        Token::EOF => Err(AssemblyError::ExpectedOperand),
        _ => Err(AssemblyError::UnexpectedToken)
    }
}

/// Parse a rjumpi instruction with a given operand label.
fn parse_rjumpi(operand: Token) -> Result<AssemblyInstruction,AssemblyError> {
    match operand {
        Token::Identifier(s) => Ok(RJUMPI(s.to_string())),
        Token::EOF => Err(AssemblyError::ExpectedOperand),
        _ => Err(AssemblyError::UnexpectedToken)
    }
}

fn parse_data(operand: Token) -> Result<AssemblyInstruction,AssemblyError> {
    match operand {
        Token::Hex(s) => Ok(DATA(parse_hex(s)?)),
        Token::EOF => Err(AssemblyError::ExpectedOperand),
        _ => Err(AssemblyError::UnexpectedToken)
    }
}

// ===================================================================
// Helpers
// ===================================================================

/// Parse a hexadecimal string
fn parse_hex(hex: &str) -> Result<Vec<u8>,AssemblyError> {
    match hex.from_hex_string() {
        Ok(bytes) => { Ok(bytes) }
        Err(_e) => Err(AssemblyError::InvalidHexString(0))
    }
}

/// Parse a given opcode from a string, and a given number of operand
/// bytes.
fn parse_opcode(insn: &str) -> Result<AssemblyInstruction,AssemblyError> {
    let insn = match insn {
        // 0s: Stop and Arithmetic Operations
        "stop"|"STOP" => STOP,
        "add"|"ADD" => ADD,
        "mul"|"MUL" => MUL,
        "sub"|"SUB" => SUB,
        "div"|"DIV" => DIV,
        "sdiv"|"SDIV" => SDIV,
        "mod"|"MOD" => MOD,
        "smod"|"SMOD" => SMOD,
        "addmod"|"ADDMOD" => ADDMOD,
        "mulmod"|"MULMOD" => MULMOD,
        "exp"|"EXP" => EXP,
        "signextend"|"SIGNEXTEND" => SIGNEXTEND,
        // 10s: Comparison & Bitwise Logic Operations
        "lt"|"LT" => LT,
        "gt"|"GT" => GT,
        "slt"|"SLT" => SLT,
        "sgt"|"SGT" => SGT,
        "eq"|"EQ" => EQ,
        "iszero"|"ISZERO" => ISZERO,
        "and"|"AND" => AND,
        "or"|"OR" => OR,
        "xor"|"XOR" => XOR,
        "not"|"NOT" => NOT,
        "byte"|"BYTE" => BYTE,
        "shl"|"SHL" => SHL,
        "shr"|"SHR" => SHR,
        "sar"|"SAR" => SAR,
        // 20s: Keccak256
        "keccak256"|"KECCAK256" => KECCAK256,
        // 30s: Environmental Information
        "address"|"ADDRESS" => ADDRESS,
        "balance"|"BALANCE" => BALANCE,
        "origin"|"ORIGIN" => ORIGIN,
        "caller"|"CALLER" => CALLER,
        "callvalue"|"CALLVALUE" => CALLVALUE,
        "calldataload"|"CALLDATALOAD" => CALLDATALOAD,
        "calldatasize"|"CALLDATASIZE" => CALLDATASIZE,
        "calldatacopy"|"CALLDATACOPY" => CALLDATACOPY,
        "codesize"|"CODESIZE" => CODESIZE,
        "codecopy"|"CODECOPY" => CODECOPY,
        "gasprice"|"GASPRICE" => GASPRICE,
        "extcodesize"|"EXTCODESIZE" => EXTCODESIZE,
        "extcodecopy"|"EXTCODECOPY" => EXTCODECOPY,
        "returndatasize"|"RETURNDATASIZE" => RETURNDATASIZE,
        "returndatacopy"|"RETURNDATACOPY" => RETURNDATACOPY,
        "extcodehash"|"EXTCODEHASH" => EXTCODEHASH,
        // 40s: Block Information
        "blockhash"|"BLOCKHASH" => BLOCKHASH,
        "coinbase"|"COINBASE" => COINBASE,
        "timestamp"|"TIMESTAMP" => TIMESTAMP,
        "number"|"NUMBER" => NUMBER,
        "difficulty"|"DIFFICULTY" => DIFFICULTY,
        "gaslimit"|"GASLIMIT" => GASLIMIT,
        "chainid"|"CHAINID" => CHAINID,
        "selfbalance"|"SELFBALANCE" => SELFBALANCE,
        // 50s: Stack, Memory, Storage and Flow Operations
        "pop"|"POP" => POP,
        "mload"|"MLOAD" => MLOAD,
        "mstore"|"MSTORE" => MSTORE,
        "mstore8"|"MSTORE8" => MSTORE8,
        "sload"|"SLOAD" => SLOAD,
        "sstore"|"SSTORE" => SSTORE,
        "jump"|"JUMP" => JUMP,
        "jumpi"|"JUMPI" => JUMPI,
        "pc"|"PC" => PC,
        "msize"|"MSIZE" => MSIZE,
        "gas"|"GAS" => GAS,
        "jumpdest"|"JUMPDEST" => JUMPDEST,
        // 60s & 70s: Push Operations
        "push"|"PUSH" => {
            // Should be impossible to get here!
            unreachable!();
        }
        // 80s: Duplication Operations
        "dup1"|"DUP1" => DUP(1),
        "dup2"|"DUP2" => DUP(2),
        "dup3"|"DUP3" => DUP(3),
        "dup4"|"DUP4" => DUP(4),
        "dup5"|"DUP5" => DUP(5),
        "dup6"|"DUP6" => DUP(6),
        "dup7"|"DUP7" => DUP(7),
        "dup8"|"DUP8" => DUP(8),
        "dup9"|"DUP9" => DUP(9),
        "dup10"|"DUP10" => DUP(10),
        "dup11"|"DUP11" => DUP(11),
        "dup12"|"DUP12" => DUP(12),
        "dup13"|"DUP13" => DUP(13),
        "dup14"|"DUP14" => DUP(14),
        "dup15"|"DUP15" => DUP(15),
        "dup16"|"DUP16" => DUP(16),
        // 90s: Swap Operations
        "swap1"|"SWAP1" => SWAP(1),
        "swap2"|"SWAP2" => SWAP(2),
        "swap3"|"SWAP3" => SWAP(3),
        "swap4"|"SWAP4" => SWAP(4),
        "swap5"|"SWAP5" => SWAP(5),
        "swap6"|"SWAP6" => SWAP(6),
        "swap7"|"SWAP7" => SWAP(7),
        "swap8"|"SWAP8" => SWAP(8),
        "swap9"|"SWAP9" => SWAP(9),
        "swap10"|"SWAP10" => SWAP(10),
        "swap11"|"SWAP11" => SWAP(11),
        "swap12"|"SWAP12" => SWAP(12),
        "swap13"|"SWAP13" => SWAP(13),
        "swap14"|"SWAP14" => SWAP(14),
        "swap15"|"SWAP15" => SWAP(15),
        "swap16"|"SWAP16" => SWAP(16),
        // a0s: Log Operations
        "log0"|"LOG0" => LOG(0),
        "log1"|"LOG1" => LOG(1),
        "log2"|"LOG2" => LOG(2),
        "log3"|"LOG3" => LOG(3),
        "log4"|"LOG4" => LOG(4),
        // f0s: System Operations
        "create"|"CREATE" => CREATE,
        "call"|"CALL" => CALL,
        "callcode"|"CALLCODE" => CALLCODE,
        "return"|"RETURN" => RETURN,
        "delegatecall"|"DELEGATECALL" => DELEGATECALL,
        "create2"|"CREATE2" => CREATE2,
        "staticcall"|"STATICCALL" => STATICCALL,
        "revert"|"REVERT" => REVERT,
        "invalid"|"INVALID" => INVALID,
        "selfdestruct"|"SELFDESTRUCT" => SELFDESTRUCT,
        //
        _ => {
            return Err(AssemblyError::InvalidInstruction);
        }
    };
    //
    Ok(insn)
}
