mod enums;
mod memory;
use std::num::Wrapping;

use memory::MemoryMappedRam;

use assemulator::{mask, run, Argument, Processor};
use enums::{Opcode, Register};

fn is_reg_ram(reg: Register) -> bool {
    return matches!(reg, Register::RamK | Register::RamX | Register::RamY);
}

fn two_op_no_imm(opcode: Opcode, mode: Register) -> Result<Vec<u8>, String> {
    Ok(vec![u8::from(opcode) << 3 | u8::from(mode)])
}

// type0 or type1 depending if register is ram or not
fn two_op_with_imm(opcode: Opcode, mode: Register, imm: u64) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];

    if is_reg_ram(mode) {
        bytes.push(u8::from(opcode) << 3 | u8::from(mode));
        bytes.push(mask::<u8>(imm, 8)?);
    } else {
        bytes.push(0b01_000_000 | u8::from(opcode) << 3 | u8::from(mode));
        bytes.push(mask::<u8>(imm, 8)?);
    }

    Ok(bytes)
}

fn move_reg_reg(dest: Register, src: Register) -> Result<Vec<u8>, String> {
    let inst: u8 = 0b1000_0000 | u8::from(dest) << 3 | u8::from(src);
    Ok(vec![inst])
}

fn move_reg_imm(dest: Register, imm: u64) -> Result<Vec<u8>, String> {
    // move immediate is encoded by having arg1 and arg2 the same
    let mut bytes = vec![];
    bytes.push(0b1000_0000 | u8::from(dest) << 3 | u8::from(dest));
    bytes.push(mask::<u8>(imm, 8)?);
    Ok(bytes)
}

fn load(dest: Register, src: Register, imm: u64) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    bytes.push(0b1000_0000 | u8::from(dest) << 3 | u8::from(src));
    bytes.push(mask::<u8>(imm, 8)?);
    Ok(bytes)
}

fn store(dest: Register, imm: u64, src: Register) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    bytes.push(0b1000_0000 | u8::from(dest) << 3 | u8::from(src));
    bytes.push(mask::<u8>(imm, 8)?);
    Ok(bytes)
}

fn store_const(imm: u64, src: Register) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    bytes.push(0b1000_0000 | u8::from(Register::RamK) << 3 | u8::from(src));
    bytes.push(mask::<u8>(imm, 8)?);
    Ok(bytes)
}

fn one_op_reg(opcode: Opcode, reg: Register) -> Result<Vec<u8>, String> {
    // implemented as sub immediate for convenience
    let mut bytes = vec![];
    bytes.push(0b1100_0000 | u8::from(opcode) << 3 | u8::from(reg));
    Ok(bytes)
}

fn one_op_ram(opcode: Opcode, reg: Register, imm: u64) -> Result<Vec<u8>, String> {
    // implemented as sub immediate for convenience
    let mut bytes = vec![];
    bytes.push(0b1100_0000 | u8::from(opcode) << 3 | u8::from(reg));
    bytes.push(mask::<u8>(imm, 8)?);
    Ok(bytes)
}

fn branch(opcode: Opcode, imm: u64) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    bytes.push(0b1111_1000 | u8::from(opcode));
    bytes.push(mask::<u8>(imm, 8)?);
    Ok(bytes)
}

fn jump_imm(imm: u64) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    bytes.push(0b1111_0000);
    bytes.push(mask::<u8>(imm, 8)?);
    Ok(bytes)
}

fn compute_instruction_length(opcode: u8) -> u8 {
    let arg1 = opcode >> 3 & 0b111;
    let reg1 = Register::from(arg1);
    let reg2 = Register::from(opcode & 0b111);

    match opcode >> 6 {
        0b00 => 1 + u8::from(is_reg_ram(reg2)),
        0b01 => 2 + u8::from(is_reg_ram(reg2)),
        0b10 => {
            1 + u8::from(is_reg_ram(reg1)) + u8::from(is_reg_ram(reg2)) + u8::from(reg1 == reg2)
        }
        0b11 if arg1 < 0b110 => 1 + u8::from(is_reg_ram(reg2)),
        0b11 => 2,
        _ => unreachable!(),
    }
}

type WordSize = u8;

#[derive(Default)]
pub struct Mira {
    program_counter: Wrapping<WordSize>,
    regs: [Wrapping<WordSize>; 5],
    program: Vec<u8>,
    data: MemoryMappedRam,

