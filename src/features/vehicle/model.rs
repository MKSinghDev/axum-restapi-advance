use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Vehicle {
    pub id: Option<String>,
    #[validate(length(
        min = 3,
        max = 25,
        message = "manufacturer must be between 3 and 25 characters"
    ))]
    pub manufacturer: String,
    #[validate(length(
        min = 3,
        max = 25,
        message = "model must be between 3 and 25 characters"
    ))]
    pub model: String,
    #[validate(length(min = 4, max = 4, message = "year must be exactly 4 characters"))]
    pub year: String,
}

#[derive(Serialize)]
pub struct VehicleId {
    pub id: String,
}
