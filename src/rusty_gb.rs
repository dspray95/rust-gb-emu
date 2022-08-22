mod bus;
mod cartridge;
mod cpu;
mod emulator_context;

use cartridge::Cartridge;
use emulator_context::EmulatorContext;
use std::env;

pub fn emulator_run() -> i64 {
    let args: Vec<String> = env::args().collect();
    if !parse_args(&args) {
        return 1;
    }

    let rom_path = args[1].clone();

    let mut cartridge: Cartridge = Cartridge::with_rom_path(rom_path);
    cartridge.load().unwrap();

    let emulator_context = EmulatorContext::get_emulator_context();

    return 0;
}

fn parse_args(args: &Vec<String>) -> bool {
    if args.len() < 2 {
        println!("only {} args provided.", args.len());
        println!(".rom file required, no filepath specified. exiting...");
        return false;
    }
    if !&args[1].ends_with(".gb") {
        println!("rom file required, must end with .gb. exiting...");
        return false;
    }
    return true;
}
