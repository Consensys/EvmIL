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
use std::fmt;
use super::{EvmWord};

/// Abstraction of the operand stack within an EVM.  This provides the
/// minimal set of operations required to implement the semantics of a
/// given bytecode instruction.  For example, pushing / popping items
/// from the stack.
pub trait EvmStack : fmt::Debug {
    /// Defines what constitutes a word in this EVM.  For example, a
    /// concrete evm will use a `w256` here whilst an abstract evm
    /// will use something that can, for example, describe unknown
    /// values.
    type Word : EvmWord;

    /// Check capacity for `n` additional items on the stack.
    fn has_capacity(&self, n: usize) -> bool {
        (1024 - self.size()) >= n
    }
    
    /// Check at least `n` operands on the stack.
    fn has_operands(&self, n: usize) -> bool {
        self.size() >= n
    }
    
    /// Get the size of the stack.
    fn size(&self) -> usize;

    /// Peek `nth` item from stack (where `n==0` is top element).
    fn peek(&self, n: usize) -> &Self::Word;

    /// Push an item onto the stack.
    fn push(&mut self, item: Self::Word);

    /// Pop an item from the stack.
    fn pop(&mut self) -> Self::Word;

    /// Set `nth` item from stack (where `n==0` is top element),
    /// whilst returning the item previously at that position.
    fn set(&mut self, n: usize, item: Self::Word) -> Self::Word;
    
    /// Swap top item on stack with nth item on stack (where `n>0`,
    /// and `n==0` would be the top element).
    fn swap(&mut self, n: usize) {
        assert!(n > 0);
        assert!(self.has_operands(n+1));
        let ith = self.pop();
        let jth = self.set(n-1,ith);
        self.push(jth);
    }        

    /// Duplicate nth item on stack (where `n==0` is the top element).
    fn dup(&mut self, n: usize) {
        assert!(self.has_operands(n+1));
        self.push(self.peek(n).clone());
    }

    /// Update internal position within code.
    fn goto(&mut self, pc: usize);    
}

// ===================================================================
// Concrete Stack
// ===================================================================

/// An implementation of `EvmStack` which gives a concrete view of the
/// stack.  In other words, it represents the stack exactly.
#[derive(Clone,Eq,Ord,PartialEq,PartialOrd)]
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

    fn set(&mut self, n: usize, item: Self::Word) -> Self::Word {
        assert!(self.has_operands(n));
        let i = self.items.len() - (n+1);
        let j = self.items.len();        
        self.items.push(item);
        self.items.swap(i,j);
        self.items.pop().unwrap()
    }

    /// Update internal position within code.
    fn goto(&mut self, _pc: usize) {
        // nop
    }
}

impl<T:EvmWord> Default for ConcreteStack<T> {
    fn default() -> Self {
        Self::new()
    }                         
}

impl<T> fmt::Display for ConcreteStack<T>
where T:EvmWord+fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i,w) in self.items.iter().rev().enumerate() {
            if i != 0 { write!(f,",")?; }
            write!(f,"{}",w)?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for ConcreteStack<T>
where T:EvmWord+fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"[")?;               
        for (i,w) in self.items.iter().rev().enumerate() {
            if i != 0 { write!(f,",")?; }
            write!(f,"{:?}",w)?;
        }
        write!(f,"]")
    }
}
