use crate::model::generic_response::GenericResponse;
use crate::model::send_message::{MessageAuthor, SendMessage};
use crate::service::auth_service::AuthService;
use crate::service::email_service::EmailService;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn send_message(
    Extension(auth_service): Extension<AuthService>,
    Extension(email_service): Extension<EmailService>,
    Extension(author): Extension<MessageAuthor>,
    Json(mut payload): Json<SendMessage>,
) -> impl IntoResponse {
    payload.author = Some(author.clone()).clone();

    if auth_service.has_user_reached_limit(&author).await {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(GenericResponse {
                status: StatusCode::TOO_MANY_REQUESTS.to_string(),
                message: "User has reached the limit of messages".to_string(),
            }),
        );
    }

    email_service.send_email(payload).await;
    auth_service.increase_user_request_count(&author).await;

    (
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            message: "Message sent".to_string(),
        }),
    )
}
