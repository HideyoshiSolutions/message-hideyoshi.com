use crate::model::generic_response::GenericResponse;
use crate::model::send_message::{MessageAuthor, SendMessage};
use crate::service::email_service::EmailService;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};


pub async fn send_message(
    Extension(email_service): Extension<EmailService>,
    Extension(author): Extension<MessageAuthor>,
    Json(payload): Json<SendMessage>,
) -> impl IntoResponse {
    let mut package = payload.clone();
    package.author = Some(author).clone();

    match email_service.send_email_smtp(package).await {
        Ok(_) => {},
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenericResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                    message: e.to_string(),
                }),
            )
        },
    };

    return (
        StatusCode::OK,
        Json(GenericResponse {
            status: StatusCode::OK.to_string(),
            message: "Message sent".to_string(),
        }),
    );
}