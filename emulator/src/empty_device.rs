use crate::addressing::Addressable;

pub struct EmptyDevice;

// TODO: Should it behave differently
impl Addressable for EmptyDevice {
    fn read(&self, address: u16) -> u8 { 0 }
    fn write(&mut self, address: u16, data: u8) {}
}
