use std::cmp::PartialEq;
use crate::addressing::Addressable;
use log::{debug, info};
use std::fmt::Debug;
use crate::mirroring::Mirroring;

pub struct VRAM {
    nametable_1: [u8; 0x400],
    nametable_2: [u8; 0x400],
    mirroring: Mirroring
}


impl VRAM {
    pub fn new() -> VRAM {
        info!("VRAM is initializing");
        VRAM {
            nametable_1: [0; 0x400],
            nametable_2: [0; 0x400],
            mirroring: Mirroring::Horizontal
        }
    }

    fn read_from_nametable_1(&self, addr: u16) -> u8 {
        debug!("Nametable 1 read at relative address {:#06X}", addr);
        self.nametable_1[addr as usize]
    }

    fn read_from_nametable_2(&self, addr: u16) -> u8 {
        debug!("Nametable 2 read at relative address {:#06X}", addr);
        self.nametable_2[addr as usize]
    }

    fn read_from_nametable(&self, addr: u16) -> u8 {
        debug!("Attempt to read from VRAM at address {:#06X}", addr + 0x2000);
        if self.mirroring == Mirroring::Horizontal {
            match addr {
                0x0000..=0x03FF => self.read_from_nametable_1(addr),
                0x0400..=0x07FF => self.read_from_nametable_1(addr - 0x400),
                0x0800..=0x0BFF => self.read_from_nametable_2(addr - 0x800),
                0x0C00..=0x0FFF => self.read_from_nametable_2(addr - 0xC00),
                _ => panic!("Invalid VRAM address: {:#06X}", addr),
            }
        } else {
            match addr {
                0x0000..=0x03FF => self.read_from_nametable_1(addr),
                0x0400..=0x07FF => self.read_from_nametable_2(addr - 0x400),
                0x0800..=0x0BFF => self.read_from_nametable_1(addr - 0x800),
                0x0C00..=0x0FFF => self.read_from_nametable_2(addr - 0xC00),
                _ => panic!("Invalid VRAM address: {:#06X}", addr),
            }
        }
    }

    fn write_to_nametable_1(&mut self, addr: u16, value: u8) {
        debug!("Nametable 1 write at relative address {:#06X} with data {:#04X}", addr, value);
        self.nametable_1[addr as usize] = value;
    }

    fn write_to_nametable_2(&mut self, addr: u16, value: u8) {
        debug!("Nametable 2 write at relative address {:#06X} with data {:#04X}", addr, value);
        self.nametable_2[addr as usize] = value;
    }

    fn write_to_nametable(&mut self, addr: u16, value: u8) {
        debug!("Attempt to write to VRAM at address {:#06X} with data {:#04X}", addr + 0x2000, value);
        if self.mirroring == Mirroring::Horizontal {
            match addr {
                0x0000..=0x03FF => self.write_to_nametable_1(addr, value),
                0x0400..=0x07FF => self.write_to_nametable_1(addr - 0x400, value),
                0x0800..=0x0BFF => self.write_to_nametable_2(addr - 0x800, value),
                0x0C00..=0x0FFF => self.write_to_nametable_2(addr - 0xC00, value),
                _ => panic!("Invalid VRAM address: {:#06X}", addr),
            }
        }
        else {
            match addr {
                0x0000..=0x03FF => self.write_to_nametable_1(addr, value),
                0x0400..=0x07FF => self.write_to_nametable_2(addr - 0x400, value),
                0x0800..=0x0BFF => self.write_to_nametable_1(addr - 0x800, value),
                0x0C00..=0x0FFF => self.write_to_nametable_2(addr - 0xC00, value),
                _ => panic!("Invalid VRAM address: {:#06X}", addr),
            }
        }
    }

    pub fn set_mirroring(&mut self, mirroring: Mirroring) {
        self.mirroring = mirroring;
    }
}

impl Addressable for VRAM {
    fn read(&mut self, addr: u16) -> u8 {
        self.read_from_nametable(addr - 0x2000)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.write_to_nametable(addr - 0x2000, data);
    }
}

impl Debug for VRAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VRAM").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirroring::Mirroring;

    #[test]
    fn vram_initializes_correctly() {
        let vram = VRAM::new();
        assert_eq!(vram.nametable_1, [0; 0x400]);
        assert_eq!(vram.nametable_2, [0; 0x400]);
        assert_eq!(vram.mirroring, Mirroring::Horizontal);
    }

    #[test]
    fn read_from_nametable_1_within_bounds() {
        let vram = VRAM::new();
        assert_eq!(vram.read_from_nametable_1(0x0000), 0);
        assert_eq!(vram.read_from_nametable_1(0x03FF), 0);
    }

    #[test]
    #[should_panic(expected = "Invalid VRAM address: 0x1000")]
    fn read_from_nametable_out_of_bounds() {
        let vram = VRAM::new();
        vram.read_from_nametable(0x1000);
    }

    #[test]
    fn write_to_nametable_1_within_bounds() {
        let mut vram = VRAM::new();
        vram.write_to_nametable_1(0x0000, 42);
        assert_eq!(vram.read_from_nametable_1(0x0000), 42);
    }

    #[test]
    fn read_write_nametable_with_horizontal_mirroring() {
        let mut vram = VRAM::new();
        vram.write_to_nametable(0x0000, 42);
        assert_eq!(vram.read_from_nametable(0x0000), 42);
        vram.write_to_nametable(0x0400, 84);
        assert_eq!(vram.read_from_nametable(0x0400), 84);
    }

    #[test]
    fn read_write_nametable_with_vertical_mirroring() {
        let mut vram = VRAM::new();
        vram.set_mirroring(Mirroring::Vertical);
        vram.write_to_nametable(0x0000, 42);
        assert_eq!(vram.read_from_nametable(0x0000), 42);
        vram.write_to_nametable(0x0400, 84);
        assert_eq!(vram.read_from_nametable(0x0400), 84);
    }
}


