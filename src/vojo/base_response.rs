use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;
use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct BaseResponse<T: Serialize> {
    pub code: i32,
    pub body: T,
}
impl<T: Serialize> IntoResponse for BaseResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
