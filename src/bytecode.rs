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
use crate::{Stmt};

pub enum Instruction {
    // 0s: Stop and Arithmetic Operations
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    // 10s: Comparison & Bitwise Logic Operations
    // 20s: Keccak256
    // 30s: Environmental Information
    // 40s: Block Information
    // 50s: Stack, Memory, Storage and Flow Operations
    // 60 & 70s: Push Operations
    PUSH(Vec<u8>),
    PUSHL(u32), // Push label offset.
    // 80s: Duplicate Operations
    DUP(u8),
    // 90s: Exchange Operations
    SWAP(u8),
    // a0s: Logging Operations
    LOG(u8),
}

/// Represents a sequence of zero or more bytecodes which can be
/// turned, for example, into a hex string.  Likewise, they can be
/// decompiled or further optimised.
pub struct Bytecode {
    /// The underlying bytecode sequence.
    bytecodes: Vec<Instruction>
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode{bytecodes:Vec::new()}
    }

    pub fn push(&mut self, insn: Instruction) {
        self.bytecodes.push(insn);
    }
}

/// Translate a sequence of IL statements into EVM bytecode, or fail
/// with an error.
impl TryFrom<&[Stmt]> for Bytecode {
    type Error = ();

    fn try_from(stmts: &[Stmt]) -> Result<Bytecode,Self::Error> {
        let mut bytecode = Bytecode::new();
        // Translate statements one-by-one
        for s in stmts {
            s.translate(&mut bytecode);
        }
        // Done
        Ok(bytecode)
    }
}
