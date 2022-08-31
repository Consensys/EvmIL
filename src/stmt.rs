use crate::{Bytecode,Expr};

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

}
