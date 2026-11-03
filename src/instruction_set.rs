/// An instruction operand parsed by the assembler.
/// The register type is specified by the instruction set, so registers can have custom names.
#[derive(Debug)]
pub enum Argument<T> {
    /// A register argument for an instruction
    Reg(T),
    /// An immediate argument for an instruction
    Imm(u64),
    /// A memory operand `[base + offset]`, e.g. `[ticker]` or `[arr+x]`
    Mem { base: Option<T>, offset: u64 },
}

/// Byte order of multi-byte values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Big,
    Little,
}

impl Endian {
    /// Converts bytes to 16-bit words in this byte order, padding a trailing
    /// odd byte with zero.
    #[must_use]
    pub fn u16_words(self, bytes: &[u8]) -> Vec<u16> {
        bytes
            .chunks(2)
            .map(|pair| {
                let pair = [pair[0], pair.get(1).copied().unwrap_or_default()];
                match self {
                    Self::Big => u16::from_be_bytes(pair),
                    Self::Little => u16::from_le_bytes(pair),
                }
            })
            .collect()
    }
}

/// Describes how to assemble an instruction set.
pub trait InstructionSet {
    type Opcode: for<'a> TryFrom<&'a str> + Copy + std::fmt::Debug;
    type Register: for<'a> TryFrom<&'a str> + Copy + std::fmt::Debug;

    /// Byte order used by multi-byte data directives such as `.u16`.
    const ENDIAN: Endian = Endian::Big;

    /// Encodes one instruction into bytes.
    ///
    /// An instruction's length must not depend on the values of its operands,
    /// since labels defined later are 0 in the assembler's first pass.
    ///
    /// # Arguments
    ///
    /// * `address` - The address the instruction will be placed at
    /// * `opcode` - The opcode of the instruction
    /// * `arguments` - The arguments of the instruction
    ///
    /// # Errors
    ///
    /// Returns a message explaining why the instruction is invalid.
    fn encode(
        address: u64,
        opcode: Self::Opcode,
        arguments: &[Argument<Self::Register>],
    ) -> Result<Vec<u8>, String>;
}

#[cfg(test)]
mod tests {
    use super::Endian;

    #[test]
    fn converts_bytes_to_words() {
        assert_eq!(Endian::Big.u16_words(&[0x12, 0x34, 0x56]), [0x1234, 0x5600]);
        assert_eq!(Endian::Little.u16_words(&[0x12, 0x34]), [0x3412]);
    }
}
