// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::error::Error;
use std::fs;

use clap::{arg, Arg, ArgMatches, Command};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
//
use evmil::analysis::{aw256,ConcreteStack,ConcreteState,ConcreteMemory,UnknownStorage};
use evmil::analysis::{find_dependencies,insert_havocs,trace};
use evmil::bytecode::{Assembly,Instruction,StructuredSection};
use evmil::il::{Compiler,Parser};
use evmil::util::{FromHexString, ToHexString};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let matches = Command::new("evmil")
        .about("EvmIL Tool")
        .version("0.1.0")
        .subcommand_required(true)
        .arg(arg!(--verbose "Show verbose output"))
        .subcommand(
            Command::new("compile")
                .about("Compile EvmIL code to EVM bytecode")
                .arg(Arg::new("eof").long("eof"))
                .arg(Arg::new("file").required(true))
                .visible_alias("c")
        )
        .subcommand(
            Command::new("disassemble")
                .about("Disassemble a raw hex string into EVM bytecode")
                .arg(Arg::new("code").short('c').long("code"))
                .arg(Arg::new("eof").long("eof"))
                .arg(Arg::new("debug").short('d').long("debug"))
                .arg(Arg::new("havoc").long("havoc"))                
                .arg(Arg::new("deps").long("deps"))
                .arg(Arg::new("target").required(true))
                .visible_alias("d")
        )
        .subcommand(
            Command::new("assemble")
                .about("Assemble EVM bytecode into a raw hex string")
                .arg(Arg::new("eof").long("eof"))
                .arg(Arg::new("target").required(true))
                .visible_alias("a")
        )
        .subcommand(
            Command::new("infer")
                .about("annotations on an assembly contract")
                .arg(Arg::new("debug").short('d').long("debug"))
                .arg(Arg::new("havoc").long("havoc"))                
                .arg(Arg::new("deps").long("deps"))
                .arg(Arg::new("target").required(true))
                .visible_alias("i")
        )
        .get_matches();
    // Extract top-level flags
    let verbose = matches.is_present("verbose");
    // Initialise logging
    if verbose {
        init_logging(LevelFilter::Info);
    }
    // Dispatch on outcome
    let ok = match matches.subcommand() {
        Some(("assemble", args)) => assemble(args),
        Some(("compile", args)) => compile(args),
        Some(("disassemble", args)) => disassemble(args),
        Some(("infer", args)) => infer(args),        
        _ => unreachable!(),
    }?;
    // Determine appropriate exit code
    let exitcode = if ok { 0 } else { 1 };
    // Done
    std::process::exit(exitcode);
}


/// Assemble a given bytecode sequence.
fn assemble(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    let target = args.get_one::<String>("target").unwrap();
    // Read from asm file
    let context = fs::read_to_string(target)?;
    // Construct assembly from input file
    let assembly = Assembly::from_str(&context)?;
    // Check whether EOF or legacy code generation
    let bytes : Vec<u8> = if args.contains_id("eof") {
        // EVM Object Format
        todo!()
        //assembly.to_eof_bytes()
    } else {
        // Legacy
        assembly.to_legacy_bytes()
    };
    // Print the final hex string
    println!("{}", bytes.to_hex_string());
    //
    Ok(true)
}

/// Compile a given file.
fn compile(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    // Extract the file to be compiled.
    let filename = args.get_one::<String>("file").unwrap();
    // Read the test file
    let input = fs::read_to_string(filename)?;
    // Parse test file
    let terms = Parser::new(&input).parse()?;
    // Translate statements into bytecode instructions
    let mut compiler = Compiler::new();
    // Translate statements one-by-one
    for t in &terms {
        // FIXME: need better error handling!!
        compiler.translate(t).unwrap();
    }
    // Compiler terms into a bytecode assembly
    let assembly = compiler.to_assembly();
    // Translate container into bytes
    let bytes : Vec<u8> = if args.contains_id("eof") {
        // EVM Object Format
        todo!()
        //assembly.to_eof_bytes()
    } else {
        // Legacy
        assembly.to_legacy_bytes()
    };
    // Print the final hex string
    println!("{}", bytes.to_hex_string());
    //
    Ok(true)
}

