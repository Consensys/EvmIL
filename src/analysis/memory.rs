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
use std::marker::PhantomData;
use crate::util::Top;
use super::EvmWord;

/// Abstraction of memory within an EVM.  This provides the minimal
/// set of operations required to implement the semantics of a given
/// bytecode instruction.  For example, reading/writing to memory.
pub trait EvmMemory : Debug {
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

/// The simplest possible implementation of `EvmMemory` which simply
/// returns "unknown" for every location.  In other words, it doesn't
/// actually analyse memory at all.
#[derive(Clone,Debug,PartialEq)]
pub struct UnknownMemory<T:EvmWord+Top> {
    dummy: PhantomData<T>
}

impl<T:EvmWord+Top> UnknownMemory<T> {
    pub fn new() -> Self { Self{dummy: PhantomData} }
}

impl<T:EvmWord+Top> EvmMemory for UnknownMemory<T> {
    type Word = T;

    fn read(&mut self, _address: Self::Word) -> Self::Word {
        T::TOP
    }

    fn write(&mut self, _address: Self::Word, _item: Self::Word) {
        // no op (for now)
    }

    fn write8(&mut self, _address: Self::Word, _item: Self::Word) {
        // no op (for now)
    }
}

impl<T:EvmWord+Top> Default for UnknownMemory<T> {
    fn default() -> Self {
        Self::new()
    }                         
}
