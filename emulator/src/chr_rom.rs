use byteorder::ReadBytesExt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};
use std::path::Path;

pub struct ChrRom {
    rom: Vec<u8>,
}
impl ChrRom {
    pub fn new(size: usize) -> ChrRom {
        ChrRom {
            rom: Vec::with_capacity(size)
        }
    }
    pub fn new_with_data(data: Vec<u8>) -> ChrRom {
        ChrRom {
            rom: data
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.rom[address as usize] = data;
    }

}