mod features;
mod middlewares;
mod routes;
mod utils;

use crate::{
    features::vehicle::repo::InMemoryVehicleRepo,
    middlewares::tracing::{metrics_middleware, tracing_middleware},
    routes::routes,
    utils::opentelemetry::init_telemetry,
};
use axum::middleware;
use tokio::net::TcpListener;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct AppState<T> {
    vehicle_repo: T,
}

#[tokio::main]
async fn main() {
    // Initialize telemetry first, before any other operations
    let _telemetry_guard = match init_telemetry().await {
        Ok(guard) => {
            info!("OpenTelemetry initialized successfully");
            Some(guard)
        }
        Err(e) => {
            eprintln!("Failed to initialize OpenTelemetry: {}", e);
            warn!("Continuing without OpenTelemetry");
            None
        }
    };

    let vehicle_repo = InMemoryVehicleRepo::default();

    // Build the application with middleware layers
    let app = routes()
        .layer(middleware::from_fn(tracing_middleware))
        .layer(middleware::from_fn(metrics_middleware))
        .with_state(AppState { vehicle_repo });

    let listener = match TcpListener::bind("0.0.0.0:8000").await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to address: {}", e);
            std::process::exit(1);
        }
    };

    info!(
        "🚀 Vehicle Manager service listening on {}",
        listener.local_addr().unwrap()
    );
    info!("Health check available at: http://0.0.0.0:8000/health");
    info!("Vehicles API available at: http://0.0.0.0:8000/api/v1/vehicles");

    // Set up graceful shutdown
    let server = axum::serve(listener, app);

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal, shutting down gracefully...");
        }
    }

    // Cleanup telemetry resources
    if let Some(guard) = _telemetry_guard {
        info!("Shutting down telemetry...");
        guard.shutdown().await;
    }

    info!("Server shutdown complete");
}
