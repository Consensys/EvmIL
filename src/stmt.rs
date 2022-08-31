use crate::{Bytecode,Instruction,Expr};

pub enum Stmt {
    Assert(Expr)
}

impl Stmt {
    pub fn translate(&self, code: &mut Bytecode) {
        match self {
            Stmt::Assert(e) => {
                translate_assert(e,code);
            }
        }
    }
}

fn translate_assert(expr: &Expr, code: &mut Bytecode) {
    // Translate expression
    expr.translate(code);
    // Implement dynamic branching
    code.push(Instruction::PUSHL(0)); // FIXME: broken.
    code.push(Instruction::JUMPI);
    code.push(Instruction::INVALID);
    code.push(Instruction::JUMPDEST);
}
