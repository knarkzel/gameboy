use gameboy::{*, emulator::Emulator};

#[throws]
fn main() {
    color_backtrace::install();
    let firmware = include_bytes!("../dmg_boot.bin");
    let mut emulator = Emulator::default();
    emulator.load(firmware)?;
    dbg!(&emulator);
}
