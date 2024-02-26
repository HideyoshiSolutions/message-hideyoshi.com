use std::collections::BTreeMap;
use redis::{AsyncCommands, ExistenceCheck, SetExpiry, SetOptions};
use crate::config::config_auth::ConfigAuth;
use crate::model::send_message::MessageAuthor;
use reqwest::header::AUTHORIZATION;
use crate::config::config_limits::ConfigLimits;
use crate::config::config_redis::ConfigRedis;

#[derive(Clone)]
pub struct AuthService {
    auth_url: String,
    redis: redis::Client,
    max_requests: u32,
    expiration_time: usize,
}

impl AuthService {
    pub fn new(config_auth: ConfigAuth, config_redis: ConfigRedis, limits: ConfigLimits) -> Self {
        let client = redis::Client::open(
            format!("redis://{}:{}", config_redis.redis_url, config_redis.redis_port).as_str()
        ).unwrap();

        AuthService {
            auth_url: config_auth.auth_url,
            redis: client,
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

    pub async fn increase_user_request(&self, user: &MessageAuthor) -> bool {
        let mut con = self.redis.get_async_connection().await.unwrap();
        let current_request_key= format!(
            "user-message:{}:requests:{}",
            user.email,
            chrono::Utc::now().timestamp()
        );

        let set_options = SetOptions::default()
            .with_expiration(SetExpiry::EX(self.expiration_time))
            .conditional_set(ExistenceCheck::NX)
            .get(false);

        return con.set_options(
            &current_request_key,
            1,
            set_options
        ).await.expect("Error setting key");

    }

    async fn count_user_requests(&self, user: &MessageAuthor) -> u32 {
        let mut con = self.redis.get_async_connection().await.unwrap();
        let query_user_requests = format!("user-message:{}:requests:*", user.email);

        let results: Vec<String>;
        match con.keys(query_user_requests).await {
            Ok(r) => {
                results = r;
            },
            Err(e) => {
                return 0;
            }

        };

        return results.len() as u32;
    }
}
