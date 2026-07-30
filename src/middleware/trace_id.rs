use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http::{HeaderMap, HeaderValue};
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

/// 为每个请求分配 trace id 的中间件，保证请求进入后该请求产生的每一条日志
/// 都带上 trace id（以及真实 IP、User-Agent）。
///
/// 实现思路：把 trace id、真实 IP、User-Agent 作为字段放进一个 tracing span，
/// 并用 `.instrument()` 包住整个请求处理。`tracing-subscriber` 的默认格式会自动
/// 把当前 span 的字段打印到日志行（形如 `request{trace_id="..." real_ip="..."
/// user_agent="..."}:`），因此下游中间件、handler、service 产生的所有日志都会
/// 自动带上这些信息，便于把一次请求链路上的所有日志串起来排查。
///
/// 若上游已经在 `X-Trace-Id` 头中带了 id，则复用；否则生成新的。
/// 同时把 trace id 回写到响应头，方便调用方排查。
pub async fn trace_id_middleware(mut req: Request, next: Next) -> Response {
    let headers = req.headers();
    let trace_id = resolve_trace_id(headers);
    let real_ip = resolve_real_ip(headers);
    let user_agent = resolve_user_agent(headers);

    // 让 handler 可以读取到 trace id
    req.extensions_mut().insert(TraceId(trace_id.clone()));

    // 用 info 级 span 携带 trace_id / real_ip / user_agent，包住整个请求处理：
    // 日志全局过滤为 INFO，trace 级 span 会被丢弃而无法记录字段，
    // 这里必须用 info_span 才能保证字段被存储、供日志格式器读取。
    let span = info_span!(
        "request",
        trace_id = %trace_id,
        real_ip = %real_ip,
        user_agent = %user_agent,
    );

    let mut response = next.run(req).instrument(span).await;

    // 回写到响应头，便于调用方关联
    if let Ok(value) = HeaderValue::from_str(&trace_id) {
        response.headers_mut().insert(TRACE_ID_HEADER, value);
    }

    response
}

/// 取 trace id：优先沿用上游 `X-Trace-Id`（去除空白后非空才采用）；否则生成新的。
fn resolve_trace_id(headers: &HeaderMap) -> String {
    headers
        .get(TRACE_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(generate_trace_id)
}

/// 取真实 IP：优先读 `x-real-ip` 头；缺失或为空则记为 `-`。
fn resolve_real_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("-")
        .to_string()
}

/// 取 User-Agent：缺失或为空则记为 `-`。
fn resolve_user_agent(headers: &HeaderMap) -> String {
    headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("-")
        .to_string()
}

/// 生成 trace_id：uuid v4 simple（32 hex）取前 16 位。
/// 单机低并发场景下碰撞可忽略，且比完整 uuid 更紧凑、日志更易读。
fn generate_trace_id() -> String {
    let full = Uuid::new_v4().simple().to_string();
    full.get(..16).map(str::to_string).unwrap_or(full)
}
