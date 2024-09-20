use std::num::Wrapping;

use assemulator::{Port, State};



#[derive(Default)]
pub struct MemoryMappedRam {
    data: Vec<u8>,
    ports: State<u8>,
}

impl MemoryMappedRam {
    pub fn read(&mut self, addr: u8) -> Wrapping<u8> {
        if addr >= 0b11110000 {
            let port = Port::try_from(usize::from(addr & 0b1111)).unwrap();
            Wrapping(self.ports.read_port(port))
        } else {
            Wrapping(self.data[usize::from(addr)])
        }
    }

    pub fn write(&mut self, addr: u8, value: u8) {
        if addr >= 0b11110000 {
            let port = Port::try_from(usize::from(addr & 0b1111)).unwrap();
            self.ports.write_port(port, value);
        } else {
            self.data[usize::from(addr)] = value;
        }
    }
}

impl From<&[u8]> for MemoryMappedRam {
    fn from(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            ports: Default::default(),
        }
    }
}

