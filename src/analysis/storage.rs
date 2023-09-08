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

/// Abstraction of peristent storage within an EVM.  This provides the
/// minimal set of operations required to implement the semantics of a
/// given bytecode instruction.  For example, reading/writing from
/// storage.
pub trait EvmStorage : Debug {
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

/// The simplest possible implementation of `EvmStorage` which simply
/// returns "unknown" for every location.  In other words, it doesn't
/// actually analyse storage at all.
#[derive(Clone,Debug,PartialEq)]
pub struct UnknownStorage<T:EvmWord+Top> {
    dummy: PhantomData<T>
}

impl<T:EvmWord+Top> UnknownStorage<T> {
    pub fn new() -> Self { Self{dummy: PhantomData} }
}

impl<T:EvmWord+Top> EvmStorage for UnknownStorage<T> {
    type Word = T;

    fn get(&mut self, _address: Self::Word) -> Self::Word {
        T::TOP
    }

    fn put(&mut self, _address: Self::Word, _item: Self::Word) {
        // no op (for now)
    }
}
