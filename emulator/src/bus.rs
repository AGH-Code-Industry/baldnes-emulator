use crate::addressing::{AddressRange, Addressable};
use crate::empty_device::EmptyDevice;

const ADDRESS_SPACE: usize = 0xFFFF + 1;

pub struct Bus {
    mappings: [usize; ADDRESS_SPACE],
    devices: Vec<Box<dyn Addressable>>,
}

impl Bus {
    pub fn new() -> Self {
        let empty_device = EmptyDevice {};
        Bus {
            mappings: [0; ADDRESS_SPACE],
            devices: vec![Box::new(empty_device)],
        }
    }

    pub fn register<A: Addressable + 'static>(
        &mut self,
        addressable: A,
        address_range: AddressRange,
    ) {
        self.devices.push(Box::new(addressable));
        self.mappings[address_range.start..=address_range.end].fill(self.devices.len() - 1);
    }

    pub fn read(&self, address: usize) -> u8 {
        let device = self.devices[self.mappings[address]].as_ref();
        device.read(address as u16)
    }

    pub fn write(&mut self, address: usize, data: u8) {
        let device = self.devices[self.mappings[address]].as_mut();
        device.write(address as u16, data);
    }
}
