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
use crate::util::ToHexString;
use super::opcode;

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum Error {
    /// A push instruction cannot push zero bytes and, likewise,
    /// cannot push more than 32 bytes.
    InvalidPush,
    /// A dup `n` instruction requires `n > 0` and `n <= 32`.
    InvalidDup,
    /// A label cannot exceed the 24Kb limit imposed by the EVM.
    InvalidLabelOffset,
}

// ============================================================================
// Bytecode Instructions
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    // 0s: Stop and Arithmetic Operations
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    // 10s: Comparison & Bitwise Logic Operations
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    // 20s: Keccak256
    KECCAK256,
    // 30s: Environmental Information
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    // 40s: Block Information
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    // 50s: Stack, Memory, Storage and Flow Operations
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,
    RJUMP(i16),  // EIP4200
    RJUMPI(i16), // EIP4200
    // 60 & 70s: Push Operations
    PUSH(Vec<u8>),
    // 80s: Duplicate Operations
    DUP(u8),
    // 90s: Exchange Operations
    SWAP(u8),
    // a0s: Logging Operations
    LOG(u8),
    // f0s: System Operations
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
    // Signals arbitrary data in the contract, rather than bytecode
    // instructions.
    DATA(Vec<u8>),
}

impl Instruction {
    /// Determine whether or not control can continue to the next
    /// instruction.
    pub fn fallthru(&self) -> bool {
        match self {
            Instruction::DATA(_) => false,
            Instruction::INVALID => false,
            Instruction::JUMP => false,
            Instruction::RJUMP(_) => false,
            Instruction::STOP => false,
            Instruction::RETURN => false,
            Instruction::REVERT => false,
            Instruction::SELFDESTRUCT => false,
            _ => true,
        }
    }

    /// Determine whether or not this instruction can branch.  That
    /// is, whether or not it is a `JUMP` or `JUMPI` instruction.
    pub fn can_branch(&self) -> bool {
        match self {
            Instruction::JUMP => true,
            Instruction::JUMPI => true,
            Instruction::RJUMP(_) => true,
            Instruction::RJUMPI(_) => true,
            _ => false,
        }
    }

    /// Encode an instruction into a byte sequence, assuming a given
    /// set of label offsets.
    pub fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Error> {
        // Push operands (if applicable)
        match self {
            Instruction::DATA(args) => {
                // Push operands
                bytes.extend(args);
            }
            Instruction::RJUMP(target) => {
                // Push opcode
                bytes.push(self.opcode()?);
                // Push operands
                bytes.extend(&target.to_be_bytes());
            }
            Instruction::RJUMPI(target) => {
                // Push opcode
                bytes.push(self.opcode()?);
                // Push operands
                bytes.extend(&target.to_be_bytes());
            }
            Instruction::PUSH(args) => {
                // Push opcode
                bytes.push(self.opcode()?);
                // Push operands
                bytes.extend(args);
            }
            _ => {
                // All other instructions have no operands.
                bytes.push(self.opcode()?);
            }
        }
        //
        Ok(())
    }

    /// Determine the length of this instruction (in bytes) assuming a
    /// given set of label offsets.
    pub fn length(&self) -> usize {
        match self {
            Instruction::DATA(bytes) => bytes.len(),
            // Static jumps
            Instruction::RJUMP(_) => 3,
            Instruction::RJUMPI(_) => 3,
            // Push instructions
            Instruction::PUSH(bs) => 1 + bs.len(),
            // Default case
            _ => 1,
        }
    }

