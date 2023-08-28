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

mod assembler;
mod bytecode;
mod execution;
mod fork;
mod instruction;
mod semantics;
mod state;
/// Functionality related to _legacy_ (i.e. pre-EOF) contracts.  For
/// example, disassembling a legacy contract, assembling a legacy
/// contract, etc.
pub mod legacy;
/// Functionality related to contracts adhering to the _EVM Object
/// Format_.  See [EIP3540](https://eips.ethereum.org/EIPS/eip-3540).
/// Again, this includes assembling / disassembling EOF contracts,
/// validing them, managing EOF versions, etc.
pub mod eof;
/// Constants identifying the opcode associated with a given EVM
/// bytecode instruction.  Observe that several instructions can have
/// the same opcode (e.g. if they are active only in specific forks).
pub mod opcode;

pub use assembler::{AssembleError,AssemblyError};
pub use bytecode::*;
pub use execution::*;
pub use fork::*;
pub use instruction::*;
pub use semantics::*;
pub use state::*;
