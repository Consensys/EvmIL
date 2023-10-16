use evmil::bytecode::{Assembly,BlockVec,Instruction,StructuredSection};
use evmil::util::{SubsliceOffset};

use Instruction::*;

#[test]
fn test_blockvec_01() {
    let asm = r#"
.code
   push 0x80
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_blockvec_02() {
    let asm = r#"
.code
   push 0x80
   stop
"#;
    check_asm(&asm,&[(0,2)]);
}

#[test]
fn test_blockvec_03() {
    let asm = r#"
.code
   push 0x80
   jumpdest
   stop
"#;
    check_asm(&asm,&[(0,1),(1,3)]);
}

#[test]
fn test_blockvec_04() {
    let asm = r#"
.code
   push 0x80
   push lab
   jump
lab:
   jumpdest
   stop
"#;
    check_asm(&asm,&[(0,3),(3,5)]);
}

#[test]
fn test_blockvec_05() {
    let asm = r#"
.code
   push 0x80
   push lab
   jumpi
   revert
lab:
   jumpdest
   stop
"#;
    check_asm(&asm,&[(0,4),(4,6)]);
}

#[test]
fn test_blockvec_06() {
    let asm = r#"
.code   
   push lab1
   jumpi
   push lab2
lab1:
   jumpdest
   stop
lab2:
   jumpdest
   return
"#;
    check_asm(&asm,&[(0,3),(3,5),(5,7)]);
}

fn check_asm(asm: &str, blocks: &[(usize,usize)]) {
    // Convert assembly into instructions
    let assembly = Assembly::from_str(&asm).unwrap();
    //
    for sect in &assembly {
        match sect {
            StructuredSection::Code(insns) => {
                check_insns(&insns,blocks);
            }
            StructuredSection::Data(_) => {
            }            
        }
    }
}

fn check_insns(insns: &[Instruction], blocks: &[(usize,usize)]) {
    // Split into blocks
    let bvec = BlockVec::new(&insns);
    // Check expected number of blocks
    assert_eq!(bvec.len(),blocks.len());
    // Check individual blocks
    for i in 0..bvec.len() {
        let ith = bvec.get(i);
        // Calculate offset within enclosing block
        let offset = insns.subslice_offset(ith);
        // Check it!
        assert_eq!(blocks[i].0,offset);
        assert_eq!(blocks[i].1,offset+ith.len());        
    }
}
