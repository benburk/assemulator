use strum_macros::EnumString;

#[derive(Debug, EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
pub enum Opcode {
    Add,
    Nand,
    Lui,
    Ld,
    St,
    Beq,
    Jalr,
}
