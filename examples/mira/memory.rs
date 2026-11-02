use std::num::Wrapping;

use assemulator::{Port, State};

/// Addresses at and above this are mapped to ports.
const PORTS_START: u8 = 0b1111_0000;

#[derive(Default)]
pub struct MemoryMappedRam {
    data: Vec<u8>,
    ports: State<u8>,
}

impl MemoryMappedRam {
    pub fn read(&mut self, addr: u8) -> Wrapping<u8> {
        if addr >= PORTS_START {
            let port = Port::try_from(usize::from(addr & 0b1111)).unwrap();
            Wrapping(self.ports.read_port(port))
        } else {
            Wrapping(self.data[usize::from(addr)])
        }
    }

    pub fn write(&mut self, addr: u8, value: u8) {
        if addr >= PORTS_START {
            let port = Port::try_from(usize::from(addr & 0b1111)).unwrap();
            self.ports.write_port(port, value);
        } else {
            self.data[usize::from(addr)] = value;
        }
    }
}

impl From<&[u8]> for MemoryMappedRam {
    fn from(data: &[u8]) -> Self {
        // RAM fills the address space below the memory-mapped ports
        let mut ram = data.to_vec();
        ram.resize(ram.len().max(PORTS_START.into()), 0);
        Self {
            data: ram,
            ports: State::default(),
        }
    }
}
