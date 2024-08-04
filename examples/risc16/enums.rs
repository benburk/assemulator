use strum_macros::EnumString;

#[derive(Debug, EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
pub enum Opcode {
    Add,
    Addi,
    Nand,
    Lui,
    Ld,
    St,
    Beq,
    Jalr,
}

impl TryFrom<u16> for Opcode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let opcode = match value {
            0b000 => Self::Add,
            0b001 => Self::Addi,
            0b010 => Self::Nand,
            0b011 => Self::Lui,
            0b100 => Self::Ld,
            0b101 => Self::St,
            0b110 => Self::Beq,
            0b111 => Self::Jalr,
            _ => {
                return Err(());
            }
        };
        Ok(opcode)
    }
}
