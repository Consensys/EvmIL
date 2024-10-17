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
use std::collections::BTreeMap;
use crate::util::{w256,Top};
use super::{EvmWord};

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
#[derive(Clone,Eq,Ord,PartialEq,PartialOrd)]
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
#[derive(Clone,Eq,Ord,PartialEq,PartialOrd)]
pub struct ConcreteMemory<T:EvmWord+Top> {
    // Indicates whether or not locations stored outside of the words
    // map have the known value zero (`top=false`), or an unknown
    // value (`top=true`).
    top: bool,
    // This stores memory in a word-aligned fashioned.  Observe that
    // we're making an implicit assumption here that addressable
    // memory never exceeds 64bits.  That seems pretty reasonable for
    // the forseeable future.
    words: BTreeMap<u64,T>
}

impl<T:EvmWord+Top> ConcreteMemory<T> {
    pub fn new() -> Self {
        let words = BTreeMap::new();
        // Memory is initially all zero
        Self{top: false, words}
    }

    fn internal_read(&self, addr: u64) -> T {
        let offset = addr%32;
        // Check alignment
        if offset == 0 {
            // Aligned read
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
            // Unaligned read
            let waddr = addr-offset;
            let aw1 = self.internal_read(waddr);
            let aw2 = self.internal_read(waddr+32);
            // Check both are constants
            if aw1.is_constant() && aw2.is_constant() {
                let mut w1 = aw1.constant();
                let mut w2 = aw2.constant();                
                let boffset = (offset as usize) * 8;
                // Yes, we can do something.
                w1 <<= boffset;
                w2 >>= 256 - boffset;
                // Done
                T::from(w1 | w2)
            } else {
                T::TOP                
            }
        }
    }

    fn internal_write(&mut self, addr: u64, aword: T) {
        let offset = addr%32;
        //
        if offset == 0 {
            // Aligned write
            self.words.insert(addr,aword);
        } else if aword.is_constant() {
            // Unaligned (constant) write
            let mut word = aword.constant();
            // Write bytes individually
            for i in (0..32).rev() {
                let ith = word & w256::from(0xFF);
                self.internal_write_byte(addr+i,ith.to());
                word >>= 8;
            }
        } else {
            let waddr = addr - offset;
            // Unaligned (non-constant) write
            self.words.insert(waddr,T::TOP);
            self.words.insert(waddr+32,T::TOP);                            
        }
    }

    fn internal_write8(&mut self, addr: u64, aword: T) {        
        if aword.is_constant() {
            // Byte being written is known, hence there is something
            // useful we can do.
            let abyte = aword.constant() & w256::from(0xFF);            
            self.internal_write_byte(addr,abyte.to());
        } else {
            // Determine enclosing word
            let word_addr = addr - (addr % 32);
            // Set to unknown
            self.words.insert(word_addr,T::TOP);                
        }
    }
    
    fn internal_write_byte(&mut self, addr: u64, byte: u8) {
        // Determine byte offset
        let offset = addr%32;
        // Determine enclosing word
        let waddr = addr - offset;
        // Read current word
        let w = self.internal_read(waddr);
        // Update (if useful)
        if w.is_constant() {
            let mut v = w.constant();            
            // Construct mask
            let moffset = 8 * (31 - offset) as usize;
            let mask = w256::from(0xFF) << moffset;
            // Construct byte
            let bword = w256::from(byte) << moffset;
            // Update word
            v &= !mask;
            v |= bword;
            // Done
            self.words.insert(waddr,T::from(v));
        } else {
            // In this case, writing a constant byte to an unknow word
            // leaves an unknown word.
        }
    }
}

impl<T:EvmWord+Top> EvmMemory for ConcreteMemory<T> {
    type Word = T;

    fn read(&mut self, address: Self::Word) -> Self::Word {
        if address.is_constant() {
            // Note the conversion here should never fail since its
            // impossible for addressible memory to exceed 64bits.
            let addr : u64 = address.constant().to();
            // Read word
            self.internal_read(addr)
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
            let addr : u64 = address.constant().to();
            self.internal_write(addr,item);
        } else {
            self.top = true;
            self.words.clear();
        }
    }

