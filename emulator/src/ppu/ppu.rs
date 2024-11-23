use log::{debug, info};
use std::fmt::Debug;

use crate::ppu::registers::ppu_addr::PPUAddr;
use crate::ppu::registers::ppu_ctrl::PPUCtrl;
use crate::ppu::registers::ppu_data::PPUData;
use crate::addressing::Addressable;
use crate::bus::Bus;

const MIRRORS_START_ADDRESS: u16 = 0x2008;
const MIRRORS_END_ADDRESS: u16 = 0x3FFF;

pub struct PPU {
    ppu_addr: PPUAddr,
    ppu_data: PPUData,
    ppu_ctrl: PPUCtrl,
    internal_read_buffer: u8,
    internal_w_register: bool,
}

impl PPU {
    pub fn new(ppu_bus: Bus) -> PPU {
        info!("PPU is initializing");
        PPU {
            ppu_addr: PPUAddr::new(),
            ppu_data: PPUData::new(ppu_bus),
            ppu_ctrl: PPUCtrl::new(),
            internal_read_buffer: 0,
            internal_w_register: true,
        }
    }

    // Read operations -----------------------------------------------------------------------------

    fn read_from_ppu_status(&mut self) -> u8 {
        todo!()
    }

    fn read_from_oam_data(&mut self) -> u8 {
        todo!()
    }

    fn read_from_ppu_data(&mut self) -> u8 {
        let addr = self.ppu_addr.read();
        debug!("PPU read from bus at address {:#06X}", addr);
        self.increment_addr();

        let current_buffer = self.internal_read_buffer;
        let result = self.ppu_data.read(addr);
        self.set_internal_read_buffer(result);
        current_buffer
    }

    // Write operations ----------------------------------------------------------------------------

    fn write_to_ppu_ctrl(&mut self, data: u8) {
        self.ppu_ctrl.write(data);
    }

    fn write_to_ppu_mask(&mut self, _data: u8) {
        todo!()
    }

    fn write_to_oam_addr(&mut self, _data: u8) {
        todo!()
    }

    fn write_to_oam_data(&mut self, _data: u8) {
        todo!()
    }

    fn write_to_ppu_scroll(&mut self, _data: u8) {
        todo!()
    }

    fn write_to_ppu_addr(&mut self, data: u8) {
        self.ppu_addr.write(data, self.internal_w_register);
        self.invert_w_register();
    }

    fn write_to_ppu_data(&mut self, data: u8) {
        let addr = self.ppu_addr.read();
        debug!(
            "PPU write to bus at address {:#06X} with data {:#04X}",
            addr, data
        );
        self.ppu_data.write(addr, data);
    }

    // Utility functions ---------------------------------------------------------------------------

    fn increment_addr(&mut self) {
        self.ppu_addr.increment(self.ppu_ctrl.get_vram_increment());
    }

    fn invert_w_register(&mut self) {
        self.internal_w_register = !self.internal_w_register;
    }

    fn mirror_write(&mut self, address: u16, data: u8) {
        let mirrored_address = address & 0x2007;
        self.write(mirrored_address, data);
    }

    fn mirror_read(&mut self, address: u16) -> u8 {
        let mirrored_address = address & 0x2007;
        self.read(mirrored_address)
    }

    fn set_internal_read_buffer(&mut self, data: u8) {
        self.internal_read_buffer = data;
    }
}

