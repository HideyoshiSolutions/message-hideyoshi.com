use cached::proc_macro::cached;
use std::env;

#[derive(Clone)]
pub struct ConfigServer {
    pub(crate) host: String,
    pub(crate) port: u16,
}

#[cached]
pub fn get_config_server() -> ConfigServer {
    let h = option_env!("HOST").unwrap_or("localhost").to_string();

    let p = option_env!("PORT")
        .unwrap_or("8500")
        .parse::<u16>()
        .unwrap();

    ConfigServer { host: h, port: p }
}
