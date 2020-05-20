

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Query {}

#[juniper::graphql_object(Context = ())]
impl Query {
    fn api_version() -> &'static str {
        VERSION
    }

    fn dag_count() -> i32 {
        10
    }
}
