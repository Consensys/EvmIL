use crate::evm;
use crate::util::{w256,JoinInto};
use std::fmt;
use std::ops::Add;

// ============================================================================
// Abstract Word
// ============================================================================

/// An abstract word is either a known quantity, or an unknown
/// (i.e. arbitrary value).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbstractWord<T: PartialEq> {
    Known(T),
    Unknown,
}

use AbstractWord::*;

impl<T: PartialEq> AbstractWord<T> {
    /// Check whether this abstract word is a known quantity or not.
    pub fn is_known(&self) -> bool {
        match self {
            Known(_) => true,
            Unknown => false,
        }
    }

    /// Extract the underlying quantitiy from this abstract word.  This assumes it must be a known quantity.
    pub fn known(&self) -> &T {
        match self {
            Known(n) => n,
            Unknown => {
                panic!("unwrapping unknown value");
            }
        }
    }
}

impl<T: PartialEq+JoinInto> AbstractWord<T> {
    /// Merge two abstract words together, producing a single abstract word.  If either is unknown, then the result is unknown.
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Known(mut v), Known(w)) => {
                v.join_into(&w);
                Known(v)
            },
            _ => Unknown,
        }
    }
}

// ==================================================
// Traits
// ==================================================

impl<T> fmt::Display for AbstractWord<T>
where
    T: PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unknown => write!(f, "{{??}}"),
            Known(n) => write!(f, "{{{}}}", n),
        }
    }
}

//impl<T: PartialEq> From<T> for AbstractWord<T> {
//    fn from(v: T) -> Self {
//        Known(v)
//    }
//}

impl<T: evm::Word> From<w256> for AbstractWord<T> {
    fn from(v: w256) -> Self {
        Known(T::from(v))
    }
}

// ==================================================
// Arithmetic
// ==================================================

impl<T: Add + PartialEq> Add for AbstractWord<T>
where
    T::Output: PartialEq,
{
    type Output = AbstractWord<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Known(v), Known(w)) => Known(v + w),
            _ => Unknown,
        }
    }
}

impl<T:evm::Word> evm::Word for AbstractWord<T> {

}
