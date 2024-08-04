use std::num::Wrapping;

use assemulator::{mask, run, Argument, Port, Processor, State};

mod enums;

type Word = u16;
type Register = assemulator::Register<8>;

fn arith_shift_right(x: u64) -> u64 {
    ((x as i64) / 2) as u64
}

fn convert_u8_to_u16(data: Vec<u8>) -> Vec<u16> {
    data.chunks_exact(2)
        .map(|x| u16::from_be_bytes([x[0], x[1]]))
        .collect()
}

fn fmt1(op: u16, a: Register, b: Register, c: Register) -> u16 {
    op << 13 | (a.0 as u16) << 10 | (b.0 as u16) << 7 | (c.0 as u16)
}

fn fmt2(op: u16, a: Register, b: Register, imm: u64) -> Result<u16, String> {
    Ok(op << 13 | (a.0 as u16) << 10 | (b.0 as u16) << 7 | mask::<u16>(imm, 7)?)
}

fn fmt3(op: u16, a: Register, imm: u64) -> Result<u16, String> {
    Ok(op << 13 | (a.0 as u16) << 10 | mask::<u16>(imm, 10)?)
}

#[derive(Default)]
struct Risc16 {
    program_counter: Wrapping<Word>,
    registers: [Wrapping<Word>; 8],
    program: Vec<u16>,
    data: Vec<Word>,
    port_state: State<u16>,
}

impl Processor for Risc16 {
    type Opcode = enums::Opcode;
    type Register = Register;

    fn new(program_counter: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        let program_counter = Word::try_from(program_counter / 2).unwrap();
        Risc16 {
            program_counter: Wrapping(program_counter),
            program: convert_u8_to_u16(program),
            data: convert_u8_to_u16(data),
            ..Default::default()
        }
    }

    fn parse(
        address: u64,
        opcode: Self::Opcode,
        arguments: &[Argument<Self::Register>],
    ) -> Result<Vec<u8>, String> {
        use enums::Opcode::*;
        use Argument::{Imm, Reg};
        let instruction = match (opcode, arguments) {
            // add
            (Add, &[Reg(a), Reg(b), Reg(c)]) => fmt1(0b000, a, b, c),
            // add immediate
            (Add, &[Reg(a), Reg(b), Imm(c)]) => fmt2(0b001, a, b, c)?,
            // nand
            (Nand, &[Reg(a), Reg(b), Reg(c)]) => fmt1(0b010, a, b, c),
            // load upper immediate
            (Lui, &[Reg(a), Imm(imm)]) => fmt3(0b011, a, imm)?,
            // port instructions
            (Ld, &[Reg(a), Reg(b), Imm(c)]) if b.0 == 0 => fmt2(0b100, a, b, c)?,
            (St, &[Reg(a), Reg(b), Imm(c)]) if b.0 == 0 => fmt2(0b101, a, b, c)?,
            // memory load and store
            (Ld, &[Reg(a), Reg(b), Imm(c)]) => fmt2(0b100, a, b, c / 2)?,
            (St, &[Reg(a), Reg(b), Imm(c)]) => fmt2(0b101, a, b, c / 2)?,
            (Beq, &[Reg(a), Reg(b), Imm(c)]) => {
                fmt2(0b110, a, b, arith_shift_right(c.wrapping_sub(address)))?
            }
            (Jalr, &[Reg(a), Reg(b)]) => fmt2(0b111, a, b, 0)?,
            // pseudo-ops
            (Add, &[Reg(a), Imm(b)]) => fmt2(0b001, a, a, b)?,
            (Add, &[Reg(a), Reg(b)]) => fmt1(0b000, a, a, b),
            _ => Err(format!("invalid instruction: {opcode:?} {:?}", arguments))?,
        };
        Ok(instruction.to_be_bytes().to_vec())
    }

    fn step(&mut self) -> usize {
        use enums::Opcode::*;
        if usize::from(self.program_counter.0) >= self.program.len() {
            return 0;
        }
        let inst = self.program[usize::from(self.program_counter.0)];
        self.program_counter += 1;

        // decode instruction
        let opcode = enums::Opcode::try_from(inst >> 13).unwrap();
        let ra = usize::from(inst >> 10 & 0b111);
        let rb = usize::from(inst >> 7 & 0b111);
        let rc = usize::from(inst & 0b111);
        // sign-extend 7-bit immediate
        let imm = Wrapping((inst & 0x7f ^ 0x40).wrapping_sub(0x40));
        let addr = (self.registers[rb] + imm).0;

        match opcode {
            // add
            Add => self.registers[ra] = self.registers[rb] + self.registers[rc],
            // add immediate
            Addi => self.registers[ra] = self.registers[rb] + imm,
            // bitwise nand
            Nand => self.registers[ra] = !(self.registers[rb] & self.registers[rc]),
            // load upper immediate
            Lui => self.registers[ra] = Wrapping(inst << 6),
            // port load
            Ld if rb == 0 => {
                let port = Port::try_from(usize::from(addr)).expect("invalid port");
                self.registers[ra] = Wrapping(self.port_state.read_port(port));
            }
            // port store
            St if rb == 0 => {
                let port = Port::try_from(usize::from(addr)).expect("invalid port");
                self.port_state.write_port(port, self.registers[ra].0);
            }
            // memory load
            Ld => {
                self.registers[ra] = Wrapping(self.data[usize::from(addr)]);
            }
            // memory store
            St => {
                self.data[usize::from(addr)] = self.registers[ra].0;
            }
            // branch if equal
            Beq => {
                if self.registers[ra] == self.registers[rb] {
                    self.program_counter = self.program_counter + imm - Wrapping(1);
                }
            }
            // jump and link register
            Jalr => {
                self.registers[ra] = self.program_counter * Wrapping(2);
                self.program_counter = self.registers[rb] / Wrapping(2);
            }
        }
        self.registers[0] = Wrapping(0); // R0 is always 0
        usize::from(usize::from(self.program_counter.0) < self.program.len())
    }
}

fn main() {
    run::<Risc16>();
}
