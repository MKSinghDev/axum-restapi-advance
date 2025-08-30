mod routes;
mod utils;
mod vehicle;

use crate::{routes::routes, vehicle::repo::InMemoryVehicleRepo};
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState<T> {
    vehicle_repo: T,
}

#[tokio::main]
async fn main() {
    let vehicle_repo = InMemoryVehicleRepo::default();

    let app = routes().with_state(AppState { vehicle_repo });
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!(
        "🚀 axra-core listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
