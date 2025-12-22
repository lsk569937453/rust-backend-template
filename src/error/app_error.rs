use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;
use serde_json::json;
// 1. 将 AppError 从 struct 修改为 enum
pub enum AppError {
    // 用于所有其他内部错误
    InternalServerError(anyhow::Error),
}

// 2. 更新 IntoResponse 的实现以处理不同的变体
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(err) => {
                // 对于所有其他错误，记录详细信息并返回 500
                error!("Internal server error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// 3. 调整 From Trait 的实现
// 这使得任何可以转换为 anyhow::Error 的错误都可以通过 `?` 运算符
// 自动转换为 AppError::InternalServerError。
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        // 默认将所有外部错误视为内部服务器错误
        Self::InternalServerError(err.into())
    }
}
