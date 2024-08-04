use strum::ParseError;

/// A token from the assembler
/// Register type is to be specified by the processor.
/// This is better than an integer because we can have custom names for registers
#[derive(Debug)]
pub enum Argument<T> {
    /// A register argument for an instruction
    Reg(T),
    /// An immediate argument for an instruction
    Imm(u64),
}

pub trait Processor: Default {
    type Opcode: for<'a> TryFrom<&'a str, Error = ParseError> + Copy + Clone + std::fmt::Debug;
    type Register: for<'a> TryFrom<&'a str, Error = ParseError> + Copy + Clone + std::fmt::Debug;

    /// Creates a new processor state with the PC initialized.
    ///
    /// # Arguments
    ///
    /// * `program_counter` - The initial value of the program counter
    /// * `program` - The program instructions
    /// * `data` - The statically defined data
    fn new(program_counter: u64, program: Vec<u8>, data: Vec<u8>) -> Self;

    /// Parses a list of tokens into a list of bytes.
    /// Passes in the address that this instruction will be at
    ///
    /// Important: don't generate different length instruction
    /// based on size of immediate. TODO?
    ///
    /// # Arguments
    ///
    /// * `opcode` - The opcode of the instruction
    /// * `arguments` - The arguments of the instruction
    /// * `address` - The address the instruction will be placed at
    ///
    /// # Errors
    ///
    /// Returns an error message of why the instruction failed to be
    /// parsed.
    fn parse(
        address: u64,
        opcode: Self::Opcode,
        arguments: &[Argument<Self::Register>],
    ) -> Result<Vec<u8>, String>;

    /// Executes one instruction. Handles reading the instruction from memory, parsing
    /// it, and executing it. The function returns the number of cycles it took to execute.
    /// If zero is returned, execution is stopped.
    fn step(&mut self) -> usize;
}
