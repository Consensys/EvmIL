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
use crate::util::{Concretizable,w256,Top};

/// Represents the fundamental unit of computation within the EVM,
/// namely a word.  This is intentially left abstract, so that it
/// could be reused across both _concrete_ and _abstract_ semantics.
pub trait EvmWord : Sized + Clone + fmt::Debug +
    From<w256> + // Allow conversion from 256 bit words
    Concretizable<Item=w256> + // Allow conversion back to 256 words
    PartialEq
{
    // Comparators
    fn less_than(self,rhs:Self)->Self;
    fn equal(self,rhs:Self)->Self;
    fn is_zero(self)->Self;
    // Arithmetic
    fn add(self,rhs:Self)->Self;
    fn sub(self,rhs:Self)->Self;
    fn mul(self,rhs:Self)->Self;
    fn div(self,rhs:Self)->Self;
    fn rem(self,rhs:Self)->Self;
    // Bitwise
    fn and(self,rhs:Self)->Self;
    fn or(self,rhs:Self)->Self;
    fn xor(self,rhs:Self)->Self;
    fn not(self)->Self;
    // Misc
    fn havoc(self)->Self;    
}

// ===================================================================
// Abstract Word
// ===================================================================

/// Simplest possible (abstract) word which is either a _concrete_
/// word or _unknown_.
#[derive(Copy,Clone,PartialEq)]
#[allow(non_camel_case_types)]
pub enum aw256 {
    Word(w256),
    Unknown
}

impl fmt::Display for aw256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl fmt::Debug for aw256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            aw256::Word(w) => {
                let mut first = true;
                write!(f,"0x")?;
                // Following is necessary because ruint::Uint doesn't
                // appear to play nicely with formatting hexadecimal.                
                for l in w.as_limbs().iter().rev() {
                    if *l != 0 || !first {
                        write!(f,"{l:02x}")?;
                        first = false;
                    }
                }
                if first {
                    write!(f,"00")?;
                } 
            }
            aw256::Unknown => {
                write!(f,"??")?;
            }
        }
        Ok(())
    }
}

impl From<w256> for aw256 {
    fn from(word: w256) -> aw256 {
        aw256::Word(word)
    }
}

impl Top for aw256 {
    const TOP : aw256 = aw256::Unknown;
}

impl Concretizable for aw256 {
    type Item = w256;

    fn is_constant(&self) -> bool {
        match self {
            aw256::Word(_) => true,
            aw256::Unknown => false
        }
    }

    fn constant(&self) -> w256 {
        match self {
            aw256::Word(w) => *w,
            aw256::Unknown => {
                panic!();
            }
        }
    }
}

impl EvmWord for aw256 {
    fn less_than(self,rhs:Self)->Self {
        match (self,rhs) {
            (aw256::Word(l),aw256::Word(r)) => {
                if l < r { aw256::Word(w256::from(1)) }
                else { aw256::Word(w256::from(0)) }
            }
            (_,_) => aw256::Unknown
        }
    }
    fn equal(self,rhs:Self)->Self {
        match (self,rhs) {
            (aw256::Word(l),aw256::Word(r)) => {
                if l == r { aw256::Word(w256::from(1)) }
                else { aw256::Word(w256::from(0)) }
            }
            (_,_) => aw256::Unknown
        }        
    }
    fn is_zero(self) -> Self {
        match self {
            aw256::Word(w) => {
                let zero = w256::from(0);
                if w == zero { aw256::Word(w256::from(1)) }
                else { aw256::Word(zero) }                                
            }
            aw256::Unknown => {               
                aw256::Unknown                
            }
        }        
    }
    // Arithmetic
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l+r),
            (_,_) => aw256::Unknown
        }
    }
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l-r),
            (_,_) => aw256::Unknown
        }
    }
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l*r),
            (_,_) => aw256::Unknown
        }
    }
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l/r),
            (_,_) => aw256::Unknown
        }
    }
    fn rem(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l%r),
            (_,_) => aw256::Unknown
        }
    }
    // bitwise
    fn and(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l&r),
            (_,_) => aw256::Unknown
        }
    }
    fn or(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l|r),
            (_,_) => aw256::Unknown
        }
    }
    fn xor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l^r),
            (_,_) => aw256::Unknown
        }
    }
    fn not(self) -> Self {
        match self {
            aw256::Word(l) => aw256::Word(!l),
            _ => aw256::Unknown
        }
    }
    fn havoc(self) -> Self {
        aw256::Unknown
    }
}

// ===================================================================
// Constant Word
// ===================================================================

/// A very simple abstract word which does not support any operations
/// (e.g. arithmetic or comparators).  Thus, it typically becomes top.
/// This is useful in some specific situations where we want to
/// prevent the possibility of infinite ascending chains (i.e. prior
/// to _havoc analysis_).
#[derive(Copy,Clone,PartialEq)]
#[allow(non_camel_case_types)]
pub enum cw256 {
    Word(w256),
    Unknown
}

impl fmt::Display for cw256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl fmt::Debug for cw256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            cw256::Word(w) => {
                let mut first = true;
                write!(f,"0x")?;
                // Following is necessary because ruint::Uint doesn't
                // appear to play nicely with formatting hexadecimal.                
                for l in w.as_limbs().iter().rev() {
                    if *l != 0 || !first {
                        write!(f,"{l:02x}")?;
                        first = false;
                    }
                }
                if first {
                    write!(f,"00")?;
                } 
            }
            cw256::Unknown => {
                write!(f,"??")?;
            }
        }
        Ok(())
    }
}

impl From<w256> for cw256 {
    fn from(word: w256) -> cw256 { cw256::Word(word) }
}

impl Top for cw256 {
    const TOP : cw256 = cw256::Unknown;
}

impl Concretizable for cw256 {
    type Item = w256;

    fn is_constant(&self) -> bool {
        match self {
            cw256::Word(_) => true,
            cw256::Unknown => false
        }
    }

    fn constant(&self) -> w256 {
        match self {
            cw256::Word(w) => *w,
            cw256::Unknown => {
                panic!();
            }
        }
    }
}

impl EvmWord for cw256 {
    fn less_than(self,rhs:Self)->Self { cw256::Unknown }
    fn equal(self,rhs:Self)->Self { cw256::Unknown }
    fn is_zero(self) -> Self { cw256::Unknown }
    fn add(self, rhs: Self) -> Self { cw256::Unknown }
    fn sub(self, rhs: Self) -> Self { cw256::Unknown }
    fn mul(self, rhs: Self) -> Self { cw256::Unknown }
    fn div(self, rhs: Self) -> Self { cw256::Unknown }
    fn rem(self, rhs: Self) -> Self { cw256::Unknown }
    fn and(self, rhs: Self) -> Self { cw256::Unknown }
    fn or(self, rhs: Self) -> Self  { cw256::Unknown }
    fn xor(self, rhs: Self) -> Self { cw256::Unknown }
    fn not(self) -> Self { cw256::Unknown }
    fn havoc(self) -> Self { cw256::Unknown }    
}
