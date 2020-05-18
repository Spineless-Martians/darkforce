#[macro_use]
extern crate slog;

use darkforce_shared::{
    get_store_from_config,
    load_settings,
    setup_logger,
    DAGDescription,
    DAGManager,
};
use failure::Error;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let logger = setup_logger()?;

    info!(logger, "Starting Darkforce Scheduler");

    let settings = load_settings(&logger)?;

    let store = get_store_from_config(&logger, &settings).await?;
    store
        .store_dag(&logger, &DAGDescription::new("TEST", None, vec![]))
        .await?;

    let _manager = DAGManager::new(&logger, &store).await?;

    Ok(())
}
