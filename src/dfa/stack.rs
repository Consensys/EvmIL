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
use std::{cmp,fmt,mem};
use crate::interval::{Interval,MAX_INTERVAL};

/// Represents the singleton set of empty abstract stacks (which is
/// distinct from empty set of stacks).
pub const EMPTY_STACK : AbstractStack = AbstractStack{lower: Interval::new_const(0,0), upper: Vec::new()};
/// Bottom represents the empty set of stacks.
pub const BOTTOM_STACK : AbstractStack = AbstractStack{lower: MAX_INTERVAL, upper: Vec::new()};

// ============================================================================
// Abstract Value
// ============================================================================

/// An abstract value is either a known constant, or an unknown
/// (i.e. arbitrary value).
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum AbstractValue {
    Known(usize),
    Unknown
}

impl AbstractValue {
    pub fn merge(self, other: AbstractValue) -> AbstractValue {
        if self == other {
            self
        } else {
            AbstractValue::Unknown
        }
    }

    pub fn unwrap(&self) -> usize {
        match self {
            AbstractValue::Known(n) => *n,
            AbstractValue::Unknown => {
                panic!("unwrapping unknown value");
            }
        }
    }
}

impl fmt::Display for AbstractValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AbstractValue::Unknown => write!(f,"(??)"),
            AbstractValue::Known(n) => write!(f,"({:#08x})",n)
        }
    }
}

// ============================================================================
// Disassembly Context
// ============================================================================

#[derive(Debug,PartialEq)]
pub struct AbstractStack {
    // The lower segment of an abstract stack represents a variable
    // number of unknown values.  An interval is used for a compact
    // representation.  So, for example, `0..1` represents two
    // possible lower segments: `[]` and `[??]`.
    lower: Interval,
    // The upper segment represents zero or more concrete values on
    // the stack.
    upper: Vec<AbstractValue>
}

