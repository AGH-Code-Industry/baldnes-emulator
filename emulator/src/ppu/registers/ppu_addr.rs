use log::debug;

pub struct PPUAddr {
    pub high_addr: u8,
    pub low_addr: u8,
}

impl PPUAddr {
    pub fn new() -> Self {
        PPUAddr {
            high_addr: 0x00,
            low_addr: 0x00,
        }
    }

    pub fn read(&self) -> u16 {
        ((self.high_addr as u16) << 8) | self.low_addr as u16
    }

    pub fn write(&mut self, data: u8, byte_flag: bool) {
        debug!("Writing to PPUAddr: {:#04X}", data);
        if byte_flag {
            self.high_addr = data;
        } else {
            self.low_addr = data;
        }

        if self.read() > 0x3FFF {
            self.mirror_address()
        }

        debug!("Current PPUAddr: {:#06X}", self.read());
    }

    pub fn increment(&mut self, increment: u8) {
        let current_low = self.low_addr;
        self.low_addr = self.low_addr.wrapping_add(increment);

        if current_low > self.low_addr {
            // That means we have overflowed
            self.high_addr = self.high_addr.wrapping_add(1);
        }

        if self.read() > 0x3FFF {
            self.mirror_address();
        }
    }

    fn mirror_address(&mut self) {
        let mirrored_data = self.read() & 0x3FFF;
        self.high_addr = (mirrored_data >> 8) as u8;
        self.low_addr = (mirrored_data & 0xFF) as u8;
    }
}
