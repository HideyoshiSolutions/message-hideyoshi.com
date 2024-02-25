use reqwest::header::AUTHORIZATION;
use crate::config::config_auth::ConfigAuth;
use crate::model::send_message::MessageAuthor;


#[derive(Clone)]
pub struct AuthService {
    auth_url: String,
}

impl AuthService {
    pub fn new(config_auth: ConfigAuth) -> Self {
        AuthService {
            auth_url: config_auth.auth_url,
        }
    }

    pub async fn validate_token(&self, token: &str) -> Option<MessageAuthor> {
        println!("Received token: {}", token);

        let validation_url = format!("{}/user/login/validate", self.auth_url);

        let client = reqwest::Client::new();
        let response = client.post(validation_url.as_str())
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send().await.unwrap();

        if response.status().is_success() {
            let text = response.text().await.unwrap();
            return serde_json::from_str(&text).unwrap();
        }

        None
    }
}