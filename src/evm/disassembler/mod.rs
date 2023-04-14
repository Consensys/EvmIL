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
use std::collections::HashMap;
use crate::evm::{Instruction,AssemblyInstruction};

use crate::evm::AbstractInstruction::*;

pub fn disassemble(bytecodes: &[Instruction]) -> Vec<AssemblyInstruction> {
    let mut count = 0;
    let mut pc = 0;
    // Identify and allocate all labels
    let mut labels = HashMap::new();
    for b in bytecodes {
        //
        match b {
            RJUMP(r16)|RJUMPI(r16) => {
                let target = from_rel_offset(pc+3,*r16);
                // Allocate label (if not already)
                if !labels.contains_key(&target) {
                    labels.insert(target,format!("lab{count}"));
                    count += 1;
                }
            }
            _ => {}
        }
        //
        pc += b.length();
    }
    // Translate all instructions whilst inserting labels.
    pc = 0;
    let mut asm = Vec::new();
    //
    for b in bytecodes {
        if labels.contains_key(&pc) {
            asm.push(LABEL(labels[&pc].clone()));
        }
        asm.push(translate_insn(pc,b,&labels));
        pc += b.length();
    }
    // Done
    asm
}

fn translate_insn(pc: usize, insn: &Instruction, labels: &HashMap<usize,String>) -> AssemblyInstruction {
    match insn {
        // 0s: Stop and Arithmetic Operations
        STOP => STOP,
        ADD => ADD,
        MUL => MUL,
        SUB => SUB,
        DIV => DIV,
        SDIV => SDIV,
        MOD => MOD,
        SMOD => SMOD,
        ADDMOD => ADDMOD,
        MULMOD => MULMOD,
        EXP => EXP,
        SIGNEXTEND => SIGNEXTEND,
        // 10s: Comparison & Bitwise Logic Operations
        LT => LT,
        GT => GT,
        SLT => SLT,
        SGT => SGT,
        EQ => EQ,
        ISZERO => ISZERO,
        AND => AND,
        OR => OR,
        XOR => XOR,
        NOT => NOT,
        BYTE => BYTE,
        SHL => SHL,
        SHR => SHR,
        SAR => SAR,
        // 20s: Keccak256
        KECCAK256 => KECCAK256,
        // 30s: Environmental Information
        ADDRESS => ADDRESS,
        BALANCE => BALANCE,
        ORIGIN => ORIGIN,
        CALLER => CALLER,
        CALLVALUE => CALLVALUE,
        CALLDATALOAD => CALLDATALOAD,
        CALLDATASIZE => CALLDATASIZE,
        CALLDATACOPY => CALLDATACOPY,
        CODESIZE => CODESIZE,
        CODECOPY => CODECOPY,
        GASPRICE => GASPRICE,
        EXTCODESIZE => EXTCODESIZE,
        EXTCODECOPY => EXTCODECOPY,
        RETURNDATASIZE => RETURNDATASIZE,
        RETURNDATACOPY => RETURNDATACOPY,
        EXTCODEHASH => EXTCODEHASH,
        // 40s: Block Information
        BLOCKHASH => BLOCKHASH,
        COINBASE => COINBASE,
        TIMESTAMP => TIMESTAMP,
        NUMBER => NUMBER,
        DIFFICULTY => DIFFICULTY,
        GASLIMIT => GASLIMIT,
        CHAINID => CHAINID,
        SELFBALANCE => SELFBALANCE,
        // 50s: Stack, Memory, Storage and Flow Operations
        POP => POP,
        MLOAD => MLOAD,
        MSTORE => MSTORE,
        MSTORE8 => MSTORE8,
        SLOAD => SLOAD,
        SSTORE => SSTORE,
        JUMP => JUMP,
        JUMPI => JUMPI,
        PC => PC,
        MSIZE => MSIZE,
        GAS => GAS,
        JUMPDEST => JUMPDEST,
        RJUMP(r16) => {
            let target = from_rel_offset(pc+3,*r16);
            RJUMP(labels[&target].clone())
        }
        RJUMPI(r16) => {
            let target = from_rel_offset(pc+3,*r16);
            RJUMPI(labels[&target].clone())
        }
        // 60s & 70s: Push Operations
        PUSH(bs) => PUSH(bs.clone()),
        // 80s: Duplication Operations
        DUP(n) => DUP(*n),
        // 90s: Swap Operations
        SWAP(n) => SWAP(*n),
        // a0s: Log Operations
        LOG(n) => LOG(*n),
        // f0s: System Operations
        CREATE => CREATE,
        CALL => CALL,
        CALLCODE => CALLCODE,
        RETURN => RETURN,
        DELEGATECALL => DELEGATECALL,
        CREATE2 => CREATE2,
        STATICCALL => STATICCALL,
        REVERT => REVERT,
        INVALID => INVALID,
        SELFDESTRUCT => SELFDESTRUCT,
        DATA(bs) => DATA(bs.clone()),
        //
        PUSHL(_)|LABEL(_) => unreachable!(),
    }
}

/// Calculate the absolute byte offset of a given relative jump target
/// from a given `pc` position (which is the `pc` after the
/// instruction in question).
fn from_rel_offset(pc: usize, rel: i16) -> usize {
    let mut r = pc as isize;
    r += rel as isize;
    r as usize
}
