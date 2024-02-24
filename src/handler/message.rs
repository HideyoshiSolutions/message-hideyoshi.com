use crate::model::send_message::SendMessage;
use axum::{http::StatusCode, response::IntoResponse, Json};
use proc_utils::guard_resource;

#[guard_resource(ResourceType::OPEN)]
pub async fn send_message(Json(payload): Json<SendMessage>) -> impl IntoResponse {
    (StatusCode::OK, Json(payload))
}
