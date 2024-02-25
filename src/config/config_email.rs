use cached::proc_macro::cached;
use std::env;

#[derive(Clone)]
pub struct ConfigEmail {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[cached]
pub fn get_config_email() -> ConfigEmail {
    let server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
    let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    ConfigEmail{
        smtp_server: server,
        smtp_port: port.parse::<u16>().unwrap(),
        smtp_username: username,
        smtp_password: password,
    }
}
