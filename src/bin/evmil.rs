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
use evmil::asm::{Assembly,AssemblyInstruction};
use evmil::bytecode::{Section};
use evmil::{eof,legacy};
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
        _ => unreachable!(),
    }?;
    // Determine appropriate exit code
    let exitcode = if ok { 0 } else { 1 };
    // Done
    std::process::exit(exitcode);
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
    // Assemble instructions into a bytecode container
    let bytecode = compiler.to_bytecode();
    // Translate container into bytes
    let bytes = if args.contains_id("eof") {
        // EVM Object Format
        eof::to_bytes(bytecode).unwrap()
    } else {
        // Legacy
        legacy::to_bytes(&bytecode)
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
        eof::from_bytes(&bytes)?
    } else {
        legacy::from_bytes(&bytes)
    };
    // Iterate bytecode sections
    for section in &asm {
        match section {
            Section::Code(insns) => {
                println!(".code");
                for insn in insns {
                    match insn {
                        AssemblyInstruction::LABEL(_) => {
                            println!("{insn}")
                        }
                        _ => {
                            println!("\t{insn}")
                        }
                    }

                }
            }
            Section::Data(bytes) => {
                println!(".data");
                println!("\t{}",bytes.to_hex_string());
            }
        }
    }
    Ok(true)
}

/// Assemble a given bytecode sequence.
fn assemble(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    let target = args.get_one::<String>("target").unwrap();
    // Read from asm file
    let context = fs::read_to_string(target)?;
    // Construct assembly from input file
    let assembly = Assembly::from_str(&context)?;
    // Convert assembly language into concrete instructions.
    let compiled = assembly.assemble().unwrap();
    // Check whether EOF or legacy code generation
    let bytes = if args.contains_id("eof") {
        // EVM Object Format
        eof::to_bytes(compiled).unwrap()
    } else {
        // Legacy
        legacy::to_bytes(&compiled)
    };
    // Print the final hex string
    println!("{}", bytes.to_hex_string());
    //
    Ok(true)
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
