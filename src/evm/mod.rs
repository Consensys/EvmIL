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

mod abstract_evm;
mod abstract_stack;
mod abstract_word;
mod concrete_evm;
mod concrete_stack;
mod disassembler;
pub mod opcode;

pub use crate::evm::abstract_evm::*;
pub use crate::evm::abstract_stack::*;
pub use crate::evm::abstract_word::*;
pub use crate::evm::concrete_evm::*;
pub use crate::evm::concrete_stack::*;
pub use crate::evm::disassembler::*;

use crate::util::{w256, Interval};

/// Represents the fundamental unit of computation within the EVM,
/// namely a word.  This is intentially left abstract, so that it
/// could be reused across both _concrete_ and _abstract_ semantics.
pub trait Word: Sized + Clone + From<w256> + PartialEq + std::ops::Add<Output = Self> {}

/// Default implementations for `w256`
impl Word for w256 {}
impl Word for Interval<w256> {}

// ===================================================================
// Stack
// ===================================================================

/// Represents an EVM stack of some form.  This could a _concrete_
/// stack (i.e. useful for execution) or an _abstract_ stack
/// (i.e. useful for analysis).
pub trait Stack: Default + PartialEq {
    /// Underlying machine word this stack operates on.  In a concrete EVM, this
    /// would be `w256` or some other concrete implementation.
    type Word: Word;

    /// Peek `nth` item from stack (where `n==0` is top element).
    fn peek(&self, n: usize) -> Self::Word;

    /// Determine number of items on stack.
    fn len(&self) -> Self::Word;

    /// Push an item onto the stack.
    fn push(&mut self, item: Self::Word);

    /// Pop an item from the stack.
    fn pop(&mut self, n: usize);

    /// Set ith item on stack (where `n==0` is top element)
    fn set(&mut self, n: usize, item: Self::Word);
}

// ===================================================================
// EVM
// ===================================================================

/// Represents an EVM of some form.  This could be a _concrete_ EVM (i.e. useful for actually
/// executing bytecodes), or an _abstract_ EVM (i.e. useful for some kind of dataflow analysis).
#[derive(Clone, Debug, PartialEq)]
pub struct Evm<'a, S: Stack> {
    /// Program Counter
    pub pc: usize,
    /// Bytecode being executed
    pub code: &'a [u8],
    // Stack
    pub stack: S,
}

impl<'a, S: Stack> Evm<'a, S> {
    /// Construct a new EVM.
    pub fn new(code: &'a [u8]) -> Self {
        // Create default stack
        let stack = S::default();
        // Create EVM!
        Evm { pc: 0, code, stack }
    }

    pub const fn new_const(code: &'a [u8], stack: S) -> Self {
        // Create EVM!
        Evm { pc: 0, code, stack }
    }

    /// Peek 'n'th item on the stack.
    pub fn peek(&self, n: usize) -> S::Word {
        self.stack.peek(n)
    }

    /// Pop `n` items of the stack.
    pub fn pop(mut self, n: usize) -> Self {
        self.stack.pop(n);
        self
    }

    /// Push a word onto the stack.
    pub fn push(mut self, word: S::Word) -> Self {
        self.stack.push(word);
        self
    }

    pub fn set(mut self, n: usize, word: S::Word) -> Self {
        self.stack.set(n, word);
        self
    }

    /// Shift the `pc` by `n` bytes.
    pub fn next(mut self, n: usize) -> Self {
        self.pc = self.pc + n;
        self
    }

    /// Update `pc` to a given location.
    pub fn goto(mut self, n: usize) -> Self {
        self.pc = n;
        self
    }
}

/// A stepper is a trait for describing a single execution step of the
/// EVM.  This is subtle because it can be abstract or concrete.
pub trait Stepper {
    type Result;

    /// Take a single step of the EVM producing a result of some kind
    /// (e.g. an updated EVM state).
    fn step(self) -> Self::Result;
}
