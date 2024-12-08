use crate::addressing::Addressable;
use std::fmt::Debug;

pub struct PrgRam {
    ram: Vec<u8>,
}

impl Debug for PrgRam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrgRam").field("ram", &self.ram).finish()
    }
}

impl Addressable for PrgRam {
    fn read(&mut self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
}

impl PrgRam {
    pub fn new(size: usize) -> PrgRam {
        PrgRam {
            ram: Vec::with_capacity(size),
        }
    }
    pub fn new_with_data(data: Vec<u8>) -> PrgRam {
        PrgRam { ram: data }
    }

    pub fn size(&self) -> usize {
        self.ram.len()
    }
}
