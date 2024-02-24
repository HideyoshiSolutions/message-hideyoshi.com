mod handler;
mod interceptor;
mod model;
mod route;

#[tokio::main]
async fn main() {
    let app = route::create_route();

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8500").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
