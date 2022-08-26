
#![allow(arithmetic_overflow)]

use instructions::instruction::Instruction;
use registers::CpuRegisters;
use processors::get_processors;

// use self::instructions::Instructions;

use self::instructions::{
    get_instructions,
    instruction::{AddressingMode, InstructionType, RegisterType},
};

use super::{bus::Bus, emulator::Emulator};

mod instructions;
mod registers;
mod processors; 

pub struct Cpu {
    pub(crate) registers: CpuRegisters,
    fetched_data: u16, //current fetched data
    mem_dest: u16,
    dest_is_mem: bool,
    current_opcode: u8,
    current_instrcution: Instruction,
    halted: bool,
    stepping: bool,
    bus: Bus,
    instructions: [Instruction; 0x100],
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        let instruction_set: [Instruction; 0x100] = get_instructions();
        return Cpu {
            registers: CpuRegisters::new(),
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            current_opcode: 0,
            current_instrcution: instruction_set[0],
            halted: false,
            stepping: false,
            bus: bus,
            instructions: instruction_set,
        };
    }

    fn fetch_instrcution(&mut self) {
        self.current_opcode = self.bus.read(self.registers.pc + 1);
        self.current_instrcution = self.instructions[self.current_opcode as usize];
        // if self.current_instrcution.instruction_type == InstructionType::NONE {
        //     panic!("No valid instruction type {:#04x}", self.current_opcode);
        // }
    }

    fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match self.current_instrcution.addressing_mode {
            AddressingMode::IMP => return,
            AddressingMode::R => {
                self.fetched_data = self.registers.read_reg(self.current_instrcution.register_1);
                return;
            }
            AddressingMode::D16 => {
                let lo = self.bus.read(self.registers.pc);
                Emulator::emulator_cycles(1);

                let hi: u8 = self.bus.read(self.registers.pc + 1);
                Emulator::emulator_cycles(1);
                self.fetched_data = (lo | (hi << 8)) as u16;
                self.registers.pc += 2;

                return;
            }
            AddressingMode::R_D8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                Emulator::emulator_cycles(1);
                self.registers.pc += 1;
                return;
            }
            _ => panic!("unknown addressing mode!"),
        }
    }

    fn read_reg(&mut self, register: RegisterType) -> u8 {
        todo!();
    }

    fn execute(&mut self) {
        let processor = get_processors(self.current_instrcution.instruction_type);
    }

    fn cpu_step(&mut self) {
        if !self.halted {
            let pc: u16 = self.registers.pc;

            self.fetch_instrcution();
            self.fetch_data();

            println!("CPU STEP");
            println!("\tpc: {}", pc);
            print!("\t"); InstructionType::print(self.current_instrcution.instruction_type);
            println!("{:#02x}, {:#02x}", self.bus.read(pc + 1), self.bus.read(pc + 2));

            self.registers.print_registers();
            self.execute();
        }
    }
}
