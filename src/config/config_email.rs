use cached::proc_macro::cached;
use std::env;

#[derive(Clone)]
pub struct ConfigEmail {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_name: String,
    pub smtp_email: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[cached]
pub fn get_config_email() -> ConfigEmail {
    dotenvy::dotenv().ok();

    let server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
    let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let name = env::var("SMTP_NAME").expect("SMTP_NAME must be set");
    let email = env::var("SMTP_EMAIL").unwrap_or(username.clone());

    ConfigEmail {
        smtp_server: server,
        smtp_port: port.parse::<u16>().unwrap(),
        smtp_name: name,
        smtp_email: email,
        smtp_username: username,
        smtp_password: password,
    }
}
