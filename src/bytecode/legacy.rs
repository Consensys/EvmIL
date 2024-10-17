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
use crate::analysis::find_reachable;
use crate::bytecode::{Assembly,Assemble,Disassemble,Instruction,StructuredSection};

//   This is defined as the point after the last reachable
//   instruction.
pub fn from_bytes(bytes: &[u8]) -> Assembly {
    // Disassemble bytes into instructions.
    let mut insns = bytes.disassemble();
    // Compute reachability information.
    let reachable = find_reachable(&insns, usize::MAX).unwrap();
    // Mark all unreachable instructions
    mark_unreachable(&mut insns,bytes,&reachable);
    // Determine start of data section using reachability infor.
    let (i,pc) = find_data_start(&insns,bytes,&reachable);
    // Split contract
    if pc < bytes.len() {
        // Split code from data.
        insns.truncate(i);
        // Strip off invalid separator.
        let data = bytes[pc..].to_vec();
        Assembly::new(vec![StructuredSection::Code(insns), StructuredSection::Data(data)])        
    } else {
        Assembly::new(vec![StructuredSection::Code(insns)])
    }
}

/// Convert this bytecode contract into a byte sequence correctly
/// formatted for legacy code.
pub fn to_bytes(bytecode: &Assembly) -> Vec<u8> {
    let mut bytes = Vec::new();
    //
    for s in bytecode {
        match s {
            StructuredSection::Data(bs) => {
                // Copy data
                bytes.extend(bs);
            }
            StructuredSection::Code(insns) => {
                let is : &[Instruction] = insns;
                bytes.extend(is.assemble())
            }
        }        
    }
    // Done
    bytes
}

/// Convert every unreachable instruction into a `DATA` instruction to
/// signal that this is not executable code.
fn mark_unreachable(insns: &mut [Instruction], bytes: &[u8], reachable: &[bool]) {
    let mut pc = 0;
    
    for i in 0..insns.len() {
        let len = insns[i].length();
        // Check if instruction is reachable
        if !reachable[i] {
            // Determine end of instruction bytes
            let end = std::cmp::min(bytes.len(),pc + len);
            // Extract instruction bytes
            let data = bytes[pc..end].to_vec();
            // Replace instruction
            insns[i] = Instruction::DATA(data);
        }
        pc += len;
    }    
}

/// Find the start of the data section by traversing backwards from
/// the end of the instruction sequence until the first reachable
/// instruction is encountered.
fn find_data_start(insns: &[Instruction], bytes: &[u8], reachable: &[bool]) -> (usize,usize) {
    let mut i = insns.len();
    let mut pc = bytes.len();
    //
    while i > 0 && !reachable[i-1] {
        i -= 1;
        pc -= insns[i].length();
    }
        //
    (i,pc)
}
