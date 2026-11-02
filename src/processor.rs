use crate::InstructionSet;

/// The outcome of executing one instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Continue,
    Halt,
}

/// Emulates a processor for an instruction set.
pub trait Processor: InstructionSet + Sized {
    /// Creates a new processor state with the PC initialized.
    ///
    /// # Arguments
    ///
    /// * `program_counter` - The initial value of the program counter
    /// * `program` - The program instructions
    /// * `data` - The statically defined data
    fn new(program_counter: u64, program: Vec<u8>, data: Vec<u8>) -> Self;

    /// Executes one instruction: reads it from memory, decodes it, and executes it.
    ///
    /// # Errors
    ///
    /// Returns a message describing the fault, e.g. an invalid address.
    fn step(&mut self) -> Result<Step, String>;

    fn debug(&self) {}
}
