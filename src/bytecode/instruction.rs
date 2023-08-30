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
use std::fmt::{Debug};
use crate::util::{ToHexString};
use super::opcode;
use super::operands::{BytecodeOperands,Operands};

/// An abstract instruction is parameterised over the type of _control
/// flow_ it supports.  In particular, _concrete_ instructions are
/// fully instantiated with specific branch targets.  In contract,
/// _labelled_ instructions employ symbolic labels instead of concrete
/// target information.  The primary purpose here is to distinguish
/// between instructions originating from bytes, versus those
/// originating from assembly language.
///
/// The intention is that all known instructions are represented here
/// in one place, rather than e.g. being separated (somehow) by fork.
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction<T:Operands = BytecodeOperands> {
    // ===============================================================
    // 0s: Stop and Arithmetic Operations
    // ===============================================================    
    /// Halts execution.
    STOP,
    /// Arithmetic addition modulo `2^256`.
    ADD,
    /// Arithmetic multiplication modulo `2^256`.    
    MUL,
    /// Arithmetic subtraction modulo `2^256`.
    SUB,
    /// Arithmetic division which rounds towards zero.
    DIV,
    /// Signed arithmetic division which rounds towards zero (i.e. it
    /// is non-Euclidian).
    SDIV,
    /// Arithmetic modulus operator.
    MOD,
    /// Signed arithmetic remainder operator.    
    SMOD,
    /// Arithmetic addition modulo a given value `n`.
    ADDMOD,
    /// Arithmetic multiplication modulo a given value `n`.
    MULMOD,
    /// Arithmetic exponentiation modulo `2^256`.
    EXP,
    /// Extend a two's complement signed integer.
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
    RJUMP(T::RelOffset16),  // EIP4200
    RJUMPI(T::RelOffset16), // EIP4200
    // 60 & 70s: Push Operations
    PUSH(Vec<u8>),
    PUSHL(bool,T::PushLabel),
    LABEL(T::Label),
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

use Instruction::*;

impl<T:Operands> Instruction<T> {
    /// Determine whether or not control can continue to the next
    /// instruction.
    pub fn fallthru(&self) -> bool {
        match self {
            DATA(_) => false,
            INVALID => false,
            JUMP => false,
            RJUMP(_) => false,
            STOP => false,
            RETURN => false,
            REVERT => false,
            SELFDESTRUCT => false,
            _ => true,
        }
    }

    /// Determine whether or not this instruction can branch.  That
    /// is, whether or not it is a `JUMP` or `JUMPI` instruction.
    pub fn can_branch(&self) -> bool {
        match self {
            JUMP => true,
            JUMPI => true,
            RJUMP(_) => true,
            RJUMPI(_) => true,
            _ => false,
        }
    }
}

impl Instruction<BytecodeOperands> {
    /// Encode an instruction into a byte sequence, assuming a given
    /// set of label offsets.
    pub fn encode(&self, bytes: &mut Vec<u8>) {
        // Push operands (if applicable)
        match self {
            DATA(args) => {
                // Push operands
                bytes.extend(args);
            }
            RJUMP(target) => {
                // Push opcode
                bytes.push(self.opcode());
                // Push operands
                bytes.extend(&target.to_be_bytes());
            }
            RJUMPI(target) => {
                // Push opcode
                bytes.push(self.opcode());
                // Push operands
                bytes.extend(&target.to_be_bytes());
            }
            PUSH(args) => {
                // Push opcode
                bytes.push(self.opcode());
                // Push operands
                bytes.extend(args);
            }
            _ => {
                // All other instructions have no operands.
                bytes.push(self.opcode());
            }
        }
    }

    /// Determine the length of this instruction (in bytes) assuming a
    /// given set of label offsets.
    pub fn length(&self) -> usize {
        match self {
            DATA(bytes) => bytes.len(),
            // Static jumps
            RJUMP(_) => 3,
            RJUMPI(_) => 3,
            // Push instructions
            PUSH(bs) => 1 + bs.len(),
            // Default case
            _ => 1,
        }
    }

    /// Determine the opcode for a given instruction.  In many cases,
    /// this is a straightforward mapping.  However, in other cases,
    /// its slightly more involved as a calculation involving the
    /// operands is required.
    pub fn opcode(&self) -> u8 {
        let op = match self {
            // 0s: Stop and Arithmetic Operations
            STOP => opcode::STOP,
            ADD => opcode::ADD,
            MUL => opcode::MUL,
            SUB => opcode::SUB,
            DIV => opcode::DIV,
            SDIV => opcode::SDIV,
            MOD => opcode::MOD,
            SMOD => opcode::SMOD,
            ADDMOD => opcode::ADDMOD,
            MULMOD => opcode::MULMOD,
            EXP => opcode::EXP,
            SIGNEXTEND => opcode::SIGNEXTEND,
            // 10s: Comparison & Bitwise Logic Operations
            LT => opcode::LT,
            GT => opcode::GT,
            SLT => opcode::SLT,
            SGT => opcode::SGT,
            EQ => opcode::EQ,
            ISZERO => opcode::ISZERO,
            AND => opcode::AND,
            OR => opcode::OR,
            XOR => opcode::XOR,
            NOT => opcode::NOT,
            BYTE => opcode::BYTE,
            SHL => opcode::SHL,
            SHR => opcode::SHR,
            SAR => opcode::SAR,
            // 20s: Keccak256
            KECCAK256 => opcode::KECCAK256,
            // 30s: Environmental Information
            ADDRESS => opcode::ADDRESS,
            BALANCE => opcode::BALANCE,
            ORIGIN => opcode::ORIGIN,
            CALLER => opcode::CALLER,
            CALLVALUE => opcode::CALLVALUE,
            CALLDATALOAD => opcode::CALLDATALOAD,
            CALLDATASIZE => opcode::CALLDATASIZE,
            CALLDATACOPY => opcode::CALLDATACOPY,
            CODESIZE => opcode::CODESIZE,
            CODECOPY => opcode::CODECOPY,
            GASPRICE => opcode::GASPRICE,
            EXTCODESIZE => opcode::EXTCODESIZE,
            EXTCODECOPY => opcode::EXTCODECOPY,
            RETURNDATASIZE => opcode::RETURNDATASIZE,
            RETURNDATACOPY => opcode::RETURNDATACOPY,
            EXTCODEHASH => opcode::EXTCODEHASH,
            // 40s: Block Information
            BLOCKHASH => opcode::BLOCKHASH,
            COINBASE => opcode::COINBASE,
            TIMESTAMP => opcode::TIMESTAMP,
            NUMBER => opcode::NUMBER,
            DIFFICULTY => opcode::DIFFICULTY,
            GASLIMIT => opcode::GASLIMIT,
            CHAINID => opcode::CHAINID,
            SELFBALANCE => opcode::SELFBALANCE,
            // 50s: Stack, Memory, Storage and Flow Operations
            POP => opcode::POP,
            MLOAD => opcode::MLOAD,
            MSTORE => opcode::MSTORE,
            MSTORE8 => opcode::MSTORE8,
            SLOAD => opcode::SLOAD,
            SSTORE => opcode::SSTORE,
            JUMP => opcode::JUMP,
            JUMPI => opcode::JUMPI,
            PC => opcode::PC,
            MSIZE => opcode::MSIZE,
            GAS => opcode::GAS,
            JUMPDEST => opcode::JUMPDEST,
            RJUMP(_) => opcode::RJUMP,
            RJUMPI(_) => opcode::RJUMPI,
            // 60s & 70s: Push Operations
            PUSH(bs) => {
                if bs.len() == 0 || bs.len() > 32 {
                    panic!("invalid push");
                } else {
                    let n = (bs.len() as u8) - 1;
                    opcode::PUSH1 + n
                }
            }
            // 80s: Duplication Operations
            DUP(n) => {
                if *n == 0 || *n > 32 { panic!("invalid dup"); }
                opcode::DUP1 + (n-1)
            }
            // 90s: Swap Operations
            SWAP(n) => {
                if *n == 0 || *n > 32 { panic!("invalid swap"); }
                opcode::SWAP1 + (n-1)
            }
            // a0s: Log Operations
            LOG(n) => {
                if *n > 4 { panic!("invalid log"); }
                opcode::LOG0 + n
            }
            // f0s: System Operations
            CREATE => opcode::CREATE,
            CALL => opcode::CALL,
            CALLCODE => opcode::CALLCODE,
            RETURN => opcode::RETURN,
            DELEGATECALL => opcode::DELEGATECALL,
            CREATE2 => opcode::CREATE2,
            STATICCALL => opcode::STATICCALL,
            REVERT => opcode::REVERT,
            INVALID => opcode::INVALID,
            SELFDESTRUCT => opcode::SELFDESTRUCT,
            //
            PUSHL(..)|LABEL(_) => {
                // Unreachable because these instructions are not
                // concrete.
                unreachable!();
            }
            //
            DATA(_) => {
                panic!("Invalid instruction ({:?})", self);
            }
        };
        //
        op
    }

    /// Decode the next instruction in a given sequence of bytes.
    pub fn decode(pc: usize, bytes: &[u8]) -> Instruction {
        let opcode = if pc < bytes.len() { bytes[pc] } else { 0x00 };
        //
        let insn = match opcode {
            // 0s: Stop and Arithmetic Operations
            opcode::STOP => STOP,
            opcode::ADD => ADD,
            opcode::MUL => MUL,
            opcode::SUB => SUB,
            opcode::DIV => DIV,
            opcode::SDIV => SDIV,
            opcode::MOD => MOD,
            opcode::SMOD => SMOD,
            opcode::ADDMOD => ADDMOD,
            opcode::MULMOD => MULMOD,
            opcode::EXP => EXP,
            opcode::SIGNEXTEND => SIGNEXTEND,
            // 10s: Comparison & Bitwise Logic Operations
            opcode::LT => LT,
            opcode::GT => GT,
            opcode::SLT => SLT,
            opcode::SGT => SGT,
            opcode::EQ => EQ,
            opcode::ISZERO => ISZERO,
            opcode::AND => AND,
            opcode::OR => OR,
            opcode::XOR => XOR,
            opcode::NOT => NOT,
            opcode::BYTE => BYTE,
            opcode::SHL => SHL,
            opcode::SHR => SHR,
            opcode::SAR => SAR,
            // 20s: SHA3
            opcode::KECCAK256 => KECCAK256,
            // 30s: Environmental Information
            opcode::ADDRESS => ADDRESS,
            opcode::BALANCE => BALANCE,
            opcode::ORIGIN => ORIGIN,
            opcode::CALLER => CALLER,
            opcode::CALLVALUE => CALLVALUE,
            opcode::CALLDATALOAD => CALLDATALOAD,
            opcode::CALLDATASIZE => CALLDATASIZE,
            opcode::CALLDATACOPY => CALLDATACOPY,
            opcode::CODESIZE => CODESIZE,
            opcode::CODECOPY => CODECOPY,
            opcode::GASPRICE => GASPRICE,
            opcode::EXTCODESIZE => EXTCODESIZE,
            opcode::EXTCODECOPY => EXTCODECOPY,
            opcode::RETURNDATASIZE => RETURNDATASIZE,
            opcode::RETURNDATACOPY => RETURNDATACOPY,
            opcode::EXTCODEHASH => EXTCODEHASH,
            // 40s: Block Information
            opcode::BLOCKHASH => BLOCKHASH,
            opcode::COINBASE => COINBASE,
            opcode::TIMESTAMP => TIMESTAMP,
            opcode::NUMBER => NUMBER,
            opcode::DIFFICULTY => DIFFICULTY,
            opcode::GASLIMIT => GASLIMIT,
            opcode::CHAINID => CHAINID,
            opcode::SELFBALANCE => SELFBALANCE,
            // 50s: Stack, Memory, Storage and Flow Operations
            opcode::POP => POP,
            opcode::MLOAD => MLOAD,
            opcode::MSTORE => MSTORE,
            opcode::MSTORE8 => MSTORE8,
            opcode::SLOAD => SLOAD,
            opcode::SSTORE => SSTORE,
            opcode::JUMP => JUMP,
            opcode::JUMPI => JUMPI,
            opcode::PC => PC,
            opcode::MSIZE => MSIZE,
            opcode::GAS => GAS,
            opcode::JUMPDEST => JUMPDEST,
            opcode::RJUMP => {
                // NOTE: these instructions are not permitted to
                // overflow, and therefore don't require padding.
                let arg = [bytes[pc+1],bytes[pc+2]];
                RJUMP(i16::from_be_bytes(arg))
            }
            opcode::RJUMPI => {
                // NOTE: these instructions are not permitted to
                // overflow, and therefore don't require padding.
                let arg = [bytes[pc+1],bytes[pc+2]];
                RJUMPI(i16::from_be_bytes(arg))
            }
            // 60s & 70s: Push Operations
            opcode::PUSH1..=opcode::PUSH32 => {
                let m = pc + 1;
                let n = pc + 2 + ((opcode - opcode::PUSH1) as usize);
                if n <= bytes.len() {
                    // Simple case: does not overflow
                    PUSH(bytes[m..n].to_vec())
                } else {
                    // Harder case: does overflow code.
                    let mut bs = bytes[m..].to_vec();
                    // Pad out with zeros
                    for _i in 0..(n - bytes.len()) {
                        bs.push(0);
                    }
                    // Done
                    PUSH(bs)
                }
            }
            // 80s: Duplicate Operations
            opcode::DUP1..=opcode::DUP16 => DUP(opcode - 0x7f),
            // 90s: Swap Operations
            opcode::SWAP1..=opcode::SWAP16 => SWAP(opcode - 0x8f),
            // a0s: Log Operations
            opcode::LOG0..=opcode::LOG4 => LOG(opcode - 0xa0),
            // f0s: System Operations
            opcode::CREATE => CREATE,
            opcode::CALL => CALL,
            opcode::CALLCODE => CALLCODE,
            opcode::RETURN => RETURN,
            opcode::DELEGATECALL => DELEGATECALL,
            opcode::CREATE2 => CREATE2,
            opcode::STATICCALL => STATICCALL,
            opcode::REVERT => REVERT,
            opcode::INVALID => INVALID,
            opcode::SELFDESTRUCT => SELFDESTRUCT,
            // Unknown
            _ => DATA(vec![opcode]),
        };
        //
        insn
    }    
}

impl<T:Operands+Debug> fmt::Display for Instruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the default (debug) formatter.  Its only for certain
        // instructions that we need to do anything different.
        match self {
            DATA(bytes) => {
                // Print bytes as hex string
                write!(f, "db {}", bytes.to_hex_string())
            }
            DUP(n) => {
                write!(f, "dup{}",n)
            }
            LOG(n) => {
                write!(f, "log{n}")
            }
            JUMPDEST => {
                write!(f, "jumpdest")
            }
            PUSH(bytes) => {
                // Convert bytes into hex string
                let hex = bytes.to_hex_string();
                // Print!
                write!(f, "push {}", hex)
            }
            RJUMP(offset) => {
                write!(f, "rjump {offset}")
            }
            RJUMPI(offset) => {
                write!(f, "rjumpi {offset}")
            }
            SWAP(n) => {
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
