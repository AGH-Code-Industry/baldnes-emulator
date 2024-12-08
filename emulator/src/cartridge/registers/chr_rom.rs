use crate::addressing::Addressable;
use std::fmt::Debug;
pub struct ChrRom {
    rom: Vec<u8>,
}

impl Debug for ChrRom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChrRom").field("rom", &self.rom).finish()
    }
}

impl Addressable for ChrRom {
    fn read(&mut self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.rom[address as usize] = data;
    }
}

impl ChrRom {
    pub fn new(size: usize) -> ChrRom {
        ChrRom {
            rom: Vec::with_capacity(size),
        }
    }
    pub fn new_with_data(data: Vec<u8>) -> ChrRom {
        ChrRom { rom: data }
    }
    pub fn size(&self) -> usize {
        self.rom.len()
    }
}
