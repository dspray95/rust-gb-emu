extern crate byteorder;
mod header;
mod licensee_codes;
mod rom_types;

use byteorder::ReadBytesExt;
use rust_gbe::NO_IMPL;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use self::header::CartridgeHeader;

// #[derive(Default)]
pub struct Cartridge {
    pub rom_path: String,
    rom_data: Vec<u8>,
    pub rom_header: header::CartridgeHeader,
}

impl Cartridge {
    pub fn new(cartridge_filepath: Option<String>) -> Cartridge {
        if cartridge_filepath.is_none() {
            return Cartridge {
                rom_path: cartridge_filepath.unwrap_or(String::from("NO ROM FILEPATH GIVEN")),
                rom_data: Vec::new(),
                rom_header: CartridgeHeader::new(None),
            };
        } else {
            return Cartridge::load_cartridge(cartridge_filepath.unwrap());
        }
    }

    fn load_cartridge(filepath: String) -> Cartridge {
        println!("loading cartridge from {}...", filepath);

        let file = match File::open(&filepath) {
            Err(why) => panic!("couldn't open {}: {}", filepath, why),
            Ok(file) => file,
        };
        let file = BufReader::new(file);
        let rom_data = Cartridge::read_instructions_to_end(file).unwrap();
        let rom_header = CartridgeHeader::new(Some(&rom_data));
        println!("...done!");
        return Cartridge {
            rom_path: filepath,
            rom_data: rom_data,
            rom_header: rom_header,
        };
    }

    pub fn read(&mut self, address: u16) -> u8 {
        //only supports ROM ONLY for now...
        return self.rom_data[address as usize];
    }

    pub fn write(&mut self, address: u16, value: u8) {
        //only supports ROM ONLY for now...
        NO_IMPL!();
    }

    fn read_instructions_to_end<R>(mut rdr: R) -> io::Result<Vec<u8>>
    where
        R: Read,
    {
        let mut instructions = Vec::new();
        loop {
            match rdr.read_u8() {
                Ok(instruction) => instructions.push(instruction),
                Err(e) => {
                    return if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        Ok(instructions)
                    } else {
                        Err(e)
                    }
                }
            }
        }
    }
}
