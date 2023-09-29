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
use crate::bytecode::Instruction;
use super::{cw256,EvmStack,EvmState,ConcreteStack,ConcreteState,trace,UnknownMemory,UnknownStorage};

/// For a given bytecode sequence, identify and insert `havoc`
/// instructions to prevent non-termination of more precise analyses.
/// A `havoc` instruction is needed anywhere the value of a given
/// stack location depends upon itself.  This can arise, for example,
/// in loops with code of the form `x := x + 1`.  To understand this,
/// consider the following example:
///
/// ```text
///    push 0x10
/// loop:
///    dup1
///    iszero
///    push exit
///    jumpi
///    push 0x1
///    swap1
///    sub
///    push loop
///    jump
/// exit:
///    stop
/// ```
///
/// This implements a simple loop which counts down from `0x10` with a
/// counter stored at the bottom of the stack.  For a precise analysis
/// which models integer values concretely, this can lead to an
/// infinite ascending chain (i.e. non-termination of the analysis).
/// To resolve this, a `havoc` statement can be inserted as follows:
///
/// ```text
///   push 0x10
/// loop:
///   havoc stack[0]
///   ...
/// ```
pub fn insert_havocs(insns: &mut [Instruction]) {
    todo!()
}
