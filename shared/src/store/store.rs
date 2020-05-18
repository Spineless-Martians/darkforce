use crate::store::dag_description::DAGDescription;
use failure::Error;
use futures::future::BoxFuture;
use slog::Logger;

pub trait Store: Send + Sync {
    fn load_dags<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<Vec<DAGDescription>, Error>>;
    fn store_dag<'a>(
        &'a self,
        logger: &'a Logger,
        description: &'a DAGDescription,
    ) -> BoxFuture<'a, Result<(), Error>>;
}
