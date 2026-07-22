use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http::HeaderValue;
use tracing::Instrument;
use uuid::Uuid;

/// 用于在 HTTP 头中传递 trace id 的头名称。
pub const TRACE_ID_HEADER: &str = "X-Trace-Id";

/// 当前请求关联的 trace id。
///
/// 会被写入 request extensions，handler 可通过
/// `req.extensions().get::<TraceId>()` 读取。
// 本模板内暂无 handler 读取，故允许 dead_code；供使用模板的项目读取。
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TraceId(pub String);

/// 为每个请求分配 trace id 的中间件，保证请求进入后该请求产生的
/// 每一条日志都带上 trace id。
///
/// 实现思路：把 trace id 作为字段放进一个 tracing span，并用
/// `.instrument()` 包住整个请求处理。`tracing-subscriber` 的默认
/// 格式会自动把当前 span 的字段打印到日志行
/// （形如 `request{trace_id="..."}:`），因此下游中间件、handler、
/// service 产生的所有日志都会自动带上 trace id。
///
/// 若上游已经在 `X-Trace-Id` 头中带了 id，则复用；否则生成新的。
/// 同时把 trace id 回写到响应头，方便调用方排查。
pub async fn trace_id_middleware(mut req: Request, next: Next) -> Response {
    let trace_id = req
        .headers()
        .get(TRACE_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // 让 handler 可以读取到 trace id
    req.extensions_mut().insert(TraceId(trace_id.clone()));

    // 用 span 字段携带 trace id，包住整个请求处理
    let span = info_span!("request", trace_id = %trace_id);

    let mut response = next.run(req).instrument(span).await;

    // 回写到响应头，便于调用方关联
    if let Ok(value) = HeaderValue::from_str(&trace_id) {
        response.headers_mut().insert(TRACE_ID_HEADER, value);
    }

    response
}
