use crate::{
    AppState,
    features::vehicle::{
        handler::{get_vehicle, get_vehicles, post_vehicle},
        repo::InMemoryVehicleRepo,
    },
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn vehicle_routes() -> Router<AppState<InMemoryVehicleRepo>> {
    Router::new()
        .route("/", post(post_vehicle).get(get_vehicles))
        .route("/{id}", get(get_vehicle))
}
