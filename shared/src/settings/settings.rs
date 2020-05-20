#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub show_banner: bool,
    pub mongodb: Option<MongoDBSettings>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MongoDBSettings {
    pub connection_uri: String,
    pub database_name: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_banner: true,
            mongodb: Some(MongoDBSettings {
                connection_uri: "mongodb://root:rootpassword@localhost:27017/".to_string(),
                database_name: None,
            }),
        }
    }
}
