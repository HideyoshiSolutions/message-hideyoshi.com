use crate::service::auth_service::AuthService;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Extension};

pub async fn auth_middleware(
    Extension(auth_service): Extension<AuthService>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = get_token(&request).ok_or(StatusCode::UNAUTHORIZED)?;

    return match auth_service.validate_token(&token).await {
        Some(author) => {
            request.extensions_mut().insert(author);
            Ok(next.run(request).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    };
}

fn get_token(req: &Request) -> Option<String> {
    req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(|header| header.replace("Bearer ", ""))
}
