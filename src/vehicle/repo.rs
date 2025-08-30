use crate::vehicle::model::{Vehicle, VehicleId};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

pub trait VehicleRepo: Sync + Send {
    async fn get_vehicle(&self, id: Uuid) -> Option<Vehicle>;
    async fn get_vehicles(&self) -> Vec<Vehicle>;
    async fn post_vehicle(&self, vehicle: Vehicle) -> Option<VehicleId>;
}

#[derive(Clone, Default)]
pub struct InMemoryVehicleRepo {
    pub map: Arc<Mutex<HashMap<Uuid, Vehicle>>>,
}

impl VehicleRepo for InMemoryVehicleRepo {
    async fn get_vehicle(&self, id: Uuid) -> Option<Vehicle> {
        self.map.lock().unwrap().get(&id).cloned()
    }

    async fn get_vehicles(&self) -> Vec<Vehicle> {
        self.map.lock().unwrap().values().cloned().collect()
    }

    async fn post_vehicle(&self, vehicle: Vehicle) -> Option<VehicleId> {
        let id = Uuid::now_v7();
        self.map.lock().unwrap().insert(
            id,
            Vehicle {
                id: Some(id.to_string()),
                manufacturer: vehicle.manufacturer,
                model: vehicle.model,
                year: vehicle.year,
            },
        );

        Some(VehicleId { id: id.to_string() })
    }
}
