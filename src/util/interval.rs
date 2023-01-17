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
use crate::util::{w256, Top, Bottom, Max, Min, Constant, OverflowingAdd, OverflowingSub, JoinInto};
use std::ops::{RangeInclusive};
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
    T: Copy + Ord
{
    pub start: T,
    pub end: T,
}

impl<T> Interval<T>
where
    T: Copy + Ord
{
    pub const fn new(start: T, end: T) -> Self {
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
where
    T: Copy + Ord + Max + Min + OverflowingAdd
{
    /// Add a constant to this range.
    pub fn add(&self, rhs: Self) -> Self {
        let (start,_) = self.start.overflowing_add(rhs.start);
        let (end,overflow) = self.end.overflowing_add(rhs.end);
        //
        if overflow { Self::TOP } else { Self{start,end} }
    }
}

impl<T> Interval<T>
where
    T: Copy + Ord + Max + Min + OverflowingSub
{
    /// Add a constant to this range.
    pub fn sub(&self, rhs: Self) -> Self {
        let (start,overflow) = self.start.overflowing_sub(rhs.start);
        let (end,_) = self.end.overflowing_sub(rhs.end);
        //
        if overflow { Self::TOP } else { Self{start,end} }
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
    T: Copy + Ord + fmt::Display,
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

impl<T:Copy+Ord+Min+Max> Bottom for Interval<T>
{
    const BOTTOM: Interval<T> = Interval {
        start: T::MAX,
        end: T::MIN
    };
}

impl<T:Copy+Ord+Min+Max> Top for Interval<T>
{
    const TOP: Interval<T> = Interval {
        start: T::MIN,
        end: T::MAX
    };
}

impl<T:Copy+Ord> Constant for Interval<T>
{
    type Item=T;

    fn is_constant(&self) -> bool {
        self.start == self.end
    }

    fn constant(&self) -> T {
        assert!(self.is_constant());
        self.start
    }
}
// ======================================================================
// Coercions
// ======================================================================

impl<T> From<RangeInclusive<T>> for Interval<T>
where T: Copy + Ord
{
    fn from(r: RangeInclusive<T>) -> Self {
        Interval::new(*r.start(), *r.end())
    }
}

impl<T> From<T> for Interval<T>
where T: Copy + Ord
{
    fn from(n: T) -> Self {
	Interval::new(n,n)
    }
}

// ======================================================================
// Add
// ======================================================================

impl<T> std::ops::Add for Interval<T>
where
    T: Ord + Copy + Min + Max + OverflowingAdd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Interval::add(&self,rhs)
    }
}

impl<T> std::ops::Add<T> for Interval<T>
where
    T: Ord + Copy + Min + Max + OverflowingAdd
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Interval::add(&self,Interval::<T>::from(rhs))
    }
}

// ======================================================================
// Sub
// ======================================================================

impl<T> std::ops::Sub for Interval<T>
where
    T: Ord + Copy + Min + Max + OverflowingSub,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Interval::sub(&self,rhs)
    }
}

impl<T> std::ops::Sub<T> for Interval<T>
where
    T: Ord + Copy + Min + Max + OverflowingSub
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Interval::sub(&self,Interval::<T>::from(rhs))
    }
}
