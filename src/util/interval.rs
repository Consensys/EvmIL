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
use crate::util::word256;
use crate::util::{w256, Max, Min, JoinInto};
use std::ops::{Range,RangeInclusive};
use std::{cmp, fmt};

/// Represents the maximum possible interval
pub const MAX_INTERVAL: Interval<w256> = Interval {
    start: w256::MIN,
    end: w256::MAX,
};

/// Represents an interval of values `x..y` (much like `Range<usize>`)
/// which supports various arithmetic operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Interval<T>
where
    T: PartialOrd + PartialEq + Copy,
{
    pub start: T,
    pub end: T,
}

impl<T> Interval<T>
where
    T: Copy + PartialOrd
{
    pub const fn new(start: T, end: T) -> Self {
        //assert!(start <= end);
        Self { start, end }
    }

    pub const fn new_const(start: T, end: T) -> Self {
        //assert!(start <= end);
        Self { start, end }
    }

    /// Check whether this interval represents a constant value.  For
    /// example, the interval `1..1` represents the constant `1`.
    pub fn is_constant(&self) -> bool {
        self.start == self.end
    }

    /// Exctract the constant value associated with this interval.
    pub fn unwrap(&self) -> T {
        if self.start != self.end {
            panic!("unwrapping non-constant interval");
        }
        self.start
    }
}

impl<T> Interval<T>
where T: Copy + PartialOrd + Max + Min
{
    /// The maximum possible interval expressible with the given type.
    pub const MAX: Interval<T> = Interval::new(T::MIN, T::MAX);

    /// An empty interval expressed with the given type.
    pub const EMPTY: Interval<T> = Interval::new(T::MIN, T::MIN);
}

impl<T> Interval<T>
where
    T: Copy + PartialOrd
        + std::ops::Add<usize, Output = T>
        + std::ops::Sub<usize, Output = T>,
{    
    /// Add a constant to this range.
    pub fn add(&self, val: usize) -> Self {
        let start = self.start + val;
        let end = self.end + val;
        Self { start, end }
    }

    /// Subtract a constant from this range.
    pub fn sub(&self, val: usize) -> Self {
        let start = self.start - val;
        let end = self.end - val;
        Self { start, end }
    }
}

impl<T> Interval<T>
where
    T: Ord+Copy
{
    /// Take the union of two intervals.
    pub fn union(&self, other: &Self) -> Self {
        let start = cmp::min(self.start, other.start);
        let end = cmp::max(self.end, other.end);
	Interval{start,end}
    }    
}

// ======================================================================
// Traits
// ======================================================================

impl<T> fmt::Display for Interval<T>
where
    T: PartialOrd + PartialEq + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl<T:Copy+Ord> JoinInto for Interval<T>
{
    fn join_into(&mut self, other: &Self) {
        self.start = cmp::min(self.start, other.start);
        self.end = cmp::max(self.end, other.end);
    }
}

// ======================================================================
// Coercions
// ======================================================================

impl<T> From<Range<T>> for Interval<T>
where T: PartialOrd + Copy
{
    fn from(r: Range<T>) -> Self {
        Interval::new(r.start, r.end)
    }
}

impl<T> From<RangeInclusive<T>> for Interval<T>
where T: PartialOrd + Copy + std::ops::Add<usize,Output=T>
{
    fn from(r: RangeInclusive<T>) -> Self {
        Interval::new(*r.start(), *r.end() + 1)
    }
}

impl<T> From<T> for Interval<T>
where T: PartialOrd + Copy + std::ops::Add<usize,Output=T>
{
    fn from(n: T) -> Self {
	Interval::new(n,n + 1)
    }
}
       
// ======================================================================
// Operators
// ======================================================================

impl<T> std::ops::Add for Interval<T>
where
    T: PartialOrd + PartialEq + Copy + std::ops::Add<T, Output = T>,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.start = self.start + rhs.start;
        self.end = self.end + rhs.end;
        self
    }
}

impl<T> std::ops::Add<usize> for Interval<T>
where
    T: PartialOrd + PartialEq + Copy
    + std::ops::Add<usize, Output = T>
    + std::ops::Sub<usize, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Interval::add(&self,rhs)
    }
}
