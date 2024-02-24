use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::model::send_message::{SendMessage};

pub async fn send_message(Json(payload): Json<SendMessage>) -> impl IntoResponse {
    (StatusCode::OK, Json(payload))
}