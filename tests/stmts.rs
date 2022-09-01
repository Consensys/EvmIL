use evmil::{Bytecode,Parser,Term,ToHexString};
use evmil::Term::*;
use evmil::BinOp::*;

// ============================================================================
// Integer encodings
// ============================================================================

#[test]
pub fn test_int_01() {
    let p = "assert 1";
    check(&p, "0x6001600657fe5b");
}

#[test]
pub fn test_int_02() {
    let p = "assert 10";
    check(&p, "0x600a600657fe5b");
}

#[test]
pub fn test_int_03() {
    let p = "assert 203";
    check(&p, "0x60cb600657fe5b");
}

#[test]
pub fn test_int_04() {
    let p = "assert 5203";
    check(&p, "0x611453600757fe5b");
}

#[test]
pub fn test_int_05() {
    let p = "assert 33873";
    check(&p, "0x618451600757fe5b");
}

#[test]
pub fn test_int_06() {
    let p = "assert 56872345";
    check(&p, "0x630363cd99600957fe5b");
}

#[test]
pub fn test_int_07() {
    let p = "assert 340282366920938463463374607431768211455";
    check(&p, "0x6fffffffffffffffffffffffffffffffff601557fe5b");
}

// #[test]
// pub fn test_int_08() {
//     // Largest possible digit.
//     let p = "assert 115792089237316195423570985008687907853269984665640564039457584007913129639935";
//     //
//     check(&p, "0x600a600657fe5b");
// }

// ============================================================================
// Integer encodings
// ============================================================================

#[test]
pub fn test_hex_01() {
    let p = "assert 0x1";
    check(&p, "0x6001600657fe5b");
}

#[test]
pub fn test_hex_02() {
    let p = "assert 0x0a";
    check(&p, "0x600a600657fe5b");
}

#[test]
pub fn test_hex_03() {
    let p = "assert 0xcb";
    check(&p, "0x60cb600657fe5b");
}

#[test]
pub fn test_hex_04() {
    let p = "assert 0x1453";
    check(&p, "0x611453600757fe5b");
}

#[test]
pub fn test_hex_05() {
    let p = "assert 0x008451";
    check(&p, "0x618451600757fe5b");
}

#[test]
pub fn test_hex_06() {
    let p = "assert 0x363cd99";
    check(&p, "0x630363cd99600957fe5b");
}

#[test]
pub fn test_hex_07() {
    let p = "assert 0xffffffffffffffffffffffffffffffff";
    check(&p, "0x6fffffffffffffffffffffffffffffffff601557fe5b");
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
