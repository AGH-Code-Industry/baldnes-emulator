use crate::bus::{Bus, BusLike};

pub struct PPUData {
    ppu_bus: Bus,
}

impl PPUData {
    pub fn new(ppu_bus: Bus) -> PPUData {
        PPUData { ppu_bus }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.ppu_bus.read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.ppu_bus.write(address, value);
    }
}
