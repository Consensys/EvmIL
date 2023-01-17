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
use crate::evm::opcode::*;
use crate::evm::{Evm, Stack, Stepable, Word};
use crate::util::w256;

// ===================================================================
// Concrete Stack
// ===================================================================

/// A concrete stack implementation backed by a `Vec`.
#[derive(Debug, PartialEq)]
pub struct ConcreteStack<T> {
    items: Vec<T>,
}

impl<T: Word> ConcreteStack<T> {
    pub fn new(items: &[T]) -> Self {
        ConcreteStack {
            items: items.to_vec(),
        }
    }
}

impl<T: Word> Default for ConcreteStack<T> {
    fn default() -> Self {
        ConcreteStack { items: Vec::new() }
    }
}

impl<T: Word> Stack for ConcreteStack<T> {
    type Word = T;

    fn peek(&self, n: usize) -> T {
        let i = self.items.len() - n;
        self.items[i - 1]
    }

    fn len(&self) -> T {
        // FIXME: broken for non-64bit architectures!
        let w: w256 = (self.items.len() as u64).into();
        // Convert into word
        w.into()
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self, n: usize) {
        for _i in 0..n {
            self.items.pop();
        }
    }

    fn set(&mut self, n: usize, item: T) {
        let i = self.items.len() - n;
        self.items[i - 1] = item;
    }
}
