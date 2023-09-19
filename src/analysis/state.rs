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
use std::fmt;
use crate::util::{JoinInto,Bottom};
use super::{EvmWord,EvmMemory,EvmStack,EvmStorage};

// ===================================================================
// State
// ===================================================================

/// Describes the state of an EVM at a given point (which could be
/// _running_ or _terminated_).  In essence, this simply packages all
/// the key components (e.g. stack, memory, storage) of the EVM state
/// together.
///
/// An `EvmState` can be _concrete_ or _abstract_.  For example, a
/// physically executing EVM operates over concrete states which are
/// updated after each executed instruction.  In contrast, a static
/// analysis over a sequence of EVM bytecodes produces abstract states
/// at each point which summarise the _set of all possible states_ at
/// that point.
pub trait EvmState : fmt::Debug {
    /// Defines what constitutes a word in this EVM.  For example, a
    /// concrete evm will use a `w256` here whilst an abstract evm
    /// will use something that can, for example, describe unknown
    /// values.
    type Word : EvmWord;

    /// Defines the stack implementation used in this EVM.
    type Stack : EvmStack<Word=Self::Word>;

    /// Defines the memory implementation used in this EVM.
    type Memory : EvmMemory<Word=Self::Word>;

    /// Defines the memory implementation used in this EVM.
    type Storage : EvmStorage<Word=Self::Word>;

    /// Get the program counter.  Every `EvmState` has a statically
    /// known `pc`.
    fn pc(&self) -> usize;

    /// Get read access to the operand stack contained within this
    /// state.
    fn stack(&self) -> &Self::Stack;

    /// Get write access to the operand stack contained within this
    /// state.
    fn stack_mut(&mut self) -> &mut Self::Stack;

    /// Get read access to the scratch memory contained within this
    /// state.
    fn memory(&self) -> &Self::Memory;

    /// Get write access to the scratch memory contained within this
    /// state.
    fn memory_mut(&mut self) -> &mut Self::Memory;

    /// Get read access to the persistent storage contained within
    /// this state.
    fn storage(&self) -> &Self::Storage;

    /// Get write access to the persistent storage contained within
    /// this state.
    fn storage_mut(&mut self) -> &mut Self::Storage;

    /// Move _program counter_ over `n` bytes in the next instruction.
    fn skip(&mut self, n: usize);

    /// Move _program counter_ to a given (byte) offset within the
    /// code section.
    fn goto(&mut self, pc: usize);
}

// ===================================================================
// Concrete State
// ===================================================================

/// An `EvmState` composed from three distinct (and potentially
/// abstract) components: _stack_, _memory_ and _storage_.
#[derive(Clone,Debug,PartialEq)]
pub struct ConcreteState<S,M,T>
where S:EvmStack,
      M:EvmMemory<Word=S::Word>,
      T:EvmStorage<Word=S::Word>    
{
    pc: usize,
    stack: S,
    memory: M,
    storage: T
}

impl<S,M,T> ConcreteState<S,M,T>
where S:EvmStack+Default,
      M:EvmMemory<Word=S::Word>+Default,
      T:EvmStorage<Word=S::Word>+Default   
{
    pub fn new() -> Self {
        let stack = S::default();
        let memory = M::default();
        let storage = T::default();
        Self{pc:0,stack,memory,storage}
    }
}

impl<S,M,T> EvmState for ConcreteState<S,M,T>
where S:EvmStack,
      M:EvmMemory<Word=S::Word>,
      T:EvmStorage<Word=S::Word>
{
    type Word = S::Word;
    type Stack = S;
    type Memory = M;
    type Storage = T;

    fn pc(&self) -> usize {
        self.pc
    }

    fn stack(&self) -> &Self::Stack {
        &self.stack
    }

    fn memory(&self) -> &Self::Memory {
        &self.memory
    }

    fn storage(&self) -> &Self::Storage {
        &self.storage
    }

    fn stack_mut(&mut self) -> &mut Self::Stack {
        &mut self.stack
    }

    fn memory_mut(&mut self) -> &mut Self::Memory {
        &mut self.memory
    }

    fn storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.storage
    }

    fn skip(&mut self, n: usize) {
        self.pc += n
    }

    /// Move _program counter_ to a given (byte) offset within the
    /// code section.
    fn goto(&mut self, pc: usize) {
        self.pc = pc;
    }
}

impl<S,M,T> fmt::Display for ConcreteState<S,M,T>
where S:EvmStack+Default+fmt::Display,
      M:EvmMemory<Word=S::Word>+Default,
      T:EvmStorage<Word=S::Word>+Default   
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"|{}|",self.stack)?;
        Ok(())
    }
}
