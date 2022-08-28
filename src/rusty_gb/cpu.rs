#![allow(arithmetic_overflow)]

use core::panic;

use instructions::instruction::Instruction;
// use processors::get_processors;
use registers::CpuRegisters;
use rust_gbe::{bit, emulator_cycles};

// use self::instructions::Instructions;

use self::instructions::{
    get_instructions,
    instruction::{AddressingMode, InstructionType},
};

use super::{bus::Bus, emulator::Emulator};

mod instructions;
mod processors;
mod registers;

pub struct Cpu {
    pub(crate) registers: CpuRegisters,
    fetched_data: u16, //current fetched data
    mem_dest: u16,
    dest_is_mem: bool,
    current_opcode: u8,
    current_instruction: Instruction,
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
            current_instruction: instruction_set[0],
            halted: false,
            stepping: false,
            bus: bus,
            instructions: instruction_set,
        };
    }

    fn fetch_instrcution(&mut self) {
        self.registers.pc += 1;
        self.current_opcode = self.bus.read(self.registers.pc);
        self.current_instruction = self.instructions[self.current_opcode as usize];
    }

    fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match self.current_instruction.addressing_mode {
            AddressingMode::IMP => return,
            AddressingMode::R => {
                self.fetched_data = self.registers.read_reg(self.current_instruction.register_1);
                return;
            }
            AddressingMode::D16 => {
                let lo = self.bus.read(self.registers.pc);
                emulator_cycles(1);
                let hi: u8 = self.bus.read(self.registers.pc + 1);
                emulator_cycles(1);
                self.fetched_data = lo as u16 | ((hi as u16) << 8);
                self.registers.pc += 2;

                return;
            }
            AddressingMode::R_D8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                emulator_cycles(1);
                self.registers.pc += 1;
                return;
            }
            _ => panic!("unknown addressing mode!"),
        }
    }

    fn execute(&mut self) {
        println!(
            "Executing instruction: {:#02x}, PC: {:#04x}",
            self.current_opcode, self.registers.pc
        );
        let processor = processors::get_processor(self.current_instruction.instruction_type);
        processor(self);
    }

    pub fn cpu_step(&mut self) -> bool {
        if !self.halted {
            let pc: u16 = self.registers.pc;

            self.fetch_instrcution();
            self.fetch_data();

            println!("CPU STEP");
            println!("\tpc: {}", pc);
            println!(
                "\tinstruction: {}",
                InstructionType::to_string(self.current_instruction.instruction_type)
            );
            // InstructionType::print(self.current_instruction.instruction_type);
            // println!(
            //     "{:#02x}, {:#02x}",
            //     self.bus.read(pc + 1),
            //     self.bus.read(pc + 2)
            // );

            // self.registers.print_registers();
            self.execute();
        }
        return true;
    }

    pub fn get_cpu_flag(&self, flag: Flag) -> bool {
        let registers_flag = self.registers.f;
        match flag {
            Flag::Z => {
                return if bit(self.registers.f, 7) == 1 {
                    true
                } else {
                    false
                }
            }
            Flag::N => {
                return if bit(self.registers.f, 6) == 1 {
                    true
                } else {
                    false
                }
            }
            Flag::H => {
                return if bit(self.registers.f, 5) == 1 {
                    true
                } else {
                    false
                }
            }
            Flag::C => {
                return if bit(self.registers.f, 4) == 1 {
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub enum Flag {
    Z,
    C,
    N,
    H,
}
