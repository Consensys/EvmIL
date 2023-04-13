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
use crate::il::{BinOp, Region, Term};
use crate::evm::AbstractInstruction::*;
use crate::evm::{AssemblyInstruction,Bytecode,Instruction};
use crate::util::*;


type Result = std::result::Result<(), CompilerError>;

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum CompilerError {
    /// An integer (or hex) literal is too large (i.e. exceeds `2^256`).
    LiteralOverflow,
    /// Attempt to read from an invalid memory region.
    InvalidMemoryAccess,
    /// Attempt to write something which doesn't exist, or is not an lval.
    InvalidLVal,
}

// ============================================================================
// Compiler
// ============================================================================

pub struct Compiler {
    /// The assembly being constructed by this compiler.
    bytecode: Vec<AssemblyInstruction>,
    /// Counts the number of labels in use
    labels: usize
}

impl Compiler {
    pub fn new() -> Self {
        Self { bytecode: Vec::new(),labels: 0 }
    }

    /// Extract the bytecode for the compiled terms.
    pub fn to_bytecode(self) -> Bytecode<Instruction> {
        // NOTE: following is safe since the assembly was constructed
        // by this compiler, and therefore is assumed to be well
        // formed.
        //self.bytecode.to_bytecode().unwrap()
        todo!()
    }

    pub fn translate(&mut self, term: &Term) -> Result {
        match term {
            // Statements
            Term::Assert(e) => self.translate_assert(e),
            Term::Assignment(e1, e2) => self.translate_assignment(e1, e2),
            Term::Fail => self.translate_fail(),
            Term::Goto(l) => self.translate_goto(l),
            Term::IfGoto(e, l) => self.translate_ifgoto(e, l),
            Term::Label(l) => self.translate_label(l),
            Term::Return(es) => self.translate_return(es),
            Term::Revert(es) => self.translate_revert(es),
            Term::Succeed(es) => self.translate_succeed(es),
            Term::Stop => self.translate_stop(),
            // Expressions
            Term::Binary(bop, e1, e2) => self.translate_binary(*bop, e1, e2),
            Term::Call(n,es) => self.translate_call(n,es),
            Term::ArrayAccess(src, index) => self.translate_array_access(src, index),
            Term::MemoryAccess(_) => Err(CompilerError::InvalidMemoryAccess),
            // Values
            Term::Int(bytes) => self.translate_literal(bytes, 10),
            Term::Hex(bytes) => self.translate_literal(bytes, 16),
            //
        }
    }

    fn fresh_label(&mut self) -> String {
        let lab = format!("lab{}",self.labels);
        self.labels += 1;
        lab
    }

    // ============================================================================
    // Statements
    // ============================================================================

    fn translate_assert(&mut self, expr: &Term) -> Result {
        // Allocate labels for true/false outcomes
        let lab = self.fresh_label();
        // Translate conditional branch
        self.translate_conditional(expr, Some(&lab), None)?;
        // False branch
        self.bytecode.push(INVALID);
        // True branch
        self.bytecode.push(LABEL(lab));
        self.bytecode.push(JUMPDEST);
        //
        Ok(())
    }

    fn translate_assignment(&mut self, lhs: &Term, rhs: &Term) -> Result {
        // Translate value being assigned
        self.translate(rhs)?;
        // Translate assignent itself
        match lhs {
            Term::ArrayAccess(src, idx) => {
                self.translate_assignment_array(&src, &idx)?;
            }
            _ => {
                return Err(CompilerError::InvalidLVal);
            }
        }
        //
        Ok(())
    }

    fn translate_assignment_array(&mut self, src: &Term, index: &Term) -> Result {
        match src {
            Term::MemoryAccess(r) => self.translate_assignment_memory(*r, index),
            _ => Err(CompilerError::InvalidMemoryAccess),
        }
    }

