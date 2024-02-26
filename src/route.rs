
use crate::depends::depends_auth_service::get_depends_auth_service;
use crate::depends::depends_email_service::get_depends_email_service;
use crate::handler::health::health_check;
use crate::handler::message::send_message;
use crate::middleware::auth_middleware::auth_middleware;


use crate::utils::router_builder::RouterBuilder;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};

fn configure_message_endpoint(router: Router) -> Router {
    router
        .route("/message", post(send_message))
        .layer(middleware::from_fn(auth_middleware))
        .layer(Extension(get_depends_auth_service()))
        .layer(Extension(get_depends_email_service()))
}

fn configure_health_endpoint(router: Router) -> Router {
    router.route("/health", get(health_check))
}

pub fn create_route() -> Router {
    RouterBuilder::new()
        .add_config(configure_message_endpoint)
        .add_config(configure_health_endpoint)
        .build()
}
