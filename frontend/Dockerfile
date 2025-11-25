FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
# Install trunk
RUN cargo install trunk
# Add wasm target
RUN rustup target add wasm32-unknown-unknown
# Build the frontend
RUN trunk build --release

FROM nginx:alpine
# Copy the built artifacts from the builder stage
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
# Copy the custom Nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
