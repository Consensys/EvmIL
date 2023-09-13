/// Functionality related to the execution or analysis of bytecode
/// contracts.  This supports both _runtime execution_ and [_static
/// analysis_](https://en.wikipedia.org/wiki/Static_program_analysis).
/// Using this module we can, for example, extract the [control-flow
/// graph](https://en.wikipedia.org/wiki/Control-flow_graph) of a
/// legacy contract.  We can also write arbitrary [dataflow
/// analyses](https://en.wikipedia.org/wiki/Data-flow_analysis) which
/// operates over bytecode contracts (e.g. for [constant
/// propagation](https://en.wikipedia.org/wiki/Constant_folding)).
pub mod analysis;
/// Functionality for working with contracts represented in [assembly
/// language](https://en.wikipedia.org/wiki/Assembly_language).  For
/// example, a contract written in assembly language can be parsed
/// into an _assembly_ which, in turn, can be _assembled_ into a
/// bytecode contract.
///
/// # Examples
/// An example contract written in assembly language is:
///
/// ```text
/// .code
///    push 0x02
///    push 0x01
///    add
///    push lab0
///    jumpi
///    invalid
/// lab0:
///    jumpdest
/// ```
///
/// The following example illustrates how to parse some assembly
/// language into an assembly, and then assemble it into a bytecode
/// contract.
///
/// ```
/// use evmil::bytecode::Assembly;
/// use evmil::util::ToHexString;
/// // Assembly language
/// let asm = r#"
///  .code
///  push 0x01
///  push 0x02
///  add
/// "#;
/// // Parse into assembly
/// let asm = Assembly::from_str(&asm).unwrap();
/// // Generate (legacy) bytecode
/// let bytecode = asm.to_legacy_bytes().to_hex_string();
/// // Check output
/// assert_eq!(bytecode,"0x6001600201");
/// ```
// pub mod asm;
/// Functionality related to the encoding and representation of
/// contract bytecode.  This includes abstractions for _contracts_ and
/// _instructions_ and support for both _legacy_ and _EOF_ contracts
/// (see [EIP3540](https://eips.ethereum.org/EIPS/eip-3540)).
pub mod bytecode;
/// Functionality related to distinguishing different forks of the
/// EVM.  This includes mechanisms for identifying what EIPs are
/// active in the current execution.
pub mod fork;
/// A low-level intermediate language which has close correspondence
/// with bytecode.
pub mod il;
/// Various utilities required by other modules.
pub mod util;
