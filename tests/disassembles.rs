use std::convert::{AsRef};
use evmil::{Instruction,FromHexString,CfaState};
use evmil::{Disassemble,Disassembly};
use evmil::Instruction::*;

// ============================================================================
// Single block Tests
// ============================================================================

#[test]
pub fn test_disassemble_single_01() {
    check("0x00", &[STOP]);
}

#[test]
pub fn test_disassemble_single_02() {
    check("0x6000", &[PUSH(vec![0])]);
}

#[test]
pub fn test_disassemble_single_03() {
    check("0x611234", &[PUSH(vec![0x12,0x34])]);
}

#[test]
pub fn test_disassemble_single_04() {
    check("0x62120034", &[PUSH(vec![0x12,0x00,0x34])]);
}

// ============================================================================
// Double block Tests
// ============================================================================

#[test]
pub fn test_disassemble_double_01() {
    // A minimal two-block program
    check("0x6003565b", &[PUSH(vec![3]),JUMP,JUMPDEST(3)]);
}

#[test]
pub fn test_disassemble_double_03() {
    // A minimal conditional two-block program
    check("0x60016005575b", &[PUSH(vec![1]),PUSH(vec![5]),JUMPI,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_double_04() {
    // A simple conditional two-block program
    check("0x6001600657005b", &[PUSH(vec![1]),PUSH(vec![6]),JUMPI,STOP,JUMPDEST(6)]);
}

// ============================================================================
// Triple block Tests
// ============================================================================

#[test]
pub fn test_disassemble_triple_01() {
    // Minimal three-block program
    check("0x6003565b6007565b", &[PUSH(vec![3]),JUMP,JUMPDEST(3),PUSH(vec![7]),JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_triple_02() {
    // Three-block program with back jump
    check("0x6005565b005b600356", &[PUSH(vec![5]),JUMP,JUMPDEST(3),STOP,JUMPDEST(5),PUSH(vec![3]),JUMP]);
}

// ============================================================================
// Split block Tests
// ============================================================================

#[test]
pub fn test_disassemble_split_01() {
    // A minimal split multiblock program
    check("0x600456005b", &[PUSH(vec![4]),JUMP,DATA(vec![0]),JUMPDEST(4)]);
}

#[test]
pub fn test_disassemble_split_02() {
    // A minimal split multiblock program.  This program contains an
    // invalid JUMPDEST.
    check("0x600456605b", &[PUSH(vec![4]),JUMP,PUSH(vec![0x5b])]);
}

#[test]
pub fn test_disassemble_split_03() {
    // A minimal split multiblock program
    check("0x6003565b0061", &[PUSH(vec![3]),JUMP,JUMPDEST(3),STOP,DATA(vec![0x61])]);
}

#[test]
pub fn test_disassemble_split_04() {
    // A minimal split multiblock program
    check("0x60055601025b", &[PUSH(vec![5]),JUMP,DATA(vec![1,2]),JUMPDEST(5)]);
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that disassembling a given hex string produces a given
/// sequence of instructions.
fn check(hex: &str, insns: &[Instruction]) {
    // Parse hex string into bytes
    let bytes = hex.from_hex_string().unwrap();
    // Disassemble bytes into instructions
    let mut disasm : Disassembly<CfaState> = Disassembly::new(&bytes);
    //
    disasm.analyse();
    // Check against expected instruction sequence
    assert_eq!(insns, disasm.to_vec());
}
