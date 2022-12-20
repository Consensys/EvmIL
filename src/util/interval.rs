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
use std::{cmp,fmt};
use std::ops::Range;

/// Represents the maximum possible interval
pub const MAX_INTERVAL : Interval = Interval{start:0,end:std::usize::MAX};

/// Represents an interval of values `x..y` (much like `Range<usize>`)
/// which supports various arithmetic operations.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub struct Interval {
    pub start: usize,
    pub end: usize
}

impl Interval {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self{start,end}
    }

    pub const fn new_const(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self{start,end}
    }

    /// Check whether this interval represents a constant value.  For
    /// example, the interval `1..1` represents the constant `1`.
    pub fn is_constant(&self) -> bool {
        self.start == self.end
    }

    /// Exctract the constant value associated with this interval.
    pub fn unwrap(&self) -> usize {
        if self.start != self.end { panic!("unwrapping non-constant interval"); }
        self.start
    }

    /// Add a constant to this range.
    pub fn add(&self, val: usize) -> Self {
        let start = self.start + val;
        let end = self.end + val;
        Self{start,end}
    }

    /// Subtract a constant from this range.
    pub fn sub(&self, val: usize) -> Self {
        let start = self.start - val;
        let end = self.end - val;
        Self{start,end}
    }

    /// Union this interval with another
    pub fn union(&self, other: &Interval) -> Self {
        let start = cmp::min(self.start,other.start);
        let end = cmp::max(self.end,other.end);
        Self{start,end}
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}..{}",self.start,self.end)
    }
}

impl From<Range<usize>> for Interval {
    fn from(r:Range<usize>) -> Interval {
	Interval::new(r.start,r.end)
    }
}
