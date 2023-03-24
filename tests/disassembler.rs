use std::fmt;
use std::fs;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use evmil::evm::{AbstractWord,AbstractStack,Disassembly};
use evmil::ll::{Assembler};
use evmil::util::{FromHexString};

pub static TESTS_DIR: &str = "tests/files";

// Include the programmatically generated test file.
include!(concat!(env!("OUT_DIR"), "/bin_tests.rs"));

fn check(test: &str) {
    // Construct input files
    let asmfile = to_asmfile(test);
    let binfile = to_binfile(test);
    // Read the test file
    let asm = fs::read_to_string(asmfile).unwrap();
    let bin = fs::read_to_string(binfile).unwrap();
    // Parse assembly into instructions
    let asm_code = match Assembler::new(&asm).parse() {
        Ok(insns) => insns,
        Err(e) => panic!("{test}.asm: {e}")
    };
    // Parse hex string into bytes
    let bin_bytes = bin.trim().from_hex_string().unwrap();
    // Construct disassembly
    let disasm: Disassembly<AbstractStack<AbstractWord>> = Disassembly::new(&bin_bytes).build();
    // Disassemble bytes into instructions
    let bin_insns = disasm.to_vec();
    // Check they match

    // ========================================================
    // TODO: reenable this!
    // ========================================================
    // assert_eq!(bin_insns,asm_code.instructions());
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
