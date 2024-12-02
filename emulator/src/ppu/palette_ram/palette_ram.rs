use std::fmt::Debug;
use log::{debug, info};
use crate::addressing::Addressable;

pub static SYSTEM_PALETTE: [(u8, u8, u8); 64] = [
    (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
    (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
    (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
    (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
    (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
    (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
    (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
    (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
    (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
    (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
    (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
    (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
    (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];

enum PaletteType {
    Background,
    Sprite
}

impl Debug for PaletteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaletteType::Background => write!(f, "PaletteType::Background"),
            PaletteType::Sprite => write!(f, "PaletteType::Sprite")
        }
    }
}

struct Palette {
    index: u8,
    background_entries: [u8; 4],
    sprite_entries: [u8; 4]
}

impl Palette {
    pub fn new(index: u8) -> Self {
        Palette {
            index,
            background_entries: [0; 4],
            sprite_entries: [0; 4]
        }
    }

    pub fn set_palette(&mut self, palette_type: PaletteType, palette_index: u8, value: u8) {
        debug!("[Palette #{}] Setting palette entry for type {:?} at index {} to value: {:#4X}", self.index, palette_type, palette_index, value);
        match palette_type {
            PaletteType::Background => self.background_entries[palette_index as usize] = value,
            PaletteType::Sprite => self.sprite_entries[palette_index as usize] = value
        }
    }

    pub fn get_palette(&self, palette_type: PaletteType, palette_index: u8) -> u8 {
        debug!("[Palette #{}] Getting palette entry for type {:?} at index {}", self.index, palette_type, palette_index);
        match palette_type {
            PaletteType::Background => self.background_entries[palette_index as usize],
            PaletteType::Sprite => self.sprite_entries[palette_index as usize]
        }
    }
}

pub struct PaletteRAM {
    palettes: [Palette; 4]
}

impl PaletteRAM {
    pub fn new() -> Self {
        info!("PaletteRAM is initializing");
        PaletteRAM {
            palettes: [Palette::new(0), Palette::new(1), Palette::new(2), Palette::new(3)]
        }
    }

    fn read_from_palette(&self, address: u16) -> u8 {
        let palette_type = match address {
            0x3F00..=0x3F0F => PaletteType::Background,
            0x3F10..=0x3F1F => PaletteType::Sprite,
            _ => unreachable!()
        };

        let index_in_palette = ((address & 0x0F) % 4) as u8;
        let index = ((address & 0x0F) >> 4) as usize;

        self.palettes[index].get_palette(palette_type, index_in_palette)
    }

    fn write_to_palette(&mut self, address: u16, data: u8) {
        let palette_type = match address {
            0x3F00..=0x3F0F => PaletteType::Background,
            0x3F10..=0x3F1F => PaletteType::Sprite,
            _ => unreachable!()
        };

        let index_in_palette = ((address & 0x0F) % 4) as u8;
        let index = ((address & 0x0F) >> 4) as usize;

        self.palettes[index].set_palette(palette_type, index_in_palette, data);
    }

    fn mirror_address(&self, address: u16) -> u16 {
        // Reduces the address to the range 0x3F00 - 0x3F1F
        debug!("Mirroring address: {:#6X} down to {:#6X}", address, 0x3F00 + (address & 0x1F));
        0x3F00 + (address & 0x1F)
    }
}

// Source for PPU Palette Reference can be found here: https://www.nesdev.org/wiki/PPU_palettes

impl Addressable for PaletteRAM {
    fn read(&mut self, address: u16) -> u8 {
        debug!("Reading from palette address: {:#6X}", address);
        match address {
            0x3F00..=0x3F1F => self.read_from_palette(address),
            0x3F20..=0x3FFF => self.read_from_palette(self.mirror_address(address)),
            _ => panic!("Invalid palette address: {:#6X}", address)
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        debug!("Reading from palette address: {:#6X}", address);
        match address {
            0x3F00..=0x3F1F => self.write_to_palette(address, data),
            0x3F20..=0x3FFF => self.write_to_palette(self.mirror_address(address), data),
            _ => panic!("Invalid palette address: {:#6X}", address)
        }
    }
}

impl Debug for PaletteRAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PaletteRAM")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_initializes_correctly() {
        let palette = Palette::new(0);
        assert_eq!(palette.background_entries, [0; 4]);
        assert_eq!(palette.sprite_entries, [0; 4]);
    }

    #[test]
    fn set_and_get_background_palette_entry() {
        let mut palette = Palette::new(0);
        palette.set_palette(PaletteType::Background, 2, 0x3F);
        assert_eq!(palette.get_palette(PaletteType::Background, 2), 0x3F);
    }

    #[test]
    fn set_and_get_sprite_palette_entry() {
        let mut palette = Palette::new(0);
        palette.set_palette(PaletteType::Sprite, 1, 0x1F);
        assert_eq!(palette.get_palette(PaletteType::Sprite, 1), 0x1F);
    }

    #[test]
    fn palette_ram_initializes_correctly() {
        let palette_ram = PaletteRAM::new();
        for palette in &palette_ram.palettes {
            assert_eq!(palette.background_entries, [0; 4]);
            assert_eq!(palette.sprite_entries, [0; 4]);
        }
    }

    #[test]
    fn read_write_palette_ram_within_bounds() {
        let mut palette_ram = PaletteRAM::new();
        palette_ram.write(0x3F00, 0x12);
        assert_eq!(palette_ram.read(0x3F00), 0x12);
    }

    #[test]
    fn read_write_palette_ram_mirrored_address() {
        let mut palette_ram = PaletteRAM::new();
        palette_ram.write(0x3F20, 0x34);
        assert_eq!(palette_ram.read(0x3F00), 0x34);
    }

    #[test]
    #[should_panic(expected = "Invalid palette address: 0x4000")]
    fn read_palette_ram_out_of_bounds() {
        let mut palette_ram = PaletteRAM::new();
        palette_ram.read(0x4000);
    }

    #[test]
    #[should_panic(expected = "Invalid palette address: 0x4000")]
    fn write_palette_ram_out_of_bounds() {
        let mut palette_ram = PaletteRAM::new();
        palette_ram.write(0x4000, 0x56);
    }
}