use std::{cmp,fmt};
use crate::util;

/// Represents a `256` bit word.  This is very similar what a `u256`
/// would be, but where all operations employ modulo arithmetic.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
#[allow(non_camel_case_types)]
pub struct w256 {
    // Least significant 16 bytes
    low: u128,
    // Most significant 16 bytes
    high: u128
}

impl w256 {

    /// The smallest value that can be represented by this word.
    pub const MIN : w256 = w256::new(0u128,0u128);
    /// The largest value that can be represented by this word.
    pub const MAX : w256 = w256::new(u128::MAX,u128::MAX);
    /// Constant for zero
    pub const ZERO : w256 = w256::from(0);
    /// Constant for one
    pub const ONE : w256 = w256::from(1);
    /// Constant for two
    pub const TWO : w256 = w256::from(2);
    /// Constant for three
    pub const THREE : w256 = w256::from(3);
    /// Constant for four
    pub const FOUR : w256 = w256::from(4);

    /// Construct a `w256` from two `u128` words.
    pub const fn new(low: u128, high: u128) -> Self {
        w256{low, high}
    }
    pub const fn from(item: u128) -> Self {
	w256{low: item, high: 0}
    }
    /// Convert a given byte array into a w256.  The array is expected
    /// to be at most 32bytes long.
    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        let n = bytes.len();
        assert!(n <= 32);
        let lw;
        let hw;
        //
        if n >= 16 {
            let m = n - 16;
            lw = util::from_be_bytes(&bytes[m..]);
            hw = util::from_be_bytes(&bytes[..m]);
        } else {
            lw = util::from_be_bytes(bytes);
            hw = 0;
        }
        w256{low: lw, high: hw}
    }
}

// =====================================================================
// Min / Max
// =====================================================================

impl util::Max for w256 {
    const MAX : Self = w256::MAX;
}

impl util::Min for w256 {
    const MIN : Self = w256::MIN;
}

// =====================================================================
// Coercions
// =====================================================================

/// Anything which can be converted into a `u128` can be converted
/// into a `w256`.
impl<T:Into<u128>> From<T> for w256 {
    fn from(val: T) -> w256 {
        w256{low: val.into(),high:0}
    }
}

impl Into<u16> for w256 {
    fn into(self) -> u16 {
        self.low as u16
    }
}

impl Into<u16> for &w256 {
    fn into(self) -> u16 {
        self.low as u16
    }
}

// =====================================================================
// Arithmetic Comparisons
// =====================================================================

impl Ord for w256 {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
	let h = u128::cmp(&self.high,&other.high);
	if h == cmp::Ordering::Equal {
	    u128::cmp(&self.low,&other.low)
	} else {
	    h
	}
    }
}

impl PartialOrd for w256 {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// =====================================================================
// Arithmetic Operators
// =====================================================================

impl std::ops::Add for w256 {
    type Output=Self;

    fn add(self,rhs: Self) -> Self {
        let (lw,c0) = self.low.overflowing_add(rhs.low);
        //let (hw,_) = self.high.carrying_add(rhs.high,c0);
        let (hw,_) = carrying_add(self.high,rhs.high,c0);
        // Done
        w256{low: lw, high: hw}
    }
}

impl std::ops::Add<usize> for w256 {
    type Output=Self;

    fn add(self,rhs: usize) -> Self {
	// Coerce usize into u128 (unsafe)
        let r : w256 = (rhs as u128).into();
	// Add two w256
	self + r
    }
}

impl std::ops::Sub for w256 {
    type Output=Self;

    fn sub(self,rhs: Self) -> Self {
        let (lw,b1) = self.low.overflowing_sub(rhs.low);
        //let (hw,_) = self.high.borrowing_sub(rhs.high,b1);
        let (hw,_) = borrowing_sub(self.high,rhs.high,b1);
        // Done
        w256{low: lw, high: hw}
    }
}


// =====================================================================
// Formatting
// =====================================================================

impl fmt::Display for w256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{},{}",self.low,self.high)
    }
}

