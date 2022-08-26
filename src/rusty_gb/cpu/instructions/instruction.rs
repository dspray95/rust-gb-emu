use std::ops::Add;
#[derive(Clone, Copy)]
pub enum InstructionType {
    NONE,
    NOP,
    JP,
    LD,
    XOR,
    DEC,
    DI
}

impl InstructionType {
    pub fn print(instruction_type: InstructionType){
        match instruction_type { 
            InstructionType::NONE => println!("NONE"),
            InstructionType::NOP => println!("NOP"),
            InstructionType::JP => println!("JP"),
            InstructionType::LD => println!("LD"),
            InstructionType::XOR => println!("XOR"),
            InstructionType::DEC => println!("DEC"),
            InstructionType::DI => println!("DI"),
            _ => panic!("Invalid instruction type")
        }
    }
}

#[derive(Clone, Copy)]
pub enum ConditionType {
    NONE,
}
#[derive(Clone, Copy)]
pub enum AddressingMode {
    R,
    IMP,
    D16,
    R_D8,
}
#[derive(Clone, Copy)]
pub enum RegisterType {
    NONE,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}
#[derive(Clone, Copy)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub addressing_mode: AddressingMode,
    pub register_1: RegisterType,
    pub register_2: RegisterType,
    pub condition_type: ConditionType,
    pub param: u8,
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