    fn translate_assignment_memory(&mut self, region: Region, address: &Term) -> Result {
        // Translate index expression
        self.translate(address)?;
        // Dispatch based on region
        match region {
            Region::Memory => self.bytecode.push(MSTORE),
            Region::Storage => self.bytecode.push(SSTORE),
            _ => {
                return Err(CompilerError::InvalidMemoryAccess);
            }
        };
        //
        Ok(())
    }

    fn translate_call(&mut self, name: &str, exprs: &[Term]) -> Result {
        let retlab = self.fresh_label();
        // Translate arguments
        for e in exprs { self.translate(e)?; }
        // Push return address
        self.bytecode.push(PUSHL(retlab.clone()));
        // Push function address
        self.bytecode.push(PUSHL(name.to_string()));
        // Perform jump
        self.bytecode.push(JUMP);
        // Identify return point
        self.bytecode.push(LABEL(retlab));
        self.bytecode.push(JUMPDEST);
        Ok(())
    }

    fn translate_fail(&mut self) -> Result {
        self.bytecode.push(INVALID);
        Ok(())
    }

    fn translate_goto(&mut self, label: &str) -> Result {
        // Translate unconditional branch
        self.bytecode.push(PUSHL(label.to_string()));
        self.bytecode.push(JUMP);
        //
        Ok(())
    }

    fn translate_ifgoto(&mut self, expr: &Term, label: &str) -> Result {
        // Translate conditional branch
        self.translate_conditional(expr, Some(label), None)
    }

    fn translate_label(&mut self, label: &str) -> Result {
        // Mark the label
        self.bytecode.push(LABEL(label.to_string()));
        self.bytecode.push(JUMPDEST);
        // Done
        Ok(())
    }

    fn translate_return(&mut self, exprs: &[Term]) -> Result {
        if exprs.len() > 0 {
            // Translate each expression (except first)
            for i in 1..exprs.len() { self.translate(&exprs[i])?; }
            // Translate first expression
            self.translate(&exprs[0])?;
            // Swap with returna address
            self.bytecode.push(SWAP(exprs.len() as u8));
        }
        // A return statement is just an unconditional jump
        self.bytecode.push(JUMP);
        //
        Ok(())
    }

    fn translate_revert(&mut self, exprs: &[Term]) -> Result {
        self.translate_succeed_revert(REVERT, exprs)
    }

    fn translate_succeed(&mut self, exprs: &[Term]) -> Result {
        if exprs.len() == 0 {
            self.bytecode.push(STOP);
            Ok(())
        } else {
            self.translate_succeed_revert(RETURN, exprs)
        }
    }

    fn translate_succeed_revert(&mut self, insn: AssemblyInstruction, exprs: &[Term]) -> Result {
        if exprs.len() == 0 {
            self.bytecode.push(PUSH(vec![0]));
            self.bytecode.push(PUSH(vec![0]));
        } else {
            for i in 0..exprs.len() {
                let addr = (i * 0x20) as u128;
                self.translate(&exprs[i])?;
                self.bytecode.push(make_push(addr)?);
                self.bytecode.push(MSTORE);
            }
            let len = (exprs.len() * 0x20) as u128;
            self.bytecode.push(PUSH(vec![0]));
            self.bytecode.push(make_push(len)?);
        }
        self.bytecode.push(insn);
        Ok(())
    }

    fn translate_stop(&mut self) -> Result {
        self.bytecode.push(STOP);
        Ok(())
    }

    // ============================================================================
    // Conditional Expressions
    // ============================================================================

    /// Translate a conditional expression using _short circuiting_
    /// semantics.  Since implementing short circuiting requires
    /// branching, we can exploit this to optimise other translations
    /// and reduce the number of branches required.  To do that, this
    /// method requires either a `true` target or a `false` target.
    /// If a `true` target is provided then, if the condition
    /// evaluates to something other than `0` (i.e. is `true`),
    /// control is transfered to this target.  Likewise, if the
    /// condition evaluates to `0` (i.e. `false`) the control is
    /// transfered to the `false` target.
    fn translate_conditional(&mut self, expr: &Term, true_lab: Option<&str>, false_lab: Option<&str>) -> Result {
        match expr {
            Term::Binary(BinOp::LogicalAnd, l, r) => {
                self.translate_conditional_conjunct(l, r, true_lab, false_lab)
            }
            Term::Binary(BinOp::LogicalOr, l, r) => {
                self.translate_conditional_disjunct(l, r, true_lab, false_lab)
            }
            _ => self.translate_conditional_other(expr, true_lab, false_lab),
        }
    }

