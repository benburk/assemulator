//! Opcodes for the CPU
use strum_macros::EnumString;

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum Opcode {
    Add,
    Cmp,
    Sub,
    Sbc,
    Adc,
    And,
    Ior,
    Xor,
    Mov,
    Shl,
    Shr,
    Rol,
    Ror,
    Inc,
    Dec,
    Jmp,
    Bcc,
    Bcs,
    Bne,
    Beq,
    Bpl,
    Bmi,
    Bvs,
    Bvc,
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Add => 0,
            Opcode::Adc => 1,
            Opcode::Sub => 2,
            Opcode::Sbc => 3,
            Opcode::Cmp => 4,
            Opcode::And => 5,
            Opcode::Ior => 6,
            Opcode::Xor => 7,
            Opcode::Mov => 0,
            Opcode::Shl => 0,
            Opcode::Shr => 1,
            Opcode::Rol => 2,
            Opcode::Ror => 3,
            Opcode::Inc => 4,
            Opcode::Dec => 5,
            Opcode::Jmp => 6,
            Opcode::Bpl => 0,
            Opcode::Bmi => 1,
            Opcode::Bne => 2,
            Opcode::Beq => 3,
            Opcode::Bcc => 4,
            Opcode::Bcs => 5,
            Opcode::Bvc => 6,
            Opcode::Bvs => 7,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Register {
    A,
    B,
    C,
    X,
    Y,
    RamK,
    RamX,
    RamY,
}

impl From<Register> for u8 {
    fn from(reg: Register) -> Self {
        match reg {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::X => 3,
            Register::Y => 4,
            Register::RamK => 5,
            Register::RamX => 6,
            Register::RamY => 7,
        }
    }
}

impl From<Register> for usize {
    fn from(reg: Register) -> Self {
        match reg {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::X => 3,
            Register::Y => 4,
            Register::RamK => 5,
            Register::RamX => 6,
            Register::RamY => 7,
        }
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::X,
            4 => Self::Y,
            5 => Self::RamK,
            6 => Self::RamX,
            7 => Self::RamY,
            _ => unreachable!(),
        }
    }
}
