mod config;
mod handler;
mod middleware;
mod model;
mod route;
mod service;

use crate::config::config_server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let server_config = config_server::get_config_server();

    let app = route::create_route();

    println!("🚀 Server started successfully");
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", server_config.host, server_config.port))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
