use bitflags::bitflags;

bitflags! {
    // Documentation taken from https://www.nesdev.org/wiki/PPU_registers

    pub struct PPUCtrl: u8 {
        const NAMETABLE_BIT_1 = 0b00000001;     // Base nametable address, two bits
        const NAMETABLE_BIT_2 = 0b00000010;     // 0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00
        const INCREMENT_MODE = 0b00000100;      // 0: add 1, going across; 1: add 32, going down; VRAM address increment
        const PATTERN_SPRITE = 0b00001000;      // 0: $0000; 1: $1000; Pattern table address for 8x8 sprites
        const PATTERN_BACKGROUND = 0b00010000;  // 0: $0000; 1: $1000; Background pattern table address
        const SPRITE_SIZE = 0b00100000;         // 0: 8x8 pixels; 1: 8x16 pixels
        const MASTER_SLAVE = 0b01000000;        // 0: read backdrop from EXT pins; 1: output color on EXT pins
        const NMI = 0b10000000;                 // 0: NMI off, 1: NMI on
    }
}

impl PPUCtrl {
    pub fn new() -> PPUCtrl {
        PPUCtrl::from_bits_truncate(0)
    }

    pub fn get_vram_increment(&self) -> u8 {
        if self.contains(PPUCtrl::INCREMENT_MODE) {
            32
        } else {
            1
        }
    }

    pub fn write(&mut self, data: u8) {
        *self = PPUCtrl::from_bits_truncate(data);
    }

    #[cfg(test)]
    pub fn read(&self) -> u8 {
        self.bits()
    }
}
