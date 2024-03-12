use cached::proc_macro::cached;
use std::env;

#[derive(Clone)]
pub struct ConfigServer {
    pub host: String,
    pub port: u16,
    pub allowed_origins: Option<Vec<String>>,
}

#[cached]
pub fn get_config_server() -> ConfigServer {
    dotenvy::dotenv().ok();

    let host = env::var("HOST").unwrap_or("localhost".to_string());

    let port = env::var("PORT")
        .unwrap_or("8500".to_string())
        .parse::<u16>()
        .unwrap();

    let allowed_origins = match env::var("ALLOWED_ORIGINS") {
        Ok(origins) => Some(origins.split(',').map(|s| s.to_string()).collect()),
        Err(_) => None,
    };

    ConfigServer {
        host,
        port,
        allowed_origins,
    }
}
