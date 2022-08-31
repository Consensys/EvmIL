use crate::{Bytecode,CompileError,Instruction};

type Result = std::result::Result<(),CompileError>;

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
            Term::Int(bytes) => translate_literal(bytes,code),
            Term::Hex(bytes) => translate_literal(bytes,code),
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

fn translate_literal(bytes: &[u8], bytecode: &mut Bytecode) -> Result {
    bytecode.push(Instruction::PUSH(bytes.to_vec()));
    //
    Ok(())
}
