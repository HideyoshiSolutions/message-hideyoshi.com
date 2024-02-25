use crate::handler::health::health_check;
use crate::handler::message::send_message;
use axum::{routing::{get, post}, Router, middleware};
use crate::middleware::auth_middleware::auth_middleware;


pub fn create_route() -> Router {
    Router::new()
        .route("/message", post(send_message))
        .layer(middleware::from_fn(auth_middleware))
        .route("/health", get(health_check))
}
