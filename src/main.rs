mod config;
mod depends;
mod handler;
mod middleware;
mod model;
mod route;
mod service;
mod utils;

use crate::config::config_server;
use crate::depends::depends_email_service;

async fn run_server() {
    let server_config = config_server::get_config_server();

    let app = route::create_route(server_config.allowed_origins);

    println!("🚀 Server started successfully");
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", server_config.host, server_config.port))
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn run_worker() {
    let mut email_service = depends_email_service::get_depends_email_service();
    loop {
        email_service.create_send_message_task().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn run_both() {
    futures::future::join(run_server(), run_worker()).await;
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let runtime: &str;
    if args.len() < 2 {
        runtime = "both";
    } else {
        runtime = args[1].as_str();
    }

    match runtime {
        "server" => run_server().await,
        "worker" => run_worker().await,
        "both" => run_both().await,
        _ => panic!("Invalid argument"),
    }
}
