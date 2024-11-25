pub trait Addressable {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

pub struct AddressRange {
    pub start: usize,
    pub end: usize,
}

impl AddressRange {
    pub fn new(start: usize, end: usize) -> AddressRange {
        if start > end {
            panic!(
                "AddressRange: start > end: start = {}, end = {}",
                start, end
            );
        }

        AddressRange { start, end }
    }
}
