use instruction::{AddressingMode as AM, Instruction, InstructionType as IT, RegisterType as RT};

pub(crate) mod instruction;

pub fn get_instructions() -> [Instruction; 0x100] {
    let mut instruction_set = [Instruction::new(IT::NONE, AM::R, None, None, None, None); 0x100];
    //
    instruction_set[0x00] = Instruction::new(IT::NOP, AM::R, None, None, None, None);
    instruction_set[0x05] = Instruction::new(IT::DEC, AM::R, Some(RT::B), None, None, None);
    instruction_set[0x0E] = Instruction::new(IT::LD, AM::R_D8, Some(RT::C), None, None, None);
    instruction_set[0xAF] = Instruction::new(IT::XOR, AM::R, Some(RT::A), None, None, None);
    instruction_set[0xC3] = Instruction::new(IT::JP, AM::D16, None, None, None, None);
    instruction_set[0xFA] = Instruction::new(IT::LD, AM::R_A16, Some(RT::A), None, None, None); //LD A, (a16)
    return instruction_set;
}

