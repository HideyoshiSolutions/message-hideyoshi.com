use crate::model::send_message::{MessageAuthor, SendMessage};
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

pub async fn send_message(
    Extension(auther): Extension<MessageAuthor>,
    Json(payload): Json<SendMessage>,
) -> impl IntoResponse {
    let mut response = payload.clone();
    response.author = Some(auther).clone();

    (StatusCode::OK, Json(response))
}
