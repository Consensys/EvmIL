use std::fs;
use std::path::{PathBuf};
use evmil::asm::{Assembly};
use evmil::legacy;
use evmil::util::{FromHexString};

pub static TESTS_DIR: &str = "tests/files";

// Include the programmatically generated test file.
include!(concat!(env!("OUT_DIR"), "/asm_tests.rs"));

fn check(test: &str) {
    // Construct input files
    let asmfile = to_asmfile(test);
    let binfile = to_binfile(test);
    // Read the test file
    let asm = fs::read_to_string(asmfile).unwrap();
    let bin = fs::read_to_string(binfile).unwrap();
    // Parse assembly into instructions
    let insns = match Assembly::from_str(&asm) {
        Ok(insns) => insns,
        Err(e) => panic!("{test}.asm: {e}")
    };
    // Translate instructions into bytes
    let asm_bytes: Vec<u8> = legacy::to_bytes(&insns.assemble().unwrap());
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
