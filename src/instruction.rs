// ============================================================================
// Label Offsets
// ============================================================================

/// Used to simplify calculation of label offsets.
#[derive(PartialEq,Copy,Clone)]
pub struct Offset(pub u16);

impl Offset {
    /// Determine the width of this offset (in bytes).
    pub fn width(&self) -> u16 {
        if self.0 > 255 { 2 } else { 1 }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if self.0 > 255 {
            vec![(self.0/256) as u8,(self.0%256) as u8]
        } else {
            vec![self.0 as u8]
        }
    }
}

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
    InvalidLabelOffset
}

// ============================================================================
// Bytecode Instructions
// ============================================================================

#[derive(Debug,PartialEq)]
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
    // 10s: Comparison & Bitwise Logic Operations
    // 20s: Keccak256
    // 30s: Environmental Information
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    // 40s: Block Information
    // 50s: Stack, Memory, Storage and Flow Operations
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    JUMPDEST(usize),
    // 60 & 70s: Push Operations
    PUSH(Vec<u8>),
    PUSHL(usize), // Push label offset.
    // 80s: Duplicate Operations
    DUP(u8),
    // 90s: Exchange Operations
    SWAP(u8),
    // a0s: Logging Operations
    LOG(u8),
    // f0s: System Operations
    RETURN,
    // ..
    REVERT,
    INVALID
}

impl Instruction {

    /// Encode an instruction into a byte sequence, assuming a given
    /// set of label offsets.
    pub fn encode(&self, offsets: &[Offset], bytes: &mut Vec<u8>) -> Result<(),Error> {
        // Push opcode
        bytes.push(self.opcode(&offsets)?);
        // Push operands (if applicable)
        match self {
            Instruction::PUSH(args) => {
                bytes.extend(args);
            }
            Instruction::PUSHL(idx) => {
                bytes.extend(offsets[*idx].to_bytes());
            }
            _ => {
                // All other instructions have no operands.
            }
        }
        //
        Ok(())
    }

    /// Determine the length of this instruction (in bytes) assuming a
    /// given set of label offsets.
    pub fn length(&self, offsets: &[Offset]) -> usize {
        let operands = match self {
            // Push instructions
            Instruction::PUSH(bs) => bs.len(),
            Instruction::PUSHL(lab) => {
                todo!("implement me");
            }
            // Default case
            _ => 0
        };
        operands + 1
    }

    /// Determine the opcode for a given instruction.  In many cases,
    /// this is a straightforward mapping.  However, in other cases,
    /// its slightly more involved as a calculation involving the
    /// operands is required.
    pub fn opcode(&self, offsets: &[Offset]) -> Result<u8,Error> {
        let op = match self {
            // 0s: Stop and Arithmetic Operations
            Instruction::STOP => 0x00,
            Instruction::ADD => 0x01,
            Instruction::MUL => 0x02,
            Instruction::SUB => 0x03,
            Instruction::DIV => 0x04,
            Instruction::SDIV => 0x05,
            Instruction::MOD => 0x06,
            Instruction::SMOD => 0x07,
            Instruction::ADDMOD => 0x08,
            Instruction::MULMOD => 0x09,
            Instruction::EXP => 0x0a,
            Instruction::SIGNEXTEND => 0x0b,
            // 10s: Comparison & Bitwise Logic Operations
            Instruction::LT => 0x10,
            Instruction::GT => 0x11,
            Instruction::SLT => 0x12,
            Instruction::SGT => 0x13,
            Instruction::EQ => 0x14,
            Instruction::ISZERO => 0x15,
            Instruction::AND => 0x16,
            Instruction::OR => 0x17,
            Instruction::XOR => 0x18,
            Instruction::NOT => 0x19,
            Instruction::BYTE => 0x1a,
            Instruction::SHL => 0x1b,
            Instruction::SHR => 0x1c,
            Instruction::SAR => 0x1d,
            // 30s: Environmental Information
            Instruction::CALLDATALOAD => 0x35,
            Instruction::CALLDATASIZE => 0x36,
            Instruction::CALLDATACOPY => 0x37,
            // 50s: Stack, Memory, Storage and Flow Operations
            Instruction::POP => 0x50,
            Instruction::MLOAD => 0x51,
            Instruction::MSTORE => 0x52,
            Instruction::MSTORE8 => 0x53,
            Instruction::SLOAD => 0x54,
            Instruction::SSTORE => 0x55,
            Instruction::JUMP => 0x56,
            Instruction::JUMPI => 0x57,
            // ...
            Instruction::JUMPDEST(_) => 0x5b,
            //
            // 60s & 70s: Push Operations
            Instruction::PUSH(bs) => {
                if bs.len() == 0 || bs.len() > 32 {
                    return Err(Error::InvalidPush);
                } else {
                    (0x5f + bs.len()) as u8
                }
            }
            //
            Instruction::PUSHL(lab) => {
                let offset = &offsets[*lab];
                if offset.width() == 2 { 0x61 }
                else { 0x60 }
            }
            // 80s: Duplication Operations
            Instruction::DUP(n) => {
                if *n == 0 || *n > 32 {
                    return Err(Error::InvalidDup);
                }
                0x7f + n
            }
            // f0s: System Operations
            Instruction::RETURN => 0xf3,
            Instruction::REVERT => 0xfd,
            Instruction::INVALID => 0xfe,
            //
            _ => {
                panic!("Invalid instruction ({:?})",self);
            }
        };
        //
        Ok(op)
    }

    /// Decode the next instruction in a given sequence of bytes.
    /// Observe that this never returns a `PUSHL` instruction.  This
    /// is because it cannot determine whether a given operand will be
    /// used as a jump destination.  A separate analysis is required
    /// to "lift" `PUSH` instructions to `PUSHL` instructions.
    pub fn decode(bytes: &[u8]) -> Instruction {
        let opcode = bytes[0];
        //
        let insn = match opcode {
            // 0s: Stop and Arithmetic Operations
            0x00 => Instruction::STOP,
            0x01 => Instruction::ADD,
            0x02 => Instruction::MUL,
            0x03 => Instruction::SUB,
            0x04 => Instruction::DIV,
            0x05 => Instruction::SDIV,
            0x06 => Instruction::MOD,
            0x07 => Instruction::SMOD,
            0x08 => Instruction::ADDMOD,
            0x09 => Instruction::MULMOD,
            0x0a => Instruction::EXP,
            0x0b => Instruction::SIGNEXTEND,
            // Unknown
            _ => Instruction::INVALID
        };
        //
        insn
    }
}
