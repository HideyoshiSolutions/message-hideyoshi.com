use crate::model::generic_response::GenericResponse;
use crate::model::send_message::{MessageAuthor, SendMessage};
use crate::service::auth_service::AuthService;
use crate::service::email_service::EmailService;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn send_message(
    Extension(auth_service): Extension<AuthService>,
    Extension(email_service): Extension<EmailService>,
    Extension(author): Extension<MessageAuthor>,
    Json(payload): Json<SendMessage>,
) -> impl IntoResponse {
    let mut package = payload.clone();
    package.author = Some(author.clone()).clone();

    if auth_service.has_user_reached_limit(&author).await {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(GenericResponse {
                status: StatusCode::TOO_MANY_REQUESTS.to_string(),
                message: "User has reached the limit of messages".to_string(),
            }),
        );
    }

    match email_service.send_email_smtp(package).await {
        Ok(_) => {}
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenericResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message: e.to_string(),
                }),
            )
        }
    };

    auth_service.increase_user_request(&author).await;

    return (
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            message: "Message sent".to_string(),
        }),
    );
}
