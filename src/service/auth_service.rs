use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, ExistenceCheck, SetExpiry, SetOptions};
use reqwest::header::AUTHORIZATION;

use crate::config::config_auth::ConfigAuth;
use crate::config::config_limits::ConfigLimits;
use crate::model::send_message::MessageAuthor;

#[derive(Clone)]
pub struct AuthService {
    auth_url: String,
    redis_client: redis::Client,
    max_requests: u32,
    expiration_time: usize,
}

impl AuthService {
    pub fn new(config_auth: ConfigAuth, redis_client: redis::Client, limits: ConfigLimits) -> Self {
        AuthService {
            redis_client,
            auth_url: config_auth.auth_url,
            max_requests: limits.max_requests,
            expiration_time: limits.expiration_time,
        }
    }

    pub async fn validate_token(&self, token: &str) -> Option<MessageAuthor> {
        println!("Received token: {}", token);

        let validation_url = format!("{}/user/login/validate", self.auth_url);

        let client = reqwest::Client::new();
        let response = client
            .post(validation_url.as_str())
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let text = response.text().await.unwrap();
            return serde_json::from_str(&text).unwrap();
        }

        None
    }

    pub async fn has_user_reached_limit(&self, user: &MessageAuthor) -> bool {
        let user_requests = self.count_user_requests(user).await;
        return user_requests >= self.max_requests;
    }

    pub async fn increase_user_request_count(&self, user: &MessageAuthor) -> bool {
        let current_request_key = format!(
            "user-message:{}:requests:{}",
            user.email,
            chrono::Utc::now().timestamp()
        );

        let set_options = SetOptions::default()
            .with_expiration(SetExpiry::EX(self.expiration_time))
            .conditional_set(ExistenceCheck::NX)
            .get(false);

        return self
            .get_async_connection()
            .await
            .set_options(&current_request_key, 1, set_options)
            .await
            .expect("Error setting key");
    }

    async fn count_user_requests(&self, user: &MessageAuthor) -> u32 {
        let query_user_requests = format!("user-message:{}:requests:*", user.email);

        let results: Vec<String>;
        match self
            .get_async_connection()
            .await
            .keys(query_user_requests)
            .await
        {
            Ok(r) => {
                results = r;
            }
            Err(_e) => {
                return 0;
            }
        };

        return results.len() as u32;
    }

    async fn get_async_connection(&self) -> MultiplexedConnection {
        self.redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap()
    }
}
