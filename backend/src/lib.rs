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

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    debug!("Greet endpoint called with name: {}", name);
    format!("Hello {}!", &name)
}

// health check
// To allow Kubernetes or Docker to know that the service is up and running
async fn health_check() -> impl Responder {
    debug!("Health check endpoint called");
    HttpResponse::Ok()
}

// Database health check - verifies connection to MariaDB
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
            // API v1 routes
            .route("/api/v1/titles", web::get().to(handlers::titles::list_titles))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    info!("HTTP server started successfully");
    Ok(server)
}
