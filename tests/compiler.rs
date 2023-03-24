use std::fmt;
use std::fs;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use evmil::ll::Bytecode;
use evmil::il::Parser;
use evmil::util::{FromHexString};

pub static TESTS_DIR: &str = "tests/files";

// Include the programmatically generated test file.
include!(concat!(env!("OUT_DIR"), "/eil_tests.rs"));

/// Run a specific test by loading the file out of the reference tests
/// repository and attempting to parse it.  All reference tests should
/// parse correctly.
fn check(test: &str) {
    // Construct input files
    let eilfile = to_eilfile(test);
    let binfile = to_binfile(test);
    // Read the test file
    let eil = fs::read_to_string(eilfile).unwrap();
    let bin = fs::read_to_string(binfile).unwrap();
    // Parse source file
    let terms = match Parser::new(&eil).parse() {
        Ok(terms) => terms,
        Err(e) => panic!("{test}.eil: {e}")
    };
    // Translate statements into bytecode instructions
    let code = Bytecode::try_from(terms.as_slice()).unwrap();
    // Translate instructions into bytes
    let eil_bytes: Vec<u8> = code.try_into().unwrap();
    // Parse hex string into bytes
    let bin_bytes = bin.trim().from_hex_string().unwrap();
    // Check they match
    assert_eq!(eil_bytes,bin_bytes);
}

fn to_eilfile(test: &str) -> PathBuf {
    let mut path = PathBuf::from(TESTS_DIR);
    path.push(test.to_string());
    path.set_extension("eil");
    path
}

fn to_binfile(test: &str) -> PathBuf {
    let mut path = PathBuf::from(TESTS_DIR);
    path.push(test.to_string());
    path.set_extension("bin");
    path
}