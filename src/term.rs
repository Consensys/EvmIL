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
    let lab = code.fresh_label();
    code.push(Instruction::PUSHL(lab));
    code.push(Instruction::JUMPI);
    code.push(Instruction::INVALID);
    code.push(Instruction::JUMPDEST(lab));
    //
    Ok(())
}

// ============================================================================
// Expressions
// ============================================================================

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum BinOp {
    // Arithmetic
    Add,
    Subtract,
    Divide,
    Multiply,
    Remainder,
    // Comparators
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    // Logical
    LogicalAnd,
    LogicalOr
}

/// Translate a binary operation.  Observe that logical operations
/// exhibit _short-circuit behaviour_.
fn translate_binary(bop: BinOp, lhs: &Term, rhs: &Term, bytecode: &mut Bytecode) -> Result {
    match bop {
        BinOp::LogicalAnd | BinOp::LogicalOr => {
            translate_logical_connective(bop,lhs,rhs,bytecode)
        }
        _ => {
            translate_binary_arithmetic(bop,lhs,rhs,bytecode)
        }
    }
}

/// Translate one of the logical connectives (e.g. `&&` or `||`).
/// These are more challenging than standard binary operators because
/// they exhibit _short circuiting behaviour_.
fn translate_logical_connective(bop: BinOp, lhs: &Term, rhs: &Term,
                                bytecode: &mut Bytecode) -> Result {
    lhs.translate(bytecode)?;
    bytecode.push(Instruction::DUP(1));
    if bop == BinOp::LogicalAnd {
        bytecode.push(Instruction::ISZERO);
    }
    // Allocate fresh label
    let lab = bytecode.fresh_label();
    bytecode.push(Instruction::PUSHL(lab));
    bytecode.push(Instruction::JUMPI);
    bytecode.push(Instruction::POP);
    rhs.translate(bytecode)?;
    bytecode.push(Instruction::JUMPDEST(lab));
    // Done
    Ok(())
}

/// Translate a binary arithmetic operation or comparison.  This is
/// pretty straightforward, as we just load items on the stack and
/// perform the op.  Observe that the right-hand side is loaded onto
/// the stack first.
fn translate_binary_arithmetic(bop: BinOp, lhs: &Term, rhs: &Term, bytecode: &mut Bytecode) -> Result {
    rhs.translate(bytecode)?;
    lhs.translate(bytecode)?;
    //
    match bop {
        // standard
        BinOp::Add => bytecode.push(Instruction::ADD),
        BinOp::Subtract => bytecode.push(Instruction::SUB),
        BinOp::Divide => bytecode.push(Instruction::DIV),
        BinOp::Multiply => bytecode.push(Instruction::MUL),
        BinOp::Remainder => bytecode.push(Instruction::MOD),
        BinOp::Equals => bytecode.push(Instruction::EQ),
        BinOp::LessThan => bytecode.push(Instruction::LT),
        BinOp::GreaterThan => bytecode.push(Instruction::GT),
        // non-standard
        BinOp::NotEquals => {
            bytecode.push(Instruction::EQ);
            bytecode.push(Instruction::ISZERO);
        }
        BinOp::LessThanOrEquals => {
            bytecode.push(Instruction::GT);
            bytecode.push(Instruction::ISZERO);
        }
        BinOp::GreaterThanOrEquals => {
            bytecode.push(Instruction::LT);
            bytecode.push(Instruction::ISZERO);
        }
        _ => {
            unreachable!();
        }
    }
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
