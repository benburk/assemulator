mod enums;
mod memory;
use std::num::Wrapping;

use memory::MemoryMappedRam;

use assemulator::{Argument, InstructionSet, Processor, Step, mask, run};
use enums::Register;

fn is_reg_ram(reg: Register) -> bool {
    matches!(reg, Register::RamK | Register::RamX | Register::RamY)
}

/// An instruction operand after mapping memory operands to addressing modes.
#[derive(Debug, Clone, Copy)]
enum Operand {
    /// A register or addressing mode, with the address byte for memory modes.
    Arg(Register, Option<u64>),
    Imm(u64),
}

fn operand(arg: &Argument<Register>) -> Result<Operand, String> {
    Ok(match *arg {
        Argument::Reg(reg) => Operand::Arg(reg, None),
        Argument::Imm(imm) => Operand::Imm(imm),
        Argument::Mem { base, offset } => {
            let mode = match base {
                None => Register::RamK,
                Some(Register::X) => Register::RamX,
                Some(Register::Y) => Register::RamY,
                Some(reg) => return Err(format!("only x and y can index memory, not {reg:?}")),
            };
            Operand::Arg(mode, Some(offset))
        }
    })
}

/// Encodes an instruction byte followed by any address and immediate bytes.
fn encode(inst: u8, bytes: &[Option<u64>]) -> Result<Vec<u8>, String> {
    let mut out = vec![inst];
    for &byte in bytes.iter().flatten() {
        out.push(mask::<u8>(byte, 8)?);
    }
    Ok(out)
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
        if plus_one { byte2 } else { byte1 }
    }

    fn read_argument(&mut self, reg: Register, imm: Wrapping<u8>) -> Result<Wrapping<u8>, String> {
        match reg {
            Register::RamK => self.data.read(imm.0),
            Register::RamX => self.data.read((self.regs[3] + imm).0),
            Register::RamY => self.data.read((self.regs[4] + imm).0),
            reg => Ok(self.regs[usize::from(reg)]),
        }
    }

    fn write_to_destination(
        &mut self,
        dest: Register,
        imm: Wrapping<u8>,
        value: Wrapping<u8>,
    ) -> Result<(), String> {
        match dest {
            Register::RamK => self.data.write(imm.0, value.0),
            Register::RamX => self.data.write((self.regs[3] + imm).0, value.0),
            Register::RamY => self.data.write((self.regs[4] + imm).0, value.0),
            reg => {
                self.regs[usize::from(reg)] = value;
                Ok(())
            }
        }
    }
}

impl InstructionSet for Mira {
    type Opcode = enums::Opcode;
    type Register = enums::Register;

    fn encode(
        _: u64,
        opcode: Self::Opcode,
        arguments: &[Argument<Self::Register>],
    ) -> Result<Vec<u8>, String> {
        use Operand::{Arg, Imm};
        use enums::Opcode::*;

        let operands = arguments
            .iter()
            .map(operand)
            .collect::<Result<Vec<_>, _>>()?;
        let op = u8::from(opcode) << 3;

        match (opcode, operands.as_slice()) {
            // group 0: A op= arg
            (Add | Adc | Sub | Sbc | Cmp | And | Ior | Xor, &[Arg(a, addr)]) => {
                encode(op | u8::from(a), &[addr])
            }
            // group 1: arg op= immediate
            (Add | Adc | Sub | Sbc | Cmp | And | Ior | Xor, &[Arg(a, addr), Imm(imm)]) => {
                encode(0b0100_0000 | op | u8::from(a), &[addr, Some(imm)])
            }
            // group 2: move, where dest == arg means an immediate
            (Mov, &[Arg(a, None), Imm(imm)]) => {
                encode(0b1000_0000 | u8::from(a) << 3 | u8::from(a), &[Some(imm)])
            }
            (Mov, &[Arg(a, a_addr), Arg(b, b_addr)])
                if a != b && (a_addr.is_none() || b_addr.is_none()) =>
            {
                encode(
                    0b1000_0000 | u8::from(a) << 3 | u8::from(b),
                    &[a_addr, b_addr],
                )
            }
            // group 3: one-operand ops and jumps
            (Shl | Shr | Rol | Ror | Inc | Dec, &[Arg(a, addr)]) => {
                encode(0b1100_0000 | op | u8::from(a), &[addr])
            }
            (Jmp, &[Imm(dest)]) => encode(0b1111_0000, &[Some(dest)]),
            (Bcc | Bcs | Bne | Beq | Bpl | Bmi | Bvs | Bvc, &[Imm(dest)]) => {
                encode(0b1111_1000 | u8::from(opcode), &[Some(dest)])
            }
            _ => Err(format!("invalid instruction: {opcode:?} {arguments:?}")),
        }
    }
}

impl Processor for Mira {
    fn new(addr: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            program_counter: Wrapping(addr as WordSize),
            program,
            data: MemoryMappedRam::from(data.as_slice()),
            ..Default::default()
        }
    }

    fn step(&mut self) -> Result<Step, String> {
        if usize::from(self.program_counter.0) >= self.program.len() {
            return Ok(Step::Halt);
        }

        let inst = self.program[usize::from(self.program_counter.0)];
        let byte1 = Wrapping(
            *self
                .program
                .get(usize::from(self.program_counter.0) + 1)
                .unwrap_or(&0),
        );
        let byte2 = Wrapping(
            *self
                .program
                .get(usize::from(self.program_counter.0) + 2)
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
                // group 1: `arg op= immediate`
                a = self.read_argument(reg2, byte1)?;
                b = self.read_immediate(byte1, byte2, is_reg_ram(reg2));
            } else if reg2 == Register::A {
                // group 0 with `arg == A` is a nop
                return Ok(Step::Continue);
            } else {
                a = self.regs[0];
                b = self.read_argument(reg2, byte1)?;
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
                    self.write_to_destination(reg2, byte1, result)?;
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
                self.write_to_destination(reg1, byte1, imm)?;
            } else {
                // move
                let value = self.read_argument(reg2, byte1)?;
                self.write_to_destination(reg1, byte1, value)?;
            }
        } else if arg1 <= 0b101 {
            // type 3
            let b = self.read_argument(reg2, byte1)?;
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
            self.write_to_destination(reg2, byte1, result)?;
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

        Ok(Step::Continue)
    }
}

fn main() {
    run::<Mira>();
}
