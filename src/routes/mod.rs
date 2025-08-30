pub mod vehicle;

use crate::{AppState, routes::vehicle::vehicle_routes, vehicle::repo::InMemoryVehicleRepo};
use axum::Router;

pub fn routes() -> Router<AppState<InMemoryVehicleRepo>> {
    Router::new().nest("/vehicles", vehicle_routes())
}

