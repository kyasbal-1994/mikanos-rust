/*
* References:
* https://github.com/yubrot/ors/blob/main/ors-kernel/src/devices/pci.rs
* http://yuma.ohgami.jp/x86_64-Jisaku-OS-4/01_pci.html
* https://www.youtube.com/watch?v=GxLL_lbOPo4
 * PCI configuration space is accessible via CONFIG_ADDRESS(0x0cf8 - 0x0cfb) register and CONFIG_DATA(0x0cfc - 0x0cff)

 */



use bit_field::BitField;
use x86_64::instructions::port::{Port,PortWriteOnly};

use derive_new::new;

static mut CONFIG_ADDRESS: PortWriteOnly<u32> = PortWriteOnly::new(0x0cf8);
static mut CONFIG_DATA:Port<u32> = Port::new(0x0cfc);

fn vendor_id_to_str(vendor_id: u16) ->&'static str{
    match vendor_id {
        0x8086 => "Intel",
        0x1022 => "AMD",
        0x10DE => "NVIDIA Corporation",
        _ => "Unknown"
    }
}

#[derive(Debug,Clone,Copy)]
struct ConfigAddress(u32);

impl ConfigAddress {
    fn new(bus: u8, device: u8, function: u8, reg: u8) -> Self {
        // Address map figure
        // http://yuma.ohgami.jp/x86_64-Jisaku-OS-4/images/01_config_address.png
        let mut value:u32 = 0;
        value.set_bits(0..8, reg as u32);
        value.set_bits(8..11,function as u32);
        value.set_bits(11..16, device as u32);
        value.set_bits(16..24, bus as u32);
        value.set_bit(31,true);
        Self(value)
    }

    unsafe fn write(self){
        CONFIG_ADDRESS.write(self.0);
    }
}

#[derive(Debug,Clone,Copy)]
struct ConfigData(u32);

impl ConfigData {
    unsafe fn read() -> Self {
        ConfigData(CONFIG_DATA.read())
    }

    unsafe fn write(self) {
        CONFIG_DATA.write(self.0)
    }
}

#[derive(Debug,Clone, Copy,new)]
pub struct Device {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

impl Device {
    unsafe fn read(self, addr: u8) -> u32{
        ConfigAddress::new(self.bus,self.device,self.function,addr).write();
        ConfigData::read().0
    }

    unsafe fn write(self, addr: u8, value: u32) {
        ConfigAddress::new(self.bus, self.device, self.function, addr).write();
        ConfigData::write(ConfigData(value))
    }

    pub unsafe fn vendor_id(self) -> u16 {
        self.read(0x00) as u16
    }

    pub unsafe fn device_id(self) -> u16 {
        (self.read(0x00) >> 16) as u16
    }
}

