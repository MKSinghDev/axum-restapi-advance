use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{Instrument, info_span};
use uuid::Uuid;

/// Tracing middleware that adds request tracking and timing
pub async fn tracing_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Generate request ID if not present
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Create span for this request
    let span = info_span!(
        "http_request",
        method = %method,
        uri = %uri,
        request_id = %request_id,
        status_code = tracing::field::Empty,
        duration_ms = tracing::field::Empty,
    );

    async move {
        let response = next.run(request).await;

        let duration = start.elapsed();
        let status_code = response.status().as_u16();

        // Record span fields within the span context
        tracing::Span::current().record("status_code", status_code);
        tracing::Span::current().record("duration_ms", duration.as_millis() as u64);

        // Add request ID to response headers
        let mut response = response;
        response
            .headers_mut()
            .insert("x-request-id", request_id.parse().unwrap());

        response
    }
    .instrument(span)
    .await
}

/// Extract OpenTelemetry trace context from incoming requests
pub fn extract_trace_context(headers: &HeaderMap) -> Option<String> {
    // Simple implementation to extract trace context
    headers
        .get("traceparent")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// Create middleware for metrics collection
pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status_code = response.status().as_u16();

    // Record metrics (you would implement actual metrics recording here)
    tracing::info!(
        method = %method,
        path = %path,
        status_code = status_code,
        duration_ms = duration.as_millis() as u64,
        "HTTP request completed"
    );

    response
}
