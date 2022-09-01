use evmil::{Bytecode,Parser,Term,ToHexString};
use evmil::Term::*;
use evmil::BinOp::*;

#[test]
pub fn test_assert_01() {
    let p = "assert 1";
    check(&p, "0x6001600657fe5b");
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that compiling a given sequence of terms produces a given
/// hex string.
fn check(stmt: &str, hex: &str) {
    // Parse statement into a term
    let t = Parser::new(stmt).parse().unwrap();
    // Translate statements into bytecode instructions
    let code = Bytecode::try_from(&[t]).unwrap();
    // Translate instructions into bytes
    let bytes : Vec<u8> = code.try_into().unwrap();
    // Check against expected hex string
    assert_eq!(hex, bytes.to_hex_string());
}
