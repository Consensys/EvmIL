use evmil::bytecode::{Assembly,BlockGraph,BlockVec,Instruction,StructuredSection};

use Instruction::*;

#[test]
fn test_blockgraph_01() {
    let asm = r#"
.code
   push 0x80
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_blockgraph_02() {
    let asm = r#"
.code
   push 0x80
   push 0x60
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_blockgraph_03() {
    let asm = r#"
.code
   push 0x80
   stop
"#;
    check_asm(&asm,&[]);
}

#[test]
fn test_blockgraph_04() {
    let asm = r#"
.code
   push 0x80 ;; blk 1
   jumpdest  ;; blk 2
   stop
"#;
    check_asm(&asm,&[(0,1)]);
}

#[test]
fn test_blockgraph_05() {
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
fn test_blockgraph_06() {
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
fn test_blockgraph_07() {
    let asm = r#"
.code
   push 0x80 ;; blk 0
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
fn test_blockgraph_08() {
    let asm = r#"
.code   
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
    let bvec = BlockVec::new(insns);
    println!("BLOCKS = {}",bvec);
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
