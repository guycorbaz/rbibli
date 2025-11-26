//! API handlers for managing genres.
//!
//! This module provides HTTP handlers for creating, reading, updating, and deleting
//! literary genres. It also includes functionality to list genres with their associated
//! title counts.

use actix_web::{web, HttpResponse, Responder};
use crate::models::{GenreWithTitleCount, CreateGenreRequest, UpdateGenreRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;

/// Lists all genres with their title counts.
///
/// **Endpoint**: `GET /api/v1/genres`
///
/// Retrieves a list of all genres, ordered alphabetically by name.
/// Includes a count of titles associated with each genre.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of `GenreWithTitleCount` objects on success
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn list_genres(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/genres - Fetching all genres with title counts");

    let query = r#"
        SELECT
            g.id,
            g.name,
            g.description,
            g.created_at,
            g.updated_at,
            COUNT(t.id) as title_count
        FROM genres g
        LEFT JOIN titles t ON g.id = t.genre_id
        GROUP BY g.id, g.name, g.description, g.created_at, g.updated_at
        ORDER BY g.name ASC
    "#;

    debug!("Executing query to fetch genres");
    match sqlx::query(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            debug!("Query successful, fetched {} rows", rows.len());
            let genres: Vec<GenreWithTitleCount> = rows
                .into_iter()
                .filter_map(|row| {
                    let id_str: String = row.get("id");
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            warn!("Failed to parse UUID '{}': {}", id_str, e);
                            return None;
                        }
                    };

                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    debug!("Processing genre: {}", row.get::<String, _>("name"));
                    Some(GenreWithTitleCount {
                        genre: crate::models::Genre {
                            id,
                            name: row.get("name"),
                            description: row.get("description"),
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        title_count: row.get("title_count"),
                    })
                })
                .collect();

            info!("Successfully returning {} genres", genres.len());
            HttpResponse::Ok().json(genres)
        }
        Err(e) => {
            error!("Database error while fetching genres: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch genres",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Retrieves a single genre by its ID.
///
/// **Endpoint**: `GET /api/v1/genres/{id}`
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the genre's UUID
///
/// # Returns
///
/// * `HttpResponse::Ok` with `Genre` object on success
/// * `HttpResponse::NotFound` if the genre does not exist
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn get_genre(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    info!("GET /api/v1/genres/{} - Fetching genre", id);

    let query = r#"
        SELECT id, name, description, created_at, updated_at
        FROM genres
        WHERE id = ?
    "#;

    match sqlx::query(query)
        .bind(id.as_str())
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => {
            let id_str: String = row.get("id");
            let genre_id = match Uuid::parse_str(&id_str) {
                Ok(uuid) => uuid,
                Err(e) => {
                    error!("Failed to parse UUID '{}': {}", id_str, e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": {
                            "code": "INTERNAL_ERROR",
                            "message": "Failed to parse genre ID"
                        }
                    }));
                }
            };

            let created_at: chrono::NaiveDateTime = row.get("created_at");
            let updated_at: chrono::NaiveDateTime = row.get("updated_at");

            let genre = crate::models::Genre {
                id: genre_id,
                name: row.get("name"),
                description: row.get("description"),
                created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
            };

            info!("Successfully fetched genre: {}", id);
            HttpResponse::Ok().json(genre)
        }
        Err(sqlx::Error::RowNotFound) => {
            warn!("Genre {} not found", id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Genre not found"
                }
            }))
        }
        Err(e) => {
            error!("Database error while fetching genre: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch genre",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Creates a new genre.
///
/// **Endpoint**: `POST /api/v1/genres`
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `req` - JSON request body containing genre details
///
/// # Request Body
///
/// ```json
/// {
///   "name": "Science Fiction",
///   "description": "Speculative fiction dealing with imaginative concepts"
/// }
/// ```
///
/// # Returns
///
/// * `HttpResponse::Created` (201) with new genre ID on success
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn create_genre(
    data: web::Data<AppState>,
    req: web::Json<CreateGenreRequest>,
) -> impl Responder {
    info!("POST /api/v1/genres - Creating new genre: {}", req.name);

    let new_id = Uuid::new_v4();

    let query = r#"
        INSERT INTO genres (id, name, description, created_at, updated_at)
        VALUES (?, ?, ?, NOW(), NOW())
    "#;

    match sqlx::query(query)
        .bind(new_id.to_string())
        .bind(&req.name)
        .bind(&req.description)
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => {
            info!("Successfully created genre with ID: {}", new_id);
            HttpResponse::Created().json(serde_json::json!({
                "id": new_id.to_string(),
                "message": "Genre created successfully"
            }))
        }
        Err(e) => {
            error!("Database error while creating genre: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to create genre",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Updates an existing genre.
///
/// **Endpoint**: `PUT /api/v1/genres/{id}`
///
/// Updates mutable fields of a genre. Only provided fields are updated.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the genre's UUID
/// * `req` - JSON request body with fields to update
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::NotFound` if genre does not exist
/// * `HttpResponse::BadRequest` if no fields provided or validation fails
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn update_genre(
    data: web::Data<AppState>,
    id: web::Path<String>,
    req: web::Json<UpdateGenreRequest>,
) -> impl Responder {
    info!("PUT /api/v1/genres/{} - Updating genre", id);

    let mut update_parts = Vec::new();
    let mut has_updates = false;

    if req.name.is_some() {
        update_parts.push("name = ?");
        has_updates = true;
    }
    if req.description.is_some() {
        update_parts.push("description = ?");
        has_updates = true;
    }

    if !has_updates {
        warn!("No fields to update for genre {}", id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "NO_UPDATES",
                "message": "No fields provided for update"
            }
        }));
    }

    update_parts.push("updated_at = NOW()");
    let update_clause = update_parts.join(", ");
    let query = format!("UPDATE genres SET {} WHERE id = ?", update_clause);

    debug!("Update query: {}", query);

    let mut query_builder = sqlx::query(&query);

    if let Some(ref name) = req.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(ref description) = req.description {
        query_builder = query_builder.bind(description);
    }

    query_builder = query_builder.bind(id.as_str());

    match query_builder.execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Genre {} not found", id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Genre not found"
                    }
                }))
            } else {
                info!("Successfully updated genre {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Genre updated successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while updating genre: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to update genre",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Deletes a genre.
///
/// **Endpoint**: `DELETE /api/v1/genres/{id}`
///
/// Removes a genre record. Note that this may fail or cascade depending on foreign key constraints
/// with `titles` (though usually handled by ON DELETE SET NULL or similar logic if configured).
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the genre's UUID
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::NotFound` if genre does not exist
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn delete_genre(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    info!("DELETE /api/v1/genres/{} - Deleting genre", id);

    let query = "DELETE FROM genres WHERE id = ?";

    match sqlx::query(query)
        .bind(id.as_str())
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Genre {} not found", id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Genre not found"
                    }
                }))
            } else {
                info!("Successfully deleted genre {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Genre deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while deleting genre: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete genre",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}