/// Disassemble a given bytecode sequence.
fn disassemble(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    // Extract hex string to be disassembled.
    let mut hex = String::new();
    // Determine disassembly target
    let target = args.get_one::<String>("target").unwrap();
    // Decide whether bytecode provided directly, or via a file.
    if args.contains_id("code") {
        // Provided directly
        hex.push_str(target);
    } else {
        // Read hex from file
        let context = fs::read_to_string(target)?;
        // Read all lines of file
        for l in context.lines() { hex.push_str(l); }
    }
    // Parse hex string into bytes
    let bytes = hex.from_hex_string().unwrap();
    // Construct bytecode representation
    let asm = if args.contains_id("eof") {
        todo!()
        //Assembly::from_eof_bytes(&bytes)?
    } else {
        Assembly::from_legacy_bytes(&bytes)
    };
    //
    disassemble_assembly(args,asm);
    //
    Ok(true)
}

fn infer(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    let target = args.get_one::<String>("target").unwrap();
    // Read from asm file
    let context = fs::read_to_string(target)?;
    // Construct assembly from input file
    let asm = Assembly::from_str(&context)?;
    //
    disassemble_assembly(args,asm);
    //
    Ok(true)
}

fn disassemble_assembly(args: &ArgMatches, mut asm: Assembly) {
    // Check whether to insert havocs (or not)
    let havoc = args.contains_id("havoc");
    // Check whether debug information enabled (or not)    
    let debug = args.contains_id("debug");
    let deps = args.contains_id("deps");
    // Apply havoc inference (if requested)
    if havoc { asm = infer_havoc_insns(asm); }    
    //
    for section in &asm {
        match section {
            StructuredSection::Code(insns) => {
                println!(".code");
                if debug {
                    disassemble_debug_code(insns);
                } else if deps {
                    disassemble_dep_code(insns);                    
                } else {
                    disassemble_code(insns);
                }
            }
            StructuredSection::Data(bytes) => {
                println!(".data");
                println!("\t{}",bytes.to_hex_string());
            }
        }
    }
}

// Disassemble a code section _without_ debug information.  The reason
// for separating out the two functions is that generating debug
// information may fail.
fn disassemble_code(insns: &[Instruction]) {
    let mut pc = 0;
    for insn in insns {
        if insn == &Instruction::JUMPDEST {
            println!("_{pc:#06x}:");
        }
        println!("\t{insn}");
        pc += insn.length();
    } 
}

// Disassemble a code section _with_ dependency information.  Note
// that this can fail if the underlying static analysis fails.
fn disassemble_dep_code(insns: &[Instruction]) {
    let mut pc = 0;
    //
    let deps = find_dependencies(insns, usize::MAX).map_err(|_| ()).unwrap();
    //
    for (i,insn) in insns.iter().enumerate() {    
        if insn == &Instruction::JUMPDEST {
            println!("_{pc:#06x}:");
        }
        print!("\t;; [{i}] pc={pc:#02x} ");
        for f in 0..deps.frames(i) {
            let fth = deps.get_frame(i,f);
            if !fth.is_empty() {
                print!("{:?}",fth);
            }
        }
        println!();
        println!("\t{insn}");
        pc += insn.length();
    } 
}

type DebugState = ConcreteState<ConcreteStack<aw256>,ConcreteMemory<aw256>,UnknownStorage<aw256>>;

// Disassemble a code section _with_ debug information.  Note that
// this can fail if the underlying static analysis fails.
fn disassemble_debug_code(insns: &[Instruction]) {
    // Run the static analysis
    let states : Vec<Vec<DebugState>> = trace(insns,DebugState::new(),usize::MAX).map_err(|_| ()).unwrap();
    // Print out info
    let mut pc = 0;
    for (i,insn) in insns.iter().enumerate() {
        if insn == &Instruction::JUMPDEST {
            println!("_{pc:#06x}:");
        }
        for st in &states[i] {
            println!("\t;; {}",st);
        }
        println!("\t{insn}");
        pc += insn.length();
    } 
}

fn infer_havoc_insns(mut asm: Assembly) -> Assembly {
    // This could probably be more efficient :)
    let sections = asm.iter_mut().map(|section| {
        match section {
            StructuredSection::Code(ref mut insns) => {    
                let ninsns = insert_havocs(insns.clone(), usize::MAX).unwrap();
	        StructuredSection::Code(ninsns)
            }
            _ => section.clone()
        }
    }).collect();
    // 
    Assembly::new(sections)
}

/// Initialise logging using a suitable pattern.
pub fn init_logging(level: LevelFilter) {
    let encoder = PatternEncoder::new("[{l}] {m}{n}");
    //
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();
    //
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    //
    let _handle = log4rs::init_config(config).unwrap();
}
