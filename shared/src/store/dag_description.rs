use chrono::{
    DateTime,
    Utc,
};

/// Struct used to describe a DAG and to old the source.
#[derive(Serialize, Deserialize, Debug)]
pub struct DAGDescription {
    /// The name of the DAG
    pub name: String,

    /// A description of the DAG
    pub description: Option<String>,

    /// When the dag was added to the darkforce instance
    pub created_at: DateTime<Utc>,

    /// The source
    pub source: Vec<u8>,
}

impl DAGDescription {
    /// Creates a DAGDescription from name, description and the source
    pub fn new(name: &str, description: Option<&str>, source: Vec<u8>) -> Self {
        Self {
            name: name.to_owned(),
            description: description.map(|i| i.to_owned()),
            created_at: Utc::now(),
            source,
        }
    }
}
