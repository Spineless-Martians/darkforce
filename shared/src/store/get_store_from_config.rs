use crate::{
    settings::Settings,
    store::{
        mongodb_store::MongoDBStore,
        store::Store,
    },
};
use failure::Error;
use slog::Logger;
use std::sync::Arc;

pub async fn get_store_from_config(
    logger: &Logger,
    settings: &Settings,
) -> Result<Arc<dyn Store>, Error> {
    if settings.mongodb.is_some() {
        debug!(logger, "Found configuration settings for mongodb store!");

        let mongo_settings = settings.mongodb.as_ref().unwrap();

        let store = MongoDBStore::connect(
            &logger,
            &mongo_settings.connection_uri,
            &mongo_settings.database_name,
        )
        .await?;

        Ok(Arc::new(store))
    } else {
        unimplemented!()
    }
}
