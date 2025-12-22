use crate::error::app_error::AppError;
use crate::to_base_response;
use crate::vojo::app_state::AppState;
use axum::{extract::State, response::IntoResponse};

pub async fn echo(State(app_state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    info!("Received device token login request");

    let r = to_base_response!(echo_with_error(app_state).await);

    Ok(r)
}

/// 内部核心逻辑：设备令牌登录
pub async fn echo_with_error(app_state: AppState) -> Result<String, anyhow::Error> {
    Ok(String::from("test"))
}
