/// Functionality for working with contracts represented in assembly
/// language.
pub mod asm;
/// Functionality related to the encoding and representation of
/// contract bytecode.  This includes abstractions for _contracts_ and
/// _instructions_.
pub mod bytecode;
/// Functionality related to bytecode containers.
pub mod contract;
/// Functionality related to contracts adhering to the _EVM Object
/// Format_.  See [EIP3540](https://eips.ethereum.org/EIPS/eip-3540).
/// Again, this includes assembling / disassembling EOF contracts,
/// validing them, managing EOF versions, etc.
pub mod eof;
/// Functionality related to execution or analysis of bytecode
/// sequences.  This supports both runtime execution, as well as
/// various kinds of static analysis and verification.
pub mod execution;
/// Functionality related to distinguishing different forks of the
/// EVM.  This includes mechanisms for identifying what EIPs are
/// active in the current execution.
pub mod fork;
pub mod il;
/// Functionality related to _legacy_ (i.e. pre-EOF) contracts.  For
/// example, disassembling a legacy contract, assembling a legacy
/// contract, etc.
pub mod legacy;
/// Abstractions of EVM state, such as stack, memory and storage.
pub mod state;
pub mod util;
