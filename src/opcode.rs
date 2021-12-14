use crate::registers::Register;

/// See [opcodes](https://meganesulli.com/generate-gb-opcodes/) for more.
#[derive(Debug)]
pub enum Opcode {
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
