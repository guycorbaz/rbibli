use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use log::{info, error, debug};
use sqlx::MySqlPool as Pool;
use std::net::TcpListener;

// Module declarations
pub mod models;
pub mod handlers;


// Application state that holds the database pool
pub struct AppState {
    pub db_pool: Pool,
}


/// Greets the user with a personalized message.
///
/// This asynchronous function serves as a request handler for the `actix-web` framework.
/// It extracts a `name` parameter from the request path. If a name is provided,
/// it returns a "Hello {name}!" message. If the `name` parameter is absent,
/// it defaults to "World".
///
/// It also logs a debug message indicating that the endpoint was called
/// and which name was used.
///
/// # Arguments
///
/// * `req` - An `HttpRequest` object from which the `name` parameter is extracted.
///
/// # Returns
///
/// An object that implements `impl Responder`, which resolves to a string
/// containing the greeting.
///
/// # Example Routes
///
/// ```
/// App::new()
///     .route("/", web::get().to(greet))       // Responds with "Hello World!"
///     .route("/{name}", web::get().to(greet)) // Responds with "Hello {name}!"
/// ```
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    debug!("Greet endpoint called with name: {}", name);
    format!("Hello {}!", &name)
}


/// Performs a basic health check of the service.
///
/// This endpoint is designed to be used by service orchestrators like Kubernetes or Docker
/// to verify that the application is running and responsive. It's a simple check that
/// doesn't involve any dependencies like databases.
///
/// It logs a debug message when called and always returns an `HTTP 200 OK` response
/// to indicate that the service is alive.
///
/// # Returns
///
/// An `impl Responder` that resolves to an `HttpResponse::Ok()`.
///
/// # Example Route
///
/// ```
/// App::new()
///     .route("/health", web::get().to(health_check))
/// ```
async fn health_check() -> impl Responder {
    debug!("Health check endpoint called");
    HttpResponse::Ok()
}


/// Checks the health of the database connection.
///
/// This asynchronous function serves as a dedicated health check endpoint for the database.
/// It attempts to execute a simple query ("SELECT 1") against the MariaDB database
/// to verify that the connection is alive and operational.
///
/// # Arguments
///
/// * `data` - A `web::Data<AppState>` object containing the shared database pool (`db_pool`).
///
/// # Returns
///
/// An `impl Responder` that resolves to one of the following:
/// - `HttpResponse::Ok()` with a JSON body `{"status": "ok", "database": "connected"}`
///   if the query is successful.
/// - `HttpResponse::ServiceUnavailable()` with a JSON body detailing the error
///   if the connection or query fails.
///
/// # Example Route
///
/// ```
/// App::new()
///     .route("/health/db", web::get().to(db_health_check))
/// ```
async fn db_health_check(data: web::Data<AppState>) -> impl Responder {
    debug!("Database health check endpoint called");
    match sqlx::query("SELECT 1").fetch_one(&data.db_pool).await {
        Ok(_) => {
            debug!("Database health check: OK");
            HttpResponse::Ok().json(serde_json::json!({
                "status": "ok",
                "database": "connected"
            }))
        }
        Err(e) => {
            error!("Database health check failed: {}", e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "database": "disconnected",
                "error": e.to_string()
            }))
        }
    }
}


