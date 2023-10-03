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
use std::marker::PhantomData;
use std::collections::HashMap;
use crate::util::{w256,W256_ZERO,W256_THIRTYTWO,Top};
use super::{EvmState,EvmWord};

/// Abstraction of memory within an EVM.  This provides the minimal
/// set of operations required to implement the semantics of a given
/// bytecode instruction.  For example, reading/writing to memory.
pub trait EvmMemory : fmt::Debug {
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

// ===================================================================
// Unknown Memory
// ===================================================================

/// The simplest possible implementation of `EvmMemory` which simply
/// returns "unknown" for every location.  In other words, it doesn't
/// actually analyse memory at all.
#[derive(Clone,PartialEq)]
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

impl<T:EvmWord+Top> fmt::Display for UnknownMemory<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"???")?;
        Ok(())
    }
}

impl<T:EvmWord+Top> fmt::Debug for UnknownMemory<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"???")?;
        Ok(())
    }
}

// ===================================================================
// Concrete Memory
// ===================================================================

/// The next simplest possible implementation of `EvmMemory` which
/// only manages "concrete" addresses (i.e. it doesn't perform any
/// symbolic analysis).
#[derive(Clone,PartialEq)]
pub struct ConcreteMemory<T:EvmWord+Top> {
    // Indicates whether or not locations stored outside of the words
    // map have the known value zero (`top=false`), or an unknown
    // value (`top=true`).
    top: bool,
    // This stores memory in a word-aligned fashioned.  Observe that
    // we're making an implicit assumption here that addressable
    // memory never exceeds 64bits.  That seems pretty reasonable for
    // the forseeable future.
    words: HashMap<u64,T>
}

impl<T:EvmWord+Top> ConcreteMemory<T> {
    pub fn new() -> Self {
        let words = HashMap::new();
        // Memory is initially all zero
        Self{top: false, words}
    }
}

impl<T:EvmWord+Top> EvmMemory for ConcreteMemory<T> {
    type Word = T;

    fn read(&mut self, address: Self::Word) -> Self::Word {
        if address.is_constant() {
            // Note the conversion here should never fail since its
            // impossible for addressible memory to exceed 64bits.            
            let addr : u64 = address.constant().to();
            // FIXME: for now assume all memory reads are
            // word-aligned.  This is clearly not always true :)
            assert!(addr % 32 == 0);
            //
            match self.words.get(&addr) {
                Some(v) => v.clone(),
                None => {
                    if self.top {
                        T::TOP
                    } else {
                        T::from(w256::ZERO)
                    }
                }
            }
        } else {
            // Read address unknown, hence unknown value returned.
            T::TOP
        }
    }

    fn write(&mut self, address: Self::Word, item: Self::Word) {
        // no op (for now)
        if address.is_constant() {
            // Note the conversion here should never fail since its
            // impossible for addressible memory to exceed 64bits.
            let addr = address.constant().to();
            // FIXME: for now assume all memory reads are
            // word-aligned.  This is clearly not always true :)
            assert!(addr % 32 == 0);
            // Update memory!
            self.words.insert(addr,item);
        } else {
            self.top = true;
            self.words.clear();
        }
    }

    fn write8(&mut self, _address: Self::Word, _item: Self::Word) {
        // FIXME: could improve this if the address is a known constant.
        self.top = true;
        self.words.clear();
    }
}

impl<T:EvmWord+Top> Default for ConcreteMemory<T> {
    fn default() -> Self {
        Self::new()
    }                         
}

impl<T:EvmWord+Top> fmt::Display for ConcreteMemory<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}",self)?;
        Ok(())
    }
}

impl<T:EvmWord+Top> fmt::Debug for ConcreteMemory<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        if self.top { write!(f,"?:")?; }
        let mut keys = Vec::from_iter(self.words.keys());
        keys.sort();
        for k in keys {
            if !first { write!(f,",")?; }
            first = false;
            write!(f,"{:#0x}:={:?}", k, self.words[k])?;
        }
        Ok(())
    }
}
