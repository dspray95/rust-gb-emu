use instruction::{AddressingMode as AM, Instruction, InstructionType as IT, RegisterType as RT};

pub(crate) mod instruction;

pub fn get_instructions() -> Vec<Instruction> {
    let mut instruction_set: Vec<Instruction> = Vec::with_capacity(100);
    instruction_set[0x00] = Instruction::new(IT::NOP, AM::R, None, None, None, None);
    instruction_set[0x05] = Instruction::new(IT::DEC, AM::R, Some(RT::B), None, None, None);
    instruction_set[0x0E] = Instruction::new(IT::LD, AM::R_D8, Some(RT::C), None, None, None);
    instruction_set[0xAF] = Instruction::new(IT::XOR, AM::R, Some(RT::A), None, None, None);
    instruction_set[0xC3] = Instruction::new(IT::JP, AM::D16, None, None, None, None);
    return instruction_set;
}
