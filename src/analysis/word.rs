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
    PartialEq +
    std::ops::Add<Output = Self> +
    std::ops::Sub<Output = Self> +
    std::ops::Mul<Output = Self> +
    std::ops::Div<Output = Self> +    
    std::ops::Rem<Output = Self> +
    std::ops::BitAnd<Output = Self> +
    std::ops::BitOr<Output = Self> +
    std::ops::BitXor<Output = Self> +
    std::ops::Not<Output = Self>
    // std::ops::Not<Output = Self> +
    // std::ops::Shl<Output = Self> +
    // std::ops::Shr<Output = Self>
{
}

// ===================================================================
// Abstract Word
// ===================================================================

/// Simplest possible (abstract) word which is either a _concrete_
/// word or _unknown_.
#[derive(Copy,Clone,Debug,PartialEq)]
#[allow(non_camel_case_types)]
pub enum aw256 {
    Word(w256),
    Unknown
}

impl fmt::Display for aw256 {
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

}

// ===================================================================
// Arithmetic Operations
// ===================================================================

impl std::ops::Add for aw256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l+r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::Sub for aw256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l-r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::Mul for aw256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l*r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::Div for aw256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l/r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::Rem for aw256 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l%r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::BitAnd for aw256 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l&r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::BitOr for aw256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l|r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::BitXor for aw256 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (aw256::Word(l),aw256::Word(r)) => aw256::Word(l^r),
            (_,_) => aw256::Unknown
        }
    }
}

impl std::ops::Not for aw256 {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            aw256::Word(l) => aw256::Word(!l),
            _ => aw256::Unknown
        }
    }
}
