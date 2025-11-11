use actix_web::{web, HttpResponse, Responder};
use crate::models::{Publisher, PublisherWithTitleCount, CreatePublisherRequest, UpdatePublisherRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;

/// GET /api/v1/publishers - List all publishers with title counts
pub async fn list_publishers(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/publishers - Fetching all publishers");

    let query = r#"
        SELECT
            p.id,
            p.name,
            p.description,
            p.website_url,
            p.country,
            p.founded_year,
            p.created_at,
            p.updated_at,
            COUNT(t.id) as title_count
        FROM publishers p
        LEFT JOIN titles t ON p.id = t.publisher
        GROUP BY p.id
        ORDER BY p.name ASC
    "#;

    debug!("Executing query to fetch publishers");
    match sqlx::query(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            debug!("Query successful, fetched {} rows", rows.len());
            let publishers: Vec<PublisherWithTitleCount> = rows
                .into_iter()
                .filter_map(|row| {
                    let id_str: String = row.get("id");
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            warn!("Failed to parse publisher UUID '{}': {}", id_str, e);
                            return None;
                        }
                    };

                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    debug!("Processing publisher: {}", row.get::<String, _>("name"));
                    Some(PublisherWithTitleCount {
                        publisher: Publisher {
                            id,
                            name: row.get("name"),
                            description: row.get("description"),
                            website_url: row.get("website_url"),
                            country: row.get("country"),
                            founded_year: row.get("founded_year"),
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        title_count: row.get("title_count"),
                    })
                })
                .collect();

            info!("Successfully returning {} publishers", publishers.len());
            HttpResponse::Ok().json(publishers)
        }
        Err(e) => {
            error!("Database error while fetching publishers: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch publishers",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// GET /api/v1/publishers/{id} - Get a single publisher by ID
pub async fn get_publisher(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let publisher_id = path.into_inner();
    info!("GET /api/v1/publishers/{} - Fetching publisher", publisher_id);

    // Validate UUID
    let uuid = match Uuid::parse_str(&publisher_id) {
        Ok(u) => u,
        Err(_) => {
            warn!("Invalid UUID format: {}", publisher_id);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "INVALID_UUID",
                    "message": "Invalid publisher ID format"
                }
            }));
        }
    };

    let query = r#"
        SELECT id, name, description, website_url, country, founded_year, created_at, updated_at
        FROM publishers
        WHERE id = ?
    "#;

    match sqlx::query(query)
        .bind(&publisher_id)
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => {
            let created_at: chrono::NaiveDateTime = row.get("created_at");
            let updated_at: chrono::NaiveDateTime = row.get("updated_at");

            let publisher = Publisher {
                id: uuid,
                name: row.get("name"),
                description: row.get("description"),
                website_url: row.get("website_url"),
                country: row.get("country"),
                founded_year: row.get("founded_year"),
                created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
            };

            info!("Successfully found publisher: {}", publisher.name);
            HttpResponse::Ok().json(publisher)
        }
        Ok(None) => {
            warn!("Publisher not found: {}", publisher_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Publisher not found"
                }
            }))
        }
        Err(e) => {
            error!("Database error while fetching publisher: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch publisher",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// POST /api/v1/publishers - Create a new publisher
pub async fn create_publisher(
    data: web::Data<AppState>,
    req: web::Json<CreatePublisherRequest>,
) -> impl Responder {
    info!("POST /api/v1/publishers - Creating new publisher: {}", req.name);

    // Generate new UUID
    let new_id = Uuid::new_v4();

    let query = r#"
        INSERT INTO publishers (id, name, description, website_url, country, founded_year, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, NOW(), NOW())
    "#;

    match sqlx::query(query)
        .bind(new_id.to_string())
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.website_url)
        .bind(&req.country)
        .bind(req.founded_year)
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => {
            info!("Successfully created publisher with ID: {}", new_id);
            HttpResponse::Created().json(serde_json::json!({
                "id": new_id.to_string(),
                "message": "Publisher created successfully"
            }))
        }
        Err(e) => {
            error!("Database error while creating publisher: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to create publisher",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// PUT /api/v1/publishers/{id} - Update a publisher
pub async fn update_publisher(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdatePublisherRequest>,
) -> impl Responder {
    let publisher_id = path.into_inner();
    info!("PUT /api/v1/publishers/{} - Updating publisher", publisher_id);

    // Validate UUID
    if Uuid::parse_str(&publisher_id).is_err() {
        warn!("Invalid UUID format: {}", publisher_id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_UUID",
                "message": "Invalid publisher ID format"
            }
        }));
    }

    // Build dynamic UPDATE query
    let mut updates = Vec::new();
    let mut query = "UPDATE publishers SET ".to_string();

    if req.name.is_some() {
        updates.push("name = ?");
    }
    if req.description.is_some() {
        updates.push("description = ?");
    }
    if req.website_url.is_some() {
        updates.push("website_url = ?");
    }
    if req.country.is_some() {
        updates.push("country = ?");
    }
    if req.founded_year.is_some() {
        updates.push("founded_year = ?");
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "NO_UPDATES",
                "message": "No fields to update"
            }
        }));
    }

    query.push_str(&updates.join(", "));
    query.push_str(", updated_at = NOW() WHERE id = ?");

    // Build query with parameters
    let mut sql_query = sqlx::query(&query);

    if let Some(ref name) = req.name {
        sql_query = sql_query.bind(name);
    }
    if let Some(ref description) = req.description {
        sql_query = sql_query.bind(description);
    }
    if let Some(ref website_url) = req.website_url {
        sql_query = sql_query.bind(website_url);
    }
    if let Some(ref country) = req.country {
        sql_query = sql_query.bind(country);
    }
    if let Some(founded_year) = req.founded_year {
        sql_query = sql_query.bind(founded_year);
    }

    sql_query = sql_query.bind(&publisher_id);

    match sql_query.execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Publisher not found: {}", publisher_id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Publisher not found"
                    }
                }))
            } else {
                info!("Successfully updated publisher: {}", publisher_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Publisher updated successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while updating publisher: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to update publisher",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// DELETE /api/v1/publishers/{id} - Delete a publisher
pub async fn delete_publisher(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let publisher_id = path.into_inner();
    info!("DELETE /api/v1/publishers/{} - Deleting publisher", publisher_id);

    // Validate UUID
    if Uuid::parse_str(&publisher_id).is_err() {
        warn!("Invalid UUID format: {}", publisher_id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_UUID",
                "message": "Invalid publisher ID format"
            }
        }));
    }

    let query = "DELETE FROM publishers WHERE id = ?";

    match sqlx::query(query)
        .bind(&publisher_id)
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Publisher not found: {}", publisher_id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Publisher not found"
                    }
                }))
            } else {
                info!("Successfully deleted publisher: {}", publisher_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Publisher deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while deleting publisher: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete publisher",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}
