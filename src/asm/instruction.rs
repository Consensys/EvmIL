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
use crate::bytecode::{AbstractInstruction,Instruction,InstructionOperands};

// ============================================================================
// Concrete Instructions
// ============================================================================

/// Representation of instruction operands (more specifically, _branch
/// offsets_) as appropriate for _assembly language_.  In assembly
/// language, branch targets are abstract (i.e. labels) rather than
/// concrete (i.e. absolute offsets).  Thus, an `AssemblyInstruction`
/// is an `Instruction` with `AssemblyOperands`.  Currently, a label
/// is simply implemented as a `String`.
#[derive(Clone,Debug,PartialEq)]
pub struct AssemblyOperands();

impl InstructionOperands for AssemblyOperands {
    type RelOffset16 = String;
    type PushLabel = String;
    type Label = String;
}

use AbstractInstruction::*;

/// An individual assembly language instruction.  Assembly
/// instructions do not have concrete information regarding branch
/// targets.  Rather, branch target information is represented using
/// _labels_.  Currently, a label is simply implemented as a `String`.
pub type AssemblyInstruction = AbstractInstruction<AssemblyOperands>;

impl AssemblyInstruction {
    /// Get the branch target label associated with this instruction
    /// (if there is one).
    pub fn target(&self) -> Option<&str> {
        match self {
            PUSHL(_,lab) => Some(lab),
            RJUMP(lab) => Some(lab),
            RJUMPI(lab) => Some(lab),
            _ => None
        }
    }

    /// Returns the minimum length of this instruction.  In many
    /// cases, the minimum length matches exactly the actual length of
    /// the instruction.  For example, an `ADD` instruction has a
    /// minimal length of `1` because it always takes up exactly one
    /// byte.  In contract, a `PUSH` instruction has a minimum length
    /// of `2` but can actually be upto `33` bytes long.
    pub fn min_len(&self) -> usize {
        match self {
            STOP|ADD|MUL|SUB|DIV|SDIV|MOD|SMOD|ADDMOD|MULMOD|EXP|SIGNEXTEND => 1,
            // 10s: Comparison & Bitwise Logic Operations
            LT|GT|SLT|SGT|EQ|ISZERO|AND|OR|XOR|NOT|BYTE|SHL|SHR|SAR => 1,
            // 20s: Keccak256
            KECCAK256 => 1,
            // 30s: Environmental Information
            ADDRESS => 1,
            BALANCE => 1,
            ORIGIN => 1,
            CALLER => 1,
            CALLVALUE|CALLDATALOAD|CALLDATASIZE|CALLDATACOPY => 1,
            CODESIZE|CODECOPY => 1,
            GASPRICE => 1,
            EXTCODESIZE|EXTCODECOPY => 1,
            RETURNDATASIZE|RETURNDATACOPY => 1,
            EXTCODEHASH => 1,
            // 40s: Block Information
            BLOCKHASH => 1,
            COINBASE => 1,
            TIMESTAMP => 1,
            NUMBER => 1,
            DIFFICULTY => 1,
            GASLIMIT => 1,
            CHAINID => 1,
            SELFBALANCE => 1,
            // 50s: Stack, Memory, Storage and Flow Operations
            POP|MLOAD|MSTORE|MSTORE8|SLOAD|SSTORE|JUMP|JUMPI|PC|MSIZE|GAS => 1,
            JUMPDEST => 1,
            RJUMP(_)|RJUMPI(_) => 2,
            // 60s & 70s: Push Operations
            PUSH(bs) => bs.len(),
            PUSHL(large,_) => if *large {3} else {2},
            LABEL(_) => 0,
            // 80s: Duplication Operations
            DUP(_) => 1,
            // 90s: Swap Operations
            SWAP(_) => 1,
            // a0s: Log Operations
            LOG(_) => 1,
            // f0s: System Operations
            CREATE => 1,
            CALL|CALLCODE => 1,
            RETURN => 1,
            DELEGATECALL => 1,
            CREATE2 => 1,
            STATICCALL => 1,
            REVERT => 1,
            INVALID => 1,
            SELFDESTRUCT => 1,
            //
            DATA(bytes) => {
                bytes.len()
            }
        }
    }

