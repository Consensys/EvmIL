use std::fs;
use std::io::Write;
use std::path::Path;

pub static TESTS_DIR: &str = "tests/files";
pub static TESTS_EXT: &str = "asm";

fn gentests(testdir: &str, ext: &str, target: &Path) {
    let mut f = fs::File::create(target).unwrap();
    // Open reference test directory
    let dir = fs::read_dir(testdir).unwrap();

    for e in dir {
        let p = e.as_ref().unwrap().path();
        let n = p.file_stem().unwrap().to_str().unwrap();
        //
        if p.extension().unwrap() == ext {
            writeln!(f).unwrap();
            writeln!(f,"#[test]").unwrap();
            writeln!(f,"fn test_{n}() {{ check(\"{n}\"); }}").unwrap();
        }
    }
}

/// The purpose of this script is to generate a set of tests for each
/// of the language reference tests.
fn main() {
    // Create destination file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    // tests
    let file = std::path::Path::new(&out_dir).join("tests.rs");
    gentests(TESTS_DIR,TESTS_EXT,&file);
}
