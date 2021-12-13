use crate::*;

#[allow(dead_code)]
#[derive(Default, Debug)]
struct Registers {
    pc: u16,
    sp: u16,
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Register {
    A, B, C, D, E, F, H, L, SP, AF, BC, DE, HL(i8)
}

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

#[allow(dead_code)]
#[derive(Debug)]
struct Instruction {
    bytes: u8,
    cycles: u8,
    opcode: Opcode,
}

impl Instruction {
    fn new(bytes: u8, cycles: u8, opcode: Opcode) -> Self {
        Self { bytes, cycles, opcode }
    }

    /// 8-bit opcodes
    fn basic(byte: u8) -> Self {
        use Opcode::*;
        use Register::*;
        match byte {
            0x00 => Self::new(1, 1, NOP),
            0x01 => Self::new(3, 3, LDd16(BC)),
            0x02 => Self::new(1, 2, LD(BC, A)),
            0x03 => Self::new(1, 2, INC(BC)),
            0x04 => Self::new(1, 1, INC(B)),
            0x05 => Self::new(1, 1, DEC(B)),
            0x06 => Self::new(2, 2, LDd8(B)),
            0x07 => Self::new(1, 1, RLC(A)),
            0x08 => Self::new(3, 5, LDAdress(SP)),
            0x09 => Self::new(1, 2, ADD(HL(0), BC)),
            0x0A => Self::new(1, 2, LD(A, BC)),
            0x0B => Self::new(1, 2, DEC(BC)),
            0x0C => Self::new(1, 1, INC(C)),
            0x0D => Self::new(1, 1, DEC(D)),
            0x0E => Self::new(2, 2, LDd8(C)),
            0x0F => Self::new(1, 1, RRC(A)),
            0x21 => Self::new(3, 3, LDd16(HL(0))),
            0x31 => Self::new(3, 3, LDd16(SP)),
            0x32 => Self::new(1, 2, LD(HL(-1), A)),
            0xAF => Self::new(1, 1, XOR(A)),
            0xFE => Self::new(2, 2, CPd8),
            0xFF => Self::new(1, 4, RST(7)),
            0x9F => Self::new(1, 2, SBC(A, A)),
            missing => todo!("8-bit opcode {:#04x}", missing),
        }
    }

    /// 16-bit opcodes, where the first 8 bits are 0xCB
    fn advanced(byte: u8) -> [Self; 2] {
        use Opcode::*;
        use Register::*;
        let instruction = match byte {
            0x00 => Self::new(1, 1, RLC(C)),
            missing => todo!("16-bit opcode {:#04x}", missing),
        };
        [Self::new(1, 1, CB), instruction]
    }
}

/// See [opcodes](https://meganesulli.com/generate-gb-opcodes/) for more.
#[derive(Debug)]
enum Opcode {
    /// Advances the program counter by 1.
    NOP,
    /// 16-bit opcode filler
    CB,
    /// Store the contents of last register in the memory location specified by
    /// first register.
    LD(Register, Register),
    /// Load the 8-bit immediate operand d8 into register.
    LDd8(Register),
    /// Load the 2 bytes of immediate data into register.
    LDd16(Register),
    /// Store the lower byte of register at the address specified by the
    /// 16-bit immediate operand a16, and store the upper byte of register at address
    /// a16 + 1.
    LDAdress(Register),
    /// Increment the contents of register by 1.
    INC(Register),
    /// Decrement the contents of register by 1.
    DEC(Register),
    /// Add the contents of last register to the contents of first register, and
    /// store the results in first register.
    ADD(Register, Register),
    /// Compare the contents of register A and the contents of the 8-bit immediate
    /// operand d8 by calculating A - d8, and set the Z flag if they are equal.
    CPd8,
    /// Push the current value of the program counter PC onto the memory stack,
    /// and load into PC the nth byte of page 0 memory addresses, 0xn8.
    RST(u8),
    /// Take the logical exclusive-OR for each bit of the contents of memory
    /// specified by register and the contents of register A, and store
    /// the results in register A.
    XOR(Register),
    /// Rotate the contents of register to the left.
    RL(Register),
    /// Rotate the contents of register to the left. The contents of bit 7 are
    /// placed in both the CY flag and bit 0 of register.
    RLC(Register),
    /// Rotate the contents of register to the right.
    RR(Register),
    /// Rotate the contents of register to the right. The contents of bit 0 are

    RRC(Register),
    /// Subtract the contents of memory specified by last register and the carry
    /// flag CY from the contents of register A, and store the results in
    /// register A.
    SBC(Register, Register),
}