    carry_flag: bool,
    zero_flag: bool,
    overflow_flag: bool,
    sign_flag: bool,
}

impl Mira {
    fn update_all_flags(&mut self, a: Wrapping<u8>, b: Wrapping<u8>, result: Wrapping<u8>) {
        self.carry_flag = result < a;
        self.overflow_flag = (a ^ result) >> 7 != (a ^ b) >> 7;
        self.update_nz_flags(result);
    }

    fn update_nz_flags(&mut self, result: Wrapping<u8>) {
        self.sign_flag = result >> 7 == Wrapping(1);
        self.zero_flag = result == Wrapping(0);
    }

    fn read_immediate(
        &self,
        byte1: Wrapping<u8>,
        byte2: Wrapping<u8>,
        plus_one: bool,
    ) -> Wrapping<u8> {
        if plus_one {
            byte2
        } else {
            byte1
        }
    }

    fn read_argument(&mut self, reg: Register, imm: Wrapping<u8>) -> Wrapping<u8> {
        match reg {
            Register::A => self.regs[usize::from(reg)],
            Register::B => self.regs[usize::from(reg)],
            Register::C => self.regs[usize::from(reg)],
            Register::RamK => self.data.read(imm.0),
            Register::X => self.regs[usize::from(reg)],
            Register::RamX => {
                let base = self.regs[usize::from(Register::X)];
                let addr = base + imm;
                self.data.read(addr.0)
            }
            Register::Y => self.regs[usize::from(reg)],
            Register::RamY => {
                let base = self.regs[usize::from(Register::Y)];
                let addr = base + imm;
                self.data.read(addr.0)
            }
        }
    }

    fn write_to_destination(&mut self, dest: Register, imm: Wrapping<u8>, value: Wrapping<u8>) {
        match dest {
            Register::A => self.regs[0] = value,
            Register::B => self.regs[1] = value,
            Register::C => self.regs[2] = value,
            Register::RamK => {
                self.data.write(imm.0, value.0);
            }
            Register::X => self.regs[3] = value,
            Register::RamX => {
                let base = self.regs[3];
                let addr = base + imm;
                self.data.write(addr.0, value.0);
            }
            Register::Y => self.regs[3] = value,
            Register::RamY => {
                let base = self.regs[3];
                let addr = base + imm;
                self.data.write(addr.0, value.0);
            }
        }
    }
}

impl Processor for Mira {
    type Opcode = enums::Opcode;
    type Register = enums::Register;

