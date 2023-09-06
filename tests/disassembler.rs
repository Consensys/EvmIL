use std::fs;
use std::path::{PathBuf};
use evmil::util::{FromHexString};
use evmil::bytecode::LegacyContract;
use evmil::asm::{Assembly};

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
    // Construct assembly from input file
    let assembly = match Assembly::from_str(&asm) {
        Ok(insns) => insns,
        Err(e) => panic!("{test}.asm: {e}")
    };
    // Parse hex string into bytes
    let bytes = bin.trim().from_hex_string().unwrap();
    // // Construct disassembly
    let disassembly = LegacyContract::from_bytes(&bytes).to_structured();
    // // Disassemble bytes into instructions
    // let bin_insns = disasm.to_vec();
    // // Check they match

    // ========================================================
    // TODO: reenable this!
    // ========================================================
    assert_eq!(assembly,disassembly);
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
