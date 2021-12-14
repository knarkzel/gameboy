use crate::{registers::Register::*, opcode::Opcode::{self, *}};

fn new(bytes: u8, cycles: u8, opcode: Opcode) -> Instruction {
    Instruction { bytes, cycles, opcode }
}

#[derive(Debug)]
pub enum Value {
    Number(u8),
    Command(Instruction),
}

#[derive(Debug)]
pub struct Instruction {
    pub bytes: u8,
    pub cycles: u8,
    pub opcode: Opcode,
}

impl Instruction {
    /// 8-bit opcodes
    pub fn basic(byte: u8) -> Value {
        let instruction = match byte {
            0x00 => new(1, 1, NOP),
            0x01 => new(3, 3, LDd16(BC)),
            0x02 => new(1, 2, LD(BC, A)),
            0x03 => new(1, 2, INC(BC)),
            0x04 => new(1, 1, INC(B)),
            0x05 => new(1, 1, DEC(B)),
            0x06 => new(2, 2, LDd8(B)),
            0x07 => new(1, 1, RLC(A)),
            0x08 => new(3, 5, LDAdress(SP)),
            0x09 => new(1, 2, ADD(HL(0), BC)),
            0x0A => new(1, 2, LD(A, BC)),
            0x0B => new(1, 2, DEC(BC)),
            0x0C => new(1, 1, INC(C)),
            0x0D => new(1, 1, DEC(D)),
            0x0E => new(2, 2, LDd8(C)),
            0x0F => new(1, 1, RRC(A)),
            0x20 => new(2, 3, JRNZ),
            0xfb => new(1, 1, CP(E)),
            0x26 => new(2, 2, LDd8(H)),
            0x11 => new(3, 3, LDd16(DE)),
            0x3e => new(2, 2, LDd8(A)),
            0x80 => new(1, 1, ADD(A, B)),
            0xe2 => new(1, 2, LD(C, A)),
            0xf3 => new(1, 1, DI),
            0x77 => new(1, 2, LD(HL(0), A)),
            0x21 => new(3, 3, LDd16(HL(0))),
            0x31 => new(3, 3, LDd16(SP)),
            0x32 => new(1, 2, LD(HL(-1), A)),
            0xAF => new(1, 1, XOR(A)),
            0xFE => new(2, 2, CPd8),
            0xFF => new(1, 4, RST(7)),
            0x9F => new(1, 2, SBC(A, A)),
            missing => todo!("8-bit opcode {:#04x}", missing),
        };
        Value::Command(instruction)
    }

    /// 16-bit opcodes, where the first 8 bits are 0xCB
    pub fn advanced(byte: u8) -> [Value; 2] {
        let instruction = match byte {
            0x00 => new(1, 1, RLC(C)),
            0x7c => new(2, 2, BIT(7, H)),
            missing => todo!("16-bit opcode {:#04x}", missing),
        };
        [Value::Number(0xCB), Value::Command(instruction)]
    }
}
