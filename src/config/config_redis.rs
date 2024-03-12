use cached::proc_macro::cached;
use std::env;

#[derive(Clone)]
pub struct ConfigRedis {
    pub redis_db: u8,
    pub redis_url: String,
    pub redis_port: u16,
    pub redis_password: Option<String>,
}

#[cached]
pub fn get_config_redis() -> ConfigRedis {
    dotenvy::dotenv().ok();

    let db = env::var("REDIS_DB").unwrap_or("0".to_string());
    let url = env::var("REDIS_URL").unwrap_or("localhost".to_string());
    let port = env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let password = env::var("REDIS_PASSWORD").ok();

    ConfigRedis {
        redis_db: db.parse::<u8>().unwrap(),
        redis_url: url,
        redis_port: port.parse::<u16>().unwrap(),
        redis_password: password,
    }
}
