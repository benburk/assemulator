#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod assembler;
mod mask;
mod port;
mod processor;
mod register;

use std::path::PathBuf;

use clap::Parser;
pub use mask::mask;
pub use port::{Port, State};
pub use processor::{Argument, Processor};
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
    Run,
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
        Action::Run => {
            let mut state = T::new(asm.start, asm.program, asm.data);
            while state.step() != 0 {}
        }
    }
}