    /// Determine the opcode for a given instruction.  In many cases,
    /// this is a straightforward mapping.  However, in other cases,
    /// its slightly more involved as a calculation involving the
    /// operands is required.
    pub fn opcode(&self) -> Result<u8, Error> {
        let op = match self {
            // 0s: Stop and Arithmetic Operations
            Instruction::STOP => opcode::STOP,
            Instruction::ADD => opcode::ADD,
            Instruction::MUL => opcode::MUL,
            Instruction::SUB => opcode::SUB,
            Instruction::DIV => opcode::DIV,
            Instruction::SDIV => opcode::SDIV,
            Instruction::MOD => opcode::MOD,
            Instruction::SMOD => opcode::SMOD,
            Instruction::ADDMOD => opcode::ADDMOD,
            Instruction::MULMOD => opcode::MULMOD,
            Instruction::EXP => opcode::EXP,
            Instruction::SIGNEXTEND => opcode::SIGNEXTEND,
            // 10s: Comparison & Bitwise Logic Operations
            Instruction::LT => opcode::LT,
            Instruction::GT => opcode::GT,
            Instruction::SLT => opcode::SLT,
            Instruction::SGT => opcode::SGT,
            Instruction::EQ => opcode::EQ,
            Instruction::ISZERO => opcode::ISZERO,
            Instruction::AND => opcode::AND,
            Instruction::OR => opcode::OR,
            Instruction::XOR => opcode::XOR,
            Instruction::NOT => opcode::NOT,
            Instruction::BYTE => opcode::BYTE,
            Instruction::SHL => opcode::SHL,
            Instruction::SHR => opcode::SHR,
            Instruction::SAR => opcode::SAR,
            // 20s: Keccak256
            Instruction::KECCAK256 => opcode::KECCAK256,
            // 30s: Environmental Information
            Instruction::ADDRESS => opcode::ADDRESS,
            Instruction::BALANCE => opcode::BALANCE,
            Instruction::ORIGIN => opcode::ORIGIN,
            Instruction::CALLER => opcode::CALLER,
            Instruction::CALLVALUE => opcode::CALLVALUE,
            Instruction::CALLDATALOAD => opcode::CALLDATALOAD,
            Instruction::CALLDATASIZE => opcode::CALLDATASIZE,
            Instruction::CALLDATACOPY => opcode::CALLDATACOPY,
            Instruction::CODESIZE => opcode::CODESIZE,
            Instruction::CODECOPY => opcode::CODECOPY,
            Instruction::GASPRICE => opcode::GASPRICE,
            Instruction::EXTCODESIZE => opcode::EXTCODESIZE,
            Instruction::EXTCODECOPY => opcode::EXTCODECOPY,
            Instruction::RETURNDATASIZE => opcode::RETURNDATASIZE,
            Instruction::RETURNDATACOPY => opcode::RETURNDATACOPY,
            Instruction::EXTCODEHASH => opcode::EXTCODEHASH,
            // 40s: Block Information
            Instruction::BLOCKHASH => opcode::BLOCKHASH,
            Instruction::COINBASE => opcode::COINBASE,
            Instruction::TIMESTAMP => opcode::TIMESTAMP,
            Instruction::NUMBER => opcode::NUMBER,
            Instruction::DIFFICULTY => opcode::DIFFICULTY,
            Instruction::GASLIMIT => opcode::GASLIMIT,
            Instruction::CHAINID => opcode::CHAINID,
            Instruction::SELFBALANCE => opcode::SELFBALANCE,
            // 50s: Stack, Memory, Storage and Flow Operations
            Instruction::POP => opcode::POP,
            Instruction::MLOAD => opcode::MLOAD,
            Instruction::MSTORE => opcode::MSTORE,
            Instruction::MSTORE8 => opcode::MSTORE8,
            Instruction::SLOAD => opcode::SLOAD,
            Instruction::SSTORE => opcode::SSTORE,
            Instruction::JUMP => opcode::JUMP,
            Instruction::JUMPI => opcode::JUMPI,
            Instruction::PC => opcode::PC,
            Instruction::MSIZE => opcode::MSIZE,
            Instruction::GAS => opcode::GAS,
            Instruction::JUMPDEST => opcode::JUMPDEST,
            Instruction::RJUMP(_) => opcode::RJUMP,
            Instruction::RJUMPI(_) => opcode::RJUMPI,
            // 60s & 70s: Push Operations
            Instruction::PUSH(bs) => {
                if bs.len() == 0 || bs.len() > 32 {
                    return Err(Error::InvalidPush);
                } else {
                    let n = (bs.len() as u8) - 1;
                    opcode::PUSH1 + n
                }
            }
            // 80s: Duplication Operations
            Instruction::DUP(n) => {
                if *n == 0 || *n > 32 {
                    return Err(Error::InvalidDup);
                }
                opcode::DUP1 + (n-1)
            }
            // 90s: Swap Operations
            Instruction::SWAP(n) => {
                if *n == 0 || *n > 32 {
                    return Err(Error::InvalidDup);
                }
                opcode::SWAP1 + (n-1)
            }
            // a0s: Log Operations
            Instruction::LOG(n) => {
                if *n > 4 {
                    return Err(Error::InvalidDup);
                }
                opcode::LOG0 + n
            }
            // f0s: System Operations
            Instruction::CREATE => opcode::CREATE,
            Instruction::CALL => opcode::CALL,
            Instruction::CALLCODE => opcode::CALLCODE,
            Instruction::RETURN => opcode::RETURN,
            Instruction::DELEGATECALL => opcode::DELEGATECALL,
            Instruction::CREATE2 => opcode::CREATE2,
            Instruction::STATICCALL => opcode::STATICCALL,
            Instruction::REVERT => opcode::REVERT,
            Instruction::INVALID => opcode::INVALID,
            Instruction::SELFDESTRUCT => opcode::SELFDESTRUCT,
            //
            Instruction::DATA(_) => {
                panic!("Invalid instruction ({:?})", self);
            }
        };
        //
        Ok(op)
    }

