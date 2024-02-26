use cached::proc_macro::cached;

#[derive(Clone)]
pub struct ConfigServer {
    pub host: String,
    pub port: u16,
    pub allowed_origins: Option<Vec<String>>,
}

#[cached]
pub fn get_config_server() -> ConfigServer {
    dotenv::dotenv().ok();

    let host = option_env!("HOST").unwrap_or("localhost").to_string();

    let port = option_env!("PORT")
        .unwrap_or("8500")
        .parse::<u16>()
        .unwrap();

    let allowed_origins = match option_env!("ALLOWED_ORIGINS") {
        Some(origins) => Some(origins.split(",").map(|s| s.to_string()).collect()),
        None => None,
    };

    ConfigServer {
        host,
        port,
        allowed_origins,
    }
}
