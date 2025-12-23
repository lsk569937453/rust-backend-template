mod app;
mod common;
mod config;
mod error;
mod handler;
mod logger;
mod middleware;
mod vojo;
use crate::app::run::main_with_error;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_json;
#[tokio::main]
async fn main() {
    if let Err(e) = main_with_error().await {
        error!("{}", e);
    }
}
