use super::{bus::Bus, cartridge::Cartridge, cpu::Cpu};

pub struct Emulator {
    pub paused: bool,
    pub running: bool,
    pub ticks: i64,
}

impl Emulator {
    pub fn new() -> Emulator {
        return Emulator {
            paused: false,
            running: false,
            ticks: 0,
        };
    }

    pub fn run(&mut self, rom_path: String) -> i64 {
        self.running = true;

        let mut cartridge: Cartridge = Cartridge::new(Some(rom_path));
        cartridge.rom_header.print_cartridge_header();

        let bus = Bus::new(Some(cartridge));
        let mut cpu = Cpu::new(bus);
        println!("STARTING CPU...");
        while self.running {
            if !cpu.cpu_step() {
                println!("...CPU STOPPED");
                self.running = false;
            }
        }
        return 0;
    }

    pub fn emulator_cycles(cpu_cycles: i64) {
        return;
    }
}
