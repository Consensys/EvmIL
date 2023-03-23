use std::fmt;
use std::fs;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use evmil::ll::{Assembler};
use evmil::util::{FromHexString};

pub static TESTS_DIR: &str = "tests/files";

// Include the programmatically generated test file.
include!(concat!(env!("OUT_DIR"), "/tests.rs"));

/// Run a specific test by loading the file out of the reference tests
/// repository and attempting to parse it.  All reference tests should
/// parse correctly.
fn check(test: &str) {
    // Construct input files
    let asmfile = to_asmfile(test);
    let binfile = to_binfile(test);
    // Read the test file
    let asm = fs::read_to_string(asmfile).unwrap();
    let bin = fs::read_to_string(binfile).unwrap();
    // Parse assembly into instructions
    let insns = match Assembler::new(&asm).parse() {
        Ok(insns) => insns,
        Err(e) => {
            panic!("Error parsing assembly: {e}");
        }
    };
    // Translate instructions into bytes
    let asm_bytes: Vec<u8> = insns.try_into().unwrap();
    // Parse hex string into bytes
    let bin_bytes = bin.trim().from_hex_string().unwrap();
    // Check they match
    assert_eq!(asm_bytes,bin_bytes);
}

fn to_asmfile(test: &str) -> PathBuf {
    let mut path = PathBuf::from(TESTS_DIR);
    path.push(test.to_string());
    path.set_extension("asm");
    path
}

fn to_binfile(test: &str) -> PathBuf {
    let mut path = PathBuf::from(TESTS_DIR);
    path.push(test.to_string());
    path.set_extension("bin");
    path
}