    /// Translate a logical conjunction as a conditional. Since
    /// such connectives require short circuiting, these must be
    /// implementing using branches.
    fn translate_conditional_conjunct(&mut self, lhs: &Term, rhs: &Term, true_lab: Option<&str>, false_lab: Option<&str>) -> Result {
        match (true_lab, false_lab) {
            (Some(_), None) => {
                // Harder case
                let lab = self.fresh_label();
                self.translate_conditional(lhs, None, Some(&lab))?;
                self.translate_conditional(rhs, true_lab, None)?;
                self.bytecode.push(LABEL(lab));
                self.bytecode.push(JUMPDEST);
            }
            (None, Some(_)) => {
                // Easy case
                self.translate_conditional(lhs, None, false_lab)?;
                self.translate_conditional(rhs, true_lab, false_lab)?;
            }
            (_, _) => unreachable!(),
        }
        // Done
        Ok(())
    }

    /// Translate a logical disjunction as a conditional. Since
    /// such connectives require short circuiting, these must be
    /// implementing using branches.
    fn translate_conditional_disjunct(&mut self, lhs: &Term, rhs: &Term, true_lab: Option<&str>, false_lab: Option<&str>) -> Result {
        match (true_lab, false_lab) {
            (None, Some(_)) => {
                // Harder case
                let lab = self.fresh_label();
                self.translate_conditional(lhs, Some(&lab), None)?;
                self.translate_conditional(rhs, None, false_lab)?;
                self.bytecode.push(LABEL(lab));
                self.bytecode.push(JUMPDEST);
            }
            (Some(_), None) => {
                // Easy case
                self.translate_conditional(lhs, true_lab, None)?;
                self.translate_conditional(rhs, true_lab, false_lab)?;
            }
            (_, _) => unreachable!(),
        }
        // Done
        Ok(())
    }

    /// Translate a conditional expression which cannot be translated
    /// by exploiting branches.  In such case, we have to generate the
    /// boolean value and dispatch based on that.
    fn translate_conditional_other(&mut self, expr: &Term, true_lab: Option<&str>, false_lab: Option<&str>) -> Result {
        // Translate conditional expression
        self.translate(expr)?;
        //
        match (true_lab, false_lab) {
            (Some(lab), None) => {
                self.bytecode.push(LABEL(lab.to_string()));
                self.bytecode.push(JUMPI);
            }
            (None, Some(lab)) => {
                self.bytecode.push(ISZERO);
                self.bytecode.push(LABEL(lab.to_string()));
                self.bytecode.push(JUMPI);
            }
            (_, _) => {
                unreachable!("")
            }
        }
        //
        Ok(())
    }

    // ============================================================================
    // Binary Expressions
    // ============================================================================

    /// Translate a binary operation.  Observe that logical operations
    /// exhibit _short-circuit behaviour_.
    fn translate_binary(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        match bop {
            BinOp::LogicalAnd | BinOp::LogicalOr => {
                self.translate_logical_connective(bop, lhs, rhs)
            }
            _ => self.translate_binary_arithmetic(bop, lhs, rhs),
        }
    }

    /// Translate one of the logical connectives (e.g. `&&` or `||`).
    /// These are more challenging than standard binary operators because
    /// they exhibit _short circuiting behaviour_.
    fn translate_logical_connective(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        self.translate(lhs)?;
        self.bytecode.push(DUP(1));
        if bop == BinOp::LogicalAnd {
            self.bytecode.push(ISZERO);
        }
        // Allocate fresh label
        let lab = self.fresh_label();
        self.bytecode.push(PUSHL(lab.clone()));
        self.bytecode.push(JUMPI);
        self.bytecode.push(POP);
        self.translate(rhs)?;
        self.bytecode.push(LABEL(lab));
        self.bytecode.push(JUMPDEST);
        // Done
        Ok(())
    }

