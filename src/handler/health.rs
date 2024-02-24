use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::model::generic_response::{GenericResponse};

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Server is running";
    let response = GenericResponse {
        status: StatusCode::OK.to_string(),
        message: MESSAGE.to_string(),
    };

    (StatusCode::OK, Json(response))
}