    fn write8(&mut self, address: Self::Word, item: Self::Word) {
        if address.is_constant() {
            // Note the conversion here should never fail since its
            // impossible for addressible memory to exceed 64bits.
            let addr : u64 = address.constant().to();            
            self.internal_write8(addr,item);
        } else {
            // Unknown write.  Everything is lost.
            self.top = true;
            self.words.clear();
        }
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

// ===================================================================
// Memory Tests
// ===================================================================

#[cfg(test)]
mod memory_tests {
    use crate::util::{w256,Top};
    use crate::analysis::{aw256,ConcreteMemory};

    // Adding these tests caught an awful lot of bugs in earlier
    // versions of the above code.
    
    #[test]
    fn mem_aligned_oob_read() {
        let mem = ConcreteMemory::<aw256>::new();
        let w = w256::from(0);
        assert_eq!(mem.internal_read(0),aw256::from(w));
    }

    #[test]
    fn mem_unaligned_oob_read() {
        let mem = ConcreteMemory::<aw256>::new();
        let w = w256::from(0);
        assert_eq!(mem.internal_read(13),aw256::from(w));
    }

    #[test]
    fn mem_aligned_known_read() {
        let mut mem = ConcreteMemory::<aw256>::new();
        let w = w256::from(12345);        
        mem.internal_write(0,aw256::from(w));
        assert_eq!(mem.internal_read(0),aw256::from(w));
    }

    #[test]
    fn mem_unaligned_known_read_1() {
        let mut mem = ConcreteMemory::<aw256>::new();
        let mut w1 = w256::from(0xf0e0d0c0b0a090807060504030201000u128);
        w1 <<= 16*8;
        w1 |= w256::from(0xf1e1d1c1b1a191817161514131211101u128);
        mem.internal_write(0,aw256::from(w1));
        // Try every possible read
        for i in 0..32 {
            let w2 = w1 << ((i*8) as usize);
            assert_eq!(mem.internal_read(i),aw256::from(w2));
        }
    }
    
    #[test]
    fn mem_unaligned_known_read_2() {
        let mut mem = ConcreteMemory::<aw256>::new();
        let mut w1 = w256::from(0xf0e0d0c0b0a090807060504030201000u128);
        w1 <<= 16*8;
        w1 |= w256::from(0xf1e1d1c1b1a191817161514131211101u128);
        mem.internal_write(32,aw256::from(w1));
        // Try every possible read
        for i in 0..32 {
            let w2 = w1 >> (((32-i)*8) as usize);
            assert_eq!(mem.internal_read(i),aw256::from(w2));
        }
    }

    #[test]
    fn mem_unaligned_unknown_read_1() {
        let mut mem = ConcreteMemory::<aw256>::new();
        mem.internal_write(0,aw256::TOP);
        // Try every possible read
        for i in 0..32 {
            assert_eq!(mem.internal_read(i),aw256::TOP);
        }
    }
    
    #[test]
    fn mem_unaligned_known_write() {
        let zero = aw256::from(w256::ZERO);
        let mut mem = ConcreteMemory::<aw256>::new();
        let mut w1 = w256::from(0xf0e0d0c0b0a090807060504030201000u128);
        w1 <<= 16*8;
        w1 |= w256::from(0xf1e1d1c1b1a191817161514131211101u128);
        // Try every possible read
        for i in 0..32 {
            mem.internal_write(0,zero); // reset
            mem.internal_write(i,aw256::from(w1));            
            let w2 = w1 >> ((i*8) as usize);
            assert_eq!(mem.internal_read(0),aw256::from(w2));
        }
    }

    #[test]
    fn mem_unaligned_unknown_write() {
        let zero = aw256::from(w256::ZERO);
        let mut mem = ConcreteMemory::<aw256>::new();
        // Try every possible read
        for i in 0..32 {
            mem.internal_write(i,aw256::TOP);            
            assert_eq!(mem.internal_read(0),aw256::TOP);
            if i > 0 {
                assert_eq!(mem.internal_read(32),aw256::TOP);
            } else {
                assert_eq!(mem.internal_read(32),zero);                
            }
        }
    }

    #[test]
    fn mem_known_write8() {
        let mut mem = ConcreteMemory::<aw256>::new();
        let mut w1 = w256::from(0xf0e0d0c0b0a0908070605040302010u128);
        for i in 0..32 {
            let w2 = w256::from(0x10) << 8*(31 - i);
            // Reset
            mem.internal_write(0,aw256::from(w256::ZERO));
            // Write byte
            mem.internal_write8(i as u64,aw256::from(w1));
            // Check
            assert_eq!(mem.internal_read(0),aw256::from(w2));
        }
    }

    #[test]
    fn mem_unknown_write8() {
        let mut mem = ConcreteMemory::<aw256>::new();
        let mut w1 = w256::from(0xf0e0d0c0b0a0908070605040302010u128);
        for i in 0..32 {
            // Reset
            mem.internal_write(0,aw256::from(w1));
            // Write byte
            mem.internal_write8(i as u64,aw256::TOP);
            // Check
            assert_eq!(mem.internal_read(0),aw256::TOP);
        }
    }    
}
