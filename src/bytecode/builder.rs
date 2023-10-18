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
use crate::util;
use super::{Instruction};
use Instruction::{PUSH,RJUMPI,RJUMP};

/// Mechanism for constructing a bytecode `Assembly` by allowing
/// instructions to be patched before the final assembly is built.
/// For example, consider the problem of constructing an assembly from
/// this assembly language:
///
/// ```txt
///    push lab
///    jump
///    stop
/// lab:
///    jumpdest
/// ```
///
/// The challenge is that, when constructing the `push lab`
/// instruction, we don't yet know the _instruction offset_ of `lab`.
/// A `Builder` allows one to register a label and create an instance
/// of `PUSH` which is later patched to ensure it has the correct
/// instruction offset.
pub struct Builder {
    /// The set of registered labels where each entry optionally
    /// identifies the relevant instruction offset.  Observe that a
    /// label may be registered before its offset is known, in which
    /// case its corresponding entry will be `None`.
    labels: Vec<(String,Option<usize>)>,
    /// Offsets of instructions requiring a patch.
    patches: Vec<usize>,
    /// Byte offset of current cursor position.
    byte_offset: usize,
    /// The set of (unpatched) instructions.  Every branch instruction
    /// in this is assumed to refer to an _instruction label_.
    insns: Vec<Instruction>
}

impl Builder {
    pub fn new() -> Self {
        Self{labels: Vec::new(),
             patches: Vec::new(),
             byte_offset: 0,
             insns: Vec::new()
        }
    }

    /// Determine the number of instructions currently pushed into
    /// this builder.
    pub fn len(&self) -> usize {
        self.insns.len()
    }

    /// Returns `true` if no instructions have yet been pushed into
    /// this builder.
    pub fn is_empty(&self) -> bool { self.insns.is_empty() }
    
    /// Get the _label index_ associated with a particular label.  If
    /// such an index does not already exist, then a new label is
    /// registered.
    pub fn get_label(&mut self, label: &str) -> usize {
        // Check for existing label
        for (i,(l,_)) in self.labels.iter().enumerate() {
            if label == l {
                // Match
                return i;
            }
        }
        // Doesn't exist        
        self.labels.push((label.to_string(),None));
        self.labels.len() - 1                
    }

    /// Set the instruction offset associated with a given label.  If
    /// the label does not yet exist (i.e. as not yet been assigned an
    /// index), then it will be.
    pub fn set_label(&mut self, label: &str, offset: usize) -> Result<(),()> {
        let index = self.get_label(label);
        match &self.labels[index] {
            (_,None) => {
                // Assign offset
                self.labels[index].1 = Some(offset);
                Ok(())
            }
            (_,Some(_)) => {
                // Duplicate label!
                Err(())
            }
        }        
    }

    /// Mark a label at the current instruction offset.  If the label
    /// does not yet exist (i.e. as not yet been assigned an index),
    /// then it will be.
    pub fn mark_label(&mut self, label: &str) -> Result<(),()> {
        self.set_label(label, self.byte_offset)
    }

    /// Push a new instruction onto the builder.  
    pub fn push_labeled(&mut self, insn: Instruction) {
        // sanity check whether instruction can be patched.
        match insn {
            PUSH(_)|RJUMP(_)|RJUMPI(_) => {}
            _ => {
                // do nothing
                panic!("unpatchable instruction!");
            }
        }
        // Mark patch
        self.patches.push(self.insns.len());
        // Store instruction
        self.push(insn);
    }
    
    /// Push a new instruction onto the builder.  
    pub fn push(&mut self, insn: Instruction) {
        self.byte_offset += insn.length();
        self.insns.push(insn);
    }

    /// Construct the final assembly by patching all labels used
    /// within instructions.
    pub fn to_insns(mut self) -> Vec<Instruction> {
        // Apply patches
        for i in &self.patches {
            self.insns[*i] = self.patch(&self.insns[*i]);
        }
        self.insns
    }

    fn patch(&self, insn: &Instruction) -> Instruction {
        match insn {
            PUSH(bytes) => {
                // Extract the label
                let lab = util::from_be_bytes(bytes) as usize;
                let offset = self.labels[lab].1.unwrap();
                // Always generate a push2 instruction
                PUSH(vec![(offset/256) as u8, (offset%256) as u8])
            }
            RJUMP(lab) => {
                RJUMP(self.labels[*lab].1.unwrap())                    
            }
            RJUMPI(lab) => {
                RJUMPI(self.labels[*lab].1.unwrap())                                        
            }
            _ => {
                unreachable!()
            }
        } 
    }
}
