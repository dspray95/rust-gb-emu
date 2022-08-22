use rust_gbe::NO_IMPL;

use super::cartridge::{self, Cartridge};

pub struct Bus {
    cartridge: Cartridge,
}

impl Bus {
    fn new(cartridge: Option<Cartridge>) -> Bus {
        return Bus {
            cartridge: Cartridge::new(),
        };
    }

    fn read(&mut self, address: u16) -> u8 {
        if address < 0x8000 {
            return self.cartridge.read(address);
        }

        NO_IMPL!();
    }

    fn write(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            self.cartridge.write(address, value);
            return;
        }

        NO_IMPL!();
    }
}
