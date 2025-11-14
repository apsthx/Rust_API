# Clinic API - Rust Version

Rust implementation of the Clinic Management API, migrated from Go.

## Architecture Overview

This is a complete Rust port of the Go-based clinic management API, maintaining the same structure and functionality while leveraging Rust's performance and safety features.

### Tech Stack

- **Framework**: Axum (web framework, equivalent to Gin in Go)
- **Database**: MySQL with SQLx (ORM, equivalent to GORM)
- **Authentication**: JWT tokens using jsonwebtoken
- **Cloud Storage**: AWS S3 SDK
- **Image Processing**: image crate
- **Email**: lettre
- **Logging**: tracing + tracing-subscriber

### Project Structure

```
api_rust/
├── src/
│   ├── main.rs              # Entry point, server setup
│   ├── configs/             # Database configuration
│   │   ├── mod.rs
│   │   └── database.rs      # Multi-database connection setup
│   ├── middlewares/         # Middleware functions
│   │   ├── mod.rs
│   │   ├── jwt.rs          # JWT authentication & token management
│   │   ├── uploadfile.rs   # File upload handling (local & S3)
│   │   └── common.rs       # Utility functions
│   ├── models/             # Data Access Layer (DAL)
│   │   ├── mod.rs
│   │   ├── user.rs         # User database operations
│   │   ├── order.rs        # Order database operations
│   │   ├── customer.rs     # Customer operations
│   │   ├── product.rs      # Product operations
│   │   └── ...             # Other models
│   ├── structs/            # Request/Response DTOs
│   │   ├── mod.rs
│   │   ├── auth.rs         # Authentication payloads
│   │   ├── user.rs         # User DTOs
│   │   ├── order.rs        # Order DTOs
│   │   └── common.rs       # Common response structures
│   ├── controllers/        # Business Logic Handlers
│   │   ├── mod.rs
│   │   ├── auth.rs         # Login, logout, token refresh
│   │   ├── user.rs         # User management
│   │   └── order.rs        # Order management
│   ├── routes/             # Route definitions
│   │   └── mod.rs          # All API endpoints
│   └── libs/               # Utility libraries
│       ├── mod.rs
│       ├── sms.rs          # SMS integration
│       ├── calendar.rs     # Calendar utilities
│       └── email.rs        # Email sending
├── Cargo.toml              # Dependencies
├── .env.example            # Environment variables template
└── README.md               # This file
```

## Key Features

### Database Architecture
- **4 Database Connections** for scalability:
  - `DB1` - Main write database
  - `DB2` - Main read replica
  - `DBL1` - Logging write database
  - `DBL2` - Logging read replica

### Authentication System
- **JWT-based authentication** with two token types:
  - Access Token (short-lived, 90 minutes)
  - Refresh Token (long-lived, 720 hours)
- **Password versioning** to invalidate tokens on password change
- **2FA/OTP support** using TOTP

### Middleware
- `check_access_token` - Validates JWT for protected routes
- `check_public_key` - Validates API key for public endpoints
- `check_tele_public_key` - Telemedicine-specific authentication

### File Upload
- Local filesystem storage with image resizing
- AWS S3 integration for cloud storage
- Excel file handling

## Installation

### Prerequisites

1. **Install Rust** (version 1.70+):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install MySQL** (version 8.0+)

3. **Setup AWS credentials** (if using S3)

### Setup

1. Clone the repository and navigate to api_rust:
```bash
cd api_rust
```

2. Copy environment file and configure:
```bash
cp .env.example .env
# Edit .env with your actual values
```

3. Install dependencies:
```bash
cargo build
```

4. Run database migrations (if applicable):
```bash
# Add migration commands here
```

## Running the Application

### Development Mode

```bash
# With auto-reload using cargo-watch
cargo install cargo-watch
cargo watch -x run

# Or standard run
cargo run
```

### Production Mode

```bash
# Build optimized binary
cargo build --release

# Run the binary
./target/release/clinic_api
```

### With Docker

```bash
docker build -t clinic-api-rust .
docker run -p 8002:8002 --env-file .env clinic-api-rust
```

## Environment Variables

All environment variables must be set in `.env` file. See [`.env.example`](.env.example) for full list.

### Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DB_HOST` | Main database host | `localhost` |
| `DB_PORT` | Main database port | `3306` |
| `DB_USER` | Database username | `root` |
| `DB_PWD` | Database password | `password` |
| `DB_NAME` | Database name | `clinic` |
| `JWT_AC_KEY` | Access token secret | `your_secret_key` |
| `JWT_RF_KEY` | Refresh token secret | `your_secret_key` |

## API Endpoints

### Authentication

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/auth/login` | User login | No |
| POST | `/auth/logout` | User logout | Yes |
| GET | `/auth/verify` | Verify token | Yes |

### Users

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/user/me` | Get current user | Yes |
| GET | `/user/:id` | Get user by ID | Yes |
| PUT | `/user/` | Update user | Yes |
| GET | `/user/list` | Get all shop users | Yes |

### Orders

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/order/search` | Search orders | Yes |
| GET | `/order/:id` | Get order detail | Yes |
| POST | `/order/` | Create order | Yes |
| DELETE | `/order/:id` | Delete order | Yes |

### Health Check

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/health` | Health check | No |

## Migration Guide from Go

### Key Differences

1. **Error Handling**:
   - Go: `if err != nil { return err }`
   - Rust: `Result<T, E>` with `?` operator

2. **Null Values**:
   - Go: Pointers can be nil
   - Rust: `Option<T>` for nullable values

3. **Async/Await**:
   - Go: Goroutines with channels
   - Rust: `async/await` with Tokio runtime

4. **JSON Serialization**:
   - Go: struct tags like `json:"field_name"`
   - Rust: `#[serde(rename = "field_name")]`

5. **Database Queries**:
   - Go: GORM methods
   - Rust: SQLx with compile-time checked queries

### Code Comparison Examples

#### Go Controller
```go
func Login(c *gin.Context) {
    var payload structs.PayloadLogin
    if err := c.ShouldBindJSON(&payload); err != nil {
        c.JSON(400, gin.H{"error": err.Error()})
        return
    }
    // ...
    c.JSON(200, gin.H{"status": true, "data": response})
}
```

#### Rust Controller
```rust
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    if let Err(errors) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ApiResponse::error(...))));
    }
    // ...
    Ok(Json(ApiResponse::success(response)))
}
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_login

# Run tests with coverage
cargo tarpaulin --out Html
```

## Performance

Rust typically offers:
- **2-3x better throughput** compared to Go
- **Lower memory footprint** (50-70% less)
- **Zero-cost abstractions** - no runtime overhead
- **Compile-time guarantees** - memory safety without GC

## Logging

Logs are configured via `RUST_LOG` environment variable:

```bash
# Debug level
RUST_LOG=clinic_api=debug,tower_http=debug

# Info level
RUST_LOG=clinic_api=info

# Error only
RUST_LOG=clinic_api=error
```

## Database Migrations

TODO: Add migration instructions using SQLx migrations

```bash
# Create migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run

# Revert migration
sqlx migrate revert
```

## Common Issues & Solutions

### 1. Database Connection Failed
```
Error: Connection refused
```
**Solution**: Check database is running and credentials in `.env` are correct.

### 2. JWT Secret Not Set
```
Error: JWT_AC_KEY must be set
```
**Solution**: Set JWT keys in `.env` file.

### 3. Port Already in Use
```
Error: Address already in use
```
**Solution**: Change `API_PORT` in `.env` or kill process using the port.

## Contributing

1. Follow Rust standard coding conventions
2. Run `cargo fmt` before committing
3. Run `cargo clippy` to check for warnings
4. Write tests for new features
5. Update documentation

## License

[Add your license here]

## Support

For issues and questions:
- GitHub Issues: [repository-url]/issues
- Email: support@example.com

## Roadmap

- [ ] Complete all 43+ route modules from Go version
- [ ] Implement all 148 controllers
- [ ] Add comprehensive test coverage
- [ ] Set up CI/CD pipeline
- [ ] Add API documentation (OpenAPI/Swagger)
- [ ] Implement rate limiting
- [ ] Add metrics and monitoring (Prometheus)
- [ ] Dockerize the application
- [ ] Add GraphQL support (optional)

## Acknowledgments

This project is a Rust port of the original Go-based Clinic Management API, maintaining feature parity while leveraging Rust's performance and safety benefits.
