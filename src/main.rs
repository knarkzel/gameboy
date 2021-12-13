use gameboy::{*, emulator::Emulator};

#[throws]
fn main() {
    pretty_env_logger::init_timed();
    color_backtrace::install();
    let firmware = include_bytes!("../roms/dmg_boot.bin");
    let mut emulator = Emulator::default();
    emulator.load(firmware)?;
    dbg!(&emulator);
}
