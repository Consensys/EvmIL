use evmil::il::BinOp::*;
use evmil::il::Term;
use evmil::il::Term::*;
use evmil::bytecode::{Assembly};
use evmil::util::ToHexString;

// ============================================================================
// Expressions
// ============================================================================

#[test]
pub fn test_literal_01() {
    let s1 = Int(vec![1]);
    check(&[s1], "0x6001");
}

#[test]
pub fn test_binop_01() {
    let s1 = Binary(Add, Box::new(Int(vec![1])), Box::new(Int(vec![2])));
    check(&[s1], "0x6002600101");
}

// ============================================================================
// Statements
// ============================================================================

#[test]
pub fn test_stmt_01() {
    let s1 = Assert(Box::new(Int(vec![1])));
    check(&[s1], "0x600161000b5760006000fd5b");
}

#[test]
pub fn test_stmt_02() {
    let s1 = Assert(Box::new(Int(vec![1])));
    check(&[s1.clone(), s1], "0x600161000b5760006000fd5b60016100175760006000fd5b");
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that compiling a given sequence of terms produces a given
/// hex string.
fn check(terms: &[Term], hex: &str) {
    // Translate statements into bytecode instructions
    let bytecode = Assembly::try_from(terms).unwrap();
    // Translate instructions into bytes
    let bytes: Vec<u8> = bytecode.to_legacy_bytes();
    // Check against expected hex string
    assert_eq!(hex, bytes.to_hex_string());
}
