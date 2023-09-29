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
use super::{Dependencies,find_dependencies};

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
///   havoc 0x0
///   ...
/// ```
pub fn insert_havocs(mut insns: Vec<Instruction>) -> Vec<Instruction> {
    let mut havocs = Vec::new();
    // Allocate storage to reuse for each instruction.
    let mut visited = vec![false;insns.len()];
    // Determine dependency information
    let deps = find_dependencies(&insns);
    // Look for cycles in the dependency graph
    for i in 0..insns.len() {
        if detect_cycle(i,&deps, &mut visited) {
            havocs.push(i+1);
        }
    }
    // Insert havoc statements
    for (i,idx) in havocs.iter().enumerate() {
        insns.insert(idx+i,Instruction::HAVOC(0));
    }
    //
    insns
}

/// Implements a straightforward (and somewhat naive) cycle detection
/// algorithm based around a depth-first traversal of the dependency
/// graph starting from instruction `i`.  During this search, if
/// instruction `i` is encountered again, then it is considered to
/// have a cycle in its dependency graph (and, hence, in need of a
/// `havoc` statement).
fn detect_cycle(i: usize, deps: &Dependencies, visited: &mut [bool]) -> bool {
    // Initialise visited storage
    visited.fill(false);
    // Initialise worklist for search.
    let mut worklist = vec![i];
    //
    while !worklist.is_empty() {
        // Pick off next instruction to explore
        let n = worklist.pop().unwrap();
        // FIXME: what we do now should depend on the instruction.
        // For example, memory assignments are terminators.
        // Iterate each frame
        for f in 0..deps.frames(n) {
            // And each dependency within that frame
            for dep in deps.get_frame(n,f) {
                // Check for match
                if *dep == i { return true; }
                //
                if !visited[n] {
                    // Mark as visited
                    visited[n] = true;
                    // Mark for subsequent search
                    worklist.push(n);
                }
            }
        }
    }
    // If we get here, then no cycle was detected.
    false
}
