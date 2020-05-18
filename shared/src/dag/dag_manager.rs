use crate::{
    store::Store,
    DAGDescription,
};
use failure::Error;
use libloading::Library;
use slog::Logger;
use std::sync::Arc;
use tokio::fs::{
    create_dir,
    OpenOptions,
};

pub struct DAGManager {
    store: Arc<dyn Store>,
    dags: Vec<DAG>,
}

struct DAG(DAGDescription, Library);

impl DAGManager {
    pub async fn new(logger: &Logger, store: &Arc<dyn Store>) -> Result<Self, Error> {
        let dag_descriptions = store.load_dags(&logger).await?;

        let mut dags = vec![];

        for dag in dag_descriptions {
            let logger = logger.new(o!("dag_name" => dag.name.to_owned(), "dag_description" => dag.description.to_owned(), "dag_created_at" => dag.created_at.to_string()));
            info!(logger, "Loading DAG");
            Self::save_lib_to_file(&logger, &dag).await?;
            let lib = Self::load_library(&logger, &dag)?;
            dags.push(DAG(dag, lib));
        }

        Ok(Self {
            store: Arc::clone(&store),
            dags,
        })
    }

    pub async fn load_dags(&mut self, logger: &Logger) -> Result<(), Error> {
        let dags = self.store.load_dags(&logger).await?;

        for dag in dags {
            info!(logger, "Loading dag"; "dag_name" => &dag.name);
        }

        Ok(())
    }

    async fn save_lib_to_file(logger: &Logger, dag: &DAGDescription) -> Result<(), Error> {
        if create_dir("dags").await.is_ok() {
            info!(logger, "dags folder was missing, created it");
        }

        let _file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("dags/{}.so", dag.name))
            .await?;

        debug!(logger, "Stored DAG source into file");

        Ok(())
    }

    fn load_library(_logger: &Logger, dag: &DAGDescription) -> Result<Library, Error> {
        let lib = Library::new(format!("dags/{}.so", dag.name))?;
        Ok(lib)
    }
}
