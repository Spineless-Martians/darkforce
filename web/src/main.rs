#[macro_use]
extern crate slog;
#[macro_use]
extern crate rust_embed;

mod query;
mod schema;
mod start_web_server;

use crate::start_web_server::start_web_server;
use darkforce_shared::{
    get_store_from_config,
    load_settings,
    print_banner,
    setup_logger,
};
use failure::Error;



#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let logger = setup_logger()?;
    let settings = load_settings(&logger)?;

    print_banner(&settings);

    info!(logger, "Starting Darkforce Web Server");

    let _store = get_store_from_config(&logger, &settings).await?;

    // let _manager = DAGManager::new(&logger, &store).await?;

    start_web_server(&logger).await?;

    Ok(())
}
