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
use crate::evm;
use crate::util::{Bottom, Interval, Join, JoinInto, JoinLattice};
use std::{cmp, fmt, mem};

// ============================================================================
// Disassembly Context
// ============================================================================

#[derive(Debug, PartialEq)]
pub struct AbstractStack<T: PartialEq> {
    // The lower segment of an abstract stack represents a variable
    // number of unknown values.  An interval is used for a compact
    // representation.  So, for example, `0..1` represents two
    // possible lower segments: `[]` and `[??]`.
    lower: Interval<usize>,
    // The upper segment represents zero or more concrete values on
    // the stack.
    upper: Vec<T>,
}

impl<T> AbstractStack<T>
where
    T: PartialEq + Copy + JoinLattice,
{
    pub fn new(lower: impl Into<Interval<usize>>, upper: Vec<T>) -> Self {
        let lower_iv = lower.into();
        // Done
        Self {
            lower: lower_iv,
            upper,
        }
    }
    /// Construct an empty stack.
    pub fn empty() -> Self {
        Self::new(0, Vec::new())
    }
    /// Determine possible lengths of the stack as an interval
    pub fn len(&self) -> Interval<usize> {
        self.lower.add(self.upper.len().into())
    }
    /// Peek nth item on the stack (where `0` is top).
    pub fn peek(&self, n: usize) -> T {
        // Should never be called on bottom
        assert!(self != &Self::BOTTOM);
        // Get the nth value!
        if n < self.upper.len() {
            // Determine stack index
            let i = self.upper.len() - (1 + n);
            // Extract value
            self.upper[i]
        } else {
            T::TOP
        }
    }
    /// Push an iterm onto this stack.
    pub fn push(&mut self, val: T) -> &mut Self {
        // Should never be called on bottom
        assert!(self != &Self::BOTTOM);
        //
        if val == T::TOP && self.upper.len() == 0 {
            self.lower = self.lower.add(1.into());
        } else {
            // Pop target address off the stack.
            self.upper.push(val);
        }
        self
    }
    // Pop a single item off the stack
    pub fn pop(&mut self) -> &mut Self {
        // Should never be called on bottom
        assert!(self != &Self::BOTTOM);
        // Pop target address off the stack.
        if self.upper.is_empty() {
            self.lower = self.lower.sub(1.into());
        } else {
            self.upper.pop();
        }
        self
    }

    /// Determine the minimum length of any stack represented by this
    /// abstract stack.
    pub fn min_len(&self) -> usize {
        self.lower.start + self.upper.len()
    }
    /// Determine the maximum length of any stack represented by this
    /// abstract stack.
    pub fn max_len(&self) -> usize {
        self.lower.end + self.upper.len()
    }
    /// Access the array of concrete values represented by this stack
    /// (i.e. the _upper_ portion of the stack).
    pub fn values<'a>(&'a self) -> &'a [T] {
        &self.upper
    }

    /// Set `ith` item from the top on this stack.  Thus, `0` is the
    /// top of the stack, etc.
    pub fn set(mut self, n: usize, val: T) -> Self {
        // Should never be called on bottom
        assert!(self != Self::BOTTOM);
        // NOTE: inefficient when putting unknown value into lower
        // portion.
        self.ensure_upper(n + 1);
        // Determine stack index
        let i = self.upper.len() - (1 + n);
        // Set value
        self.upper[i] = val;
        // Rebalance (which can be necessary is val unknown)
        self.rebalance()
    }

    /// Join two abstract stacks together.
    pub fn join(self, other: &AbstractStack<T>) -> Self {
        let slen = self.upper.len();
        let olen = other.upper.len();
        // Determine common upper length
        let n = cmp::min(slen, olen);
        // Normalise lower segments
        let lself = self.lower.add(Interval::from(slen - n));
        let lother = other.lower.add(Interval::from(olen - n));
        let mut merger = AbstractStack::new(lself.union(&lother), Vec::new());
        // Push merged items from upper segment
        for i in (0..n).rev() {
            let ithself = self.peek(i);
            let ithother = other.peek(i);
            merger.push(ithself.join(&ithother));
        }
        // Done
        merger
    }

    /// Rebalance the stack if necessary.  This is necessary when the
    /// upper portion contains unknown values which can be shifted
    /// into the lower portion.
    fn rebalance(mut self) -> Self {
        let mut i = 0;
        // Determine whether any rebalancing necessary.
        while i < self.upper.len() {
            if self.upper[i] != T::TOP {
                break;
            }
            i = i + 1;
        }
        // Rebalance only if necessary
        if i > 0 {
            // Increase lower portion
            self.lower = self.lower.add(i.into());
            // Decrease upper portion
            self.upper.drain(0..i);
        }
        //
        self
    }

    /// Ensure the upper portion has space for at least `n` elements.
    fn ensure_upper(&mut self, n: usize) {
        // FIXME: inefficient!!
        while n > self.upper.len() {
            self.upper.insert(0, T::TOP);
            self.lower = self.lower.sub(1.into());
        }
    }
}

// ==================================================================
// Standard Traits
// ==================================================================

impl<T: PartialEq + Copy + JoinLattice> Default for AbstractStack<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Clone for AbstractStack<T>
where
    T: PartialEq + Clone,
{
    fn clone(&self) -> Self {
        AbstractStack {
            lower: self.lower.clone(),
            upper: self.upper.clone(),
        }
    }
}

impl<T> fmt::Display for AbstractStack<T>
where
    T: Copy + PartialEq + Bottom + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &Self::BOTTOM {
            write!(f, "_|_")
        } else {
            write!(f, "({})[", self.lower)?;
            for i in 0..self.upper.len() {
                write!(f, "{}", self.upper[i])?;
            }
            write!(f, "]")
        }
    }
}

// ==================================================================
// Lattice Traits
// ==================================================================

impl<T> Bottom for AbstractStack<T>
where
    T: PartialEq + Copy + Bottom,
{
    const BOTTOM: Self = Self {
        lower: Interval::BOTTOM,
        upper: Vec::new(),
    };
}

impl<T> JoinInto for AbstractStack<T>
where
    T: PartialEq + Copy + JoinLattice,
{
    /// Merge an abstract stack into this stack, whilst reporting
    /// whether this stack changed or not.
    fn join_into(&mut self, other: &AbstractStack<T>) -> bool {
        // NOTE: this could be done more efficiently.
        let old = self.clone();
        let mut tmp = Self::empty();
        // Install dummy value to keep self alive
        mem::swap(self, &mut tmp);
        // Perform merge
        *self = tmp.join(other);
        // Check for change
        *self != old
    }
}

// ===================================================================
// evm::Word
// ===================================================================

impl<T: evm::Word> evm::Stack for AbstractStack<T>
where
    T: PartialEq + JoinLattice,
{
    type Word = T;

    /// Determine number of items on stack.
    fn len(&self) -> T {
        todo!("FIX ME");
    }

    /// Peek nth item on the stack (where `0` is top).
    fn peek(&self, n: usize) -> T {
        self.peek(n)
    }
    /// Push an iterm onto this stack.
    fn push(&mut self, val: T) {
        self.push(val);
    }
    /// Pop `n` items of the stack.
    fn pop(&mut self, n: usize) {
        (0..n).for_each(|_| {
            self.pop();
        });
    }
    /// Set nth item on stack
    fn set(&mut self, n: usize, item: T) {
        if n < self.upper.len() {
            let i = self.upper.len() - n;
            //
            self.upper[i - 1] = item;
        } else {
            todo!("")
        }
    }
}
