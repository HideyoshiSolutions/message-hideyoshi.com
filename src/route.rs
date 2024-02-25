use crate::config::config_auth;
use crate::handler::health::health_check;
use crate::handler::message::send_message;
use crate::middleware::auth_middleware::auth_middleware;
use crate::service::auth_service::AuthService;
use crate::service::email_service::EmailService;
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
        .layer(Extension(EmailService::new()))
}

fn configure_health_endpoint(router: Router) -> Router {
    router.route("/health", get(health_check))
}

pub fn create_route() -> Router {
    let mut router = Router::new();

    router = configure_message_endpoint(router);
    router = configure_health_endpoint(router);

    router
}
