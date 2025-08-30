use tracing::info;
use tracing_subscriber::{
    Layer, filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

/// Error types for telemetry initialization
#[derive(thiserror::Error, Debug)]
pub enum TelemetryError {
    #[error("Failed to initialize tracer: {0}")]
    TracerInit(String),
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Simple telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub service_name: String,
    pub service_version: String,
    pub environment: String,
    pub otlp_endpoint: String,
    pub enable_tracing: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: "vehicle-manager-axum".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            otlp_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string()),
            enable_tracing: std::env::var("OTEL_TRACES_ENABLED")
                .map(|v| v.parse().unwrap_or(true))
                .unwrap_or(true),
        }
    }
}

/// Simple telemetry initialization
pub async fn init_telemetry() -> Result<TelemetryGuard, TelemetryError> {
    let config = TelemetryConfig::default();
    init_telemetry_with_config(config).await
}

/// Initialize telemetry with custom configuration
pub async fn init_telemetry_with_config(
    config: TelemetryConfig,
) -> Result<TelemetryGuard, TelemetryError> {
    info!("Initializing telemetry with config: {:?}", config);

    // Initialize basic tracing subscriber
    init_tracing_subscriber(&config)?;

    info!("Telemetry initialization completed successfully");
    Ok(TelemetryGuard {})
}

/// Initialize tracing subscriber with JSON formatting
fn init_tracing_subscriber(config: &TelemetryConfig) -> Result<(), TelemetryError> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .map_err(|e| TelemetryError::Config(e.to_string()))?;

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .json()
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .try_init()
        .map_err(|e| TelemetryError::Config(e.to_string()))?;

    info!(
        "Structured logging initialized for service: {}",
        config.service_name
    );
    Ok(())
}

/// Guard for cleanup
pub struct TelemetryGuard {}

impl TelemetryGuard {
    /// Gracefully shutdown telemetry providers
    pub async fn shutdown(self) {
        info!("Shutting down telemetry...");
        // Placeholder for future cleanup operations
    }
}

/// Backward compatibility function
pub fn init_trace() -> Result<(), ()> {
    // Simple initialization for backward compatibility
    Ok(())
}
