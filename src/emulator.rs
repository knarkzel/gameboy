use crate::{decode::Instruction, registers::Registers, *};

#[allow(dead_code)]
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
            debug!("Parsing opcode {:#04x} at index {}", input[i], i);
            match input[i] != 0xCB {
                true => { self.instructions.push(Instruction::basic(input[i])); i += 1; }
                false => { self.instructions.extend(Instruction::advanced(input[i + 1])); i += 2; }
            }
        }
    }
}
