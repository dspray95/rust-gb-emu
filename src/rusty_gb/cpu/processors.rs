use super::{instructions::instruction::InstructionType};
use super::Cpu;


pub fn none_processor(){
    panic!("Invalid instruction type!")
}

pub fn nop_processor(){
    return
}

pub fn xor_processor(cpu: &mut Cpu){
    cpu.registers.a ^= cpu.fetched_data & 0xFF;
}

pub fn get_processors(instruction_type: InstructionType) -> fn() {
    match instruction_type{
        InstructionType::NOP => return nop_processor,
        InstructionType::NONE => return none_processor,
        InstructionType::JP => todo!(),
        InstructionType::XOR => todo!(),
        InstructionType::LD => todo!(),
        InstructionType::DEC => todo!(),
        InstructionType::DI => todo!(),
    }
}

// pub fn get_processors(){
//     println!("some function")
// }