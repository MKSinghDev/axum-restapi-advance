use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    features::vehicle::{
        model::{Vehicle, VehicleId},
        repo::{InMemoryVehicleRepo, VehicleRepo},
    },
    utils::validator::ValidatedPayload,
};

#[debug_handler]
pub async fn get_vehicle(
    State(state): State<AppState<InMemoryVehicleRepo>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vehicle>, StatusCode> {
    match state.vehicle_repo.get_vehicle(id).await {
        Some(vehicle) => Ok(Json::from(vehicle)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[debug_handler]
pub async fn get_vehicles(
    State(state): State<AppState<InMemoryVehicleRepo>>,
) -> Result<Json<Vec<Vehicle>>, StatusCode> {
    let vehicles = state.vehicle_repo.get_vehicles().await;
    Ok(Json::from(vehicles))
}

#[debug_handler]
pub async fn post_vehicle(
    State(state): State<AppState<InMemoryVehicleRepo>>,
    ValidatedPayload(v): ValidatedPayload<Vehicle>,
) -> Json<VehicleId> {
    let vehicle_id = state.vehicle_repo.post_vehicle(v).await.unwrap();
    Json::from(vehicle_id)
}
