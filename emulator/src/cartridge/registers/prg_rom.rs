use crate::addressing::Addressable;
use std::fmt::Debug;
pub struct PrgRom {
    rom: Vec<u8>,
}

impl Debug for PrgRom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrgRom").field("rom", &self.rom).finish()
    }
}

impl Addressable for PrgRom {
    fn read(&mut self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.rom[address as usize] = data;
    }
}

impl PrgRom {
    pub fn new(size: usize) -> PrgRom {
        PrgRom {
            rom: Vec::with_capacity(size),
        }
    }
    pub fn new_with_data(data: Vec<u8>) -> PrgRom {
        PrgRom { rom: data }
    }

    pub fn size(&self) -> usize {
        self.rom.len()
    }
}
