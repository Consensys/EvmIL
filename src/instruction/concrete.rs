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
use super::{AbstractInstruction,InstructionOperands,VoidOperand};
use crate::instruction::opcode;

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

use AbstractInstruction::*;

// ============================================================================
// Concrete Instructions
// ============================================================================

/// Representation of instruction operands (more specifically, _branch
/// offsets_) as appropriate for _concrete bytecode instructions_.  In
/// legacy contracts, branch targets are implemented using _absolute
/// offsets_.  In EOF contracts, branch targets can also be
/// implemented using _relative offsets_.
#[derive(Clone,Debug,PartialEq)]
pub struct ConcreteOperands();

impl InstructionOperands for ConcreteOperands {
    type RelOffset16 = i16;
    /// We do not permit the `PUSHL` instruction here, since it is
    /// already represented by `PUSH`.
    type PushLabel = VoidOperand;
    /// Likewise, we do not permit the `LABEL` instruction here, since
    /// it has no concrete meaning.
    type Label = VoidOperand;
}

/// An EVM instruction is an abstract instruction with concrete operands.
pub type Instruction = AbstractInstruction<ConcreteOperands>;

impl Instruction {
    /// Encode an instruction into a byte sequence, assuming a given
    /// set of label offsets.
    pub fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Error> {
        // Push operands (if applicable)
        match self {
            DATA(args) => {
                // Push operands
                bytes.extend(args);
            }
            RJUMP(target) => {
                // Push opcode
                bytes.push(self.opcode()?);
                // Push operands
                bytes.extend(&target.to_be_bytes());
            }
            RJUMPI(target) => {
                // Push opcode
                bytes.push(self.opcode()?);
                // Push operands
                bytes.extend(&target.to_be_bytes());
            }
            PUSH(args) => {
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
    pub fn opcode(&self) -> Result<u8, Error> {
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
                    return Err(Error::InvalidPush);
                } else {
                    let n = (bs.len() as u8) - 1;
                    opcode::PUSH1 + n
                }
            }
            // 80s: Duplication Operations
            DUP(n) => {
                if *n == 0 || *n > 32 {
                    return Err(Error::InvalidDup);
                }
                opcode::DUP1 + (n-1)
            }
            // 90s: Swap Operations
            SWAP(n) => {
                if *n == 0 || *n > 32 {
                    return Err(Error::InvalidDup);
                }
                opcode::SWAP1 + (n-1)
            }
            // a0s: Log Operations
            LOG(n) => {
                if *n > 4 {
                    return Err(Error::InvalidDup);
                }
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
            DATA(_) => {
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
