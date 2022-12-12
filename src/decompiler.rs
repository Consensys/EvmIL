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
use crate::{BinOp,Bytecode,Instruction,Region,Term};

type Result = std::result::Result<(),Error>;


// ============================================================================
// Errors
// ============================================================================

#[derive(Debug)]
pub enum Error {
   Unknown
}

// ============================================================================
// Decompiler
// ============================================================================

pub struct Decompiler<'a> {
    /// Access to the list of terms being deconstructed.
    terms: &'a mut Vec<Term>,
}

impl<'a> Decompiler<'a> {
    pub fn new(terms: &'a mut Vec<Term>) -> Self {
        Self{terms}
    }

    /// Decompile a given sequence of bytecodes into zero or more
    /// terms.
    pub fn decompile(&mut self, insns: &[Instruction]) -> Result {
	Ok(())
    }
}
