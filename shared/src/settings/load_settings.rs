use crate::settings::Settings;
use config::{
    Config,
    Environment,
    File,
};
use failure::Error;
use slog::Logger;

pub fn load_settings(logger: &Logger) -> Result<Settings, Error> {
    debug!(logger, "Loading settings");

    // let mut config = Config::try_from(&Settings::default())?;
    let mut config = Config::new();

    config.merge(Config::try_from(&Settings::default())?)?;

    config.merge(File::with_name("darkforce").required(false))?;
    config.merge(Environment::with_prefix("darkforce").separator("_"))?;

    let settings = config.try_into()?;

    debug!(logger, "Full configuration was: {:?}", settings);

    Ok(settings)
}
