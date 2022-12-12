use evmil::{Disassembler,Instruction,FromHexString};

// ============================================================================
//
// ============================================================================

#[test]
pub fn test_disassemble_01() {
    check("00", &[Instruction::STOP]);
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that disassembling a given hex string produces a given
/// sequence of instructions.
fn check(hex: &str, insns: &[Instruction]) {
    // Parse hex string into bytes
    let bytes = hex.from_hex_string().unwrap();
    // Disassembler bytes into instructions
    let is = Disassembler::new(&bytes).disassemble();
    // Check against expected instruction sequence
    assert_eq!(insns, is);
}
