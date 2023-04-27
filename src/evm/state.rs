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
use crate::util::{Concretizable,w256,Interval};

/// Represents the fundamental unit of computation within the EVM,
/// namely a word.  This is intentially left abstract, so that it
/// could be reused across both _concrete_ and _abstract_ semantics.
pub trait EvmWord : Sized + Clone + Debug +
    From<w256> + // Allow conversion from 256 bit words
    Concretizable<Item=w256> + // Allow conversion back to 256 words
    PartialEq
    // std::ops::Add<Output = Self> +
    // std::ops::Sub<Output = Self> +
    // std::ops::Mul<Output = Self> +
    // std::ops::Rem<Output = Self> +
    // std::ops::Not<Output = Self> +
    // std::ops::Shl<Output = Self> +
    // std::ops::Shr<Output = Self>
{
}

/// Default implementations for `EvmWord`
// impl EvmWord for w256 {}
// impl EvmWord for Interval<w256> {}

// ===================================================================
// State
// ===================================================================

/// Describes a state of the EVM, which could be running or
/// terminated.
pub trait EvmState : Debug {
    /// Defines what constitutes a word in this EVM.  For example, a
    /// concrete evm will use a `w256` here whilst an abstract evm
    /// will use something that can, for example, describe unknown
    /// values.
    type Word : EvmWord;

    /// Defines the stack implementation used in this EVM.
    type Stack : EvmStack<Word=Self::Word>;

    /// Defines the memory implementation used in this EVM.
    type Memory : EvmMemory<Word=Self::Word>;

    /// Defines the memory implementation used in this EVM.
    type Storage : EvmStorage<Word=Self::Word>;

    /// Get the program counter.  Every `EvmState` has a statically
    /// known `pc`.
    fn pc(&self) -> usize;

    /// Get write access to the operand stack contained within this
    /// state.
    fn stack(&mut self) -> &mut Self::Stack;

    /// Get write access to the scratch memory contained within this
    /// state.
    fn memory(&mut self) -> &mut Self::Memory;

    /// Get write access to the persistent storage contained within
    /// this state.
    fn storage(&mut self) -> &mut Self::Storage;

    /// Move _program counter_ over `n` bytes in the next instruction.
    fn skip(&mut self, n: usize);

    /// Move _program counter_ to a given (byte) offset within the
    /// code section.
    fn goto(&mut self, pc: usize);
}

// ===============================================================
// Operand Stack
// ===============================================================

/// Represents the operand stack within the EVM.
pub trait EvmStack {
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

// ===============================================================
// Scratch Memory
// ===============================================================

pub trait EvmMemory {
    /// Defines what constitutes a word in this EVM.  For example, a
    /// concrete evm will use a `w256` here whilst an abstract evm
    /// will use something that can, for example, describe unknown
    /// values.
    type Word;

    /// Read a word Get the word at a given location in storage.
    fn read(&mut self, address: Self::Word) -> Self::Word;

    /// Write a given value at a given address in memory, expanding
    /// memory as necessary.
    fn write(&mut self, address: Self::Word, item: Self::Word);

    /// Write a given value at a given address in memory, expanding
    /// memory as necessary.
    fn write8(&mut self, address: Self::Word, item: Self::Word);
}

// ===============================================================
// Peristent Storage
// ===============================================================

pub trait EvmStorage {
    /// Defines what constitutes a word in this EVM.  For example, a
    /// concrete evm will use a `w256` here whilst an abstract evm
    /// will use something that can, for example, describe unknown
    /// values.
    type Word : EvmWord;

    /// Get the word at a given location in storage.
    fn get(&mut self, address: Self::Word) -> Self::Word;

    /// Put a given value at a given location in storage.
    fn put(&mut self, address: Self::Word, item: Self::Word);
}