    fn new(addr: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            program_counter: Wrapping(addr as WordSize),
            program,
            data: MemoryMappedRam::from(data.as_slice()),
            ..Default::default()
        }
    }

    fn parse_assembly_line(
        _: u64,
        opcode: Self::Opcode,
        arguments: &[Argument<Self::Register>],
    ) -> Result<Vec<u8>, String> {
        use enums::Opcode::*;
        use Argument::{Imm, Reg};
        use Register::*;

        let instruction = match (opcode, arguments) {
            (opcode @ (Add | Adc | Sub | Sbc | Cmp | And | Ior | Xor), &[Reg(a)]) => {
                two_op_no_imm(opcode, a)
            }
            (opcode @ (Add | Adc | Sub | Sbc | Cmp | And | Ior | Xor), &[Reg(a), Imm(b)]) => {
                two_op_with_imm(opcode, a, b)
            }

            (Mov, &[Reg(a), Reg(b @ (A | B | C | X | Y))]) if a != b => move_reg_reg(a, b),
            (Mov, &[Reg(a @ (A | B | C | X | Y)), Imm(b)]) => move_reg_imm(a, b),
            (Mov, &[Reg(a @ (A | B | C | X | Y)), Reg(b @ (RamK | RamX | RamY)), Imm(c)]) => {
                load(a, b, c)
            }
            (Mov, &[Reg(a @ (RamK | RamX | RamY)), Imm(b), Reg(c @ (A | B | C | X | Y))]) => {
                store(a, b, c)
            }
            (Mov, &[Imm(a), Reg(b @ (A | B | C | X | Y))]) => store_const(a, b),

            (opcode @ (Shl | Shr | Rol | Ror | Inc | Dec), &[Reg(a)]) => one_op_reg(opcode, a),
            (opcode @ (Shl | Shr | Rol | Ror | Inc | Dec), &[Reg(a), Imm(b)]) => {
                one_op_ram(opcode, a, b)
            }

            (Jmp, &[Imm(a)]) => jump_imm(a),
            (opcode @ (Bcc | Bcs | Bne | Beq | Bpl | Bmi | Bvs | Bvc), &[Imm(a)]) => {
                branch(opcode, a)
            }
            _ => Err(format!("invalid instruction: {opcode:?} {:?}", arguments))?,
        };

        Ok(instruction?.to_vec())
    }

    fn step(&mut self) -> usize {
        if usize::from(self.program_counter.0) >= self.program.len() {
            return 0;
        }

        let inst = self.program[usize::from(self.program_counter.0)];
        let byte1 = Wrapping(
            *self
                .program
                .get(usize::from(self.program_counter.0 + 1))
                .unwrap_or(&0),
        );
        let byte2 = Wrapping(
            *self
                .program
                .get(usize::from(self.program_counter.0 + 2))
                .unwrap_or(&0),
        );

        let instruction_length = compute_instruction_length(inst);
        self.program_counter += Wrapping(instruction_length);

        let inst_type = inst >> 6;
        let arg1 = inst >> 3 & 0b111;
        let arg2 = inst & 0b111;

        let reg1 = Register::from(arg1);
        let reg2 = Register::from(arg2);
        let carry = Wrapping(u8::from(self.carry_flag));
        let is_two_op_instruction = inst >> 7 == 0;

        if is_two_op_instruction {
            let a;
            let b;

            if inst_type == 0b01 {
                a = if reg2 == Register::A {
                    byte1
                } else {
                    self.read_argument(reg2, byte1)
                };
                b = self.read_immediate(byte1, byte2, is_reg_ram(reg2));
            } else {
                a = self.regs[0];
                b = if reg2 == Register::A {
                    byte1
                } else {
                    self.read_argument(reg2, byte1)
                };
            }

            let result = match arg1 {
                0b000 => a + b,
                0b001 => a + b + carry,
                0b010 => a - b,
                0b011 => a - b - carry,
                0b100 => a - b,
                0b101 => a & b,
                0b110 => a | b,
                0b111 => a ^ b,
                _ => unreachable!(),
            };

            if arg1 != 0b100 {
                if inst_type == 0b01 {
                    self.write_to_destination(reg2, byte1, result);
                } else {
                    self.regs[0] = result;
                }
            }

            if arg1 <= 0b100 {
                self.update_all_flags(a, b, result);
            } else {
                self.update_nz_flags(result);
            }
        } else if inst_type == 0b10 {
            if reg1 == reg2 {
                // move immediate
                let imm = self.read_immediate(byte1, byte2, is_reg_ram(reg1));
                self.write_to_destination(reg1, byte1, imm);
            } else {
                // move
                let value = self.read_argument(reg2, byte1);
                self.write_to_destination(reg1, byte1, value);
            }
        } else if arg1 <= 0b101 {
            // type 3
            let b = self.read_argument(reg2, byte1);
            let result = match arg1 {
                0b000 => {
                    self.carry_flag = b >> 7 == Wrapping(1);
                    b << 1
                }
                0b001 => {
                    self.carry_flag = b.0 & 1 == 1;
                    b >> 1
                }
                0b010 => {
                    self.carry_flag = b >> 7 == Wrapping(1);
                    b << 1 | carry
                }
                0b011 => {
                    self.carry_flag = b.0 & 1 == 1;
                    b >> 1 | carry << 7
                }
                0b100 => b + Wrapping(1),
                0b101 => b - Wrapping(1),
                _ => unreachable!(),
            };
            self.update_nz_flags(result);
            self.write_to_destination(reg2, byte1, result);
        } else {
            match arg1 {
                0b110 => {
                    // jump
                    self.program_counter = byte1;
                }
                0b111 => {
                    let condition = match arg2 {
                        0b000 => !self.sign_flag,
                        0b001 => self.sign_flag,
                        0b010 => !self.zero_flag,
                        0b011 => self.zero_flag,
                        0b100 => !self.carry_flag,
                        0b101 => self.carry_flag,
                        0b110 => !self.overflow_flag,
                        0b111 => self.overflow_flag,
                        _ => unreachable!(),
                    };

                    if condition {
                        self.program_counter = byte1;
                    }
                }
                _ => unreachable!(),
            }
        }

        usize::from(compute_instruction_length(inst))
    }
}

fn main() {
    run::<Mira>();
}
