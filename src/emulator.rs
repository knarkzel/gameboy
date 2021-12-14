use crate::{decode::{Instruction, Value}, registers::Registers, *};

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Emulator {
    memory: Vec<Value>,
    registers: Registers,
}

impl Emulator {
    #[throws]
    pub fn load(&mut self, input: &[u8]) {
        let mut i = 0;
        while i < input.len() {
            debug!("Parsing opcode {:#04x} at index {}", input[i], i);
            debug!("{:#?}", self);
            match input[i] != 0xCB {
                true => { self.memory.push(Instruction::basic(input[i])); i += 1; }
                false => { self.memory.extend(Instruction::advanced(input[i + 1])); i += 2; }
            }
            if let Some(Value::Command(instruction)) = self.memory.last() {
                if instruction.bytes > 1 {
                    let values = instruction.bytes - 1;
                    self.memory.extend(input[i..][..values as _].iter().map(|v| Value::Number(*v)));
                    i += values as usize;
                }
            }
        }
    }
}
