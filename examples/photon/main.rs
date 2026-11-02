mod enums;
use assemulator::{mask, run, Argument, InstructionSet, Port, Processor, State, Step};
use std::num::Wrapping;
type Word = u8;

#[derive(Default)]
pub struct Photon {
    pc: Wrapping<Word>,
    regs: [Wrapping<Word>; 8],
    x: bool,
    program: Vec<u16>,
    data: Vec<Word>,
    port_state: State<u8>,
}

impl Photon {
    fn load(&self, addr: Wrapping<Word>) -> Result<Wrapping<Word>, String> {
        let value = self.data.get(usize::from(addr.0));
        Ok(Wrapping(*value.ok_or_else(|| invalid_address(addr))?))
    }

    fn store(&mut self, addr: Wrapping<Word>, value: Wrapping<Word>) -> Result<(), String> {
        let slot = self.data.get_mut(usize::from(addr.0));
        *slot.ok_or_else(|| invalid_address(addr))? = value.0;
        Ok(())
    }
}

fn invalid_address(addr: Wrapping<Word>) -> String {
    format!("invalid data address {:#x}", addr.0)
}

impl InstructionSet for Photon {
    type Opcode = enums::Opcode;
    type Register = enums::Register;

    fn encode(
        _: u64,
        opcode: Self::Opcode,
        arguments: &[Argument<Self::Register>],
    ) -> Result<Vec<u8>, String> {
        use enums::Opcode::*;
        use Argument::{Imm, Mem, Reg};

        let inst: u16 = match (opcode, arguments) {
            (And, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b(),
            (Or, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b001,
            (Xor, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b010,
            (Mov, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b011,
            (Tst, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b100,
            (Eq, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b101,
            (Geq, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b110,
            (Ges, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b111,
            (And, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x800 | mask::<u16>(x, 8)?,
            (Or, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x900 | mask::<u16>(x, 8)?,
            (Xor, &[Reg(ra), Imm(x)]) => ra.to_a() | 0xa00 | mask::<u16>(x, 8)?,
            (Mov, &[Reg(ra), Imm(x)]) => ra.to_a() | 0xb00 | mask::<u16>(x, 8)?,
            (Tst, &[Reg(ra), Imm(x)]) => ra.to_a() | 0xc00 | mask::<u16>(x, 8)?,
            (Eq, &[Reg(ra), Imm(x)]) => ra.to_a() | 0xd00 | mask::<u16>(x, 8)?,
            (Geq, &[Reg(ra), Imm(x)]) => ra.to_a() | 0xe00 | mask::<u16>(x, 8)?,
            (Ges, &[Reg(ra), Imm(x)]) => ra.to_a() | 0xf00 | mask::<u16>(x, 8)?,
            (Add, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01000,
            (Addc, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01001,
            (Addx, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01010,
            (Addb, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01011,
            (Sub, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01100,
            (Subc, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01101,
            (Subx, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01110,
            (Subb, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b01111,
            (Shl, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10000,
            (Shlc, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10001,
            (Shlx, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10010,
            (Shlb, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10011,
            (Shr, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10100,
            (Shrc, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10101,
            (Shrx, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10110,
            (Shrb, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b10111,
            (Mvt, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11000,
            (Mvf, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11001,
            (Cad, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11010,
            (Csb, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11011,
            (Neg, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11100,
            (Swap, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11101,
            (Psh, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11110,
            (Pop, &[Reg(ra), Reg(rb)]) => ra.to_a() | rb.to_b() | 0b11111,

            (Bt, &[Imm(x)]) => 0x1000 | mask::<u16>(x / 2, 8)?,
            (Bf, &[Imm(x)]) => 0x1100 | mask::<u16>(x / 2, 8)?,
            (Jmp, &[Imm(x)]) => 0x1200 | mask::<u16>(x / 2, 8)?,
            (Jsr, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1300 | mask::<u16>(x / 2, 8)?,
            (Bt, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1400 | mask::<u16>(x / 2, 8)?,
            (Bf, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1500 | mask::<u16>(x / 2, 8)?,
            (Jmp, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1600 | mask::<u16>(x / 2, 8)?,
            (Btd, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1700 | mask::<u16>(x / 2, 8)?,
            (Ld, &[Reg(ra), Mem { base: None, offset }]) => {
                ra.to_a() | 0x1800 | mask::<u16>(offset, 8)?
            }
            (St, &[Reg(ra), Mem { base: None, offset }]) => {
                ra.to_a() | 0x1900 | mask::<u16>(offset, 8)?
            }
            (Pld, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1a00 | mask::<u16>(x, 8)?,
            (Pst, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1b00 | mask::<u16>(x, 8)?,
            (
                Ld,
                &[Reg(ra), Mem {
                    base: Some(rb),
                    offset,
                }],
            ) => ra.to_a() | 0x1c00 | rb.to_b() | mask::<u16>(offset, 5)?,
            (
                St,
                &[Reg(ra), Mem {
                    base: Some(rb),
                    offset,
                }],
            ) => ra.to_a() | 0x1d00 | rb.to_b() | mask::<u16>(offset, 5)?,
            (Add, &[Reg(ra), Reg(rb), Imm(x)]) => {
                ra.to_a() | 0x1e00 | rb.to_b() | mask::<u16>(x, 5)?
            }
            (Jsr, &[Imm(x)]) => 0xf300 | mask::<u16>(x / 2, 8)?,
            (Ret, &[]) => 0xf600,
            (Add, &[Reg(ra), Imm(x)]) => ra.to_a() | 0x1e00 | ra.to_b() | mask::<u16>(x, 5)?,
            (Shl, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10000,
            (Shlc, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10001,
            (Shlx, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10010,
            (Shlb, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10011,
            (Shr, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10100,
            (Shrc, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10101,
            (Shrx, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10110,
            (Shrb, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b10111,
            (Neg, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b11100,
            (Swap, &[Reg(ra)]) => ra.to_a() | ra.to_b() | 0b11101,
            _ => Err("Invalid instruction".to_owned())?,
        };
        Ok(inst.to_be_bytes().to_vec())
    }
}

impl Processor for Photon {
    fn debug(&self) {
        println!(
            "pc: {:02x}, regs: {:?}, x: {}, data: {:?}",
            self.pc.0, self.regs, self.x, self.data
        );
    }

    fn new(addr: u64, program: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            pc: Wrapping((addr / 2) as Word),
            program: program
                .chunks_exact(2)
                .map(|x| u16::from_be_bytes([x[0], x[1]]))
                .collect(),
            data,
            ..Default::default()
        }
    }
    fn step(&mut self) -> Result<Step, String> {
        let Some(&inst) = self.program.get(usize::from(self.pc.0)) else {
            return Ok(Step::Halt);
        };
        self.pc += 1;

        let ra = usize::from(inst.wrapping_shr(13) & 0b111);
        let rb = usize::from(inst.wrapping_shr(5) & 0b111);

        let z = if inst & 0x1c00 == 0x1c00 {
            // 5 bit sign extension
            (inst & 0x1f ^ 0x10).wrapping_sub(0x10)
        } else {
            inst & 0xff
        };

        let imm = Wrapping(z as u8);

        if inst.wrapping_shr(8) & 0b11111 == 0 {
            match inst & 0b11111 {
                0x00 => self.regs[ra] &= self.regs[rb],
                0x01 => self.regs[ra] |= self.regs[rb],
                0x02 => self.regs[ra] ^= self.regs[rb],
                0x03 => self.regs[ra] = self.regs[rb],
                0x04 => self.x = self.regs[ra] & self.regs[rb] != Wrapping(0),
                0x05 => self.x = self.regs[ra] == self.regs[rb],
                0x06 => self.x = self.regs[ra] >= self.regs[rb],
                0x07 => self.x = (self.regs[ra].0 as i8) >= (self.regs[rb].0 as i8),
                // add: bit 0 adds the carry in, bit 1 writes the carry out
                op @ 0x08..=0x0b => {
                    let carry_in = u16::from(op & 1 == 1 && self.x);
                    let sum = u16::from(self.regs[ra].0) + u16::from(self.regs[rb].0) + carry_in;
                    self.regs[ra] = Wrapping(sum.to_le_bytes()[0]);
                    if op & 2 != 0 {
                        self.x = sum > 0xff;
                    }
                }
                // sub: bit 0 subtracts the borrow in, bit 1 writes the borrow out
                op @ 0x0c..=0x0f => {
                    let borrow_in = u16::from(op & 1 == 1 && self.x);
                    let subtrahend = u16::from(self.regs[rb].0) + borrow_in;
                    let a = u16::from(self.regs[ra].0);
                    self.regs[ra] = Wrapping(a.wrapping_sub(subtrahend).to_le_bytes()[0]);
                    if op & 2 != 0 {
                        self.x = a < subtrahend;
                    }
                }
                // shl
                0x10 => self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1)),
                // shlc
                0x11 => {
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1) | u8::from(self.x));
                }
                // shlx
                0x12 => {
                    self.x = self.regs[rb].0 & 0x80 != 0;
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1));
                }
                // shlb
                0x13 => {
                    let tmp = self.regs[rb].0 & 0x80 != 0;
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shl(1) | u8::from(self.x));
                    self.x = tmp;
                }
                // shr
                0x14 => self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shr(1)),
                // shrc
                0x15 => {
                    self.regs[ra] = Wrapping(
                        self.regs[rb].0.wrapping_shr(1) | u8::from(self.x).wrapping_shl(7),
                    );
                }
                // shrx
                0x16 => {
                    self.x = self.regs[rb].0 & 0x01 != 0;
                    self.regs[ra] = Wrapping(self.regs[rb].0.wrapping_shr(1));
                }
                // shrb
                0x17 => {
                    let tmp = self.regs[rb].0 & 0x01 != 0;
                    self.regs[ra] = Wrapping(
                        self.regs[rb].0.wrapping_shr(1) | u8::from(self.x).wrapping_shl(7),
                    );
                    self.x = tmp;
                }
                0x18 => {
                    if self.x {
                        self.regs[ra] = self.regs[rb];
                    }
                }
                0x19 => {
                    if !self.x {
                        self.regs[ra] = self.regs[rb];
                    }
                }
                // cad
                0x1a => {
                    if self.x {
                        self.regs[ra] += self.regs[rb];
                    }
                }
                // csb
                0x1b => {
                    if self.x {
                        self.regs[ra] -= self.regs[rb];
                    }
                }
                0x1c => self.regs[ra] = -self.regs[rb],
                // swap nibbles
                0x1d => self.regs[ra] = Wrapping(self.regs[rb].0.rotate_left(4)),

                // push
                0x1e => {
                    self.regs[rb] -= Wrapping(1);
                    self.store(self.regs[rb], self.regs[ra])?;
                }
                // pop
                0x1f => {
                    self.regs[ra] = self.load(self.regs[rb])?;
                    self.regs[rb] += Wrapping(1);
                }
                _ => unreachable!(),
            }
        } else if inst.wrapping_shr(11) & 0b11 == 0b01 {
            match inst.wrapping_shr(8) & 0b111 {
                0b000 => self.regs[ra] &= imm,
                0b001 => self.regs[ra] |= imm,
                0b010 => self.regs[ra] ^= imm,
                0b011 => self.regs[ra] = imm,
                0b100 => self.x = self.regs[ra] & imm != Wrapping(0),
                0b101 => self.x = self.regs[ra] == imm,
                0b110 => self.x = self.regs[ra] >= imm,
                0b111 => self.x = (self.regs[ra].0 as i8) >= (imm.0 as i8),
                _ => unreachable!(),
            }
        } else {
            match inst.wrapping_shr(8) & 0x1f {
                // branch true flag
                0b10000 => {
                    if self.x {
                        self.pc = imm;
                    }
                }
                // branch false flag
                0b10001 => {
                    if !self.x {
                        self.pc = imm;
                    }
                }
                // jump to imm
                0b10010 => {
                    self.pc = imm;
                }
                // jump subroutine
                0b10011 => {
                    self.regs[ra] = self.pc;
                    self.pc = imm;
                }
                // branch true reg
                0b10100 => {
                    if self.regs[ra].0 != 0 {
                        self.pc = imm;
                    }
                }
                // branch false reg
                0b10101 => {
                    if self.regs[ra].0 == 0 {
                        self.pc = imm;
                    }
                }
                // jump register
                0b10110 => {
                    self.pc = self.regs[ra] + imm;
                }
                // branch if true and decrement
                0b10111 => {
                    if self.regs[ra].0 != 0 {
                        self.regs[ra] -= Wrapping(1);
                        self.pc = imm;
                    }
                }
                // load direct
                0b11000 => self.regs[ra] = self.load(imm)?,
                // store direct
                0b11001 => self.store(imm, self.regs[ra])?,
                // port load
                0b11010 => {
                    let port = Port::try_from(usize::from(imm.0))?;
                    self.regs[ra] = Wrapping(self.port_state.read_port(port));
                }
                // port store
                0b11011 => {
                    let port = Port::try_from(usize::from(imm.0))?;
                    self.port_state.write_port(port, self.regs[ra].0)?;
                }
                // load offset
                0b11100 => self.regs[ra] = self.load(self.regs[rb] + imm)?,
                // store offset
                0b11101 => self.store(self.regs[rb] + imm, self.regs[ra])?,
                // add imm
                0b11110 => {
                    self.regs[ra] = self.regs[rb] + imm;
                }
                // halt
                0b11111 => return Ok(Step::Halt),
                _ => return Err(format!("invalid instruction {inst:#06x}")),
            }
        }

        Ok(Step::Continue)
    }
}

fn main() {
    run::<Photon>();
}
