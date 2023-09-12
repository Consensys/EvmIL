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
use crate::bytecode::{Assembly,Assemble,Disassemble,Instruction,StructuredSection};

pub fn from_bytes(bytes: &[u8]) -> Assembly {
    let insns = bytes.disassemble();
    // Find start of data section, as determined by the first INVALID
    // instruction encountered.
    let mut fe = bytes.len();
    let mut pc = 0;
    //
    for i in &insns {        
        if i == &Instruction::INVALID {
            fe = pc;
            break;
        }
        pc += i.length();
    }
    //
    if fe != bytes.len() {
        // Split code from data.  Note that we could something more
        // sophisticated here.  However, for contracts compiled from
        // Solidity, I don't believe there is any need.
        let code = bytes[..fe].disassemble();
        // Strip off invalid separator.
        let data = bytes[fe+1..].to_vec();
        Assembly::new(vec![StructuredSection::Code(code), StructuredSection::Data(data)])        
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
                // Signal start of data
                bytes.push(0xfe);
                // Copy data
                bytes.extend(bs);
            }
            StructuredSection::Code(insns) => {
                let is : &[Instruction] = &insns;
                bytes.extend(is.assemble())
            }
        }        
    }
    // Done
    bytes
}
