use axum::{
    response::Response,
    middleware::Next,
    extract::Request,
    http::StatusCode,
};
use reqwest::header::AUTHORIZATION;
use crate::model::send_message::MessageAuthor;

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = get_token(&request).ok_or(StatusCode::UNAUTHORIZED)?;

    return match validate_token(&token).await {
        Some(author) => {
            println!("Author: {:?}", author);
            request.extensions_mut().insert(author);
            Ok(next.run(request).await)
        },
        None => Err(StatusCode::UNAUTHORIZED)
    }
}

fn get_token(req: &Request) -> Option<String> {
    req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(|header| header.replace("Bearer ", ""))
}

async fn validate_token(token: &str) -> Option<MessageAuthor> {
    println!("Received token: {}", token);

    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8070/user/login/validate")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send().await.unwrap();

    if response.status().is_success() {
        let text = response.text().await.unwrap();
        return serde_json::from_str(&text).unwrap();
    }

    None
}