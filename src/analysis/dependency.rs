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

/// Identifies the dependency frames for each instruction in a given
/// sequence of instructions.
pub struct Dependencies {
    frames: Vec<Vec<Vec<usize>>>
}

impl Dependencies {
    /// Determine the number of dependency frames for a given
    /// instruction.
    pub fn frames(&self, insn: usize) -> usize {
        self.frames[insn].len()
    }

    /// Get the _kth_ dependency frame for a given instruction.
    pub fn get_frame(&self, insn: usize, k: usize) -> &[usize] {
        &self.frames[insn][k]
    }
}

/// For a given bytecode sequence, identify the _dependency frames_
/// for all instructions.  A dependency frame contains an entry for
/// each operand of the instruction, where each entry identifies a
/// source instruction within the sequence.  If there are multiple
/// paths through the sequence to the given instruction, then there
/// may be multiple frames for a given instruction.
///
/// # Examples
/// The following illustrates a minimal example:
///
/// ```
/// [0]   push 0x1
/// [1]   push 0x2
/// [2]   add          ;; [0,1]
/// ```
///
/// Here, instruction indices have been inserted to aid readability
/// and the dependency frames (if any) are given next to each
/// instruction.  In this case, there are no frames for the first two
/// instructions (i.e. as they have no operands).  For the `add`
/// instruction, we have one frame `[0,1]` which identifies the two
/// `push` instructions as its dependencies.
///
/// A more interesting example is:
///
/// ```
/// [0]   calldatasize
/// [1]   push lab0
/// [2]   jumpi        ;; [1]
/// [3]   push 0x1
/// [4]   jump lab1    ;; [3]
/// lab0:
/// [5]   push 0x2
/// lab1:
/// [6]   neg          ;; [3][5]
/// ```
///
/// Here, we can see that the `neg` instruction has two dependency
/// frames indicating its dependency is either `push 0x1` _or_ `push
/// 0x2` (i.e. depending on which path was taken through the
/// control-flow graph).
pub fn find_dependencies(insns: &[Instruction]) -> Dependencies {
    todo!()
}
