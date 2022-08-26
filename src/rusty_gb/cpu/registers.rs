use super::{instructions::instruction::RegisterType as RT};

pub struct CpuRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl CpuRegisters {
    pub fn new() -> CpuRegisters {
        return CpuRegisters {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        };
    }

    fn reverse(n: u16) -> u16 {
        return ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8);
    }

    pub fn read_reg(&self, register_type: RT) -> u16 {
        match register_type {
            RT::A => return self.a.into(),
            RT::B => return self.b.into(),
            RT::F => return self.f.into(),
            RT::C => return self.c.into(),
            RT::D => return self.d.into(),
            RT::E => return self.e.into(),
            RT::H => return self.h.into(),
            RT::L => return self.l.into(),
            RT::AF => CpuRegisters::reverse(self.a as u16),
            RT::BC => CpuRegisters::reverse(self.b as u16),
            RT::DE => CpuRegisters::reverse(self.d as u16),
            RT::HL => CpuRegisters::reverse(self.h as u16),
            RT::PC => return self.pc,
            RT::SP => return self.sp,
            RT::NONE => todo!(),
        }
    }

    pub fn print_registers(&self){
        println!()
    }
}
