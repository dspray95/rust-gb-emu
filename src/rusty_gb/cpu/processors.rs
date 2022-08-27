use std::os::windows::process;

use super::{instructions::instruction::InstructionType, Cpu};

pub fn process_none(cpu: Cpu, b: Option<u8>) {
    panic!("Invalid instruction type!")
}

pub fn process_nop(cpu: Cpu, b: Option<u8>) {
    return;
}

pub fn process_xor(mut cpu: Cpu, b: Option<u8>) {
    cpu.registers.a ^= b.unwrap() & 0xFF;
}

pub fn process_jp(cpu: Cpu, b: Option<u8>) {
    return;
}

pub fn process_ld(cpu: Cpu, b: Option<u8>) {
    return;
}

pub fn process_dec(cpu: Cpu, b: Option<u8>) {
    return;
}

pub fn process_di(cpu: Cpu, b: Option<u8>) {
    return;
}

pub fn get_processor(instruction_type: InstructionType) -> fn(Cpu, Option<u8>) {
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