impl fmt::LowerHex for w256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.high, f)?;
        fmt::LowerHex::fmt(&self.low, f)
    }
}

// =====================================================================
// Helpers
// =====================================================================

/// A simple method (which hopefully should be deprecated at some
/// point by `u128::carrying_add`) for managing addition with carry.
fn carrying_add(lhs: u128, rhs: u128, carry: bool) -> (u128,bool) {
    // Add one if carry set.
    let (r0,c0) = if carry { lhs.overflowing_add(1) } else { (lhs,false) };
    // Add the rest.
    let (r1,c1) = r0.overflowing_add(rhs);
    // If either overflowed ... then we overflowed.
    (r1,c0|c1)
}

/// A simple method (which hopefully should be deprecated at some
/// point by `u128::borrowing_sub`) for managing subtraction with
/// borrow.
fn borrowing_sub(lhs: u128, rhs: u128, borrow: bool) -> (u128,bool) {
    // Add one if carry set.
    let (r0,b0) = if borrow { lhs.overflowing_sub(1) } else { (lhs,false) };
    // Add the rest.
    let (r1,b1) = r0.overflowing_sub(rhs);
    // If either underflowed ... then we underflowed.
    (r1,b0|b1)
}

#[cfg(test)]
mod tests {
    use crate::util::word256;
    use crate::util::w256;

    const ZERO : w256 = w256::new(0,0);
    const ONE : w256 = w256::new(1,0);
    const TWO : w256 = w256::new(2,0);
    const THREE : w256 = w256::new(3,0);
    const FOUR : w256 = w256::new(4,0);
    const MAX128 : w256 = w256::new(u128::MAX,0);
    const MAX128M1 : w256 = w256::new(u128::MAX-1,0);
    const MAX128P1 : w256 = w256::new(0,1);
    const MAX256 : w256 = w256::MAX;
    const MAX256M1 : w256 = w256::new(u128::MAX-1,u128::MAX);

    // === Addition ===

    /// Handy definition for testing both ways (since addition is
    /// commutative).
    macro_rules! assert_add {
        ($l:expr,$r:expr,$e:expr) => {
            assert_eq!($l+$r,$e);
            assert_eq!($r+$l,$e);
        };
    }

    #[test]
    fn test_add_01() { assert_add!(ZERO,ZERO,ZERO); }
    #[test]
    fn test_add_02() { assert_add!(ZERO,ONE,ONE); }
    #[test]
    fn test_add_03() { assert_add!(ONE,ONE,TWO); }
    #[test]
    fn test_add_04() { assert_add!(ONE,TWO,THREE); }
    #[test]
    fn test_add_05() { assert_add!(TWO,TWO,FOUR); }
    #[test]
    fn test_add_06() { assert_add!(ZERO,MAX128,MAX128); }
    #[test]
    fn test_add_07() { assert_add!(ONE,MAX128,MAX128P1); }
    #[test]
    fn test_add_08() { assert_add!(ONE,MAX128M1,MAX128); }
    #[test]
    fn test_add_09() { assert_add!(ONE,MAX256,ZERO); }
    #[test]
    fn test_add_10() { assert_add!(ONE,MAX256M1,MAX256); }

    // === Subtraction ===

    /// Handy definition for testing both ways (since addition is
    /// commutative).
    macro_rules! assert_sub {
        ($l:expr,$r:expr,$e1:expr,$e2:expr) => {
            assert_eq!($l-$r,$e1);
            assert_eq!($r-$l,$e2);
        };
    }

    #[test]
    fn test_sub_01() { assert_sub!(ZERO,ZERO,ZERO,ZERO); }
    #[test]
    fn test_sub_02() { assert_sub!(ONE,ONE,ZERO,ZERO); }
    #[test]
    fn test_sub_03() { assert_sub!(ZERO,ONE,MAX256,ONE); }
    #[test]
    fn test_sub_04() { assert_sub!(ZERO,TWO,MAX256M1,TWO); }
    #[test]
    fn test_sub_05() { assert_sub!(ONE,TWO,MAX256,ONE); }

}
