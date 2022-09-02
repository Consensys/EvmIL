use crate::{Bytecode,Instruction};

type Result = std::result::Result<(),CompileError>;

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum CompileError {
    /// An integer (or hex) literal is too large (i.e. exceeds `2^256`).
    LiteralOverflow,
    /// Attempt to read from an invalid memory region.
    InvalidMemoryAccess,
    /// Attempt to write something which doesn't exist, or is not an lval.
    InvalidLVal
}

// ============================================================================
// Terms
// ============================================================================

#[derive(Clone)]
pub enum Term {
    // Statements
    Assert(Box<Term>),
    Assignment(Box<Term>,Box<Term>),
    // Expressions
    Binary(BinOp,Box<Term>,Box<Term>),
    ArrayAccess(Box<Term>,Box<Term>),
    MemoryAccess(Region),
    // Values
    Int(Vec<u8>),
    Hex(Vec<u8>),
}

impl Term {
    pub fn translate(&self, code: &mut Bytecode) -> Result {
        match self {
            // Statements
            Term::Assert(e) => translate_assert(e,code),
            Term::Assignment(e1,e2) => translate_assignment(e1,e2,code),
            // Expressions
            Term::Binary(bop,e1,e2) => translate_binary(*bop,e1,e2,code),
            Term::ArrayAccess(src,index) => translate_array_access(src,index,code),
            Term::MemoryAccess(_) => Err(CompileError::InvalidMemoryAccess),
            // Values
            Term::Int(bytes) => translate_literal(bytes,10,code),
            Term::Hex(bytes) => translate_literal(bytes,16,code),
            //
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

fn translate_assignment(lhs: &Term, rhs: &Term, code: &mut Bytecode) -> Result {
    // Translate value being assigned
    rhs.translate(code)?;
    // Translate assignent itself
    match lhs {
        Term::ArrayAccess(src,idx) => {
            translate_assignment_array(&src,&idx,code)?;
        }
        _ => {
            return Err(CompileError::InvalidLVal);
        }
    }
    //
    Ok(())
}

fn translate_assignment_array(src: &Term, index: &Term, code: &mut Bytecode) -> Result {
    match src {
        Term::MemoryAccess(r) => {
            translate_assignment_memory(*r,index,code)
        }
        _ => {
            Err(CompileError::InvalidMemoryAccess)
        }
    }
}

fn translate_assignment_memory(region: Region, address: &Term, code: &mut Bytecode) -> Result {
    // Translate index expression
    address.translate(code)?;
    // Dispatch based on region
    match region {
        Region::Memory => {
            code.push(Instruction::MSTORE);
        }
        Region::Storage => {
            code.push(Instruction::SSTORE);
        }
        _ => {
            return Err(CompileError::InvalidMemoryAccess);
        }
    };
    //
    Ok(())
}

// ============================================================================
// Binary Expressions
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
fn translate_binary(bop: BinOp, lhs: &Term, rhs: &Term, code: &mut Bytecode) -> Result {
    match bop {
        BinOp::LogicalAnd | BinOp::LogicalOr => {
            translate_logical_connective(bop,lhs,rhs,code)
        }
        _ => {
            translate_binary_arithmetic(bop,lhs,rhs,code)
        }
    }
}

/// Translate one of the logical connectives (e.g. `&&` or `||`).
/// These are more challenging than standard binary operators because
/// they exhibit _short circuiting behaviour_.
fn translate_logical_connective(bop: BinOp, lhs: &Term, rhs: &Term,
                                code: &mut Bytecode) -> Result {
    lhs.translate(code)?;
    code.push(Instruction::DUP(1));
    if bop == BinOp::LogicalAnd {
        code.push(Instruction::ISZERO);
    }
    // Allocate fresh label
    let lab = code.fresh_label();
    code.push(Instruction::PUSHL(lab));
    code.push(Instruction::JUMPI);
    code.push(Instruction::POP);
    rhs.translate(code)?;
    code.push(Instruction::JUMPDEST(lab));
    // Done
    Ok(())
}

/// Translate a binary arithmetic operation or comparison.  This is
/// pretty straightforward, as we just load items on the stack and
/// perform the op.  Observe that the right-hand side is loaded onto
/// the stack first.
fn translate_binary_arithmetic(bop: BinOp, lhs: &Term, rhs: &Term, code: &mut Bytecode) -> Result {
    rhs.translate(code)?;
    lhs.translate(code)?;
    //
    match bop {
        // standard
        BinOp::Add => code.push(Instruction::ADD),
        BinOp::Subtract => code.push(Instruction::SUB),
        BinOp::Divide => code.push(Instruction::DIV),
        BinOp::Multiply => code.push(Instruction::MUL),
        BinOp::Remainder => code.push(Instruction::MOD),
        BinOp::Equals => code.push(Instruction::EQ),
        BinOp::LessThan => code.push(Instruction::LT),
        BinOp::GreaterThan => code.push(Instruction::GT),
        // non-standard
        BinOp::NotEquals => {
            code.push(Instruction::EQ);
            code.push(Instruction::ISZERO);
        }
        BinOp::LessThanOrEquals => {
            code.push(Instruction::GT);
            code.push(Instruction::ISZERO);
        }
        BinOp::GreaterThanOrEquals => {
            code.push(Instruction::LT);
            code.push(Instruction::ISZERO);
        }
        _ => {
            unreachable!();
        }
    }
    //
    Ok(())
}

// ============================================================================
// Array Access Expressions
// ============================================================================

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Region {
    Memory,
    Storage,
    CallData
}

/// Translate an array access of the form `src[index]`.  The actual
/// form of the translation depends on whether its a direct access
/// (e.g. to storage or memory), or indirect (e.g. via a pointer to
/// memory).
fn translate_array_access(src: &Term, index: &Term, code:
                          &mut Bytecode) -> Result {
    match src {
        Term::MemoryAccess(r) => {
            translate_memory_access(*r,index,code)
        }
        _ => {
            Err(CompileError::InvalidMemoryAccess)
        }
    }
}

fn translate_memory_access(region: Region, index: &Term, code:
                           &mut Bytecode) -> Result {
    // Translate index expression
    index.translate(code)?;
    // Dispatch based on region
    match region {
        Region::Memory => {
            code.push(Instruction::MLOAD);
        }
        Region::Storage => {
            code.push(Instruction::SLOAD);
        }
        Region::CallData => {
            code.push(Instruction::CALLDATALOAD);
        }
    }
    //
    Ok(())
}

// ============================================================================
// Values
// ============================================================================

fn translate_literal(digits: &[u8], radix: u32, code: &mut Bytecode) -> Result {
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
        code.push(Instruction::PUSH(bytes));
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
