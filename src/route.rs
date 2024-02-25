use crate::config::{config_auth, config_email};
use crate::handler::health::health_check;
use crate::handler::message::send_message;
use crate::middleware::auth_middleware::auth_middleware;
use crate::service::auth_service::AuthService;
use crate::service::email_service::EmailService;
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
        .layer(Extension(AuthService::new(config_auth::get_config_auth())))
        .layer(Extension(EmailService::new(config_email::get_config_email())))
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
