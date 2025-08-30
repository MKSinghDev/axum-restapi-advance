# Axum REST API Advanced - Vehicle Manager

A lightweight, high-performance vehicle management REST API built with Rust and Axum framework. This project demonstrates modern async Rust web development patterns, modular route organization, type-safe JSON handling, and comprehensive input validation.

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-0.8.4-blue.svg)](https://github.com/tokio-rs/axum)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## 🏢 Features

- **Complete CRUD Operations**: Create, read, and list vehicle records
- **Modular Route Organization**: Dedicated routes module for scalable API structure
- **Production-Grade Observability**: OpenTelemetry integration with tracing, metrics, and structured logging
- **Health Check Endpoints**: Kubernetes-ready health, liveness, and readiness probes
- **Request Tracing**: Automatic request ID generation and distributed tracing
- **Async/Await**: Built on Tokio for high-performance async I/O
- **Type Safety**: Comprehensive use of Rust's type system with Serde
- **Input Validation**: Automatic request validation using the `validator` crate
- **UUID v7**: Modern unique identifier generation
- **In-Memory Storage**: Fast HashMap-based data persistence
- **Error Handling**: Proper HTTP status codes and error responses
- **Custom Extractors**: Validated payload extraction for robust input handling
- **Middleware Integration**: Request processing and metrics collection
- **Graceful Shutdown**: Proper cleanup of resources on application termination

## 📋 API Endpoints

| Method | Endpoint | Description | Request Body | Response |
|--------|----------|-------------|--------------|----------|
| `POST` | `/api/v1/vehicles` | Create a new vehicle | `Vehicle` JSON | `VehicleId` JSON |
| `GET` | `/api/v1/vehicles` | Get all vehicles | None | Array of `Vehicle` JSON |
| `GET` | `/api/v1/vehicles/{id}` | Get vehicle by UUID | None | `Vehicle` JSON |
| `GET` | `/health` | Health check | None | Service status JSON |
| `GET` | `/health/live` | Liveness probe | None | Liveness status JSON |
| `GET` | `/health/ready` | Readiness probe | None | Readiness status JSON |

### Vehicle Model

```rust
{
  "manufacturer": "string",  // 3-25 characters
  "model": "string",        // 3-25 characters  
  "year": "string"          // exactly 4 characters
}
```

### Example Requests

**Create Vehicle:**
```bash
curl -X POST http://localhost:8000/api/v1/vehicles \
  -H "Content-Type: application/json" \
  -d '{
    "manufacturer": "Toyota",
    "model": "Camry", 
    "year": "2023"
  }'
```

**Get All Vehicles:**
```bash
curl http://localhost:8000/api/v1/vehicles
```

**Get Specific Vehicle:**
```bash
curl http://localhost:8000/api/v1/vehicles/{vehicle-id}
```

**Health Check:**
```bash
curl http://localhost:8000/health
```

## 🏗️ Project Architecture

```
src/
├── main.rs              # Application entry point with telemetry
├── routes/
│   ├── mod.rs           # Main router and route organization
│   ├── vehicle.rs       # Vehicle-specific route definitions
│   └── health.rs        # Health check endpoints
├── middlewares/
│   ├── mod.rs           # Middleware module declarations
│   └── tracing.rs       # Request tracing and metrics middleware
├── utils/
│   ├── mod.rs           # Utils module declarations
│   ├── validator.rs     # Custom validation extractor
│   └── opentelemetry.rs # Telemetry configuration
└── features/
    └── vehicle/
        ├── mod.rs       # Vehicle module declaration
        ├── handler.rs   # HTTP request handlers with tracing
        ├── model.rs     # Data models & validation rules
        └── repo.rs      # Data repository & business logic
```

### Architecture Pattern

The project follows a **layered architecture** with clear separation of concerns:

1. **Routes Layer** (`routes/`): Route organization, HTTP method binding, and health endpoints
2. **Middleware Layer** (`middlewares/`): Request processing, tracing, and metrics collection
3. **Handler Layer** (`features/vehicle/handler.rs`): HTTP request/response handling with instrumentation
4. **Repository Layer** (`features/vehicle/repo.rs`): Data access and business logic
5. **Model Layer** (`features/vehicle/model.rs`): Data structures and validation
6. **Utils Layer** (`utils/`): Cross-cutting concerns like validation and telemetry

### Key Design Patterns

- **Repository Pattern**: Abstracted data access through `VehicleRepo` trait
- **Modular Routing**: Dedicated routes module with nested route organization
- **Middleware Pattern**: Request processing and tracing through dedicated middleware
- **State Management**: Shared application state using Axum's `State` extractor
- **Custom Extractors**: `ValidatedPayload` for automatic input validation
- **Instrumentation**: Distributed tracing and observability integration
- **Health Checks**: Kubernetes-ready health endpoints for monitoring

## 🛠️ Technology Stack

| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| **Language** | Rust | 2024 edition | Systems programming language |
| **Web Framework** | Axum | 0.8.4 | Async web framework |
| **Async Runtime** | Tokio | 1.47.1 | Async runtime |
| **Serialization** | Serde | 1.0.219 | JSON serialization/deserialization |
| **Validation** | Validator | 0.20.0 | Input validation |
| **UUID Generation** | UUID | 1.18.0 | Unique identifier generation |
| **Error Handling** | ThisError | 2.0.16 | Error type derivation |
| **Observability** | OpenTelemetry | 0.30.0 | Distributed tracing and metrics |
| **Logging** | Tracing | 0.1.41 | Structured logging and instrumentation |
| **Time Handling** | Chrono | 0.4.38 | Date and time handling |

### Axum Features Used

- `http2`: HTTP/2 support
- `macros`: Procedural macros for handlers
- `tracing`: Built-in tracing support
- `ws`: WebSocket support (available but not used)

## 🚦 Getting Started

### Prerequisites

- **Rust** (2024 edition): Install via [rustup.rs](https://rustup.rs/)
- **Cargo**: Comes with Rust installation

### Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone git@github.com:MKSinghDev/axum-restapi-advance.git
   cd axum-restapi-advance
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Run the application:**
   ```bash
   cargo run
   ```

4. **The server will start on:**
   ```
   🚀 Vehicle Manager service listening on 0.0.0.0:8000
   Health check available at: http://0.0.0.0:8000/health
   Vehicles API available at: http://0.0.0.0:8000/api/v1/vehicles
   ```

### Development Commands

```bash
# Build the project
cargo build

# Run in development mode
cargo run

# Build for production
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Run clippy lints
cargo clippy
```

## 🔧 Configuration

The application supports configuration through:

- **Port**: Default `8000` (configurable via environment)
- **Host**: Binds to `0.0.0.0` for all interfaces
- **Storage**: In-memory HashMap (no persistence)
- **Telemetry**: OpenTelemetry configuration via environment variables
- **Logging**: Structured JSON logging with configurable levels

## 📝 Code Examples

### Modular Route Organization

The project demonstrates clean route organization using Axum's nested routing:

```rust
// routes/mod.rs - Main router with health endpoints
pub fn routes() -> Router<AppState<InMemoryVehicleRepo>> {
    let health_routes = Router::new()
        .route("/", get(health_check))
        .route("/live", get(liveness_check))
        .route("/ready", get(readiness_check));

    Router::new()
        .nest("/health", health_routes)
        .nest("/api/v1", Router::new().nest("/vehicles", vehicle_routes()))
}

// routes/vehicle.rs - Vehicle-specific routes
pub fn vehicle_routes() -> Router<AppState<InMemoryVehicleRepo>> {
    Router::new()
        .route("/", post(post_vehicle).get(get_vehicles))
        .route("/{id}", get(get_vehicle))
}
```

### Custom Validation Extractor

The project includes a custom `ValidatedPayload` extractor that automatically validates incoming JSON:

```rust
pub async fn post_vehicle(
    State(state): State<AppState<InMemoryVehicleRepo>>,
    ValidatedPayload(vehicle): ValidatedPayload<Vehicle>,
) -> Json<VehicleId> {
    // Vehicle is automatically validated here
    let vehicle_id = state.vehicle_repo.post_vehicle(vehicle).await.unwrap();
    Json::from(vehicle_id)
}
```

### Repository Pattern Implementation

```rust
pub trait VehicleRepo: Sync + Send {
    async fn get_vehicle(&self, id: Uuid) -> Option<Vehicle>;
    async fn get_vehicles(&self) -> Vec<Vehicle>;
    async fn post_vehicle(&self, vehicle: Vehicle) -> Option<VehicleId>;
}
```

### Model with Validation

```rust
#[derive(Clone, Deserialize, Serialize, Validate)]
pub struct Vehicle {
    pub id: Option<String>,
    #[validate(length(min = 3, max = 25))]
    pub manufacturer: String,
    #[validate(length(min = 3, max = 25))]
    pub model: String,
    #[validate(length(min = 4, max = 4))]
    pub year: String,
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🔍 Key Learning Points

This project demonstrates several important Rust and Axum concepts:

1. **Modular Architecture**: Organized route structure with dedicated modules for different concerns
2. **Async/Await**: Proper use of async functions and the Tokio runtime
3. **Middleware Integration**: Request processing and tracing through dedicated middleware
4. **Extractors**: Custom extractors for validation and state management
5. **Error Handling**: Implementing `IntoResponse` for custom error types
6. **Traits**: Using traits for abstraction (Repository pattern)
7. **Generics**: Generic state management with `AppState<T>`
8. **Validation**: Compile-time and runtime validation strategies
9. **UUID v7**: Modern UUID generation for better performance
10. **Nested Routing**: Clean API organization using Axum's routing capabilities
11. **Observability**: Production-grade telemetry with OpenTelemetry integration
12. **Health Checks**: Kubernetes-ready health endpoints for monitoring

## 📊 Observability & OpenTelemetry

This project includes a production-grade OpenTelemetry setup for comprehensive observability including traces, metrics, and logs.

### 🔍 Features

- **Distributed Tracing**: Complete request tracing across the application
- **Metrics Collection**: Application and system metrics via Prometheus
- **Structured Logging**: JSON-formatted logs with trace correlation
- **Health Checks**: Kubernetes-ready health endpoints
- **Request Tracking**: Automatic request ID generation and propagation
- **Graceful Shutdown**: Proper telemetry cleanup on application shutdown

### 📈 Observability Stack

| Component | Purpose | URL | Description |
|-----------|---------|-----|-------------|
| **Jaeger** | Distributed Tracing | http://localhost:16686 | View traces and spans |
| **Prometheus** | Metrics Storage | http://localhost:9090 | Query metrics and alerts |
| **Grafana** | Visualization | http://localhost:3000 | Dashboards and monitoring |
| **OTEL Collector** | Telemetry Gateway | http://localhost:8888 | Process and export telemetry |

### 🚀 Quick Start with Observability

1. **Start the observability stack:**
   ```bash
   docker-compose up -d
   ```

2. **Build and run the application:**
   ```bash
   cargo run
   ```

3. **Generate some traffic:**
   ```bash
   # Create a vehicle
   curl -X POST http://localhost:8000/api/v1/vehicles \
     -H "Content-Type: application/json" \
     -d '{"manufacturer": "Toyota", "model": "Camry", "year": "2023"}'
   
   # Get all vehicles
   curl http://localhost:8000/api/v1/vehicles
   
   # Check health
   curl http://localhost:8000/health
   ```

4. **View observability data:**
   - **Traces**: http://localhost:16686 (Jaeger UI)
   - **Metrics**: http://localhost:9090 (Prometheus)
   - **Dashboards**: http://localhost:3000 (Grafana, admin/admin)

### 🔧 Configuration

The OpenTelemetry setup supports configuration via environment variables:

```bash
# Service identification
OTEL_SERVICE_NAME=vehicle-manager-axum
OTEL_SERVICE_VERSION=0.1.0
ENVIRONMENT=production

# OTLP endpoint
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Feature toggles
OTEL_TRACES_ENABLED=true
OTEL_METRICS_ENABLED=true
OTEL_LOGS_ENABLED=true

# Sampling configuration
OTEL_TRACES_SAMPLER_ARG=1.0  # 100% sampling

# Batch configuration
OTEL_BSP_SCHEDULE_DELAY=5000
OTEL_BSP_MAX_EXPORT_BATCH_SIZE=512
OTEL_BSP_MAX_QUEUE_SIZE=2048

# Logging
RUST_LOG=info
```

### 📊 Health Endpoints

The application provides Kubernetes-ready health check endpoints:

| Endpoint | Purpose | Description |
|----------|---------|-------------|
| `/health` | General health | Basic service health check |
| `/health/live` | Liveness probe | Kubernetes liveness check |
| `/health/ready` | Readiness probe | Kubernetes readiness check |

### 🏗️ Observability Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Application    │───▶│  OTEL Collector  │───▶│   Jaeger        │
│  (Axum)         │    │                  │    │   (Traces)      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                        │                       │
        │                        │              ┌─────────────────┐
        │                        └─────────────▶│   Prometheus    │
        │                                       │   (Metrics)     │
        │                                       └─────────────────┘
        │                                                │
        │                               ┌─────────────────┐
        └──────────────────────────────▶│   Grafana       │
                                        │   (Dashboards)  │
                                        └─────────────────┘
```

### 📝 Instrumentation Features

- **Automatic Request Tracing**: Every HTTP request is automatically traced
- **Request ID Propagation**: Unique request IDs for correlation
- **Custom Spans**: Manual instrumentation for business logic
- **Error Tracking**: Automatic error capture in spans
- **Performance Metrics**: Request duration and status code metrics
- **Resource Detection**: Automatic service discovery and metadata

### 🔧 Manual Instrumentation

The telemetry setup allows for custom instrumentation:

```rust
use tracing::{info, instrument};

#[instrument(skip(state), fields(vehicle_id = %id))]
pub async fn get_vehicle(
    State(state): State<AppState<InMemoryVehicleRepo>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vehicle>, StatusCode> {
    info!("Fetching vehicle with ID: {}", id);
    // ... handler implementation
}
```

### 🐳 Docker Compose Services

The `docker-compose.yml` includes:

- **otel-collector**: OpenTelemetry Collector with custom configuration
- **jaeger**: Jaeger all-in-one for tracing
- **prometheus**: Prometheus for metrics storage
- **grafana**: Grafana for visualization with pre-configured datasources
- **vehicle-manager**: The Rust application with telemetry enabled

### 📁 Observability Files

```
observability/
├── otel-collector-config.yaml    # OpenTelemetry Collector configuration
├── prometheus.yml                 # Prometheus scraping configuration
└── grafana/
    ├── dashboards/
    │   └── dashboards.yml         # Dashboard provisioning
    └── datasources/
        └── datasources.yml        # Datasource configuration
```

### 🔒 Production Considerations

- **Security**: Use TLS in production environments
- **Sampling**: Adjust trace sampling rates for high-traffic services
- **Resource Limits**: Configure memory limits for the collector
- **Data Retention**: Set appropriate retention policies for metrics and traces
- **Authentication**: Secure access to observability tools
- **Alerting**: Set up alerts based on metrics and trace data

### 🚧 Current Limitations

- **No Persistence**: Data is stored in memory only (lost on restart)
- **No Authentication**: No security layer implemented
- **Limited CRUD**: Only CREATE and READ operations implemented
- **Simplified Telemetry**: Basic OpenTelemetry setup (can be extended with full OTLP)
- **No Configuration Management**: Environment-based configuration partially implemented

## 🔮 Future Improvements

- [ ] Add database integration (PostgreSQL/SQLite)
- [ ] Implement UPDATE and DELETE operations
- [ ] Add authentication and authorization
- [x] Implement structured logging (tracing) ✓
- [ ] Add configuration management
- [ ] Add comprehensive test suite
- [ ] Add OpenAPI/Swagger documentation
- [x] Implement graceful shutdown ✓
- [x] Add metrics and health checks ✓
- [ ] Add rate limiting
- [ ] Add pagination for vehicle listing
- [ ] Implement vehicle search and filtering
- [x] Add OpenTelemetry observability ✓
- [x] Add distributed tracing ✓
- [x] Add Prometheus metrics ✓
- [x] Add health check endpoints ✓

## 📚 Repository

This project is available on GitHub: [axum-restapi-advance](https://github.com/MKSinghDev/axum-restapi-advance)

```bash
# Clone with SSH
git clone git@github.com:MKSinghDev/axum-restapi-advance.git

# Clone with HTTPS
git clone https://github.com/MKSinghDev/axum-restapi-advance.git
```

## 📚 Learning Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Serde Guide](https://serde.rs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Validator Crate](https://docs.rs/validator/)

## 🤝 Contributing

This is a learning project, but contributions and suggestions are welcome! Feel free to:

- Open issues for bugs or suggestions
- Submit pull requests for improvements
- Share feedback on the architecture and code quality

## 📄 License

This project is for educational purposes. Feel free to use it as a reference for your own learning journey!

---

*Built with ❤️ and Rust for learning modern web development patterns*