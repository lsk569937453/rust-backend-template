use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http::{HeaderMap, StatusCode};
use std::time::Instant;
use tracing::info;

pub async fn log_requests(
    _headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = Instant::now();
    let response = next.run(req).await;
    let latency = start.elapsed();
    let status = response.status();
    info!("{} | {:.2?} | {} {}", status, latency, method, path);
    Ok(response)
}
