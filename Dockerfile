# Stage 1: Build the Frontend (WASM)
FROM rust:latest AS frontend-builder
WORKDIR /usr/src/app

# Install trunk and add wasm target
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

# Copy entire project to satisfy workspace dependencies
COPY . .

# Build frontend
WORKDIR /usr/src/app/frontend
RUN trunk build --release

# Stage 2: Build the Backend
FROM rust:latest AS backend-builder
WORKDIR /usr/src/app

# Copy entire project
COPY . .

# Enable SQLx offline mode
ENV SQLX_OFFLINE=true

# Build backend binary
RUN cargo build --release --bin backend

# Stage 3: Runtime Environment
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/local/bin

# Copy backend binary
COPY --from=backend-builder /usr/src/app/target/release/backend /usr/local/bin/backend

# Copy frontend static files
# We copy them to ./static because the backend is configured to serve from there
COPY --from=frontend-builder /usr/src/app/frontend/dist /usr/local/bin/static

# Expose port
EXPOSE 8080

# Environment variables
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

# Run application
CMD ["backend"]