/// Configures and starts the HTTP server.
///
/// This function sets up the `actix-web` `HttpServer`, configures its routes,
/// and binds it to a given `TcpListener`. It wraps the database connection pool
/// (`db_pool`) in `web::Data` to make it accessible to all request handlers in a
/// thread-safe manner.
///
/// The server is configured with several routes, including basic greeting endpoints,
/// health checks for the service and database, and the main API routes.
///
/// # Arguments
///
/// * `listener` - A `std::net::TcpListener` that the server will listen on. This allows
///   the caller to control the address and port.
/// * `db_pool` - A `sqlx::MySqlPool` (aliased as `Pool`) for database connections.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(Server)` on successful binding. The `Server` handle can be awaited to run the server.
/// - `Err(std::io::Error)` if the server fails to bind to the listener.
pub async fn run(listener: TcpListener, db_pool: Pool) -> Result<Server, std::io::Error> {
    // Wrap the pool in Arc for thread-safe sharing
    info!("Listening on: {:?}", &listener);
    let db_pool = web::Data::new(AppState { db_pool });

    info!("Configuring HTTP server routes");
    let server = HttpServer::new(move || {
        debug!("Creating new App instance");
        App::new()
            .app_data(db_pool.clone())
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(health_check))
            .route("/health/db", web::get().to(db_health_check))
            // API v1 routes - Titles
            .route("/api/v1/titles", web::get().to(handlers::titles::list_titles))
            .route("/api/v1/titles", web::post().to(handlers::titles::create_title))
            .route("/api/v1/titles/{id}", web::put().to(handlers::titles::update_title))
            .route("/api/v1/titles/{id}", web::delete().to(handlers::titles::delete_title))
            // API v1 routes - Volumes
            .route("/api/v1/titles/{title_id}/volumes", web::get().to(handlers::volumes::list_volumes_by_title))
            .route("/api/v1/volumes", web::post().to(handlers::volumes::create_volume))
            .route("/api/v1/volumes/{id}", web::get().to(handlers::volumes::get_volume))
            .route("/api/v1/volumes/{id}", web::put().to(handlers::volumes::update_volume))
            .route("/api/v1/volumes/{id}", web::delete().to(handlers::volumes::delete_volume))
            // API v1 routes - Locations
            .route("/api/v1/locations", web::get().to(handlers::locations::list_locations))
            .route("/api/v1/locations", web::post().to(handlers::locations::create_location))
            .route("/api/v1/locations/{id}", web::get().to(handlers::locations::get_location))
            .route("/api/v1/locations/{id}", web::put().to(handlers::locations::update_location))
            .route("/api/v1/locations/{id}", web::delete().to(handlers::locations::delete_location))
            // API v1 routes - Authors
            .route("/api/v1/authors", web::get().to(handlers::authors::list_authors))
            .route("/api/v1/authors", web::post().to(handlers::authors::create_author))
            .route("/api/v1/authors/{id}", web::get().to(handlers::authors::get_author))
            .route("/api/v1/authors/{id}", web::put().to(handlers::authors::update_author))
            .route("/api/v1/authors/{id}", web::delete().to(handlers::authors::delete_author))
            // API v1 routes - Publishers
            .route("/api/v1/publishers", web::get().to(handlers::publishers::list_publishers))
            .route("/api/v1/publishers", web::post().to(handlers::publishers::create_publisher))
            .route("/api/v1/publishers/{id}", web::get().to(handlers::publishers::get_publisher))
            .route("/api/v1/publishers/{id}", web::put().to(handlers::publishers::update_publisher))
            .route("/api/v1/publishers/{id}", web::delete().to(handlers::publishers::delete_publisher))
            // API v1 routes - Genres
            .route("/api/v1/genres", web::get().to(handlers::genres::list_genres))
            .route("/api/v1/genres", web::post().to(handlers::genres::create_genre))
            .route("/api/v1/genres/{id}", web::get().to(handlers::genres::get_genre))
            .route("/api/v1/genres/{id}", web::put().to(handlers::genres::update_genre))
            .route("/api/v1/genres/{id}", web::delete().to(handlers::genres::delete_genre))
            // API v1 routes - Uploads
            .route("/api/v1/uploads/cover", web::post().to(handlers::uploads::upload_cover))
            .route("/api/v1/uploads/cover/{title_id}", web::get().to(handlers::uploads::get_cover))
            .route("/api/v1/uploads/cover/{title_id}", web::delete().to(handlers::uploads::delete_cover))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    info!("HTTP server started successfully");
    Ok(server)
}
