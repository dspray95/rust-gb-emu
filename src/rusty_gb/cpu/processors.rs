use crate::rusty_gb::emulator_cycles;

// use super::super::emulator::Emulator::emulator_cycles;
use super::instructions::instruction::{ConditionType, InstructionType, RegisterType};
use super::Cpu;


type Processor = fn(&mut Cpu);

pub fn process_none(_cpu: &mut Cpu) {
    panic!("Invalid instruction type!")
}

pub fn process_nop(_cpu: &mut Cpu) {
    return;
}

pub fn process_xor(cpu: &mut Cpu) {
    cpu.registers.a ^= cpu.registers.b & 0xFF;
}

pub fn process_jp(cpu: &mut Cpu) {
    if check_condition(cpu) {
        cpu.registers.pc = cpu.fetched_data;
        emulator_cycles(1)
    }
}

pub fn process_ld(cpu: &mut Cpu) {
    if cpu.destination_is_memory {
        //LD (BC), A etc
        if cpu.current_instruction.register_2 >= RegisterType::AF {

        }
    }
}

pub fn process_dec(cpu: &mut Cpu) {
    return;
}

pub fn process_di(cpu: &mut Cpu) {
    return;
}

fn check_condition(cpu: &Cpu) -> bool {
    let z = cpu.get_cpu_flag(super::Flag::Z);
    let c = cpu.get_cpu_flag(super::Flag::C);
    match cpu.current_instruction.condition_type {
        ConditionType::NONE => return true,
        ConditionType::NZ => return !z,
        ConditionType::Z => return z,
        ConditionType::NC => return !c,
        ConditionType::C => return c,
        _ => return false,
    }
}

pub fn get_processor(instruction_type: InstructionType) -> Processor {
    match instruction_type {
        InstructionType::NOP => return process_nop,
        InstructionType::NONE => return process_none,
        InstructionType::JP => return process_jp,
        InstructionType::XOR => return process_xor,
        InstructionType::LD => return process_ld,
        InstructionType::DEC => return process_dec,
        InstructionType::DI => return process_di,
        _ => panic!(
            "INVALID INSTRUCTION TYPE: {}",
            InstructionType::to_string(instruction_type)
        ),
    }
}
