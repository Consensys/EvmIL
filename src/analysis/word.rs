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
use crate::util::{Concretizable,w256,Top};

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
