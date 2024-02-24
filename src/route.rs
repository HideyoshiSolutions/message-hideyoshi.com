use crate::handler::health::health_check;
use crate::handler::message::send_message;
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_route() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/message", post(send_message))
}
