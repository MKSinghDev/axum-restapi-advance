pub mod vehicle;

use crate::{
    AppState, features::vehicle::repo::InMemoryVehicleRepo, routes::vehicle::vehicle_routes,
};
use axum::Router;

pub fn routes() -> Router<AppState<InMemoryVehicleRepo>> {
    Router::new().nest("/vehicles", vehicle_routes())
}
