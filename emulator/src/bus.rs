use crate::addressing::{AddressRange, Addressable};
use crate::empty_device::EmptyDevice;
use log::{debug, info};
use std::fmt::Debug;

const ADDRESS_SPACE: usize = 0xFFFF + 1;

pub struct Bus {
    mappings: Vec<usize>,
    devices: Vec<Box<dyn Addressable>>,
}

impl Bus {
    pub fn new() -> Self {
        info!("New Bus has been created");
        let empty_device = EmptyDevice {};
        Bus {
            mappings: vec![0; ADDRESS_SPACE],
            devices: vec![Box::new(empty_device)],
        }
    }

    pub fn register<A: Addressable + Debug + 'static>(
        &mut self,
        addressable: A,
        address_range: AddressRange,
    ) {
        debug!(
            "Registering device at address range: {:?} with device: {:?}",
            address_range, addressable
        );
        self.devices.push(Box::new(addressable));
        self.mappings[address_range.start as usize..=address_range.end as usize]
            .fill(self.devices.len() - 1);
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let device = self.devices[self.mappings[address as usize] as usize].as_mut();
        device.read(address)
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let device = self.devices[self.mappings[address as usize] as usize].as_mut();
        device.write(address, data);
    }
}
