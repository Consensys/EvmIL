/// Functionality related to the analysis (or execution) of bytecode
/// contracts. Using this module we can, for example, extract the
/// [control-flow
/// graph](https://en.wikipedia.org/wiki/Control-flow_graph) of a
/// legacy contract.  We can also write arbitrary [dataflow
/// analyses](https://en.wikipedia.org/wiki/Data-flow_analysis) which
/// operate over bytecode contracts (e.g. for [constant
/// propagation](https://en.wikipedia.org/wiki/Constant_folding)).
///
/// # Examples
///
/// A minimal example is the following, which determines the
/// instruction reachability.  Here, an instruction is considered
/// unreachable if there is no path to it through the contract's
/// control-flow graph, starting from the first instruction.
///
/// ```
/// use evmil::analysis::find_reachable;
/// use evmil::bytecode::Disassemble;
/// use evmil::util::FromHexString;
///
/// // Convert hex string into bytes
/// let bytes = "0x600456fe00".from_hex_string().unwrap();
/// // Disassemble bytes into instructions.
/// let insns = bytes.disassemble();
/// // Compute reachability information.
/// let reachable = find_reachable(&insns);
/// // Check `INVALID` instruction (`0xfe`) is never executed.
/// assert_eq!(reachable,vec![true,true,false,true]);
/// ```
///
/// Here, the sequence `0x600456fe00` corresponds to the following
/// program:
///
/// ```txt
/// .code
///    push lab0
///    jump
///    invalid
/// lab0:
///    stop
/// ```
///
/// Thus, we can see that the `invalid` instruction can never be
/// executed.
pub mod analysis;
/// Functionality for working with bytecode contracts.  This includes
/// support for assembling contracts written in [assembly
/// language](https://en.wikipedia.org/wiki/Assembly_language) into
/// bytecode.
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
