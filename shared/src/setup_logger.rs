use slog::Logger;
use sloggers::{
    terminal::TerminalLoggerBuilder,
    types::Severity,
    Build,
    Error,
};

pub fn setup_logger() -> Result<Logger, Error> {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Trace);
    let logger = builder.build()?;
    Ok(logger)
}
