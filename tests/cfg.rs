use evmil::bytecode::{Assembly,Instruction,StructuredSection};
use evmil::analysis::{BlockGraph};

#[test]
fn test_cfg_01() {
    let asm = r#"
.code
   push 0x80
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_02() {
    let asm = r#"
.code
   push 0x80
   push 0x60
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_03() {
    let asm = r#"
.code
   push 0x80
   stop
"#;
    check_asm(&asm,&[]);
}

#[test]
fn test_cfg_04() {
    let asm = r#"
.code
   push 0x80 ;; blk 1
   jumpdest  ;; blk 2
   stop
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_05() {
    let asm = r#"
.code
   push 0x80
   push lab
   jump
lab:
   jumpdest
   stop
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_06() {
    let asm = r#"
.code
   push 0x80
   push lab
   jump
   stop
lab:
   jumpdest
   stop
"#;
    check_asm(&asm,&[(0,2)]);
}

#[test]
fn test_cfg_07() {
    let asm = r#"
.code
   calldatasize ;; blk 0
   push lab
   jumpi
   revert    
lab:
   jumpdest  ;; blk 1
   stop
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_08() {
    let asm = r#"
.code
   calldatasize
   push lab1  ;; blk 0   
   jumpi
   push lab2
   jump
lab1:
   jumpdest   ;; blk 1
   stop
lab2:
   jumpdest   ;; blk 2
   return
"#;
    check_asm(&asm,&[(0,1),(0,2)]);
}

#[test]
fn test_cfg_10() {
    let asm = r#"
.code
   calldatasize
   push lab1  ;; blk 0   
   jumpi
   push lab2
   jump
lab1:
   jumpdest   ;; blk 1
   push lab3
   jump
lab2:
   jumpdest   ;; blk 2
   calldatasize
lab3:
   jumpdest   ;; blk 3
   stop
"#;
    check_asm(&asm,&[(0,1),(0,2),(1,3),(2,3)]);
}

#[test]
fn test_cfg_11() {
    let asm = r#"
.code
   push 0x80
   db 0xee
"#;
    check_asm(&asm,&[]);
}

#[test]
fn test_cfg_12() {
    let asm = r#"
.code
   push 0x0
   push 0x1
   div
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_13() {
    let asm = r#"
.code
   push 0x0
   push 0x1
   mod
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_14() {
    let asm = r#"
.code
   push 0x0
   push 0x1
   push 0x2
   addmod
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_cfg_15() {
    let asm = r#"
.code
   push 0x0
   push 0x1
   push 0x2
   mulmod
"#;
    check_asm(&asm,&[(0,1)]);
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

fn check_insns(insns: &[Instruction], edges: &[(usize,usize)]) {
    // Construct control-flow graph
    let cfg = BlockGraph::from(insns);
    // Extract edgesets from graph
    let mut in_edges : Vec<_> = cfg.in_iter().collect();
    let mut out_edges : Vec<_>  = cfg.out_iter().collect();    
    // Check edge sets match
    in_edges.sort();
    out_edges.sort();
    assert_eq!(in_edges,out_edges);
    assert_eq!(in_edges,edges);    
}
