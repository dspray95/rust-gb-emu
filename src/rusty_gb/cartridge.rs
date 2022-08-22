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
    rom_header: header::CartridgeHeader,
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

        let mut file = match File::open(&filepath) {
            Err(why) => panic!("couldn't open {}: {}", filepath, why),
            Ok(file) => file,
        };
        let mut file = BufReader::new(file);
        let rom_data = Cartridge::read_instructions_to_end(file).unwrap();
        let rom_header = CartridgeHeader::new(Some(rom_data));
        return Cartridge {
            rom_path: filepath,
            rom_data: rom_data,
            rom_header: rom_header,
        };
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        println!("loading cartridge from {}...", self.rom_path);
        let f = File::open(&self.rom_path)?;
        let mut f = BufReader::new(f);
        f.read_u8()?;

        self.rom_data = Cartridge::read_instructions_to_end(f)?;
        self.rom_header.load_header(&self.rom_data);

        Ok(())
    }

    pub fn read(&mut self, address: u16) -> u8 {
        //only supports ROM ONLY for now...
        return self.rom_data[address as usize];
    }

    pub fn write(&mut self, address: u16, value: u8) {
        //only supports ROM ONLY for now...
        NO_IMPL!();
    }

    pub fn with_rom_path(rom_path: String) -> Cartridge {
        let header = header::CartridgeHeader {
            entry: [0, 0, 0, 0],
            nintendo_logo: [0; 0x30],
            title: [0; 0x10],
            new_licensee_code: 0,
            sgb_flag: 0,
            cartridge_type: 0,
            rom_size: 0,
            ram_size: 0,
            dest_code: 0,
            licencee_code: vec![0, 0],
            version: 0,
            checksum: 0,
            checksum_passed: false,
            global_checksum: 0,
            gbc_flag: 0,
        };
        return Cartridge {
            rom_path: rom_path,
            rom_data: Vec::new(),
            rom_header: header,
        };
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
