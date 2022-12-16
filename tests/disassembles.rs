use std::convert::{AsRef};
use evmil::{Instruction,FromHexString,CfaState};
use evmil::{Disassemble,Disassembly};
use evmil::Instruction::*;

// ============================================================================
// Instruction Tests
// ============================================================================
//
// The aim of these tests is to execute every single instruction at
// least once.

#[test]
pub fn test_disassemble_insn_01() {
    check("0x00", &[STOP]);
}

// 30s

#[test]
pub fn test_disassemble_insn_34() {
    check("0x60053450565b", &[PUSH(vec![0x05]),CALLVALUE,POP,JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_35() {
    check("0x600760003550565b", &[PUSH(vec![0x07]),PUSH(vec![0x00]),CALLDATALOAD,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_36() {
    check("0x60053650565b", &[PUSH(vec![0x05]),CALLDATASIZE,POP,JUMP,JUMPDEST(5)]);
}

// 50s

#[test]
pub fn test_disassemble_insn_50() {
    check("0x6006600150565b", &[PUSH(vec![0x06]),PUSH(vec![0x01]),POP,JUMP,JUMPDEST(6)]);
}

#[test]
pub fn test_disassemble_insn_51() {
    check("0x600760015150565b", &[PUSH(vec![0x07]),PUSH(vec![0x01]),MLOAD,POP,JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_54() {
    check("0x600760015450565b", &[PUSH(vec![0x07]),PUSH(vec![0x01]),SLOAD,POP,JUMP,JUMPDEST(7)]);
}

// 60s

#[test]
pub fn test_disassemble_insn_60() {
    check("0x6003565b", &[PUSH(vec![0x03]),JUMP,JUMPDEST(3)]);
}

#[test]
pub fn test_disassemble_insn_61() {
    check("0x610004565b", &[PUSH(vec![0x0,0x04]),JUMP,JUMPDEST(4)]);
}

#[test]
pub fn test_disassemble_insn_62() {
    check("0x62000005565b", &[PUSH(vec![0x0,0x0,0x05]),JUMP,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_insn_63() {
    check("0x6300000006565b", &[PUSH(vec![0x0,0x0,0x0,0x06]),JUMP,JUMPDEST(6)]);
}

#[test]
pub fn test_disassemble_insn_64() {
    check("0x640000000007565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x07]),JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_insn_65() {
    check("0x65000000000008565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x08]),JUMP,JUMPDEST(8)]);
}

#[test]
pub fn test_disassemble_insn_66() {
    check("0x6600000000000009565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x09]),JUMP,JUMPDEST(9)]);
}

#[test]
pub fn test_disassemble_insn_67() {
    check("0x67000000000000000a565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0a]),JUMP,JUMPDEST(10)]);
}

#[test]
pub fn test_disassemble_insn_68() {
    check("0x6800000000000000000b565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0b]),JUMP,JUMPDEST(11)]);
}

#[test]
pub fn test_disassemble_insn_69() {
	check("0x690000000000000000000c565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0c]),JUMP,JUMPDEST(12)]);
}

#[test]
pub fn test_disassemble_insn_6a() {
    check("0x6a000000000000000000000d565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0d]),JUMP,JUMPDEST(13)]);
}

#[test]
pub fn test_disassemble_insn_6b() {
    check("0x6b00000000000000000000000e565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0e]),JUMP,JUMPDEST(14)]);
}

#[test]
pub fn test_disassemble_insn_6c() {
    check("0x6c0000000000000000000000000f565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0f]),JUMP,JUMPDEST(15)]);
}

#[test]
pub fn test_disassemble_insn_6d() {
    check("0x6d0000000000000000000000000010565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x10]),JUMP,JUMPDEST(16)]);
}

#[test]
pub fn test_disassemble_insn_6e() {
    check("0x6e000000000000000000000000000011565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x11]),JUMP,JUMPDEST(17)]);
}

#[test]
pub fn test_disassemble_insn_6f() {
    check("0x6f00000000000000000000000000000012565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x12]),JUMP,JUMPDEST(18)]);
}

#[test]
pub fn test_disassemble_insn_70() {
    check("0x700000000000000000000000000000000013565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x13]),JUMP,JUMPDEST(19)]);
}

#[test]
pub fn test_disassemble_insn_71() {
    check("0x71000000000000000000000000000000000014565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x14]),JUMP,JUMPDEST(20)]);
}

#[test]
pub fn test_disassemble_insn_72() {
    check("0x7200000000000000000000000000000000000015565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x15]),JUMP,JUMPDEST(21)]);
}

#[test]
pub fn test_disassemble_insn_73() {
    check("0x730000000000000000000000000000000000000016565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x16]),JUMP,JUMPDEST(22)]);
}

#[test]
pub fn test_disassemble_insn_74() {
    check("0x74000000000000000000000000000000000000000017565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x17]),JUMP,JUMPDEST(23)]);
}

#[test]
pub fn test_disassemble_insn_75() {
    check("0x7500000000000000000000000000000000000000000018565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x18]),JUMP,JUMPDEST(24)]);
}

#[test]
pub fn test_disassemble_insn_76() {
    check("0x760000000000000000000000000000000000000000000019565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x19]),JUMP,JUMPDEST(25)]);
}

#[test]
pub fn test_disassemble_insn_77() {
    check("0x7700000000000000000000000000000000000000000000001a565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1a]),JUMP,JUMPDEST(26)]);
}

#[test]
pub fn test_disassemble_insn_78() {
    check("0x780000000000000000000000000000000000000000000000001b565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1b]),JUMP,JUMPDEST(27)]);
}

#[test]
pub fn test_disassemble_insn_79() {
    check("0x79000000000000000000000000000000000000000000000000001c565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1c]),JUMP,JUMPDEST(28)]);
}

#[test]
pub fn test_disassemble_insn_7a() {
    check("0x7a00000000000000000000000000000000000000000000000000001d565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1d]),JUMP,JUMPDEST(29)]);
}

#[test]
pub fn test_disassemble_insn_7b() {
    check("0x7b0000000000000000000000000000000000000000000000000000001e565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1e]),JUMP,JUMPDEST(30)]);
}

#[test]
pub fn test_disassemble_insn_7c() {
    check("0x7c000000000000000000000000000000000000000000000000000000001f565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x1f]),JUMP,JUMPDEST(31)]);
}

#[test]
pub fn test_disassemble_insn_7d() {
    check("0x7d000000000000000000000000000000000000000000000000000000000020565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x20]),JUMP,JUMPDEST(32)]);
}

#[test]
pub fn test_disassemble_insn_7e() {
    check("0x7e00000000000000000000000000000000000000000000000000000000000021565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x21]),JUMP,JUMPDEST(33)]);
}

#[test]
pub fn test_disassemble_insn_7f() {
    check("0x7f0000000000000000000000000000000000000000000000000000000000000022565b", &[PUSH(vec![0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x22]),JUMP,JUMPDEST(34)]);
}

// 80s

#[test]
pub fn test_disassemble_insn_80() {
    check("0x600480565b", &[PUSH(vec![0x04]),DUP(1),JUMP,JUMPDEST(0x04)]);
}

#[test]
pub fn test_disassemble_insn_81() {
    check("0x6006600081565b", &[PUSH(vec![0x06]),PUSH(vec![0x00]),DUP(2),JUMP,JUMPDEST(0x06)]);
}

#[test]
pub fn test_disassemble_insn_82() {
    check("0x60086000600082565b", &[PUSH(vec![0x08]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(3),JUMP,JUMPDEST(0x08)]);
}

#[test]
pub fn test_disassemble_insn_83() {
    check("0x600a60006000600083565b", &[PUSH(vec![0x0a]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(4),JUMP,JUMPDEST(0x0a)]);
}

#[test]
pub fn test_disassemble_insn_84() {
    check("0x600c600060006000600084565b", &[PUSH(vec![0x0c]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(5),JUMP,JUMPDEST(0x0c)]);
}

#[test]
pub fn test_disassemble_insn_85() {
    check("0x600e6000600060006000600085565b", &[PUSH(vec![0x0e]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(6),JUMP,JUMPDEST(0x0e)]);
}

#[test]
pub fn test_disassemble_insn_86() {
    check("0x601060006000600060006000600086565b", &[PUSH(vec![0x10]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(7),JUMP,JUMPDEST(0x10)]);
}

#[test]
pub fn test_disassemble_insn_87() {
    check("0x6012600060006000600060006000600087565b", &[PUSH(vec![0x12]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(8),JUMP,JUMPDEST(0x12)]);
}

#[test]
pub fn test_disassemble_insn_88() {
    check("0x60146000600060006000600060006000600088565b", &[PUSH(vec![0x14]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(9),JUMP,JUMPDEST(0x14)]);
}

#[test]
pub fn test_disassemble_insn_89() {
    check("0x601660006000600060006000600060006000600089565b", &[PUSH(vec![0x16]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(10),JUMP,JUMPDEST(0x16)]);
}

#[test]
pub fn test_disassemble_insn_8a() {
    check("0x601860006000600060006000600060006000600060008a565b", &[PUSH(vec![0x18]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(11),JUMP,JUMPDEST(0x18)]);
}

#[test]
pub fn test_disassemble_insn_8b() {
    check("0x601a600060006000600060006000600060006000600060008b565b", &[PUSH(vec![0x1a]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(12),JUMP,JUMPDEST(0x1a)]);
}

#[test]
pub fn test_disassemble_insn_8c() {
    check("0x601c6000600060006000600060006000600060006000600060008c565b", &[PUSH(vec![0x1c]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(13),JUMP,JUMPDEST(0x1c)]);
}

#[test]
pub fn test_disassemble_insn_8d() {
    check("0x601e60006000600060006000600060006000600060006000600060008d565b", &[PUSH(vec![0x1e]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(14),JUMP,JUMPDEST(0x1e)]);
}

#[test]
pub fn test_disassemble_insn_8e() {
    check("0x6020600060006000600060006000600060006000600060006000600060008e565b", &[PUSH(vec![0x20]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(15),JUMP,JUMPDEST(0x20)]);
}

#[test]
pub fn test_disassemble_insn_8f() {
    check("0x60226000600060006000600060006000600060006000600060006000600060008f565b", &[PUSH(vec![0x22]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),DUP(16),JUMP,JUMPDEST(0x22)]);
}

// 90s

#[test]
pub fn test_disassemble_insn_90() {
    check("0x600490565b", &[PUSH(vec![0x04]),SWAP(1),JUMP,JUMPDEST(0x04)]);
}

#[test]
pub fn test_disassemble_insn_91() {
    check("0x6006600091565b", &[PUSH(vec![0x06]),PUSH(vec![0x00]),SWAP(2),JUMP,JUMPDEST(0x06)]);
}

#[test]
pub fn test_disassemble_insn_92() {
    check("0x60086000600092565b", &[PUSH(vec![0x08]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(3),JUMP,JUMPDEST(0x08)]);
}

#[test]
pub fn test_disassemble_insn_93() {
    check("0x600a60006000600093565b", &[PUSH(vec![0x0a]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(4),JUMP,JUMPDEST(0x0a)]);
}

#[test]
pub fn test_disassemble_insn_94() {
    check("0x600c600060006000600094565b", &[PUSH(vec![0x0c]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(5),JUMP,JUMPDEST(0x0c)]);
}

#[test]
pub fn test_disassemble_insn_95() {
    check("0x600e6000600060006000600095565b", &[PUSH(vec![0x0e]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(6),JUMP,JUMPDEST(0x0e)]);
}

#[test]
pub fn test_disassemble_insn_96() {
    check("0x601060006000600060006000600096565b", &[PUSH(vec![0x10]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(7),JUMP,JUMPDEST(0x10)]);
}

#[test]
pub fn test_disassemble_insn_97() {
    check("0x6012600060006000600060006000600097565b", &[PUSH(vec![0x12]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(8),JUMP,JUMPDEST(0x12)]);
}

#[test]
pub fn test_disassemble_insn_98() {
    check("0x60146000600060006000600060006000600098565b", &[PUSH(vec![0x14]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(9),JUMP,JUMPDEST(0x14)]);
}

#[test]
pub fn test_disassemble_insn_99() {
    check("0x601660006000600060006000600060006000600099565b", &[PUSH(vec![0x16]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(10),JUMP,JUMPDEST(0x16)]);
}

#[test]
pub fn test_disassemble_insn_9a() {
    check("0x601860006000600060006000600060006000600060009a565b", &[PUSH(vec![0x18]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(11),JUMP,JUMPDEST(0x18)]);
}

#[test]
pub fn test_disassemble_insn_9b() {
    check("0x601a600060006000600060006000600060006000600060009b565b", &[PUSH(vec![0x1a]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(12),JUMP,JUMPDEST(0x1a)]);
}

#[test]
pub fn test_disassemble_insn_9c() {
    check("0x601c6000600060006000600060006000600060006000600060009c565b", &[PUSH(vec![0x1c]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(13),JUMP,JUMPDEST(0x1c)]);
}

#[test]
pub fn test_disassemble_insn_9d() {
    check("0x601e60006000600060006000600060006000600060006000600060009d565b", &[PUSH(vec![0x1e]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(14),JUMP,JUMPDEST(0x1e)]);
}

#[test]
pub fn test_disassemble_insn_9e() {
    check("0x6020600060006000600060006000600060006000600060006000600060009e565b", &[PUSH(vec![0x20]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(15),JUMP,JUMPDEST(0x20)]);
}

#[test]
pub fn test_disassemble_insn_9f() {
    check("0x60226000600060006000600060006000600060006000600060006000600060009f565b", &[PUSH(vec![0x22]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),PUSH(vec![0x00]),SWAP(16),JUMP,JUMPDEST(0x22)]);
}

// ============================================================================
// Single block Tests
// ============================================================================

// more complex things here?

// ============================================================================
// Double block Tests
// ============================================================================

#[test]
pub fn test_disassemble_double_01() {
    // A minimal two-block program
    check("0x6003565b", &[PUSH(vec![3]),JUMP,JUMPDEST(3)]);
}

#[test]
pub fn test_disassemble_double_03() {
    // A minimal conditional two-block program
    check("0x60016005575b", &[PUSH(vec![1]),PUSH(vec![5]),JUMPI,JUMPDEST(5)]);
}

#[test]
pub fn test_disassemble_double_04() {
    // A simple conditional two-block program
    check("0x6001600657005b", &[PUSH(vec![1]),PUSH(vec![6]),JUMPI,STOP,JUMPDEST(6)]);
}

// ============================================================================
// Triple block Tests
// ============================================================================

#[test]
pub fn test_disassemble_triple_01() {
    // Minimal three-block program
    check("0x6003565b6007565b", &[PUSH(vec![3]),JUMP,JUMPDEST(3),PUSH(vec![7]),JUMP,JUMPDEST(7)]);
}

#[test]
pub fn test_disassemble_triple_02() {
    // Three-block program with back jump
    check("0x6005565b005b600356", &[PUSH(vec![5]),JUMP,JUMPDEST(3),STOP,JUMPDEST(5),PUSH(vec![3]),JUMP]);
}

// ============================================================================
// Split block Tests
// ============================================================================

#[test]
pub fn test_disassemble_split_01() {
    // A minimal split multiblock program
    check("0x600456005b", &[PUSH(vec![4]),JUMP,DATA(vec![0]),JUMPDEST(4)]);
}

#[test]
pub fn test_disassemble_split_02() {
    // A minimal split multiblock program.  This program contains an
    // invalid JUMPDEST.
    check("0x600456605b", &[PUSH(vec![4]),JUMP,PUSH(vec![0x5b])]);
}

#[test]
pub fn test_disassemble_split_03() {
    // A minimal split multiblock program
    check("0x6003565b0061", &[PUSH(vec![3]),JUMP,JUMPDEST(3),STOP,DATA(vec![0x61])]);
}

#[test]
pub fn test_disassemble_split_04() {
    // A minimal split multiblock program
    check("0x60055601025b", &[PUSH(vec![5]),JUMP,DATA(vec![1,2]),JUMPDEST(5)]);
}

// ============================================================================
// Helpers
// ============================================================================

/// Check that disassembling a given hex string produces a given
/// sequence of instructions.
fn check(hex: &str, insns: &[Instruction]) {
    // Parse hex string into bytes
    let bytes = hex.from_hex_string().unwrap();
    // Disassemble bytes into instructions
    let mut disasm : Disassembly<CfaState> = Disassembly::new(&bytes).build();
    // Check against expected instruction sequence
    assert_eq!(insns, disasm.to_vec());
}
