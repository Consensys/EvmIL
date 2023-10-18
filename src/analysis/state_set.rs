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
use crate::util::{Bottom,JoinInto};
use super::EvmState;

/// An `EvmStateSet` represents a set of distinct states at a given
/// point in a bytecode sequence.  The assumption is that all states
/// in a given set have the same `pc` value.  The purpose of this is
/// to collect all distinct states encountered during an execution or
/// analysis of a bytecode sequence.
pub trait EvmStateSet : JoinInto<Self::State> {
    /// The underlying type of states stored in this set.
    type State : EvmState;
    /// Determine the number of states represented in this state set.
    fn size(&self) -> usize;
    /// Iterate over all distinct states in this set.
    fn iter(&self) -> std::slice::Iter<'_,Self::State>;
}

// ===================================================================
// Vec<Vec<T>>
// ===================================================================

impl<T:Clone+EvmState+PartialEq> JoinInto<T> for Vec<T> {
    fn join_into(&mut self, other: &T) -> bool {
        let n = self.len();
        // Simplest possible join operator (for now)        
        self.push(other.clone());
        // Deduplicate matching entries
        self.dedup();
        // Check whether anything actually changed.
        n != self.len()
    }
}

impl<T:Clone+EvmState+PartialEq> EvmStateSet for Vec<T> {
    type State = T;

    fn size(&self) -> usize {
        self.len()
    }
    
    fn iter(&self) -> std::slice::Iter<'_,T> {
        self.into_iter()
    }
}

impl<T> Bottom for Vec<T> {
    const BOTTOM : Vec<T> = Vec::new();
}
