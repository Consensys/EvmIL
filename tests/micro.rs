use evmil::{Bytecode,Term,ToHexString};
use evmil::Term::*;

// ============================================================================
// Expressions
// ============================================================================

#[test]
pub fn test_expr_01() {
    let s1 = Int(vec![1]);
    check(&[s1], "0x6001");
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that compiling a given sequence of terms produces a given
/// hex string.
fn check(terms: &[Term], hex: &str) {
    // Translate statements into bytecode instructions
    let code = Bytecode::try_from(terms).unwrap();
    // Translate instructions into bytes
    let bytes : Vec<u8> = code.try_into().unwrap();
    // Check against expected hex string
    assert_eq!(hex, bytes.to_hex_string());
}
