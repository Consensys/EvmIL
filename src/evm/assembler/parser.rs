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
use crate::util::FromHexString;
use crate::evm::opcode;
use crate::evm::{Instruction};
use super::lexer::{Lexer,Token};
use super::{Assembly,AssemblyInstruction,AssemblyLanguageError};

pub struct Parser {
    bytecode: Assembly
}

impl Parser {
    /// Construct a new parser from a given string slice.
    pub fn new() -> Self {
        let bytecode = Assembly::new();
        //
        Parser { bytecode }
    }

    pub fn to_assembly(self) -> Assembly {
        self.bytecode
    }

    /// Parse a single line of assembly language.
    pub fn parse(&mut self, line: &str) -> Result<(),AssemblyLanguageError> {
        let mut lexer = Lexer::new(line);
        //
        match lexer.next()? {
            Token::Section("code") => {
                self.bytecode.push(AssemblyInstruction::CodeSection);
            }
            Token::Section("data") => {
                self.bytecode.push(AssemblyInstruction::DataSection);
            }
            Token::Hex(s) => {
                self.bytecode.push(AssemblyInstruction::DataBytes(parse_hex(s)?))
            }
            Token::Identifier("push"|"PUSH") => {
                self.parse_push(lexer.next()?)?
            }
            Token::Identifier("rjump"|"RJUMP") => {
                self.parse_rjump(lexer.next()?)?
            }
            Token::Identifier("rjumpi"|"RJUMPI") => {
                self.parse_rjumpi(lexer.next()?)?
            }
            Token::Identifier(id) => {
                self.bytecode.push(parse_opcode(id)?);
            }
            Token::Label(s) => {
                // Mark label in bytecode sequence
                self.bytecode.push(AssemblyInstruction::Label(s.to_string()));
            }
            _ => {
                // Something went wrong
                return Err(AssemblyLanguageError::UnexpectedToken);
            }
        };
        // Sanity check what's left
        match lexer.next()? {
            Token::EOF => Ok(()),
            _ => Err(AssemblyLanguageError::UnexpectedToken)
        }
    }

    /// Parse a push instruction with a given operand.
    fn parse_push(&mut self, operand: Token) -> Result<(),AssemblyLanguageError> {
        // Push always expects an argument, though it could be a
        // label or a hexadecimal operand.
        match operand {
            Token::Hex(s) => {
                self.bytecode.push(Instruction::PUSH(parse_hex(s)?));
                Ok(())
            }
            Token::Identifier(s) => {
                // This indicates we have an incomplete push
                // instruction which requires a label to be resolved
                // before it can be fully instantiated.
                let insn = AssemblyInstruction::Partial(2,s.to_string(),|_,lab| Instruction::PUSH(lab.to_bytes()));
                self.bytecode.push(insn);
                Ok(())
            },
            Token::EOF => Err(AssemblyLanguageError::ExpectedOperand),
            _ => Err(AssemblyLanguageError::UnexpectedToken)
        }
    }

    /// Parse a rjump instruction with a given operand label.
    fn parse_rjump(&mut self, operand: Token) -> Result<(),AssemblyLanguageError> {
        match operand {
            Token::Identifier(s) => {
                let insn = AssemblyInstruction::Partial(3,s.to_string(),|pc,lab| Instruction::RJUMP(lab.relative_to(pc+3)));
                self.bytecode.push(insn);
                Ok(())
            },
            Token::EOF => Err(AssemblyLanguageError::ExpectedOperand),
            _ => Err(AssemblyLanguageError::UnexpectedToken)
        }
    }

    /// Parse a rjumpi instruction with a given operand label.
    fn parse_rjumpi(&mut self, operand: Token) -> Result<(),AssemblyLanguageError> {
        match operand {
            Token::Identifier(s) => {
                let insn = AssemblyInstruction::Partial(3,s.to_string(),|pc,lab| Instruction::RJUMPI(lab.relative_to(pc+3)));
                self.bytecode.push(insn);
                Ok(())
            },
            Token::EOF => Err(AssemblyLanguageError::ExpectedOperand),
            _ => Err(AssemblyLanguageError::UnexpectedToken)
        }
    }
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
fn parse_opcode(insn: &str) -> Result<Instruction,AssemblyLanguageError> {
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
            return Err(AssemblyLanguageError::InvalidInstruction);
        }
    };
    //
    Ok(Instruction::decode(0, &[opcode]))
}
