pub mod emulator;

// Error handling
pub use anyhow::bail;
pub use fehler::throws;
pub type Error = anyhow::Error;

