use evmil::Parser;
use clap::{arg, Arg, ArgMatches, Command};
use std::error::Error;
use std::fs;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::{PatternEncoder};

fn main() -> Result<(),Box<dyn Error>> {
    // Parse command-line arguments
    let matches = Command::new("evmil")
	.about("EvmIL Tool")
	.version("0.1.0")
        .subcommand_required(true)
	.arg(arg!(--verbose "Show verbose output"))
	.subcommand(
	    Command::new("compile")
                .about("Compile EvmIL code to EVM bytecode")
                .arg(Arg::new("file").required(true))
                .visible_alias("c"))
	.get_matches();
    // Extract top-level flags
    let verbose = matches.is_present("verbose");
    // Initialise logging
    if verbose {
	init_logging(LevelFilter::Info);
    }
    // Dispatch on outcome
    let ok = match matches.subcommand() {
	Some(("compile", args)) => compile(args),
	_ => unreachable!()
    }?;
    // Determine appropriate exit code
    let exitcode = if ok { 0 } else { 1 };
    // Done
    std::process::exit(exitcode);
}

/// Compile a given file.
fn compile(args: &ArgMatches) -> Result<bool,Box<dyn Error>> {
    // Extract the file to be compiled.
    let filename = args.get_one::<String>("file").unwrap();
    // Read the test file
    let input = fs::read_to_string(filename)?;
    //
    for l in input.lines() {
        let t = Parser::new(l).parse();
        if t.is_err() {
            println!("Error: {}",l);
        }
    }
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
