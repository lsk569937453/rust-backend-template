mod common;
mod config;
mod error;
mod handler;
mod logger;
mod middleware;
mod vojo;
use crate::config::app_config::Config;
use crate::config::cli::Cli;
use crate::handler::echo_handler::echo;
use crate::logger::log::setup_logger;
use crate::middleware::log::log_requests;

use crate::vojo::app_state::AppState;
use axum::Router;
use axum::middleware as axum_middleware;
use axum::routing::any;
use clap::Parser;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_appender::rolling;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_json;
#[tokio::main]
async fn main() {
    let logger = setup_logger();
    if let Err(e) = logger {
        error!("{}", e);
    }

    if let Err(e) = main_with_error().await {
        error!("{}", e);
    }
}
async fn main_with_error() -> Result<(), anyhow::Error> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 加载配置文件
    let config = Config::load_or_default(&cli.config);

    // Initialize app state with database and config
    let port = config.server.port;
    let app_state = AppState::new(config).await?;
    let cors = CorsLayer::new()
        .allow_origin(Any) // In production, you'd want to restrict this to your frontend's domain
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/echo", any(echo))
        .layer(axum_middleware::from_fn(log_requests))
        .with_state(app_state)
        .layer(cors); // <-- 3. Apply the CORS layer to your entire application

    // 使用配置文件中的端口
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server listening on http://{}", addr);
    println!("Server listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
