use crate::{Bytecode,Instruction,Value};

pub enum Expr {
    Literal(Value),
    Binary(BinOp,Box<Expr>,Box<Expr>)
}

impl Expr {
    /// Translate a given expression at a given stack depth.
    pub fn translate(&self, bytecode: &mut Bytecode) {
        match self {
            Expr::Literal(v) => {
                v.translate(bytecode);
            }
            Expr::Binary(bop,e1,e2) => {
                translate_binary(*bop,e1,e2,bytecode);
            }
        }
    }
}

// ============================================================================
// Binary operators
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
fn translate_binary(bop: BinOp, lhs: &Expr, rhs: &Expr, bytecode: &mut Bytecode) {
    lhs.translate(bytecode);
    rhs.translate(bytecode);
    bytecode.push(Instruction::from(bop));
}
