//! Main entry point for the rbibli backend application.
//!
//! This file initializes the application, sets up logging, connects to the database,
//! and starts the HTTP server. It uses the `backend::run` function to configure
//! the Actix Web server and its routes.

use backend::run;
use log::{info, error};
use sqlx::mysql::MySqlPoolOptions;
use std::net::TcpListener;

/// Main entry point for the rbibli backend server.
///
/// This asynchronous function initializes and starts the rbibli (personal library management)
/// backend API server. It performs the following initialization steps:
///
/// 1. **Logging Setup**: Configures the logging system using `env_logger`. If the `RUST_LOG`
///    environment variable is not set, it defaults to the "info" level.
///
/// 2. **Environment Configuration**: Loads environment variables from a `.env` file using
///    the `dotenv` crate. Required variables include:
///    - `DATABASE_URL`: Connection string for the MariaDB database
///    - `HOST`: Server host address (defaults to "127.0.0.1")
///    - `PORT`: Server port number (defaults to "8000")
///
/// 3. **Database Connection**: Establishes a connection pool to the MariaDB database using
///    SQLx with a maximum of 5 connections.
///
/// 4. **Server Startup**: Creates a TCP listener on the configured host:port and starts
///    the actix-web HTTP server with all configured routes.
///
/// # Returns
///
/// * `Ok(())` - Server started successfully and shut down gracefully
/// * `Err(std::io::Error)` - Server failed to start due to:
///   - Missing or invalid `DATABASE_URL` environment variable
///   - Database connection failure
///   - Failed to bind to the specified address (port already in use, invalid address, etc.)
///   - Server runtime error
///
/// # Panics
///
/// The function will panic if:
/// - `DATABASE_URL` environment variable is not set
///
/// # Example
///
/// To run the server:
/// ```bash
/// # Create a .env file with:
/// # DATABASE_URL=mysql://user:password@localhost:3306/rbibli
/// # HOST=127.0.0.1
/// # PORT=8000
///
/// cargo run
/// ```
///
/// # Safety
///
/// Uses `unsafe` to set the `RUST_LOG` environment variable before any threads are spawned.
/// This is safe because it occurs at the very beginning of the main function before any
/// other initialization.
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



    // Load configuration
    let configuration = backend::configuration::get_configuration().expect("Failed to read configuration.");

    // Get configuration from environment
    let database_url = configuration.database.connection_string();
    let host = configuration.application.host;
    let port = configuration.application.port;
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

    // Run database migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| {
            error!("Failed to run database migrations: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
    info!("Database migrations applied successfully");

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
