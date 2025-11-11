use backend::run;
use log::{info, error};
use sqlx::mysql::MySqlPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize logger
    // Set default log level to info if RUST_LOG is not set
    if std::env::var("RUST_LOG").is_err() {
        // SAFETY: This is safe because we're setting it at the very start of main,
        // before any threads are spawned or other code runs
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    info!("Starting rbibli backend application");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get configuration from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let address = format!("{}:{}", host, port);

    info!("Configuration loaded: host={}, port={}", host, port);

    // Create database connection pool
    info!("Connecting to database...");
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

    info!("Database connection established successfully");
    info!("Starting server on {}", address);

    // Create TCP listener
    let listener = TcpListener::bind(&address)
        .map_err(|e| {
            error!("Failed to bind to address {}: {}", address, e);
            e
        })?;

    info!("Server bound to {}", address);

    // Run the server
    run(listener, db_pool).await?.await
}
