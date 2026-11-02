#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod assembler;
mod get_input;
mod mask;
mod parse_int;
mod port;
mod processor;
mod register;

use std::path::PathBuf;

use clap::Parser;
use get_input::input;
pub use mask::mask;
pub use port::{Port, State};
pub use processor::{Argument, Endian, Processor};
pub use register::Register;

#[derive(Parser)]
#[command(about = "Assemulator")]
struct Args {
    /// Action
    #[command(subcommand)]
    action: Action,

    /// Input file
    file: PathBuf,
}

/// Actions that can be performed
#[derive(clap::Subcommand)]
enum Action {
    /// Assemble the program
    Assemble,
    /// Run the program
    Run {
        /// Print the processor state and wait for enter after each step
        #[arg(long)]
        debug: bool,
    },
}

pub fn run<T: Processor>() {
    let args = Args::parse();

    let asm = assembler::Assembler::<T>::assemble(&args.file).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    match args.action {
        Action::Assemble => {
            println!("Data: {:#04x?}", asm.data);
            println!("Program: {:#04x?}", asm.program);
        }
        Action::Run { debug } => {
            let mut state = T::new(asm.start, asm.program, asm.data);
            if debug {
                state.debug();
                input("> ");
            }
            while state.step() != 0 {
                if debug {
                    state.debug();
                    input("> ");
                }
            }
        }
    }
}
