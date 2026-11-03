#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod assembler;
mod get_input;
mod instruction_set;
mod mask;
mod parse_int;
mod port;
mod processor;
mod register;

use std::path::{Path, PathBuf};

use assembler::AssemblyResult;

use clap::Parser;
use get_input::input;
pub use instruction_set::{Argument, Endian, InstructionSet};
pub use mask::mask;
pub use port::{Port, State};
pub use processor::{Processor, Step};
pub use register::Register;

#[derive(Parser)]
#[command(about = "Assemulator")]
struct Args {
    #[command(subcommand)]
    action: Action,
}

/// Actions that can be performed
#[derive(clap::Subcommand)]
enum Action {
    /// Assemble the program and print its bytes
    Assemble {
        /// Input file
        file: PathBuf,
    },
    /// Run the program
    Run {
        /// Input file
        file: PathBuf,
        /// Print the processor state and wait for enter after each step
        #[arg(long)]
        debug: bool,
    },
}

fn assemble<T: Processor>(file: &Path) -> AssemblyResult {
    assembler::Assembler::<T>::assemble(file).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    })
}

pub fn run<T: Processor>() {
    match Args::parse().action {
        Action::Assemble { file } => {
            let asm = assemble::<T>(&file);
            println!("Data: {:#04x?}", asm.data);
            println!("Program: {:#04x?}", asm.program);
        }
        Action::Run { file, debug } => {
            let asm = assemble::<T>(&file);
            let mut state = T::new(asm.start, asm.program, asm.data);
            loop {
                if debug {
                    state.debug();
                    input("> ");
                }
                match state.step() {
                    Ok(Step::Continue) => {}
                    Ok(Step::Halt) => break,
                    Err(err) => {
                        eprintln!("error: {err}");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}
