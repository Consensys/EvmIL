use std::io::Write;
use evmil::evm::{AbstractStack, AbstractWord, Disassembly};
use evmil::ll::Instruction;
use evmil::ll::Instruction::*;
use evmil::util::{w256, FromHexString, Interval};

mod util;

// ============================================================================
// Instruction Tests
// ============================================================================
//
// The aim of these tests is to execute every single instruction at
// least once.

#[test]
pub fn test_disassemble_insn_00() {
    check("opcode","0x00", &[STOP]);
}

#[test]
pub fn test_disassemble_insn_01() {
    let bytecode = format!("0x6008600180{}50565b", "01");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            ADD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_02() {
    let bytecode = format!("0x6008600180{}50565b", "02");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            MUL,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_03() {
    let bytecode = format!("0x6008600180{}50565b", "03");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SUB,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_04() {
    let bytecode = format!("0x6008600180{}50565b", "04");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            DIV,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_05() {
    let bytecode = format!("0x6008600180{}50565b", "05");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SDIV,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_06() {
    let bytecode = format!("0x6008600180{}50565b", "06");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            MOD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_07() {
    let bytecode = format!("0x6008600180{}50565b", "07");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SMOD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_08() {
    let bytecode = format!("0x600960018080{}50565b", "08");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x09]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            ADDMOD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_09() {
    let bytecode = format!("0x600960018080{}50565b", "09");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x09]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            MULMOD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_0a() {
    let bytecode = format!("0x6008600180{}50565b", "0a");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            EXP,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_0b() {
    let bytecode = format!("0x6008600180{}50565b", "0b");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SIGNEXTEND,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

// 10s

#[test]
pub fn test_disassemble_insn_10() {
    let bytecode = format!("0x6008600180{}50565b", "10");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            LT,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_11() {
    let bytecode = format!("0x6008600180{}50565b", "11");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            GT,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_12() {
    let bytecode = format!("0x6008600180{}50565b", "12");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SLT,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_13() {
    let bytecode = format!("0x6008600180{}50565b", "13");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SGT,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_14() {
    let bytecode = format!("0x6008600180{}50565b", "14");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            EQ,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_15() {
    let bytecode = format!("0x60076001{}50565b", "15");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            ISZERO,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_16() {
    let bytecode = format!("0x6008600180{}50565b", "16");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            AND,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_17() {
    let bytecode = format!("0x6008600180{}50565b", "17");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            OR,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_18() {
    let bytecode = format!("0x6008600180{}50565b", "18");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            XOR,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_19() {
    let bytecode = format!("0x60076001{}50565b", "19");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            NOT,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_1a() {
    let bytecode = format!("0x6008600180{}50565b", "1a");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            BYTE,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_1b() {
    let bytecode = format!("0x6008600180{}50565b", "1b");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SHL,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_1c() {
    let bytecode = format!("0x6008600180{}50565b", "1c");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SHR,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_1d() {
    let bytecode = format!("0x6008600180{}50565b", "1d");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            SAR,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

// 20s

#[test]
pub fn test_disassemble_insn_20() {
    let bytecode = format!("0x6008600180{}50565b", "20");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            KECCAK256,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

// 30s

#[test]
pub fn test_disassemble_insn_30() {
    let bytecode = format!("0x6005{}50565b", "30");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), ADDRESS, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_31() {
    let bytecode = format!("0x60076001{}50565b", "31");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            BALANCE,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_32() {
    let bytecode = format!("0x6005{}50565b", "32");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), ORIGIN, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_33() {
    let bytecode = format!("0x6005{}50565b", "33");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), CALLER, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_34() {
    check("opcode",
        "0x60053450565b",
        &[PUSH(vec![0x05]), CALLVALUE, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_35() {
    check("opcode",
        "0x600760003550565b",
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x00]),
            CALLDATALOAD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_36() {
    check("opcode",
        "0x60053650565b",
        &[PUSH(vec![0x05]), CALLDATASIZE, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_37() {
    let bytecode = format!("0x600860018080{}565b", "37");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            CALLDATACOPY,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_38() {
    let bytecode = format!("0x6005{}50565b", "38");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), CODESIZE, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_39() {
    let bytecode = format!("0x600860018080{}565b", "39");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            CODECOPY,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_3a() {
    let bytecode = format!("0x6005{}50565b", "3a");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), GASPRICE, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_3b() {
    let bytecode = format!("0x60076001{}50565b", "3b");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            EXTCODESIZE,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_3c() {
    let bytecode = format!("0x60096001808080{}565b", "3c");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x09]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            EXTCODECOPY,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_3d() {
    let bytecode = format!("0x6005{}50565b", "3d");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), RETURNDATASIZE, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_3e() {
    let bytecode = format!("0x600860018080{}565b", "3e");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            RETURNDATACOPY,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_3f() {
    let bytecode = format!("0x60076001{}50565b", "3f");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            EXTCODEHASH,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

// 40s

#[test]
pub fn test_disassemble_insn_40() {
    let bytecode = format!("0x60076001{}50565b", "40");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            BLOCKHASH,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_41() {
    let bytecode = format!("0x6005{}50565b", "41");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), COINBASE, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_42() {
    let bytecode = format!("0x6005{}50565b", "42");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), TIMESTAMP, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_43() {
    let bytecode = format!("0x6005{}50565b", "43");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), NUMBER, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_44() {
    let bytecode = format!("0x6005{}50565b", "44");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), DIFFICULTY, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_45() {
    let bytecode = format!("0x6005{}50565b", "45");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), GASLIMIT, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_46() {
    let bytecode = format!("0x6005{}50565b", "46");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), CHAINID, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_47() {
    let bytecode = format!("0x6005{}50565b", "47");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), SELFBALANCE, POP, JUMP, JUMPDEST],
    );
}

// 50s

#[test]
pub fn test_disassemble_insn_50() {
    check("opcode",
        "0x6006600150565b",
        &[PUSH(vec![0x06]), PUSH(vec![0x01]), POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_51() {
    check("opcode",
        "0x600760015150565b",
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            MLOAD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_52() {
    let bytecode = format!("0x6007600180{}565b", "52");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            DUP(1),
            MSTORE,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_53() {
    let bytecode = format!("0x6007600180{}565b", "53");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            DUP(1),
            MSTORE8,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_54() {
    let bytecode = format!("0x60076001{}50565b", "54");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            SLOAD,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_55() {
    let bytecode = format!("0x6007600180{}565b", "55");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            DUP(1),
            SSTORE,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_56() {
    // A minimal two-block program
    check("opcode",
        "0x60076005565b565b",
        &[
            PUSH(vec![7]),
            PUSH(vec![5]),
            JUMP,
            JUMPDEST,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_57() {
    // A minimal two-block program
    check("opcode",
        "0x600a6000516008575b565b",
        &[
            PUSH(vec![0xa]),
            PUSH(vec![0]),
            MLOAD,
            PUSH(vec![0x8]),
            JUMPI,
            JUMPDEST,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_58() {
    let bytecode = format!("0x6005{}50565b", "58");
    check("opcode",&bytecode, &[PUSH(vec![0x05]), PC, POP, JUMP, JUMPDEST]);
}

#[test]
pub fn test_disassemble_insn_59() {
    let bytecode = format!("0x6005{}50565b", "59");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x05]), MSIZE, POP, JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_5a() {
    let bytecode = format!("0x6005{}50565b", "5a");
    check("opcode",&bytecode, &[PUSH(vec![0x05]), GAS, POP, JUMP, JUMPDEST]);
}

#[test]
pub fn test_disassemble_insn_5b() {
    let bytecode = format!("0x6004{}565b", "5b");
    check("opcode",
        &bytecode,
        &[PUSH(vec![0x04]), JUMPDEST, JUMP, JUMPDEST],
    );
}

// 60s

#[test]
pub fn test_disassemble_insn_60() {
    check("opcode","0x6003565b", &[PUSH(vec![0x03]), JUMP, JUMPDEST]);
}

#[test]
pub fn test_disassemble_insn_61() {
    check("opcode","0x610004565b", &[PUSH(vec![0x0, 0x04]), JUMP, JUMPDEST]);
}

#[test]
pub fn test_disassemble_insn_62() {
    check("opcode",
        "0x62000005565b",
        &[PUSH(vec![0x0, 0x0, 0x05]), JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_63() {
    check("opcode",
        "0x6300000006565b",
        &[PUSH(vec![0x0, 0x0, 0x0, 0x06]), JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_64() {
    check("opcode",
        "0x640000000007565b",
        &[PUSH(vec![0x0, 0x0, 0x0, 0x0, 0x07]), JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_65() {
    check("opcode",
        "0x65000000000008565b",
        &[PUSH(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x08]), JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_66() {
    check("opcode",
        "0x6600000000000009565b",
        &[
            PUSH(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x09]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_67() {
    check("opcode",
        "0x67000000000000000a565b",
        &[
            PUSH(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0a]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_68() {
    check("opcode",
        "0x6800000000000000000b565b",
        &[
            PUSH(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0b]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_69() {
    check("opcode",
        "0x690000000000000000000c565b",
        &[
            PUSH(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0c]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_6a() {
    check("opcode",
        "0x6a000000000000000000000d565b",
        &[
            PUSH(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0d]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_6b() {
    check("opcode",
        "0x6b00000000000000000000000e565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0e,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_6c() {
    check("opcode",
        "0x6c0000000000000000000000000f565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0f,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_6d() {
    check("opcode",
        "0x6d0000000000000000000000000010565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_6e() {
    check("opcode",
        "0x6e000000000000000000000000000011565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x11,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_6f() {
    check("opcode",
        "0x6f00000000000000000000000000000012565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x12,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_70() {
    check("opcode",
        "0x700000000000000000000000000000000013565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x13,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_71() {
    check("opcode",
        "0x71000000000000000000000000000000000014565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x14,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_72() {
    check("opcode",
        "0x7200000000000000000000000000000000000015565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x15,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_73() {
    check("opcode",
        "0x730000000000000000000000000000000000000016565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x16,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_74() {
    check("opcode",
        "0x74000000000000000000000000000000000000000017565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x17,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_75() {
    check("opcode",
        "0x7500000000000000000000000000000000000000000018565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x18,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_76() {
    check("opcode",
        "0x760000000000000000000000000000000000000000000019565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x19,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_77() {
    check("opcode",
        "0x7700000000000000000000000000000000000000000000001a565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_78() {
    check("opcode",
        "0x780000000000000000000000000000000000000000000000001b565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1b,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_79() {
    check("opcode",
        "0x79000000000000000000000000000000000000000000000000001c565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1c,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_7a() {
    check("opcode",
        "0x7a00000000000000000000000000000000000000000000000000001d565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1d,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_7b() {
    check("opcode",
        "0x7b0000000000000000000000000000000000000000000000000000001e565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1e,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_7c() {
    check("opcode",
        "0x7c000000000000000000000000000000000000000000000000000000001f565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1f,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_7d() {
    check("opcode",
        "0x7d000000000000000000000000000000000000000000000000000000000020565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_7e() {
    check("opcode",
        "0x7e00000000000000000000000000000000000000000000000000000000000021565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x21,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_7f() {
    check("opcode",
        "0x7f0000000000000000000000000000000000000000000000000000000000000022565b",
        &[
            PUSH(vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x22,
            ]),
            JUMP,
            JUMPDEST,
        ],
    );
}

// 80s

#[test]
pub fn test_disassemble_insn_80() {
    check("opcode",
        "0x600480565b",
        &[PUSH(vec![0x04]), DUP(1), JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_81() {
    check("opcode",
        "0x6006600081565b",
        &[
            PUSH(vec![0x06]),
            PUSH(vec![0x00]),
            DUP(2),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_82() {
    check("opcode",
        "0x60086000600082565b",
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(3),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_83() {
    check("opcode",
        "0x600a60006000600083565b",
        &[
            PUSH(vec![0x0a]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(4),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_84() {
    check("opcode",
        "0x600c600060006000600084565b",
        &[
            PUSH(vec![0x0c]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(5),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_85() {
    check("opcode",
        "0x600e6000600060006000600085565b",
        &[
            PUSH(vec![0x0e]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(6),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_86() {
    check("opcode",
        "0x601060006000600060006000600086565b",
        &[
            PUSH(vec![0x10]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(7),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_87() {
    check("opcode",
        "0x6012600060006000600060006000600087565b",
        &[
            PUSH(vec![0x12]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(8),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_88() {
    check("opcode",
        "0x60146000600060006000600060006000600088565b",
        &[
            PUSH(vec![0x14]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(9),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_89() {
    check("opcode",
        "0x601660006000600060006000600060006000600089565b",
        &[
            PUSH(vec![0x16]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(10),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_8a() {
    check("opcode",
        "0x601860006000600060006000600060006000600060008a565b",
        &[
            PUSH(vec![0x18]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(11),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_8b() {
    check("opcode",
        "0x601a600060006000600060006000600060006000600060008b565b",
        &[
            PUSH(vec![0x1a]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(12),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_8c() {
    check("opcode",
        "0x601c6000600060006000600060006000600060006000600060008c565b",
        &[
            PUSH(vec![0x1c]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(13),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_8d() {
    check("opcode",
        "0x601e60006000600060006000600060006000600060006000600060008d565b",
        &[
            PUSH(vec![0x1e]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(14),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_8e() {
    check("opcode",
        "0x6020600060006000600060006000600060006000600060006000600060008e565b",
        &[
            PUSH(vec![0x20]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(15),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_8f() {
    check("opcode",
        "0x60226000600060006000600060006000600060006000600060006000600060008f565b",
        &[
            PUSH(vec![0x22]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            DUP(16),
            JUMP,
            JUMPDEST,
        ],
    );
}

// 90s

#[test]
pub fn test_disassemble_insn_90() {
    check("opcode",
        "0x60053490565b",
        &[PUSH(vec![0x05]), CALLVALUE, SWAP(1), JUMP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_insn_91() {
    check("opcode",
        "0x600760003491565b",
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(2),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_92() {
    check("opcode",
        "0x6009600060003492565b",
        &[
            PUSH(vec![0x09]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(3),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_93() {
    check("opcode",
        "0x600b6000600060003493565b",
        &[
            PUSH(vec![0x0b]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(4),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_94() {
    check("opcode",
        "0x600d60006000600060003494565b",
        &[
            PUSH(vec![0x0d]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(5),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_95() {
    check("opcode",
        "0x600f600060006000600060003495565b",
        &[
            PUSH(vec![0x0f]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(6),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_96() {
    check("opcode",
        "0x60116000600060006000600060003496565b",
        &[
            PUSH(vec![0x11]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(7),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_97() {
    check("opcode",
        "0x601360006000600060006000600060003497565b",
        &[
            PUSH(vec![0x13]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(8),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_98() {
    check("opcode",
        "0x6015600060006000600060006000600060003498565b",
        &[
            PUSH(vec![0x15]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(9),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_99() {
    check("opcode",
        "0x60176000600060006000600060006000600060003499565b",
        &[
            PUSH(vec![0x17]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(10),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_9a() {
    check("opcode",
        "0x60196000600060006000600060006000600060006000349a565b",
        &[
            PUSH(vec![0x19]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(11),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_9b() {
    check("opcode",
        "0x601b60006000600060006000600060006000600060006000349b565b",
        &[
            PUSH(vec![0x1b]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(12),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_9c() {
    check("opcode",
        "0x601d600060006000600060006000600060006000600060006000349c565b",
        &[
            PUSH(vec![0x1d]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(13),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_9d() {
    check("opcode",
        "0x601f6000600060006000600060006000600060006000600060006000349d565b",
        &[
            PUSH(vec![0x1f]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(14),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_9e() {
    check("opcode",
        "0x602160006000600060006000600060006000600060006000600060006000349e565b",
        &[
            PUSH(vec![0x21]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(15),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_9f() {
    check("opcode",
        "0x6023600060006000600060006000600060006000600060006000600060006000349f565b",
        &[
            PUSH(vec![0x23]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            PUSH(vec![0x00]),
            CALLVALUE,
            SWAP(16),
            JUMP,
            JUMPDEST,
        ],
    );
}

// a0s: Logging Operations

#[test]
pub fn test_disassemble_insn_a0() {
    let bytecode = format!("0x6007600180{}565b", "a0");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            DUP(1),
            LOG(0),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_a1() {
    let bytecode = format!("0x600860018080{}565b", "a1");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x08]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            LOG(1),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_a2() {
    let bytecode = format!("0x60096001808080{}565b", "a2");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x09]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            LOG(2),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_a3() {
    let bytecode = format!("0x600a600180808080{}565b", "a3");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x0a]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            LOG(3),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_a4() {
    let bytecode = format!("0x600b60018080808080{}565b", "a4");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x0b]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            LOG(4),
            JUMP,
            JUMPDEST,
        ],
    );
}

// f0s: System Operations

#[test]
pub fn test_disassemble_insn_f0() {
    let bytecode = format!("0x600960018080{}50565b", "f0");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x09]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            CREATE,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_f1() {
    let bytecode = format!("0x600d6001808080808080{}50565b", "f1");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x0d]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            CALL,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_f2() {
    let bytecode = format!("0x600d6001808080808080{}50565b", "f2");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x0d]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            CALLCODE,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_f3() {
    let bytecode = format!("0x6007600180{}56", "f3");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            DUP(1),
            RETURN,
            DATA(vec![0x56]),
        ],
    );
}

#[test]
pub fn test_disassemble_insn_f4() {
    let bytecode = format!("0x600c60018080808080{}50565b", "f4");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x0c]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DELEGATECALL,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_f5() {
    let bytecode = format!("0x600a6001808080{}50565b", "f5");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x0a]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            CREATE2,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_fa() {
    let bytecode = format!("0x600c60018080808080{}50565b", "fa");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x0c]),
            PUSH(vec![0x01]),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            DUP(1),
            STATICCALL,
            POP,
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_insn_fd() {
    let bytecode = format!("0x6007600180{}56", "fd");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            DUP(1),
            REVERT,
            DATA(vec![0x56]),
        ],
    );
}

#[test]
pub fn test_disassemble_insn_fe() {
    let bytecode = format!("0x6007600180{}56", "fe");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x07]),
            PUSH(vec![0x01]),
            DUP(1),
            INVALID,
            DATA(vec![0x56]),
        ],
    );
}

#[test]
pub fn test_disassemble_insn_ff() {
    let bytecode = format!("0x60066001{}565b", "ff");
    check("opcode",
        &bytecode,
        &[
            PUSH(vec![0x06]),
            PUSH(vec![0x01]),
            SELFDESTRUCT,
            DATA(vec![0x56]),
            DATA(vec![0x5b])
        ],
    );
}

// ============================================================================
// Single block Tests
// ============================================================================

// more complex things here?

// ============================================================================
// Double block Tests
// ============================================================================

#[test]
pub fn test_disassemble_jdouble_01() {
    // A minimal two-block program
    check("cfg","0x6003565b", &[PUSH(vec![3]), JUMP, JUMPDEST]);
}

#[test]
pub fn test_disassemble_jdouble_03() {
    // A minimal conditional two-block program
    check("cfg",
        "0x60016005575b",
        &[PUSH(vec![1]), PUSH(vec![5]), JUMPI, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_jdouble_04() {
    // A simple conditional two-block program
    check("cfg",
        "0x6001600657005b",
        &[PUSH(vec![1]), PUSH(vec![6]), JUMPI, STOP, JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_jdouble_05() {
    // A minimal example requiring different stack heights
    check("cfg",
        "0x60ff600054600957505b6000",
        &[
            PUSH(vec![0xff]),
            PUSH(vec![0x00]),
            SLOAD,
            PUSH(vec![0x9]),
            JUMPI,
            POP,
            JUMPDEST,
            PUSH(vec![0x0]),
        ],
    );
}

// ============================================================================
// Triple block Tests
// ============================================================================

#[test]
pub fn test_disassemble_triple_01() {
    // Minimal three-block program
    check("cfg",
        "0x6003565b6007565b",
        &[
            PUSH(vec![3]),
            JUMP,
            JUMPDEST,
            PUSH(vec![7]),
            JUMP,
            JUMPDEST,
        ],
    );
}

#[test]
pub fn test_disassemble_triple_02() {
    // Three-block program with back jump
    check("cfg",
        "0x6005565b005b600356",
        &[
            PUSH(vec![5]),
            JUMP,
            JUMPDEST,
            STOP,
            JUMPDEST,
            PUSH(vec![3]),
            JUMP,
        ],
    );
}

// ============================================================================
// Split block Tests
// ============================================================================

#[test]
pub fn test_disassemble_split_01() {
    // A minimal split multiblock program
    check("cfg",
        "0x600456005b",
        &[PUSH(vec![4]), JUMP, DATA(vec![0]), JUMPDEST],
    );
}

#[test]
pub fn test_disassemble_split_02() {
    // A minimal split multiblock program.  This program contains an
    // invalid JUMPDEST.
    check("cfg","0x600456605b", &[PUSH(vec![4]), JUMP, PUSH(vec![0x5b])]);
}

#[test]
pub fn test_disassemble_split_03() {
    // A minimal split multiblock program
    check("cfg",
        "0x6003565b0061",
        &[PUSH(vec![3]), JUMP, JUMPDEST, STOP, DATA(vec![0x61])],
    );
}

#[test]
pub fn test_disassemble_split_04() {
    // A minimal split multiblock program
    check("cfg",
        "0x60055601025b",
        &[PUSH(vec![5]), JUMP, DATA(vec![1, 2]), JUMPDEST],
    );
}

// ============================================================================
// Function Call tests
// ============================================================================

#[test]
pub fn test_disassemble_zcall_01() {
    check("cfg",
        "0x60056007565b005b56",
        &[PUSH(vec![5]), PUSH(vec![7]), JUMP, JUMPDEST, STOP, JUMPDEST, JUMP],
    );
}

#[test]
pub fn test_disassemble_zcall_02() {
//         if storage[0] goto l1;
//         call fn();
//         succeed;
// .l1
//         call fn();
//         revert;
// .fn
//         return;
    check("cfg",
        "0x600054600d57600b6019565b005b60136019565b60006000fd5b5600",
        &[PUSH(vec![0x0]),SLOAD,PUSH(vec![0xd]),JUMPI,PUSH(vec![0xb]),PUSH(vec![0x19]),JUMP,JUMPDEST,STOP,JUMPDEST,PUSH(vec![0x13]),PUSH(vec![0x19]),JUMP,JUMPDEST,PUSH(vec![0]),PUSH(vec![0]),REVERT,JUMPDEST,JUMP,DATA(vec![0x00])]);
}

#[test]
pub fn test_disassemble_zcall_03() {
//         if storage[0] goto l1;
//         call fn();
//         succeed;
// .l1
//         call fn();
//         revert;
// .fn
//         if storage[0] goto l2;
//         return;
//         stop;
// .l2
//         revert;
    check("cfg","0x600054600d57600b6019565b005b60136019565b60006000fd5b60005460225756005b60006000fd",&[
        PUSH(vec![0x0]),
        SLOAD,
        PUSH(vec![0xd]),
        JUMPI,
        PUSH(vec![0xb]),
        PUSH(vec![0x19]),
        JUMP,
        JUMPDEST,
        STOP,
        JUMPDEST,
        PUSH(vec![0x13]),
        PUSH(vec![0x19]),
        JUMP,
        JUMPDEST,
        PUSH(vec![0]),
        PUSH(vec![0]),
        REVERT,
        JUMPDEST,
        PUSH(vec![0]),
        SLOAD,
        PUSH(vec![0x22]),
        JUMPI,
        JUMP,
        DATA(vec![0x00]),
        JUMPDEST,
        PUSH(vec![0]),
        PUSH(vec![0]),
        REVERT
    ]);
}


// ============================================================================
// Helpers
// ============================================================================

/// Check that disassembling a given hex string produces a given
/// sequence of instructions.
fn check(prefix: &str, hex: &str, insns: &[Instruction]) {
    util::log_half_test(prefix,hex,insns);
    // Parse hex string into bytes
    let bytes = hex.from_hex_string().unwrap();
    // Disassemble bytes into instructions
    let disasm: Disassembly<AbstractStack<AbstractWord>> = Disassembly::new(&bytes).build();
    // Check against expected instruction sequence
    assert_eq!(insns, disasm.to_vec());
}
