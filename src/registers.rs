#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Registers {
    pc: u16,
    sp: u16,
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Register {
    A, B, C, D, E, F, H, L, SP, AF, BC, DE, HL(i8)
}
