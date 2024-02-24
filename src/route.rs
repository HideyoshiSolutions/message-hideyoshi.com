use axum::{
    routing::{get, post},
    Router
};
use crate::handler::health::{health_check};
use crate::handler::message::{send_message};


pub fn create_route() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/message", post(send_message))
}