    /// Decode the next instruction in a given sequence of bytes.
    pub fn decode(pc: usize, bytes: &[u8]) -> Instruction {
        let opcode = if pc < bytes.len() { bytes[pc] } else { 0x00 };
        //
        let insn = match opcode {
            // 0s: Stop and Arithmetic Operations
            opcode::STOP => Instruction::STOP,
            opcode::ADD => Instruction::ADD,
            opcode::MUL => Instruction::MUL,
            opcode::SUB => Instruction::SUB,
            opcode::DIV => Instruction::DIV,
            opcode::SDIV => Instruction::SDIV,
            opcode::MOD => Instruction::MOD,
            opcode::SMOD => Instruction::SMOD,
            opcode::ADDMOD => Instruction::ADDMOD,
            opcode::MULMOD => Instruction::MULMOD,
            opcode::EXP => Instruction::EXP,
            opcode::SIGNEXTEND => Instruction::SIGNEXTEND,
            // 10s: Comparison & Bitwise Logic Operations
            opcode::LT => Instruction::LT,
            opcode::GT => Instruction::GT,
            opcode::SLT => Instruction::SLT,
            opcode::SGT => Instruction::SGT,
            opcode::EQ => Instruction::EQ,
            opcode::ISZERO => Instruction::ISZERO,
            opcode::AND => Instruction::AND,
            opcode::OR => Instruction::OR,
            opcode::XOR => Instruction::XOR,
            opcode::NOT => Instruction::NOT,
            opcode::BYTE => Instruction::BYTE,
            opcode::SHL => Instruction::SHL,
            opcode::SHR => Instruction::SHR,
            opcode::SAR => Instruction::SAR,
            // 20s: SHA3
            opcode::KECCAK256 => Instruction::KECCAK256,
            // 30s: Environmental Information
            opcode::ADDRESS => Instruction::ADDRESS,
            opcode::BALANCE => Instruction::BALANCE,
            opcode::ORIGIN => Instruction::ORIGIN,
            opcode::CALLER => Instruction::CALLER,
            opcode::CALLVALUE => Instruction::CALLVALUE,
            opcode::CALLDATALOAD => Instruction::CALLDATALOAD,
            opcode::CALLDATASIZE => Instruction::CALLDATASIZE,
            opcode::CALLDATACOPY => Instruction::CALLDATACOPY,
            opcode::CODESIZE => Instruction::CODESIZE,
            opcode::CODECOPY => Instruction::CODECOPY,
            opcode::GASPRICE => Instruction::GASPRICE,
            opcode::EXTCODESIZE => Instruction::EXTCODESIZE,
            opcode::EXTCODECOPY => Instruction::EXTCODECOPY,
            opcode::RETURNDATASIZE => Instruction::RETURNDATASIZE,
            opcode::RETURNDATACOPY => Instruction::RETURNDATACOPY,
            opcode::EXTCODEHASH => Instruction::EXTCODEHASH,
            // 40s: Block Information
            opcode::BLOCKHASH => Instruction::BLOCKHASH,
            opcode::COINBASE => Instruction::COINBASE,
            opcode::TIMESTAMP => Instruction::TIMESTAMP,
            opcode::NUMBER => Instruction::NUMBER,
            opcode::DIFFICULTY => Instruction::DIFFICULTY,
            opcode::GASLIMIT => Instruction::GASLIMIT,
            opcode::CHAINID => Instruction::CHAINID,
            opcode::SELFBALANCE => Instruction::SELFBALANCE,
            // 50s: Stack, Memory, Storage and Flow Operations
            opcode::POP => Instruction::POP,
            opcode::MLOAD => Instruction::MLOAD,
            opcode::MSTORE => Instruction::MSTORE,
            opcode::MSTORE8 => Instruction::MSTORE8,
            opcode::SLOAD => Instruction::SLOAD,
            opcode::SSTORE => Instruction::SSTORE,
            opcode::JUMP => Instruction::JUMP,
            opcode::JUMPI => Instruction::JUMPI,
            opcode::PC => Instruction::PC,
            opcode::MSIZE => Instruction::MSIZE,
            opcode::GAS => Instruction::GAS,
            opcode::JUMPDEST => Instruction::JUMPDEST,
            opcode::RJUMP => {
                // NOTE: these instructions are not permitted to
                // overflow, and therefore don't require padding.
                let arg = [bytes[pc+1],bytes[pc+2]];
                Instruction::RJUMP(i16::from_be_bytes(arg))
            }
            opcode::RJUMPI => {
                // NOTE: these instructions are not permitted to
                // overflow, and therefore don't require padding.
                let arg = [bytes[pc+1],bytes[pc+2]];
                Instruction::RJUMPI(i16::from_be_bytes(arg))
            }
            // 60s & 70s: Push Operations
            opcode::PUSH1..=opcode::PUSH32 => {
                let m = pc + 1;
                let n = pc + 2 + ((opcode - opcode::PUSH1) as usize);
                if n <= bytes.len() {
                    // Simple case: does not overflow
                    Instruction::PUSH(bytes[m..n].to_vec())
                } else {
                    // Harder case: does overflow code.
                    let mut bs = bytes[m..].to_vec();
                    // Pad out with zeros
                    for _i in 0..(n - bytes.len()) {
                        bs.push(0);
                    }
                    // Done
                    Instruction::PUSH(bs)
                }
            }
            // 80s: Duplicate Operations
            opcode::DUP1..=opcode::DUP16 => Instruction::DUP(opcode - 0x7f),
            // 90s: Swap Operations
            opcode::SWAP1..=opcode::SWAP16 => Instruction::SWAP(opcode - 0x8f),
            // a0s: Log Operations
            opcode::LOG0..=opcode::LOG4 => Instruction::LOG(opcode - 0xa0),
            // f0s: System Operations
            opcode::CREATE => Instruction::CREATE,
            opcode::CALL => Instruction::CALL,
            opcode::CALLCODE => Instruction::CALLCODE,
            opcode::RETURN => Instruction::RETURN,
            opcode::DELEGATECALL => Instruction::DELEGATECALL,
            opcode::CREATE2 => Instruction::CREATE2,
            opcode::STATICCALL => Instruction::STATICCALL,
            opcode::REVERT => Instruction::REVERT,
            opcode::INVALID => Instruction::INVALID,
            opcode::SELFDESTRUCT => Instruction::SELFDESTRUCT,
            // Unknown
            _ => Instruction::DATA(vec![opcode]),
        };
        //
        insn
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the default (debug) formatter.  Its only for certain
        // instructions that we need to do anything different.
        match self {
            Instruction::DATA(bytes) => {
                // Print bytes as hex string
                write!(f, "{}", bytes.to_hex_string())
            }
            Instruction::DUP(n) => {
                write!(f, "dup{}",n)
            }
            Instruction::LOG(n) => {
                write!(f, "log{n}")
            }
            Instruction::JUMPDEST => {
                write!(f, "jumpdest")
            }
            Instruction::PUSH(bytes) => {
                // Convert bytes into hex string
                let hex = bytes.to_hex_string();
                // Print!
                write!(f, "push {}", hex)
            }
            Instruction::RJUMP(offset) => {
                if offset < &0 {
                    write!(f, "rjump -{:#x}", offset)
                } else {
                    write!(f, "rjump {:#x}", offset)
                }
            }
            Instruction::RJUMPI(offset) => {
                if offset < &0 {
                    write!(f, "rjumpi -{:#x}", offset)
                } else {
                    write!(f, "rjumpi {:#x}", offset)
                }
            }
            Instruction::SWAP(n) => {
                write!(f, "swap{n}")
            }
            _ => {
                let s = format!("{:?}",self).to_lowercase();
                write!(f, "{s}")
            }
        }
    }
}

// ============================================================================
// Traits
// ============================================================================

/// A trait for converting something (e.g. a byte sequence) into a
/// vector of instructions.
pub trait ToInstructions {
    fn to_insns(&self) -> Vec<Instruction>;
}

impl<'a> ToInstructions for &'a [u8] {
    fn to_insns(&self) -> Vec<Instruction> {
        let mut insns = Vec::new();
        let mut index = 0;
        //
        while index < self.len() {
            let insn = Instruction::decode(index, self);
            // Shift us along
            index += insn.length();
            // Store the instruction!
            insns.push(insn);
        }
        //
        insns
    }
}
