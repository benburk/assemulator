//! Opcodes for the CPU
use strum_macros::EnumString;

/// CPU opcodes
#[derive(Debug, EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Opcode {
    /// bitwise and
    And,
    /// bitwise inclusive or
    Or,
    /// bitwise exclusive or
    Xor,
    /// move
    Mov,
    /// test
    Tst,
    /// equal
    Eq,
    /// greater than or equal
    Geq,
    /// greater than or equal signed
    Ges,
    /// add
    Add,
    /// add with carry
    Addc,
    /// add set x
    Addx,
    /// add with carry, set x
    Addb,
    /// subtract
    Sub,
    /// sub set c
    Subc,
    /// sub set x
    Subx,
    /// sub with borrow, set x
    Subb,
    /// Shift left
    Shl,
    /// Shift left, lsb=x
    Shlc,
    /// Shift left, x=msb
    Shlx,
    /// Shilf left, x=msb, lsb=x
    Shlb,
    /// Shift right
    Shr,
    /// Shift right, lsb=x
    Shrc,
    /// Shift right, x=msb
    Shrx,
    /// Shift right, x=msb, lsb=x
    Shrb,
    /// move if true
    Mvt,
    /// move if false
    Mvf,
    /// conditional add
    Cad,
    /// conditional subtract
    Csb,
    /// negate
    Neg,
    /// bitwise not
    Swap,
    /// push
    Psh,
    /// pop
    Pop,
    /// Branch if true
    Bt,
    /// Branch if false
    Bf,
    /// Jump
    Jmp,
    /// Jump subroutine
    Jsr,
    /// Branch if true, decrement
    Btd,
    /// Load from memory
    Ld,
    /// Store to memory
    St,
    /// Port load
    Pld,
    /// Port store
    Pst,
    /// Return from subroutine
    Ret,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Register {
    /// A register.
    A,
    /// B register.
    B,
    /// C register.
    C,
    /// D register.
    D,
    /// E register.
    E,
    /// F register.
    F,
    /// G register.
    G,
    /// H register.
    H,
}

impl From<Register> for u16 {
    fn from(reg: Register) -> Self {
        match reg {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::D => 3,
            Register::E => 4,
            Register::F => 5,
            Register::G => 6,
            Register::H => 7,
        }
    }
}

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value & 0b111 {
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => Self::A,
        }
    }
}

impl Register {
    /// Places the register in the A field of an instruction.
    pub fn to_a(self) -> u16 {
        u16::from(self).wrapping_shl(13)
    }

    /// Places the register in the B field of the instruction.
    pub fn to_b(self) -> u16 {
        u16::from(self).wrapping_shl(5)
    }
}
