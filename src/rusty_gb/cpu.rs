use instructions::instruction::Instruction;
use registers::CpuRegisters;

use self::instructions::Instructions;

use super::bus::Bus;

mod instructions;
mod registers;

struct Cpu {
    registers: CpuRegisters,
    fetch_data: u16, //current fetch
    mem_dest: u16,
    current_opcode: u8,
    current_instrcution: Instruction,
    halted: bool,
    stepping: bool,
    bus: Bus,
    instructions: Vec<Instruction>,
}

impl Cpu {
    fn new() -> Cpu {
        let instruction_set: Vec<Instruction> = Instructions::get_instructions();
        return Cpu {
            registers: CpuRegisters::new(),
            fetch_data: 0,
            mem_dest: 0,
            current_opcode: 0,
            current_instrcution: instruction_set[0],
            halted: false,
            stepping: false,
            bus: Bus::new(),
            instructions: instruction_set,
        };
    }

    fn fetch_instrcution(&mut self) {}

    fn fetch_data(&mut self) {}

    fn execute(&mut self) {}

    fn cpu_step(&mut self) {
        if !self.halted {
            self.fetch_instrcution();
            self.fetch_data();
            self.execute();
        }
    }
}
