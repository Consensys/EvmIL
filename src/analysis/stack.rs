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
use std::fmt::Debug;
use super::EvmWord;

/// Abstraction of the operand stack within an EVM.  This provides the
/// minimal set of operations required to implement the semantics of a
/// given bytecode instruction.  For example, pushing / popping items
/// from the stack.
pub trait EvmStack : Debug {
    /// Defines what constitutes a word in this EVM.  For example, a
    /// concrete evm will use a `w256` here whilst an abstract evm
    /// will use something that can, for example, describe unknown
    /// values.
    type Word : EvmWord;

    /// Check capacity for `n` additional items on the stack.
    fn has_capacity(&self, n: usize) -> bool;

    /// Check at least `n` operands on the stack.
    fn has_operands(&self, n: usize) -> bool;

    /// Get the size of the stack.
    fn size(&self) -> usize;

    /// Peek `nth` item from stack (where `n==0` is top element).
    fn peek(&self, n: usize) -> &Self::Word;

    /// Push an item onto the stack.
    fn push(&mut self, item: Self::Word);

    /// Pop an item from the stack.
    fn pop(&mut self) -> Self::Word;

    /// Swap top item on stack with nth item on stack (where `n>0`,
    /// and `n==0` would be the top element).
    fn swap(&mut self, n: usize);

    /// Duplicate nth item on stack (where `n==0` is the top element).
    fn dup(&mut self, n: usize);
}

// ===================================================================
// Legacy Stack
// ===================================================================

/// An implementation of `EvmStack` which gives a concrete view of the
/// stack.  In other words, it represents the stack exactly.
#[derive(Clone,Debug,PartialEq)]
pub struct ConcreteStack<T:EvmWord> {
    items: Vec<T>
}

impl<T:EvmWord> ConcreteStack<T> {
    pub fn new() -> Self {
        Self{items: Vec::new()}
    }
}

impl<T:EvmWord> EvmStack for ConcreteStack<T> {
    type Word = T;

    fn has_capacity(&self, n: usize) -> bool {
        (1024 - self.items.len()) >= n
    }

    fn has_operands(&self, n: usize) -> bool {
        self.items.len() >= n
    }

    fn size(&self) -> usize {
        self.items.len()
    }

    fn peek(&self, n: usize) -> &Self::Word {
        assert!(self.has_operands(n));
        &self.items[self.items.len() - (n+1)]
    }

    fn push(&mut self, item: Self::Word) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Self::Word {
        assert!(self.has_operands(1));
        self.items.pop().unwrap()
    }

    fn dup(&mut self, n: usize) {
        assert!(self.has_operands(n+1));
        let i = self.items.len() - (n+1);
        self.items.push(self.items[i].clone());
    }

    fn swap(&mut self, n: usize) {
        assert!(n > 0);
        assert!(self.has_operands(n+1));
        let i = self.items.len() - (n+1);
        let j = self.items.len() - 1;
        // Use slice swap to avoid cloning.
        self.items.swap(i,j);
    }
}

impl<T:EvmWord> Default for ConcreteStack<T> {
    fn default() -> Self {
        Self::new()
    }                         
}
