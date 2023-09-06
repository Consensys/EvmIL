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

/// Provides a mechanism by which an instruction can be parameterised
/// to support different forms of control flow.
pub trait Operands {
    /// Identifies the type of 16bit relative offsets.
    type RelOffset16 : fmt::Display+fmt::Debug;
    /// Identifies the type for _push label_ instructions.
    type PushLabel : fmt::Display+fmt::Debug;
    /// Identifies the type for _label_ instructions.
    type Label : fmt::Display+fmt::Debug;
}

/// A void operand is used to signal that something is impossible
/// (i.e. because this instruction cannot be used in a particular
/// setting, etc).
#[derive(Clone,Debug,PartialEq)]
pub enum VoidOperand{}

impl fmt::Display for VoidOperand {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

/// Representation of instruction operands (more specifically, _branch
/// offsets_) as appropriate for _concrete bytecode instructions_.  In
/// legacy contracts, branch targets are implemented using _absolute
/// offsets_.  In EOF contracts, branch targets can also be
/// implemented using _relative offsets_.
#[derive(Clone,Debug,PartialEq)]
pub struct BytecodeOperands();

impl Operands for BytecodeOperands {
    type RelOffset16 = i16;
    /// We do not permit the `PUSHL` instruction here, since it is
    /// already represented by `PUSH`.
    type PushLabel = VoidOperand;
    /// Likewise, we do not permit the `LABEL` instruction here, since
    /// it has no concrete meaning.
    type Label = VoidOperand;
}
