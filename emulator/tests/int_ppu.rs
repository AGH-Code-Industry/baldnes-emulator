#[cfg(test)]
mod tests {
    use emulator::addressing::Addressable;

    #[test]
    fn test_ppu_vram_write() {
        // emulator::logging::nes_logging::init_logging();
        let vram = emulator::ppu::vram::vram::VRAM::new();
        let mut ppu_bus = emulator::bus::Bus::new();
        ppu_bus.register(
            vram,
            emulator::addressing::AddressRange::new(0x2000, 0x3FFF),
        );

        let mut ppu = emulator::ppu::ppu::PPU::new(ppu_bus);
        ppu.write(*&0x2006, 0x23);
        ppu.write(*&0x2006, 0x06);
        ppu.write(*&0x2007, 0x66);

        ppu.write(*&0x2006, 0x23);
        ppu.write(*&0x2006, 0x06);

        let vram_data = ppu.read(*&0x2007);
        assert_eq!(vram_data, 0x00);
        let vram_data_valid = ppu.read(*&0x2007);
        assert_eq!(vram_data_valid, 0x66);
    }

    #[test]
    fn test_ppu_palette_ram_write() {
        // emulator::logging::nes_logging::init_logging();
        let palette_ram = emulator::ppu::palette_ram::palette_ram::PaletteRAM::new();
        let mut ppu_bus = emulator::bus::Bus::new();
        ppu_bus.register(
            palette_ram,
            emulator::addressing::AddressRange::new(0x3F00, 0x3FFF),
        );

        let mut ppu = emulator::ppu::ppu::PPU::new(ppu_bus);
        ppu.write(*&0x2006, 0x3F);
        ppu.write(*&0x2006, 0x2C);
        ppu.write(*&0x2007, 0b00101001);

        ppu.write(*&0x2006, 0x3F);
        ppu.write(*&0x2006, 0x2C);

        let color_index = ppu.read(*&0x2007);
        assert_eq!(color_index, 0x00);
        let color_index_valid = ppu.read(*&0x2007);
        assert_eq!(color_index_valid, 0b00101001);
    }
}
