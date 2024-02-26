use cached::proc_macro::cached;
use std::env;

#[derive(Clone)]
pub struct ConfigLimits {
    pub max_requests: u32,
    pub expiration_time: usize,
}

#[cached]
pub fn get_config_limits() -> ConfigLimits {
    dotenv::dotenv().ok();

    let max_requests = env::var("MAX_REQUESTS").unwrap_or("10".to_string())
        .parse::<u32>().unwrap();
    let expiration_time = env::var("EXPIRATION_TIME").unwrap_or("604800".to_string())
        .parse::<usize>().unwrap();

    ConfigLimits {
        max_requests,
        expiration_time,
    }
}
