pub mod health;
pub mod vehicle;

use crate::{
    AppState,
    features::vehicle::repo::InMemoryVehicleRepo,
    routes::{
        health::{health_check, liveness_check, readiness_check},
        vehicle::vehicle_routes,
    },
};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState<InMemoryVehicleRepo>> {
    let health_routes = Router::new()
        .route("/", get(health_check))
        .route("/live", get(liveness_check))
        .route("/ready", get(readiness_check));

    Router::new()
        .nest("/health", health_routes)
        // API v1 routes
        .nest("/api/v1", Router::new().nest("/vehicles", vehicle_routes()))
}
