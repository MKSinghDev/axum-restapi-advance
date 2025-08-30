use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{info, instrument, warn};
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
#[instrument(skip(state), fields(vehicle_id = %id))]
pub async fn get_vehicle(
    State(state): State<AppState<InMemoryVehicleRepo>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vehicle>, StatusCode> {
    info!("Fetching vehicle with ID: {}", id);
    
    match state.vehicle_repo.get_vehicle(id).await {
        Some(vehicle) => {
            info!("Vehicle found: {:?}", vehicle);
            Ok(Json::from(vehicle))
        }
        None => {
            warn!("Vehicle not found with ID: {}", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

#[debug_handler]
#[instrument(skip(state))]
pub async fn get_vehicles(
    State(state): State<AppState<InMemoryVehicleRepo>>,
) -> Result<Json<Vec<Vehicle>>, StatusCode> {
    info!("Fetching all vehicles");
    
    let vehicles = state.vehicle_repo.get_vehicles().await;
    
    info!("Found {} vehicles", vehicles.len());
    Ok(Json::from(vehicles))
}

#[debug_handler]
#[instrument(skip(state, v), fields(vehicle_manufacturer = %v.manufacturer, vehicle_model = %v.model))]
pub async fn post_vehicle(
    State(state): State<AppState<InMemoryVehicleRepo>>,
    ValidatedPayload(v): ValidatedPayload<Vehicle>,
) -> Json<VehicleId> {
    info!("Creating new vehicle: {} {}", v.manufacturer, v.model);
    
    let vehicle_id = state.vehicle_repo.post_vehicle(v).await.unwrap();
    
    info!("Vehicle created with ID: {}", vehicle_id.id);
    Json::from(vehicle_id)
}
