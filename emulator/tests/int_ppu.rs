#[cfg(test)]
mod tests {
    use emulator::addressing::Addressable;

    #[test]
    fn test_ppu_vram_write() {
        emulator::logging::nes_logging::init_logging();
        let vram = emulator::vram::vram::VRAM::new();
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
        let vram_data = ppu.read(*&0x2007);
        assert_eq!(vram_data, 0x66);
    }
}
