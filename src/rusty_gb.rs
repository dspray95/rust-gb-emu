mod bus;
mod cartridge;
mod cpu;
mod emulator;

use emulator::Emulator;
use std::env;

pub fn run() -> i64 {
    let args = env::args().collect();
    if !parse_args(&args) {
        return 1;
    }

    let mut emulator: Emulator = Emulator::new();
    emulator.run(args[1].clone());
    return 0;
}

pub fn emulator_cycles(n_cycles: u8){
    return;
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