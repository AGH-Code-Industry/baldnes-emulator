use std::fmt::Debug;

pub trait Addressable {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

pub struct AddressRange {
    pub start: u16,
    pub end: u16,
}

impl AddressRange {
    pub fn new(start: u16, end: u16) -> AddressRange {
        if start > end {
            panic!(
                "AddressRange: start > end: start = {}, end = {}",
                start, end
            );
        }

        AddressRange { start, end }
    }
}

impl Debug for AddressRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "AddressRange {{ start: 0x{:04X}, end: 0x{:04X} }}",
            self.start, self.end
        )
    }
}