impl Addressable for PPU {
    fn read(&mut self, address: u16) -> u8 {
        debug!("PPU read at address {:#06X}", address);
        match address {
            0x2002 => self.read_from_ppu_status(),
            0x2004 => self.read_from_oam_data(),
            0x2007 => self.read_from_ppu_data(),
            MIRRORS_START_ADDRESS..=MIRRORS_END_ADDRESS => self.mirror_read(address),
            _ => {
                panic!("PPU read at address {:#06X} not implemented", address);
            }
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        debug!(
            "PPU write at address {:#06X} with data {:#04X}",
            address, data
        );
        match address {
            0x2000 => self.write_to_ppu_ctrl(data),
            0x2001 => self.write_to_ppu_mask(data),
            0x2003 => self.write_to_oam_addr(data),
            0x2004 => self.write_to_oam_data(data),
            0x2005 => self.write_to_ppu_scroll(data),
            0x2006 => self.write_to_ppu_addr(data),
            0x2007 => self.write_to_ppu_data(data),
            MIRRORS_START_ADDRESS..=MIRRORS_END_ADDRESS => self.mirror_write(address, data),
            0x4014 => {
                todo!()
            }
            _ => {
                panic!("PPU write at address {:#06X} not implemented", address);
            }
        }
    }
}

impl Debug for PPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PPU")
            .field("data_buffer", &self.internal_read_buffer)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::Bus;

    fn setup_ppu() -> PPU {
        let bus = Bus::new();
        PPU::new(bus)
    }

    #[test]
    fn ppu_initialization() {
        let ppu = setup_ppu();

        assert_eq!(ppu.internal_read_buffer, 0);
        assert!(ppu.internal_w_register);
    }

    #[test]
    fn ppu_write_to_ppu_ctrl() {
        let mut ppu = setup_ppu();

        ppu.write_to_ppu_ctrl(0b10000001);

        assert_eq!(ppu.ppu_ctrl.read(), 0b10000001);
    }

    #[test]
    fn ppu_write_to_ppu_addr() {
        let mut ppu = setup_ppu();

        ppu.write_to_ppu_addr(0x21);
        assert_eq!(ppu.ppu_addr.read(), 0x2100);

        ppu.write_to_ppu_addr(0x37);
        assert_eq!(ppu.ppu_addr.read(), 0x2137);
    }

    #[test]
    fn ppu_read_from_bus_returns_internal_buffer() {
        let mut ppu = setup_ppu();
        let internal_buffer = 0x69;

        ppu.set_internal_read_buffer(internal_buffer);
        ppu.ppu_addr.write(0x20, true);
        ppu.ppu_data.write(0x2000, 0xAB);
        let result = ppu.read_from_ppu_data();

        assert_eq!(result, internal_buffer);
    }

    #[test]
    fn ppu_increment_addr_by_one_on_default_ppu_ctrl_mode() {
        let mut ppu = setup_ppu();

        ppu.ppu_addr.write(0x21, true);
        ppu.ppu_addr.write(0x36, false);
        ppu.increment_addr();

        assert_eq!(ppu.ppu_addr.read(), 0x2137);
    }

    #[test]
    fn ppu_increment_addr_by_32_on_toggled_increment_mode() {
        let mut ppu = setup_ppu();

        ppu.ppu_addr.write(0x21, true);
        ppu.ppu_addr.write(0x17, false);
        ppu.ppu_ctrl.write(0b00000100);
        ppu.increment_addr();

        assert_eq!(ppu.ppu_addr.read(), 0x2137);
    }

    #[test]
    fn ppu_mirror_write_to_ppu_addr() {
        let mut ppu = setup_ppu();
        assert_eq!(ppu.ppu_addr.read(), 0x0000);
    }

    #[test]
    fn ppu_mirror_read_from_bus() {
        let mut ppu = setup_ppu();
        let internal_buffer = 0x69;

        ppu.set_internal_read_buffer(internal_buffer);
        let result = ppu.read(0x2247);

        assert_eq!(result, internal_buffer);
    }

    #[test]
    #[should_panic(expected = "PPU read at address 0x2003 not implemented")]
    fn ppu_read_unimplemented_address() {
        let mut ppu = setup_ppu();
        ppu.read(0x2003);
    }

    #[test]
    #[should_panic(expected = "PPU write at address 0x4001 not implemented")]
    fn ppu_write_unimplemented_address() {
        let mut ppu = setup_ppu();
        ppu.write(0x4001, 0xFF);
    }
}
