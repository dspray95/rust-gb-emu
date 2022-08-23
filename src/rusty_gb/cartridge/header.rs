use std::str;

use crate::rusty_gb::cartridge::{licensee_codes, rom_types};

pub struct CartridgeHeader {
    pub entry: [u8; 4],
    pub nintendo_logo: [u8; 0x30],

    pub title: [u8; 0x10],
    pub new_licensee_code: u16,
    pub sgb_flag: u8,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub licencee_code: Vec<u8>,
    pub version: u8,
    pub checksum: u8,
    pub checksum_passed: bool,
    pub global_checksum: u16,
    pub gbc_flag: u8,
}

const ROM_SIZES: [u8; 6] = [0, 2, 8, 32, 128, 64];

impl CartridgeHeader {
    pub fn new(rom_data: Option<&Vec<u8>>) -> CartridgeHeader {
        if rom_data.is_none() {
            return CartridgeHeader {
                entry: [0; 4],
                nintendo_logo: [0; 48],
                title: [0; 16],
                new_licensee_code: 0 as u16,
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
        } else {
            return CartridgeHeader::load_cartidge_header_from_rom_data(&rom_data.unwrap());
        }
    }

    fn load_cartidge_header_from_rom_data(rom_data: &[u8]) -> CartridgeHeader {
        let nintendo_logo = rom_data[0x104..=0x133]
            .try_into()
            .expect("slice with incorrect length");

        let title = rom_data[0x134..=0x143].try_into().expect("re");
        let licencee_code = CartridgeHeader::load_licencee_code(rom_data);
        let cartridge_type = rom_data[0x147];

        let gbc_flag = rom_data[0x143];
        let sgb_flag = rom_data[0x146];

        let rom_size = rom_data[0x148];
        let ram_size = rom_data[0x149];

        let dest_code = rom_data[0x14A];
        let version = rom_data[0x014C];

        let checksum = rom_data[0x14D];
        let checksum_passed = CartridgeHeader::calculate_checksum(checksum, rom_data);

        return CartridgeHeader {
            entry: [0; 4],
            nintendo_logo: nintendo_logo,
            title: title,
            new_licensee_code: 0,
            sgb_flag: sgb_flag,
            cartridge_type: cartridge_type,
            rom_size: rom_size,
            ram_size: ram_size,
            dest_code: dest_code,
            licencee_code: licencee_code,
            version: version,
            checksum: checksum,
            checksum_passed: checksum_passed,
            global_checksum: 0,
            gbc_flag: gbc_flag,
        };
    }

    // pub fn load_header(&mut self, rom_data: &[u8]) {
    //     self.nintendo_logo = rom_data[0x103..=0x132]
    //         .try_into()
    //         .expect("slice with incorrect length");

    //     self.title = rom_data[0x133..=0x142].try_into().expect("re");
    //     self.licencee_code = CartridgeHeader::load_licencee_code(rom_data);
    //     self.cartridge_type = rom_data[0x146];

    //     self.gbc_flag = rom_data[0x142];
    //     self.sgb_flag = rom_data[0x145];

    //     self.rom_size = rom_data[0x147];
    //     self.ram_size = rom_data[0x148];

    //     self.dest_code = rom_data[0x149];
    //     self.version = rom_data[0x014B];

    //     self.checksum = rom_data[0x14C];
    //     self.checksum_passed = CartridgeHeader::calculate_checksum(self.checksum, rom_data);
    //     self.print_cartridge_header();
    // }

    fn load_licencee_code(header_bytes: &[u8]) -> Vec<u8> {
        let old_licencee_code = header_bytes[0x14B];
        if old_licencee_code == 0x33 {
            return header_bytes[0x144..=0x145].try_into().expect("msg");
        } else {
            return vec![old_licencee_code];
        }
    }

    fn licencee_code_to_int(code_hex: &[u8]) -> u8 {
        if let Ok(s) = str::from_utf8(code_hex) {
            return s.parse::<u8>().unwrap();
        } else {
            panic!("could not convert licencee code")
        }
    }

    fn print_title(&mut self, as_hex: bool) {
        if let Ok(s) = str::from_utf8(&self.title) {
            println!("\tTITLE: {}", s)
        }
        if as_hex {
            let mut hex_string = "".to_string();
            for i in self.title {
                hex_string = hex_string + &format!("{:#04x}, ", i).to_string();
            }
            println!("{}", hex_string);
        }
    }

    fn calculate_checksum(checksum_byte: u8, rom_data: &[u8]) -> bool {
        let mut checksum: u8 = 0;
        for address in 0x0134..=0x014C {
            checksum = checksum.wrapping_sub(rom_data[address]);
            checksum = checksum.wrapping_sub(1);
        }
        return checksum == checksum_byte;
    }

    pub fn print_cartridge_header(&mut self) {
        let licencee_code_concat: u8;
        if self.licencee_code.len() == 2 {
            licencee_code_concat = CartridgeHeader::licencee_code_to_int(&[
                self.licencee_code[0],
                self.licencee_code[1],
            ]);
        } else {
            licencee_code_concat = self.licencee_code[0];
        }

        println!("CARTRIDGE HEADER:");
        self.print_title(false);
        println!("\tVERSION NUMBER: {}", self.version);

        println!(
            "\tLICENCEE CODE: {}, {} ({})",
            licencee_code_concat,
            if self.licencee_code.len() == 2 {
                licensee_codes::get_licensee_codes()[&licencee_code_concat]
            } else {
                licensee_codes::get_old_licencee_codes()[&self.licencee_code[0]]
            },
            if self.licencee_code.len() == 2 {
                "new licencee code"
            } else {
                "old licencee code"
            },
        );
        println!(
            "\tDESINTATION CODE: {}, {}",
            self.dest_code,
            if self.dest_code == 0x00 {
                "Japan (and possibly overseas)"
            } else if self.dest_code == 0x01 {
                "Overseas only"
            } else {
                panic!("Invalid dest code {:#04x}", self.dest_code);
            }
        );
        println!(
            "\tCARTRIDGE TYPE: {:#04x}, {}",
            self.cartridge_type,
            rom_types::get_rom_types()[self.cartridge_type as usize]
        );
        println!("\tROM SIZE: {} KiB", 32 << self.rom_size);

        println!("\tRAM SIZE: {} KiB", ROM_SIZES[self.ram_size as usize]);

        println!(
            "\tCHECKSUM: {} ({})",
            self.checksum,
            if self.checksum_passed {
                "PASSED"
            } else {
                "FAILED"
            }
        );
        println!(
            "\tGBC FLAG: {:#04x}, {}",
            self.gbc_flag,
            if self.gbc_flag == 0x80 {
                "Supports CGB enhancements, backwards compatible with monochrome"
            } else if self.gbc_flag == 0xC0 {
                "Works on CGB only"
            } else {
                "No CGB flag"
            }
        );
        println!(
            "\tSGB FLAG: {:#04x}, {}",
            self.sgb_flag,
            if self.sgb_flag == 0x03 {
                "Supports SGB functions"
            } else {
                "Does not support SGB functions"
            }
        );
    }
}
