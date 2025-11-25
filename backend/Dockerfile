FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
# Enable SQLx offline mode
ENV SQLX_OFFLINE=true
# Build the backend binary
# We use --bin backend to ensure we only build the backend binary
RUN cargo build --release --bin backend

FROM debian:bookworm-slim
# Install runtime dependencies
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

# Set the working directory
WORKDIR /usr/local/bin

# Expose the application port
EXPOSE 8080

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=8080

# Run the application
CMD ["backend"]
