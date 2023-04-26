use std::fmt;
use std::fmt::{Debug};
use crate::util::{ToHexString};

/// Provides a mechanism by which an instruction can be parameterised
/// to support different forms of control flow.
pub trait InstructionOperands {
    /// Identifies the type of 16bit relative offsets.
    type RelOffset16 : fmt::Display+Debug;
    /// Identifies the type for _push label_ instructions.
    type PushLabel : fmt::Display+Debug;
    /// Identifies the type for _label_ instructions.
    type Label : fmt::Display+Debug;
}

/// A void operand is used to signal that something is impossible
/// (i.e. because this instruction cannot be used in a particular
/// setting, etc).
#[derive(Clone,Debug,PartialEq)]
pub enum VoidOperand{}

impl fmt::Display for VoidOperand {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

/// An abstract instruction is parameterised over the type of _control
/// flow_ it supports.  In particular, _concrete_ instructions are
/// fully instantiated with specific branch targets.  In contract,
/// _labelled_ instructions employ symbolic labels instead of concrete
/// target information.  The primary purpose here is to distinguish
/// between instructions originating from bytes, versus those
/// originating from assembly language.
///
/// The intention is that all known instructions are represented here
/// in one place, rather than e.g. begin separated (somehow) by fork.
#[derive(Clone, Debug, PartialEq)]
pub enum AbstractInstruction<T:InstructionOperands> {
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
    RJUMP(T::RelOffset16),  // EIP4200
    RJUMPI(T::RelOffset16), // EIP4200
    // 60 & 70s: Push Operations
    PUSH(Vec<u8>),
    PUSHL(T::PushLabel),
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

impl<T:InstructionOperands> AbstractInstruction<T> {
    /// Determine whether or not control can continue to the next
    /// instruction.
    pub fn fallthru(&self) -> bool {
        match self {
            AbstractInstruction::DATA(_) => false,
            AbstractInstruction::INVALID => false,
            AbstractInstruction::JUMP => false,
            AbstractInstruction::RJUMP(_) => false,
            AbstractInstruction::STOP => false,
            AbstractInstruction::RETURN => false,
            AbstractInstruction::REVERT => false,
            AbstractInstruction::SELFDESTRUCT => false,
            _ => true,
        }
    }

    /// Determine whether or not this instruction can branch.  That
    /// is, whether or not it is a `JUMP` or `JUMPI` instruction.
    pub fn can_branch(&self) -> bool {
        match self {
            AbstractInstruction::JUMP => true,
            AbstractInstruction::JUMPI => true,
            AbstractInstruction::RJUMP(_) => true,
            AbstractInstruction::RJUMPI(_) => true,
            _ => false,
        }
    }
}

impl<T:InstructionOperands+Debug> fmt::Display for AbstractInstruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the default (debug) formatter.  Its only for certain
        // instructions that we need to do anything different.
        match self {
            AbstractInstruction::DATA(bytes) => {
                // Print bytes as hex string
                write!(f, "db {}", bytes.to_hex_string())
            }
            AbstractInstruction::DUP(n) => {
                write!(f, "dup{}",n)
            }
            AbstractInstruction::LABEL(lab) => {
                write!(f, "{lab}:")
            }
            AbstractInstruction::LOG(n) => {
                write!(f, "log{n}")
            }
            AbstractInstruction::JUMPDEST => {
                write!(f, "jumpdest")
            }
            AbstractInstruction::PUSH(bytes) => {
                // Convert bytes into hex string
                let hex = bytes.to_hex_string();
                // Print!
                write!(f, "push {}", hex)
            }
            AbstractInstruction::RJUMP(offset) => {
                write!(f, "rjump {offset}")
            }
            AbstractInstruction::RJUMPI(offset) => {
                write!(f, "rjumpi {offset}")
            }
            AbstractInstruction::SWAP(n) => {
                write!(f, "swap{n}")
            }
            _ => {
                let s = format!("{:?}",self).to_lowercase();
                write!(f, "{s}")
            }
        }
    }
}
