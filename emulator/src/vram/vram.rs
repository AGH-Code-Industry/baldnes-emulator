use crate::addressing::Addressable;
use log::{debug, info};
use std::fmt::Debug;

pub struct VRAM {
    pub(crate) data: [u8; 0x2000],
}

impl VRAM {
    pub fn new() -> VRAM {
        info!("VRAM is initializing");
        VRAM { data: [0; 0x2000] }
    }
}

impl Addressable for VRAM {
    fn read(&mut self, addr: u16) -> u8 {
        debug!(
            "VRAM read: addr: {:#06X}, data: {:#04X}",
            addr,
            self.data[addr as usize - 0x2000]
        );
        self.data[addr as usize - 0x2000]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.data[addr as usize - 0x2000] = data;
        debug!("VRAM write: addr: {:#06X}, data: {:#04X}", addr, data);
    }
}

impl Debug for VRAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VRAM").finish()
    }
}
