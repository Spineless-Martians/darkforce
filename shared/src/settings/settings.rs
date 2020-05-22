use std::net::SocketAddr;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub show_banner: bool,
    pub mongodb: Option<MongoDBSettings>,
    web: Option<WebSettings>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MongoDBSettings {
    pub connection_uri: String,
    pub database_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WebSettings {
    address: SocketAddr,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_banner: true,
            mongodb: Some(MongoDBSettings {
                connection_uri: "mongodb://root:rootpassword@localhost:27017/".to_string(),
                database_name: None,
            }),
            web: None,
        }
    }
}

impl Settings {
    pub fn web_address(&self) -> SocketAddr {
        if let Some(web_settings) = &self.web {
            web_settings.address
        } else {
            ([127, 0, 0, 1], 3000).into()
        }
    }
}
