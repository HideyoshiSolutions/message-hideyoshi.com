use cached::proc_macro::cached;
use std::env;

#[derive(Clone)]
pub struct ConfigAuth {
    pub auth_url: String,
}

#[cached]
pub fn get_config_auth() -> ConfigAuth {
    dotenvy::dotenv().ok();

    let url = env::var("AUTH_URL").expect("AUTH_URL must be set");

    ConfigAuth { auth_url: url }
}
