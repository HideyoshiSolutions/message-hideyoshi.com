use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::model::{GenericResponse};

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Server is running";
    let response = GenericResponse {
        status: StatusCode::OK.to_string(),
        message: MESSAGE.to_string(),
    };

    (StatusCode::OK, Json(response))
}