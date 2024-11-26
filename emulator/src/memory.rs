use crate::addressing::Addressable;

pub struct Memory {
    mem: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            mem: Vec::with_capacity(size),
        }
    }
}

impl Addressable for Memory {
    fn read(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.mem[address as usize] = data;
    }
}
