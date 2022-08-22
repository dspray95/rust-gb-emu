use std::ops::Add;

pub enum InstructionType {
    NOP,
    JP,
    LD,
    XOR,
    DEC,
}
pub enum ConditionType {
    NONE,
}
pub enum AddressingMode {
    R,
    IMP,
    D16,
    R_D8,
}
pub enum RegisterType {
    NONE,
    A,
    C,
    B,
}

pub struct Instruction {
    instruction_type: InstructionType,
    addressing_mode: AddressingMode,
    register_1: RegisterType,
    register_2: RegisterType,
    condition_type: ConditionType,
    param: u8,
}

impl Instruction {
    pub fn new(
        instruction_type: InstructionType,
        addressing_mode: AddressingMode,
        register_1: Option<RegisterType>,
        register_2: Option<RegisterType>,
        condition_type: Option<ConditionType>,
        param: Option<u8>,
    ) -> Instruction {
        // let mut vec:
        return Instruction {
            instruction_type: instruction_type,
            addressing_mode: addressing_mode,
            register_1: register_1.unwrap_or(RegisterType::NONE),
            register_2: register_2.unwrap_or(RegisterType::NONE),
            condition_type: condition_type.unwrap_or(ConditionType::NONE),
            param: param.unwrap_or(0),
        };
    }
}
