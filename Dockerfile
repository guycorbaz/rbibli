# Stage 1: Builder
# We use a single builder stage to build both frontend and backend
# This avoids copying source files multiple times
FROM rust:latest AS builder

# Install trunk for frontend build
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app
COPY . .

# Build Frontend
# The output will be in frontend/dist
WORKDIR /usr/src/app/frontend
RUN trunk build --release

# Build Backend
WORKDIR /usr/src/app
# We use --bin backend to ensure we only build the backend binary
RUN cargo build --release --bin backend

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the backend binary
COPY --from=builder /usr/src/app/target/release/backend ./backend

# Copy the frontend assets to a 'static' directory next to the binary
# The backend is configured to serve files from ./static
COPY --from=builder /usr/src/app/frontend/dist ./static

# Expose the application port
EXPOSE 8080

# Set default environment variables
# These can be overridden by docker-compose or runtime env vars
ENV APP_APPLICATION__HOST=0.0.0.0
ENV APP_APPLICATION__PORT=8080
ENV RUST_LOG=info

# Run the application
CMD ["./backend"]
