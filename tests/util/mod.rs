use std::io::Write;
use evmil::ll::{Bytecode,Instruction};

const enable : bool = true;
static mut counter : usize = 0x0;
static TESTS_DIR: &str = "tests/files";

pub fn log_full_test(prefix: &str, src: &str, hex: &str, code: &Bytecode) {
    if enable {
        let id = get_id();
        write_bin_file(prefix,&id,hex);
        write_asm_file(prefix,&id,code.instructions());
        write_src_file(prefix,&id,src);
    }
}

pub fn log_half_test(prefix: &str, hex: &str, instructions: &[Instruction]) {
    if enable {
        let id = get_id();
        write_bin_file(prefix,&id,hex);
        write_asm_file(prefix,&id,instructions);
    }
}

fn get_id() -> String {
    unsafe {
        counter = counter + 1;
        format!("{:x}",(counter-1))
    }
}

fn write_bin_file(prefix: &str, id: &str, hex: &str) {
    let bin_name = format!("{prefix}_{:0>6}.bin",id);
    let bin_filename = std::path::Path::new(TESTS_DIR).join(bin_name);
    let mut bin_file = std::fs::File::create(bin_filename).unwrap();
    writeln!(bin_file,"{}",hex);
}

fn write_asm_file(prefix: &str, id: &str, instructions: &[Instruction]) {
    let asm_name = format!("{prefix}_{:0>6}.asm",id);
    let asm_filename = std::path::Path::new(TESTS_DIR).join(asm_name);
    let mut asm_file = std::fs::File::create(asm_filename).unwrap();
    for insn in instructions {
        writeln!(asm_file,"{}",insn);
    }
}

fn write_src_file(prefix: &str, id: &str, src: &str) {
    let src_name = format!("{prefix}_{:0>6}.eil",id);
    let src_filename = std::path::Path::new(TESTS_DIR).join(src_name);
    let mut src_file = std::fs::File::create(src_filename).unwrap();
    writeln!(src_file,"{}",src);
}
