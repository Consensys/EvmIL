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
use std::ops::Index;

/// Abstracts a sequence of elements.  This is similar to a slice,
/// except that it may not represent elements actually stored
/// consecutively in memory.  In other words, elements can be
/// constructed via computation on-the-fly.
pub trait Seq {
    type Output;
    
    /// Returns the number of elements in the sequence, also referred
    /// to as its 'length'.
    fn len(&self) -> usize;

    /// Returns a reference to an element of this sequence.
    fn get(&self, index: usize) -> Option<Self::Output>;

    /// Returns `true` if the sequence contains no elements.
    fn is_empty(&self) -> bool { self.len() == 0 }
}

// =============================================================================
// Instantiations
// =============================================================================

// impl<T> Seq for Vec<T> {
//     type Output = &'_ T;
        
//     fn len(&self) -> usize { self.len() }

//     fn get(&self, index: usize) -> Option<Self::Output> {
//         Some(<Self as Index<usize>>::index(self,index))
//     }
// }

impl<'a,T> Seq for &'a [T] {
    type Output = &'a T;
    
    fn len(&self) -> usize {
        // Unsure how to write this directly!
        let tmp : &'a [T] = self;
        tmp.len()
    }

    fn get(&self, index: usize) -> Option<Self::Output> {
        Some(&self[index])
    }
}

impl<'a,T,const N: usize> Seq for &'a [T; N] {
    type Output = &'a T;
    
    fn len(&self) -> usize { N }

    fn get(&self, index: usize) -> Option<Self::Output> {
        Some(&self[index])
    }
}

impl<T:Copy> Seq for [T] {
    type Output = T;
    
    fn len(&self) -> usize { self.len() }

    fn get(&self, index: usize) -> Option<Self::Output> {
        Some(self[index])
    }
}

impl<T:Copy,const N: usize> Seq for [T; N] {
    type Output = T;
    
    fn len(&self) -> usize { N }

    fn get(&self, index: usize) -> Option<Self::Output> {
        Some(self[index])
    }
}
