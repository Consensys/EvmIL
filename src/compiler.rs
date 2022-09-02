use crate::{BinOp,Bytecode,Instruction,Region,Term};

type Result = std::result::Result<(),Error>;

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum Error {
    /// An integer (or hex) literal is too large (i.e. exceeds `2^256`).
    LiteralOverflow,
    /// Attempt to read from an invalid memory region.
    InvalidMemoryAccess,
    /// Attempt to write something which doesn't exist, or is not an lval.
    InvalidLVal
}

// ============================================================================
// Compiler
// ============================================================================

pub struct Compiler<'a> {
    bytecode: &'a mut Bytecode
}

impl<'a> Compiler<'a> {
    pub fn new(bytecode: &'a mut Bytecode) -> Self {
        Self{bytecode}
    }

    pub fn translate(&mut self, term: &Term) -> Result {
        match term {
            // Statements
            Term::Assert(e) => self.translate_assert(e),
            Term::Assignment(e1,e2) => self.translate_assignment(e1,e2),
            Term::IfGoto(e,l) => self.translate_ifgoto(e,l),
            Term::Label(l) => self.translate_label(l),
            // Expressions
            Term::Binary(bop,e1,e2) => self.translate_binary(*bop,e1,e2),
            Term::ArrayAccess(src,index) => self.translate_array_access(src,index),
            Term::MemoryAccess(_) => Err(Error::InvalidMemoryAccess),
            // Values
            Term::Int(bytes) => self.translate_literal(bytes,10),
            Term::Hex(bytes) => self.translate_literal(bytes,16),
            //
        }
    }

    // ============================================================================
    // Statements
    // ============================================================================

    fn translate_assert(&mut self, expr: &Term) -> Result {
        // Translate expression
        self.translate(expr)?;
        // Implement dynamic branching
        let lab = self.bytecode.fresh_label();
        self.bytecode.push(Instruction::PUSHL(lab));
        self.bytecode.push(Instruction::JUMPI);
        self.bytecode.push(Instruction::INVALID);
        self.bytecode.push(Instruction::JUMPDEST(lab));
        //
        Ok(())
    }

    fn translate_assignment(&mut self, lhs: &Term, rhs: &Term) -> Result {
        // Translate value being assigned
        self.translate(rhs)?;
        // Translate assignent itself
        match lhs {
            Term::ArrayAccess(src,idx) => {
                self.translate_assignment_array(&src,&idx)?;
            }
            _ => {
                return Err(Error::InvalidLVal);
            }
        }
        //
        Ok(())
    }

    fn translate_assignment_array(&mut self, src: &Term, index: &Term) -> Result {
        match src {
            Term::MemoryAccess(r) => {
                self.translate_assignment_memory(*r,index)
            }
            _ => {
                Err(Error::InvalidMemoryAccess)
            }
        }
    }

    fn translate_assignment_memory(&mut self, region: Region, address: &Term) -> Result {
        // Translate index expression
        self.translate(address)?;
        // Dispatch based on region
        match region {
            Region::Memory => self.bytecode.push(Instruction::MSTORE),
            Region::Storage => self.bytecode.push(Instruction::SSTORE),
            _ => {
                return Err(Error::InvalidMemoryAccess);
            }
        };
        //
        Ok(())
    }

    fn translate_ifgoto(&mut self, cond: &Term, label: &str) -> Result {
        return Err(Error::InvalidMemoryAccess);
    }

    fn translate_label(&mut self, label: &str) -> Result {
        return Err(Error::InvalidMemoryAccess);
    }

    // ============================================================================
    // Binary Expressions
    // ============================================================================

    /// Translate a binary operation.  Observe that logical operations
    /// exhibit _short-circuit behaviour_.
    fn translate_binary(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        match bop {
            BinOp::LogicalAnd | BinOp::LogicalOr => {
                self.translate_logical_connective(bop,lhs,rhs)
            }
            _ => {
                self.translate_binary_arithmetic(bop,lhs,rhs)
            }
        }
    }

    /// Translate one of the logical connectives (e.g. `&&` or `||`).
    /// These are more challenging than standard binary operators because
    /// they exhibit _short circuiting behaviour_.
    fn translate_logical_connective(&mut self, bop: BinOp, lhs: &Term, rhs: &Term) -> Result {
        self.translate(lhs)?;
        self.bytecode.push(Instruction::DUP(1));
        if bop == BinOp::LogicalAnd {
            self.bytecode.push(Instruction::ISZERO);
        }
        // Allocate fresh label
        let lab = self.bytecode.fresh_label();
        self.bytecode.push(Instruction::PUSHL(lab));
        self.bytecode.push(Instruction::JUMPI);
        self.bytecode.push(Instruction::POP);
        self.translate(rhs)?;
        self.bytecode.push(Instruction::JUMPDEST(lab));
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
            BinOp::Add => self.bytecode.push(Instruction::ADD),
            BinOp::Subtract => self.bytecode.push(Instruction::SUB),
            BinOp::Divide => self.bytecode.push(Instruction::DIV),
            BinOp::Multiply => self.bytecode.push(Instruction::MUL),
            BinOp::Remainder => self.bytecode.push(Instruction::MOD),
            BinOp::Equals => self.bytecode.push(Instruction::EQ),
            BinOp::LessThan => self.bytecode.push(Instruction::LT),
            BinOp::GreaterThan => self.bytecode.push(Instruction::GT),
            // non-standard
            BinOp::NotEquals => {
                self.bytecode.push(Instruction::EQ);
                self.bytecode.push(Instruction::ISZERO);
            }
            BinOp::LessThanOrEquals => {
                self.bytecode.push(Instruction::GT);
                self.bytecode.push(Instruction::ISZERO);
            }
            BinOp::GreaterThanOrEquals => {
                self.bytecode.push(Instruction::LT);
                self.bytecode.push(Instruction::ISZERO);
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
            Term::MemoryAccess(r) => {
                self.translate_memory_access(*r,index)
            }
            _ => {
                Err(Error::InvalidMemoryAccess)
            }
        }
    }

    fn translate_memory_access(&mut self, region: Region, index: &Term) -> Result {
        // Translate index expression
        self.translate(index)?;
        // Dispatch based on region
        match region {
            Region::Memory => {
                self.bytecode.push(Instruction::MLOAD);
            }
            Region::Storage => {
                self.bytecode.push(Instruction::SLOAD);
            }
            Region::CallData => {
                self.bytecode.push(Instruction::CALLDATALOAD);
            }
        }
        //
        Ok(())
    }

    // ============================================================================
    // Values
    // ============================================================================

    fn translate_literal(&mut self, digits: &[u8], radix: u32) -> Result {
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
            Err(Error::LiteralOverflow)
        } else {
            self.bytecode.push(Instruction::PUSH(bytes));
            Ok(())
        }
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
