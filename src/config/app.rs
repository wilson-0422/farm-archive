pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn default() -> Self {
        Self {
            database_url: "farm_archive.db".to_string(),
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}