    /// Instantiate an instruction at a given byte offset in the
    /// instruction stream using a given mapping from labels to
    /// _absolute_ byte offsets.  The byte offset at which the
    /// instruction is being instantiated is needed to calculate
    /// _relative addresses_.  Finally, observe that not all
    /// instructions instantiate to something concrete.  For example,
    /// labels instantiate to nothing.
    pub fn instantiate<F>(&self, offset: usize, mapper: F) -> Result<Option<Instruction>,()>
    where F : Fn(&str) -> Option<usize> {
        let insn = match self {
            // 0s: Stop and Arithmetic Operations
            STOP => STOP,
            ADD => ADD,
            MUL => MUL,
            SUB => SUB,
            DIV => DIV,
            SDIV => SDIV,
            MOD => MOD,
            SMOD => SMOD,
            ADDMOD => ADDMOD,
            MULMOD => MULMOD,
            EXP => EXP,
            SIGNEXTEND => SIGNEXTEND,
            // 10s: Comparison & Bitwise Logic Operations
            LT => LT,
            GT => GT,
            SLT => SLT,
            SGT => SGT,
            EQ => EQ,
            ISZERO => ISZERO,
            AND => AND,
            OR => OR,
            XOR => XOR,
            NOT => NOT,
            BYTE => BYTE,
            SHL => SHL,
            SHR => SHR,
            SAR => SAR,
            // 20s: Keccak256
            KECCAK256 => KECCAK256,
            // 30s: Environmental Information
            ADDRESS => ADDRESS,
            BALANCE => BALANCE,
            ORIGIN => ORIGIN,
            CALLER => CALLER,
            CALLVALUE => CALLVALUE,
            CALLDATALOAD => CALLDATALOAD,
            CALLDATASIZE => CALLDATASIZE,
            CALLDATACOPY => CALLDATACOPY,
            CODESIZE => CODESIZE,
            CODECOPY => CODECOPY,
            GASPRICE => GASPRICE,
            EXTCODESIZE => EXTCODESIZE,
            EXTCODECOPY => EXTCODECOPY,
            RETURNDATASIZE => RETURNDATASIZE,
            RETURNDATACOPY => RETURNDATACOPY,
            EXTCODEHASH => EXTCODEHASH,
            // 40s: Block Information
            BLOCKHASH => BLOCKHASH,
            COINBASE => COINBASE,
            TIMESTAMP => TIMESTAMP,
            NUMBER => NUMBER,
            DIFFICULTY => DIFFICULTY,
            GASLIMIT => GASLIMIT,
            CHAINID => CHAINID,
            SELFBALANCE => SELFBALANCE,
            // 50s: Stack, Memory, Storage and Flow Operations
            POP => POP,
            MLOAD => MLOAD,
            MSTORE => MSTORE,
            MSTORE8 => MSTORE8,
            SLOAD => SLOAD,
            SSTORE => SSTORE,
            JUMP => JUMP,
            JUMPI => JUMPI,
            PC => PC,
            MSIZE => MSIZE,
            GAS => GAS,
            JUMPDEST => JUMPDEST,
            RJUMP(lab) => {
                match mapper(lab) {
                    Some(loffset) => RJUMP(to_rel_offset(offset,loffset)),
                    None => {
                        return Err(());
                    }
                }
            }
            RJUMPI(lab) => {
                match mapper(lab) {
                    Some(loffset) => RJUMPI(to_rel_offset(offset,loffset)),
                    None => {
                        return Err(());
                    }
                }
            }
            // 60s & 70s: Push Operations
            PUSH(bs) => PUSH(bs.clone()),
            PUSHL(large,lab) => {
                match mapper(lab) {
                    Some(loffset) => PUSH(to_abs_bytes(*large,loffset)),
                    None => {
                        return Err(());
                    }
                }
            }
            LABEL(_) => {
                return Ok(None);
            }
            // 80s: Duplication Operations
            DUP(n) => DUP(*n),
            // 90s: Swap Operations
            SWAP(n) => SWAP(*n),
            // a0s: Log Operations
            LOG(n) => LOG(*n),
            // f0s: System Operations
            CREATE => CREATE,
            CALL => CALL,
            CALLCODE => CALLCODE,
            RETURN => RETURN,
            DELEGATECALL => DELEGATECALL,
            CREATE2 => CREATE2,
            STATICCALL => STATICCALL,
            REVERT => REVERT,
            INVALID => INVALID,
            SELFDESTRUCT => SELFDESTRUCT,
            DATA(bs) => DATA(bs.clone())
        };
        Ok(Some(insn))
    }
}

/// Calculate the relative offset for a given branch target expressed
/// as an _abstolute byte offset_ from the program counter position
/// where the instruction in question is being instantiated.
fn to_rel_offset(pc: usize, target: usize) -> i16 {
    let mut n = target as isize;
    n -= pc as isize;
    // Following should always be true!
    n as i16
}

/// Calculate the variable bytes for an absolute branch target.
fn to_abs_bytes(large: bool, target: usize) -> Vec<u8> {
    if large || target > 255 {
        vec![(target / 256) as u8, (target % 256) as u8]
    } else {
        vec![target as u8]
    }
}
