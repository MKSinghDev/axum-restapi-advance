# Axum REST API Advanced - Vehicle Manager

A lightweight, high-performance vehicle management REST API built with Rust and Axum framework. This project demonstrates modern async Rust web development patterns, modular route organization, type-safe JSON handling, and comprehensive input validation.

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-0.8.4-blue.svg)](https://github.com/tokio-rs/axum)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## 🚀 Features

- **Complete CRUD Operations**: Create, read, and list vehicle records
- **Modular Route Organization**: Dedicated routes module for scalable API structure
- **Async/Await**: Built on Tokio for high-performance async I/O
- **Type Safety**: Comprehensive use of Rust's type system with Serde
- **Input Validation**: Automatic request validation using the `validator` crate
- **UUID v7**: Modern unique identifier generation
- **In-Memory Storage**: Fast HashMap-based data persistence
- **Error Handling**: Proper HTTP status codes and error responses
- **Custom Extractors**: Validated payload extraction for robust input handling

## 📋 API Endpoints

| Method | Endpoint | Description | Request Body | Response |
|--------|----------|-------------|--------------|----------|
| `POST` | `/vehicles` | Create a new vehicle | `Vehicle` JSON | `VehicleId` JSON |
| `GET` | `/vehicles` | Get all vehicles | None | Array of `Vehicle` JSON |
| `GET` | `/vehicles/{id}` | Get vehicle by UUID | None | `Vehicle` JSON |

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
curl -X POST http://localhost:8000/vehicles \
  -H "Content-Type: application/json" \
  -d '{
    "manufacturer": "Toyota",
    "model": "Camry", 
    "year": "2023"
  }'
```

**Get All Vehicles:**
```bash
curl http://localhost:8000/vehicles
```

**Get Specific Vehicle:**
```bash
curl http://localhost:8000/vehicles/{vehicle-id}
```

## 🏗️ Project Architecture

```
src/
├── main.rs           # Application entry point
├── routes/
│   ├── mod.rs        # Routes module & main router
│   └── vehicle.rs    # Vehicle-specific route definitions
├── utils/
│   ├── mod.rs        # Utils module declaration
│   └── validator.rs  # Custom validation extractor
└── vehicle/
    ├── mod.rs        # Vehicle module declaration
    ├── handler.rs    # HTTP request handlers
    ├── model.rs      # Data models & validation rules
    └── repo.rs       # Data repository & business logic
```

### Architecture Pattern

The project follows a **layered architecture** with clear separation of concerns:

1. **Routes Layer** (`routes/`): Route organization and HTTP method binding
2. **Handler Layer** (`handler.rs`): HTTP request/response handling
3. **Repository Layer** (`repo.rs`): Data access and business logic
4. **Model Layer** (`model.rs`): Data structures and validation
5. **Utils Layer** (`utils/`): Cross-cutting concerns like validation

### Key Design Patterns

- **Repository Pattern**: Abstracted data access through `VehicleRepo` trait
- **Modular Routing**: Dedicated routes module with nested route organization
- **State Management**: Shared application state using Axum's `State` extractor
- **Custom Extractors**: `ValidatedPayload` for automatic input validation
- **Error Handling**: Custom error types with proper HTTP responses

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

### Axum Features Used

- `http2`: HTTP/2 support
- `macros`: Procedural macros for handlers
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
   🚀 axra-core listening on 0.0.0.0:8000
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

The application is configured through:

- **Port**: Hardcoded to `8000` (modify in `main.rs`)
- **Host**: Binds to `0.0.0.0` for all interfaces
- **Storage**: In-memory HashMap (no persistence)

## 📝 Code Examples

### Modular Route Organization

The project demonstrates clean route organization using Axum's nested routing:

```rust
// routes/mod.rs
pub fn routes() -> Router<AppState<InMemoryVehicleRepo>> {
    Router::new().nest("/vehicles", vehicle_routes())
}

// routes/vehicle.rs
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

1. **Modular Architecture**: Organized route structure with dedicated modules
2. **Async/Await**: Proper use of async functions and the Tokio runtime
3. **Extractors**: Custom extractors for validation and state management
4. **Error Handling**: Implementing `IntoResponse` for custom error types
5. **Traits**: Using traits for abstraction (Repository pattern)
6. **Generics**: Generic state management with `AppState<T>`
7. **Validation**: Compile-time and runtime validation strategies
8. **UUID v7**: Modern UUID generation for better performance
9. **Nested Routing**: Clean API organization using Axum's routing capabilities

## 🚧 Current Limitations

- **No Persistence**: Data is stored in memory only (lost on restart)
- **No Authentication**: No security layer implemented
- **Limited CRUD**: Only CREATE and READ operations implemented
- **No Logging**: No structured logging implementation
- **No Configuration**: Hardcoded configuration values

## 🔮 Future Improvements

- [ ] Add database integration (PostgreSQL/SQLite)
- [ ] Implement UPDATE and DELETE operations
- [ ] Add authentication and authorization
- [ ] Implement structured logging (tracing)
- [ ] Add configuration management
- [ ] Add comprehensive test suite
- [ ] Add OpenAPI/Swagger documentation
- [ ] Implement graceful shutdown
- [ ] Add metrics and health checks
- [ ] Add rate limiting
- [ ] Add pagination for vehicle listing
- [ ] Implement vehicle search and filtering

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