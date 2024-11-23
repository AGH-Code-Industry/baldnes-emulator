use crate::addressing::Addressable;

pub struct EmptyDevice;

// TODO: Should it behave differently
impl Addressable for EmptyDevice {
    fn read(&mut self, _address: u16) -> u8 {
        0
    }
    fn write(&mut self, _address: u16, _data: u8) {}
}
