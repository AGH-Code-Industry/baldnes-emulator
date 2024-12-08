use crate::addressing::Addressable;
use std::fmt::Debug;

pub struct ChrRam {
    ram: Vec<u8>,
}

impl Debug for ChrRam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChrRam").field("ram", &self.ram).finish()
    }
}

impl Addressable for ChrRam {
    fn read(&mut self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
}

impl ChrRam {
    pub fn new(size: usize) -> ChrRam {
        ChrRam {
            ram: Vec::with_capacity(size),
        }
    }
    pub fn new_with_data(data: Vec<u8>) -> ChrRam {
        ChrRam { ram: data }
    }
    pub fn size(&self) -> usize {
        self.ram.len()
    }
}