    /// Translate a binary arithmetic operation or comparison.  This is
    /// pretty straightforward, as we just load items on the stack and
    /// perform the op.  Observe that the right-hand side is loaded onto
    /// the stack first.
    fn translate_binary_arithmetic(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        self.translate(rhs)?;
        self.translate(lhs)?;
        //
        match bop {
            // standard
            BinOp::Add => self.bytecode.push(ADD),
            BinOp::Subtract => self.bytecode.push(SUB),
            BinOp::Divide => self.bytecode.push(DIV),
            BinOp::Multiply => self.bytecode.push(MUL),
            BinOp::Remainder => self.bytecode.push(MOD),
            BinOp::Equals => self.bytecode.push(EQ),
            BinOp::LessThan => self.bytecode.push(LT),
            BinOp::GreaterThan => self.bytecode.push(GT),
            // non-standard
            BinOp::NotEquals => {
                self.bytecode.push(EQ);
                self.bytecode.push(ISZERO);
            }
            BinOp::LessThanOrEquals => {
                self.bytecode.push(GT);
                self.bytecode.push(ISZERO);
            }
            BinOp::GreaterThanOrEquals => {
                self.bytecode.push(LT);
                self.bytecode.push(ISZERO);
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

    /// Translate an array access of the form `src[index]`.  The actual
    /// form of the translation depends on whether its a direct access
    /// (e.g. to storage or memory), or indirect (e.g. via a pointer to
    /// memory).
    fn translate_array_access(&mut self, src: &Term, index: &Term) -> Result {
        match src {
            Term::MemoryAccess(r) => self.translate_memory_access(*r, index),
            _ => Err(CompilerError::InvalidMemoryAccess),
        }
    }

    fn translate_memory_access(&mut self, region: Region, index: &Term) -> Result {
        // Translate index expression
        self.translate(index)?;
        // Dispatch based on region
        match region {
            Region::Memory => {
                self.bytecode.push(MLOAD);
            }
            Region::Storage => {
                self.bytecode.push(SLOAD);
            }
            Region::CallData => {
                self.bytecode.push(CALLDATALOAD);
            }
        }
        //
        Ok(())
    }

    // ============================================================================
    // Values
    // ============================================================================

    fn translate_literal(&mut self, digits: &[u8], radix: u32) -> Result {
        let val = from_be_digits(digits, radix);
        self.bytecode.push(make_push(val)?);
        Ok(())
    }
}

// ===================================================================
// Traits
// ===================================================================

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl TryFrom<&[Term]> for Bytecode<Instruction> {
    type Error = CompilerError;

    fn try_from(terms: &[Term]) -> std::result::Result<Bytecode<Instruction>, Self::Error> {
        try_from(terms)
    }
}

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl<const N: usize> TryFrom<&[Term; N]> for Bytecode<Instruction> {
    type Error = crate::il::CompilerError;

    fn try_from(terms: &[Term; N]) -> std::result::Result<Bytecode<Instruction>, Self::Error> {
        try_from(terms)
    }
}

// ===================================================================
// Helpers
// ===================================================================

fn try_from(terms: &[Term]) -> std::result::Result<Bytecode<Instruction>, CompilerError> {
    let mut compiler = Compiler::new();
    // Translate statements one-by-one
    for t in terms {
        compiler.translate(t)?;
    }
    // Done
    Ok(compiler.to_bytecode())
}

/// Construct a push instruction from a value.
fn make_push(val: u128) -> std::result::Result<AssemblyInstruction, CompilerError> {
    let bytes = to_be_bytes(val);
    // Sanity check size of literal
    if bytes.len() > 32 {
        // Too big!!
        Err(CompilerError::LiteralOverflow)
    } else {
        Ok(PUSH(bytes))
    }
}
