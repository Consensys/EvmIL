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
use std::cmp::{PartialOrd,Ordering};

/// Identifies a sequential block of instructions within the original
/// bytecode sequence.  That is, a sequence does not contain a jump
/// destination (other than at the very start), and ends either with a
/// terminating instruction (e.g. `RETURN`, `REVERT`, etc) or an
/// unconditional branch (to another block).
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Block {
    /// Starting offset (in bytes) of this block.
    pub start: usize,
    /// End offset (in bytes) of this block.  That is the first byte
    /// which is not part of this block.
    pub end: usize
}

impl Block {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end);
        //
        Block{start,end}
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.start < other.start || (self.start == other.start && self.end < other.end)
    }

    fn gt(&self, other: &Self) -> bool {
        self.start > other.start || (self.start == other.start && self.end > other.end)
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        let n = self.start.cmp(&other.start);
        //
        match n {
            Equal => self.end.cmp(&other.end),
            _ => n
        }
    }
}
