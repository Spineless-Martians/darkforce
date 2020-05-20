#[macro_use]
extern crate slog;

use darkforce_shared::{
    get_store_from_config,
    load_settings,
    print_banner,
    setup_logger,
    DAGManager,
};
use failure::Error;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let logger = setup_logger()?;
    let settings = load_settings(&logger)?;

    print_banner(&settings);

    info!(logger, "Starting Darkforce Scheduler");

    let store = get_store_from_config(&logger, &settings).await?;

    let _manager = DAGManager::new(&logger, &store).await?;

    Ok(())
}