impl AbstractStack {
    pub fn new<T:Into<Interval>>(lower: T, upper: Vec<AbstractValue>) -> Self {
        // Convert input range into an interval
        let lower_iv = lower.into();
        // Sanity check (maximum) stack height
        assert!((lower_iv.end+upper.len()) <= 1024);
        // Done
        Self{lower:lower_iv,upper}
    }
    pub fn is_bottom(&self) -> bool {
        *self == BOTTOM_STACK
    }
    /// Determine possible lengths of the stack as an interval
    pub fn len(&self) -> Interval {
        self.lower.add(self.upper.len())
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
    /// Push an iterm onto this stack.
    pub fn push(mut self, val: AbstractValue) -> Self {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        //
        if val == AbstractValue::Unknown && self.upper.len() == 0 {
            self.lower = self.lower.add(1);
        } else {
            // Pop target address off the stack.
            self.upper.push(val);
        }
        // Done
        self
    }
    /// Pop an item of this stack, producing an updated state.
    pub fn pop(mut self) -> Self {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        // Pop target address off the stack.
        if self.upper.is_empty() {
            self.lower = self.lower.sub(1);
        } else {
            self.upper.pop();
        }
        // Done
        self
    }
    /// Perk nth item on the stack (where `0` is top).
    pub fn peek(&self, n: usize) -> AbstractValue {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        // Get the nth value!
        if n < self.upper.len() {
            // Determine stack index
            let i = self.upper.len() - (1+n);
            // Extract value
            self.upper[i]
        } else {
            AbstractValue::Unknown
        }
    }
    /// Set specific item on this stack.
    pub fn set(mut self, n: usize, val: AbstractValue) -> Self {
        // Should never be called on bottom
        assert!(!self.is_bottom());
        if n < self.upper.len() {
            // Determine stack index
            let i = self.upper.len() - (1+n);
            // Set value
            self.upper[i] = val;
            // Done
            self
        } else {
            // This case is complicated because we need to expand the
            // upper portion to include the new value (where
            // possible).
            todo!("Implement me!");
        }
    }

    /// Merge two abstract stacks together.
    pub fn merge(self, other: &AbstractStack) -> Self {
        let slen = self.upper.len();
        let olen = other.upper.len();
        // Determine common upper length
        let n = cmp::min(slen,olen);
        // Normalise lower segments
        let lself = self.lower.add(slen - n);
        let lother = other.lower.add(olen - n);
        let mut merger = AbstractStack::new(lself.union(&lother),Vec::new());
        // Push merged items from upper segment
        for i in (0..n).rev() {
            let ithself = self.peek(i);
            let ithother = other.peek(i);
            merger = merger.push(ithself.merge(ithother));
        }
        // Done
        merger
    }

    /// Merge an abstract stack into this stack, whilst reporting
    /// whether this stack changed or not.
    pub fn merge_into(&mut self, other: &AbstractStack) -> bool {
        // NOTE: this could be done more efficiently.
        let old = self.clone();
        let mut tmp = EMPTY_STACK;
        // Install dummy value to keep self alive
        mem::swap(self, &mut tmp);
        // Perform merge
        *self = tmp.merge(other);
        // Check for change
        *self != old
    }
}

impl Clone for AbstractStack {
    fn clone(&self) -> Self {
        AbstractStack{lower:self.lower.clone(),upper:self.upper.clone()}
    }
}

impl fmt::Display for AbstractStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == BOTTOM_STACK {
            write!(f,"_|_")
        } else {
            write!(f,"({})[",self.lower)?;
            for i in 0..self.upper.len() {
                write!(f,"{}",self.upper[i])?;
            }
            write!(f,"]")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dfa::{AbstractStack,AbstractValue,EMPTY_STACK};

    const ZERO : AbstractValue = AbstractValue::Known(0);
    const ONE : AbstractValue = AbstractValue::Known(1);
    const TWO : AbstractValue = AbstractValue::Known(2);
    const THREE : AbstractValue = AbstractValue::Known(3);
    const UNKNOWN : AbstractValue = AbstractValue::Unknown;

    #[test]
    fn test_abstract_stack_01() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN), AbstractStack::new(1..1,vec![]));
    }

    #[test]
    fn test_abstract_stack_02() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN).push(UNKNOWN), AbstractStack::new(2..2,vec![]));
    }

    #[test]
    fn test_abstract_stack_03() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN).push(UNKNOWN).pop(), AbstractStack::new(1..1,vec![]));
    }

    #[test]
    fn test_abstract_stack_04() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN).push(ZERO), AbstractStack::new(1..1,vec![ZERO]));
    }

    #[test]
    fn test_abstract_stack_05() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE), AbstractStack::new(1..1,vec![ZERO,ONE]));
    }

    #[test]
    fn test_abstract_stack_06() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE).pop(), AbstractStack::new(1..1,vec![ZERO]));
    }

    #[test]
    fn test_abstract_stack_07() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop(), AbstractStack::new(1..1,vec![]));
    }

    #[test]
    fn test_abstract_stack_08() {
        let st = AbstractStack::new(0..0,vec![]);
        assert_eq!(st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop().pop(), EMPTY_STACK);
    }

    #[test]
    fn test_abstract_stack_09() {
        let st = AbstractStack::new(0..0,vec![ONE,TWO,THREE]);
        assert_eq!(st.set(0,ZERO),AbstractStack::new(0..0,vec![ONE,TWO,ZERO]));
    }

    #[test]
    fn test_abstract_stack_0a() {
        let st = AbstractStack::new(0..0,vec![ONE,TWO,THREE]);
        assert_eq!(st.set(1,ZERO),AbstractStack::new(0..0,vec![ONE,ZERO,THREE]));
    }

    #[test]
    fn test_abstract_stack_10() {
        let st1 = AbstractStack::new(1..1,vec![ONE]);
        let st2 = AbstractStack::new(0..0,vec![ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(0..1,vec![ONE]));
    }
    #[test]
    fn test_abstract_stack_11() {
        let st1 = AbstractStack::new(0..0,vec![ONE]);
        let st2 = AbstractStack::new(1..1,vec![ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(0..1,vec![ONE]));
    }
    #[test]
    fn test_abstract_stack_12() {
        let st1 = AbstractStack::new(0..0,vec![TWO,ONE]);
        let st2 = AbstractStack::new(1..1,vec![ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(1..1,vec![ONE]));
    }
    #[test]
    fn test_abstract_stack_13() {
        let st1 = AbstractStack::new(1..1,vec![ONE]);
        let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(1..1,vec![ONE]));
    }
    #[test]
    fn test_abstract_stack_14() {
        let st1 = AbstractStack::new(0..0,vec![ONE,ONE]);
        let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(1..1,vec![ONE]));
    }
    #[test]
    fn test_abstract_stack_15() {
        let st1 = AbstractStack::new(1..1,vec![ONE,ONE]);
        let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(1..2,vec![ONE]));
    }
    #[test]
    fn test_abstract_stack_16() {
        let st1 = AbstractStack::new(0..1,vec![ONE,ONE]);
        let st2 = AbstractStack::new(0..0,vec![TWO,ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(1..2,vec![ONE]));
    }
    #[test]
    fn test_abstract_stack_17() {
        let st1 = AbstractStack::new(0..0,vec![ONE,ONE]);
        let st2 = AbstractStack::new(0..1,vec![TWO,ONE]);
        assert_eq!(st1.merge(&st2),AbstractStack::new(1..2,vec![ONE]));
    }
}
