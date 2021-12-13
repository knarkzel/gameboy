use crate::*;

#[derive(Default, Debug)]
struct Registers {
    pc: u16,
    sp: u16,
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
}

#[derive(Default, Debug)]
pub struct Emulator {
    registers: Registers,
    instructions: Vec<Instruction>,
}

impl Emulator {
    #[throws]
    pub fn load(&mut self, input: &[u8]) {
        let mut i = 0;
        while i < input.len() {
            match input[i] != 0xCB {
                true => { self.instructions.push(Instruction::basic(input[i])); i += 1; }
                false => { self.instructions.push(Instruction::advanced(input[i])); i += 2; }
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Nop,
}

impl Instruction {
    /// 8-bit opcodes
    fn basic(byte: u8) -> Self {
        match byte {
            0x00 => Self::Nop,
            missing => todo!("8-bit opcode {:#04x}", missing),
        }
    }

    /// 16-bit opcodes, where the first 8 bits are 0xCB
    fn advanced(byte: u8) -> Self {
        match byte {
            missing => todo!("16-bit opcode {:#04x}", missing),
        }
    }
}
