use crate::{Bytecode,CompileError,Instruction};

type Result = std::result::Result<(),CompileError>;

#[derive(Clone)]
pub enum Term {
    // Statements
    Assert(Box<Term>),
    // Expressions
    Binary(BinOp,Box<Term>,Box<Term>),
    // Values
    Int(Vec<u8>),
    Hex(Vec<u8>),
}

impl Term {
    pub fn translate(&self, code: &mut Bytecode) -> Result {
        match self {
            // Statements
            Term::Assert(e) => translate_assert(e,code),
            // Expressions
            Term::Binary(bop,e1,e2) => translate_binary(*bop,e1,e2,code),
            // Values
            Term::Int(bytes) => translate_literal(bytes,10,code),
            Term::Hex(bytes) => translate_literal(bytes,16,code),
        }
    }
}

// ============================================================================
// Statements
// ============================================================================

fn translate_assert(expr: &Term, code: &mut Bytecode) -> Result {
    // Translate expression
    expr.translate(code)?;
    // Implement dynamic branching
    code.push(Instruction::PUSHL(code.num_labels()));
    code.push(Instruction::JUMPI);
    code.push(Instruction::INVALID);
    code.push(Instruction::JUMPDEST);
    //
    Ok(())
}

// ============================================================================
// Expressions
// ============================================================================

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum BinOp {
    ADD,
    SUB,
    DIV,
    MUL
}

impl From<BinOp> for Instruction {
    fn from(bop: BinOp) -> Instruction {
        match bop {
            BinOp::ADD => Instruction::ADD,
            BinOp::SUB => Instruction::SUB,
            BinOp::DIV => Instruction::DIV,
            BinOp::MUL => Instruction::MUL
        }
    }
}

/// Translate a binary operation.  This is pretty straightforward, as
/// we just load items on the stack and perform the op.
fn translate_binary(bop: BinOp, lhs: &Term, rhs: &Term, bytecode: &mut Bytecode) -> Result {
    lhs.translate(bytecode)?;
    rhs.translate(bytecode)?;
    bytecode.push(Instruction::from(bop));
    //
    Ok(())
}


// ============================================================================
// Values
// ============================================================================

fn translate_literal(digits: &[u8], radix: u32, bytecode: &mut Bytecode) -> Result {
    let mut bytes : Vec<u8> = Vec::new();
    let mut val = from_be_digits(digits,radix);
    // Convert digits in a given radix into bytes (in little endian)
    if val == 0 {
        bytes.push(0);
    } else {
        while val != 0 {
            bytes.push((val % 256) as u8);
            val = val >> 8;
        }
    }
    // Convert from big endian to little endian format.
    bytes.reverse();
    // Sanity check size of literal
    if bytes.len() > 32 {
        // Too big!!
        Err(CompileError::LiteralOverflow)
    } else {
        bytecode.push(Instruction::PUSH(bytes));
        Ok(())
    }
}

/// Convert a sequence of digits into a u128.
fn from_be_digits(digits: &[u8], radix: u32) -> u128 {
    let mut acc : u128 = 0;
    let mut base : u128 = 1;
    //
    for i in (0..digits.len()).rev() {
        let d = digits[i] as u128;
        // NOTE: this could overflow.
        acc = acc + (d * base);
        if i > 0 {
            // NOTE: Following overflows on last iteration, so just
            // don't do it :)
            base = base * (radix as u128);
        }
    }
    // Done
    acc
}
