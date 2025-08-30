use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use tracing::info;

/// Health check endpoint for monitoring and load balancer probes
pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    info!("Health check requested");
    
    Ok(Json(json!({
        "status": "healthy",
        "service": "vehicle-manager-axum",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Readiness check for Kubernetes readiness probes
pub async fn readiness_check() -> Result<Json<Value>, StatusCode> {
    info!("Readiness check requested");
    
    // In a real application, you would check:
    // - Database connectivity
    // - External service dependencies
    // - Required configuration presence
    
    Ok(Json(json!({
        "status": "ready",
        "service": "vehicle-manager-axum",
        "checks": {
            "database": "ok",
            "external_apis": "ok"
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Liveness probe for Kubernetes liveness checks
pub async fn liveness_check() -> Result<Json<Value>, StatusCode> {
    info!("Liveness check requested");
    
    Ok(Json(json!({
        "status": "alive",
        "service": "vehicle-manager-axum",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}