use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
}
