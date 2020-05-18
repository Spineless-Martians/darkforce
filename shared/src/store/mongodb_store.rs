use crate::store::{
    dag_description::DAGDescription,
    store::Store,
};
use bson::{
    from_bson,
    to_bson,
    Bson::Document,
};
use failure::Error;
use futures::{
    future::BoxFuture,
    stream::StreamExt,
    FutureExt,
};
use mongodb::{
    Client,
    Database,
};
use slog::Logger;

const DEFAULT_DATABASE_NAME: &'static str = "darkforce";
const DAG_COLLECTION_NAME: &'static str = "dags";

pub struct MongoDBStore {
    database: Database,
}

impl MongoDBStore {
    pub async fn connect(
        logger: &Logger,
        mongo_db_uri: &str,
        database_name: &Option<String>,
    ) -> Result<Self, Error> {
        let client = Client::with_uri_str(mongo_db_uri).await?;
        info!(
            logger,
            "Successfully established connection to mongodb server"
        );

        let db_name = match database_name.as_ref() {
            None => {
                warn!(
                    logger,
                    "No database name was provided, using default name '{}'", DEFAULT_DATABASE_NAME
                );
                DEFAULT_DATABASE_NAME
            }
            Some(name) => name,
        };

        let database = client.database(db_name);

        info!(logger, "Using database {}", database.name());

        Ok(Self { database })
    }
}

impl Store for MongoDBStore {
    fn load_dags<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<Vec<DAGDescription>, Error>> {
        async move {
            debug!(logger, "Loading a dags from mongodb");

            let collection = self.database.collection(DAG_COLLECTION_NAME);

            let cursor = collection.find(None, None).await?;

            let descriptions: Vec<DAGDescription> = cursor
                .filter_map(|item| {
                    match item {
                        Ok(doc) => {
                            match from_bson::<DAGDescription>(Document(doc)) {
                                Ok(dag) => futures::future::ready(Some(dag)),
                                Err(parse_err) => {
                                    crit!(logger, "Failed to parse one or more dag entry"; "error" => parse_err.to_string());
                                    futures::future::ready(None)
                                },
                            }
                        },
                        Err(err) => {
                            crit!(logger, "Failed to read one or more dag entry from mongodb"; "error" => err.to_string());
                            futures::future::ready(None)
                        }
                    }
                })
                .collect().await;
            Ok(descriptions)
        }.boxed()
    }

    fn store_dag<'a>(
        &'a self,
        logger: &'a Logger,
        description: &'a DAGDescription,
    ) -> BoxFuture<'a, Result<(), Error>> {
        async move {
            let log = logger.new(o!("dag_name" => description.name.clone(), "dag_description" => description.description.clone(), "dag_created_at" => description.created_at.to_string()));

            info!(log, "Storing dag into mongodb");

            let collection = self.database.collection(DAG_COLLECTION_NAME);

            let document = to_bson(&description)?.as_document().unwrap().clone();

            collection.insert_one(document, None).await?;

            info!(log, "Successfully inserted dag into mongodb");

            Ok(())
        }.boxed()
    }
}
