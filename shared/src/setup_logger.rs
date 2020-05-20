use slog::Logger;
use sloggers::{
    terminal::TerminalLoggerBuilder,
    types::Severity,
    Build,
    Error,
};

/// Constructs a default logger that is used throughout darkforce
pub fn setup_logger() -> Result<Logger, Error> {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Trace); // <- Log everything
    let logger = builder.build()?;
    Ok(logger)
}
