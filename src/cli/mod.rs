mod commands;
#[cfg(feature = "unstable-dynamic")]
pub mod setup;

pub use commands::{Cli, ColorModeArg, OutputFormat};
