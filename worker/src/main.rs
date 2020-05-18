#[macro_use]
extern crate slog;

use failure::Error;
use sloggers::{
    terminal::TerminalLoggerBuilder,
    Build,
};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let builder = TerminalLoggerBuilder::new();
    let logger = builder.build()?;

    info!(logger, "Initializing Darkforce Worker!");

    Ok(())
}
