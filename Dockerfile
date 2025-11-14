# Multi-stage Dockerfile for Rust Clinic API

# Build stage
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 appuser

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/clinic_api /app/clinic_api

# Copy .env.example (user should mount actual .env)
COPY .env.example /app/.env.example

# Create upload directories
RUN mkdir -p uploads/images uploads/excels && \
    chown -R appuser:appuser /app

USER appuser

EXPOSE 8002

CMD ["./clinic_api"]
