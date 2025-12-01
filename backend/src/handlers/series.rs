//! Series management API handlers.
//!
//! This module provides HTTP handlers for managing book series (collections) in the library.
//! Series allow grouping related titles such as comic series, book series, or magazine collections.
//!
//! # Endpoints
//!
//! - `GET /api/v1/series` - List all series with title counts
//! - `GET /api/v1/series/{id}` - Get a single series by ID
//! - `POST /api/v1/series` - Create a new series
//! - `PUT /api/v1/series/{id}` - Update an existing series
//! - `DELETE /api/v1/series/{id}` - Delete a series (only if no titles associated)
//!
//! # Business Rules
//!
//! - Series can contain multiple titles (one-to-many relationship)
//! - Each title can belong to at most one series
//! - Series with associated titles cannot be deleted (delete protection)
//! - Series are ordered alphabetically by name

use actix_web::{web, HttpResponse, Responder};

use uuid::Uuid;

use crate::models::{CreateSeriesRequest, Series, SeriesWithTitleCount, UpdateSeriesRequest};
use crate::AppState;

/// Lists all series with their associated title counts.
///
/// # Endpoint
///
/// `GET /api/v1/series`
///
/// # Description
///
/// Retrieves all series ordered alphabetically by name, with a count of how many
/// titles belong to each series. Uses a LEFT JOIN to include series even if they
/// have no titles yet.
///
/// # Query Details
///
/// - Uses LEFT JOIN to include series with 0 titles
/// - Groups by series fields to aggregate title counts
/// - Orders results by series name (A-Z)
///
/// # Returns
///
/// * `200 OK` - Array of SeriesWithTitleCount objects
/// * `500 Internal Server Error` - Database query failed
///
/// # Success Response
///
/// ```json
/// [
///   {
///     "id": "series-uuid",
///     "name": "Asterix",
///     "description": "French comic book series about Gaulish warriors",
///     "created_at": 1699564800,
///     "updated_at": 1699564800,
///     "title_count": 38
///   }
/// ]
/// ```
///
/// # Usage
///
/// - Display series list in management UI
/// - Show collection statistics
/// - Populate series dropdown for title assignment
pub async fn list_series(data: web::Data<AppState>) -> impl Responder {
    // Query to get all series with their title counts
    let query = r#"
        SELECT
            s.id,
            s.name,
            s.description,
            s.created_at,
            s.updated_at,
            COUNT(t.id) as title_count
        FROM series s
        LEFT JOIN titles t ON s.id = t.series_id
        GROUP BY s.id, s.name, s.description, s.created_at, s.updated_at
        ORDER BY s.name ASC
    "#;

    match sqlx::query_as::<_, (String, String, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, i64)>(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            let series_with_counts: Vec<SeriesWithTitleCount> = rows
                .into_iter()
                .map(|(id, name, description, created_at, updated_at, title_count)| {
                    SeriesWithTitleCount {
                        series: Series {
                            id: Uuid::parse_str(&id).unwrap(),
                            name,
                            description,
                            created_at,
                            updated_at,
                        },
                        title_count,
                    }
                })
                .collect();

            HttpResponse::Ok().json(series_with_counts)
        }
        Err(e) => {
            eprintln!("Failed to fetch series: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch series")
        }
    }
}

/// Retrieves a single series by its unique identifier.
///
/// # Endpoint
///
/// `GET /api/v1/series/{id}`
///
/// # Path Parameters
///
/// * `id` - UUID of the series to retrieve
///
/// # Returns
///
/// * `200 OK` - Series object
/// * `404 Not Found` - Series doesn't exist
/// * `500 Internal Server Error` - Database query failed
///
/// # Success Response
///
/// ```json
/// {
///   "id": "series-uuid",
///   "name": "Harry Potter",
///   "description": "Fantasy series by J.K. Rowling",
///   "created_at": 1699564800,
///   "updated_at": 1699564800
/// }
/// ```
///
/// # Use Cases
///
/// - Display series details page
/// - Fetch series for editing
/// - Verify series exists before operations
pub async fn get_series(data: web::Data<AppState>, series_id: web::Path<String>) -> impl Responder {
    let series_id = series_id.into_inner();

    match sqlx::query_as::<_, Series>("SELECT * FROM series WHERE id = ?")
        .bind(&series_id)
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(series) => HttpResponse::Ok().json(series),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().body(format!("Series with id {} not found", series_id))
        }
        Err(e) => {
            eprintln!("Failed to fetch series: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch series")
        }
    }
}

/// Creates a new series in the library.
///
/// # Endpoint
///
/// `POST /api/v1/series`
///
/// # Request Body
///
/// ```json
/// {
///   "name": "Lord of the Rings",
///   "description": "Epic fantasy trilogy by J.R.R. Tolkien"
/// }
/// ```
///
/// # Validation
///
/// - Name is required and must not be empty
/// - Description is optional
/// - No duplicate name validation (multiple series can have similar names)
///
/// # Returns
///
/// * `201 Created` - Series created successfully with new ID
/// * `500 Internal Server Error` - Database insert failed
///
/// # Success Response
///
/// ```json
/// {
///   "id": "new-series-uuid"
/// }
/// ```
///
/// # Side Effects
///
/// - Generates new UUID for the series
/// - Sets created_at and updated_at to current timestamp (via database defaults)
pub async fn create_series(
    data: web::Data<AppState>,
    request: web::Json<CreateSeriesRequest>,
) -> impl Responder {
    let series_id = Uuid::new_v4().to_string();

    let query = r#"
        INSERT INTO series (id, name, description)
        VALUES (?, ?, ?)
    "#;

    match sqlx::query(query)
        .bind(&series_id)
        .bind(&request.name)
        .bind(&request.description)
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({ "id": series_id })),
        Err(e) => {
            eprintln!("Failed to create series: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create series")
        }
    }
}

/// Updates an existing series with partial data.
///
/// # Endpoint
///
/// `PUT /api/v1/series/{id}`
///
/// # Path Parameters
///
/// * `id` - UUID of the series to update
///
/// # Request Body
///
/// All fields are optional. Only provided fields will be updated.
///
/// ```json
/// {
///   "name": "Updated Series Name",
///   "description": "Updated description"
/// }
/// ```
///
/// # Validation
///
/// - At least one field must be provided
/// - Returns 400 Bad Request if request body is empty
///
/// # Returns
///
/// * `200 OK` - Series updated successfully
/// * `400 Bad Request` - No fields provided to update
/// * `404 Not Found` - Series doesn't exist
/// * `500 Internal Server Error` - Database update failed
///
/// # Dynamic Query Building
///
/// Constructs UPDATE query dynamically based on which fields are provided,
/// preventing unnecessary updates and allowing partial updates.
///
/// # Side Effects
///
/// - Database automatically updates `updated_at` timestamp via trigger
pub async fn update_series(
    data: web::Data<AppState>,
    series_id: web::Path<String>,
    request: web::Json<UpdateSeriesRequest>,
) -> impl Responder {
    let series_id = series_id.into_inner();

    // Build dynamic update query based on provided fields
    let mut query_parts = Vec::new();
    let mut has_updates = false;

    if request.name.is_some() {
        query_parts.push("name = ?");
        has_updates = true;
    }

    if request.description.is_some() {
        query_parts.push("description = ?");
        has_updates = true;
    }

    if !has_updates {
        return HttpResponse::BadRequest().body("No fields to update");
    }

    let query = format!(
        "UPDATE series SET {} WHERE id = ?",
        query_parts.join(", ")
    );

    let mut query_builder = sqlx::query(&query);

    if let Some(ref name) = request.name {
        query_builder = query_builder.bind(name);
    }

    if let Some(ref description) = request.description {
        query_builder = query_builder.bind(description);
    }

    query_builder = query_builder.bind(&series_id);

    match query_builder.execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().body(format!("Series with id {} not found", series_id))
            } else {
                HttpResponse::Ok().body("Series updated successfully")
            }
        }
        Err(e) => {
            eprintln!("Failed to update series: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update series")
        }
    }
}

/// Deletes a series if it has no associated titles.
///
/// # Endpoint
///
/// `DELETE /api/v1/series/{id}`
///
/// # Path Parameters
///
/// * `id` - UUID of the series to delete
///
/// # Business Rule: Delete Protection
///
/// Series can only be deleted if they have NO associated titles. This prevents
/// orphaning titles and maintains referential integrity.
///
/// # Workflow
///
/// 1. **Check Title Count**: Query how many titles reference this series
/// 2. **Validate Deletion**: If count > 0, reject with 400 Bad Request
/// 3. **Delete Series**: Only if no titles are associated
///
/// # Returns
///
/// * `200 OK` - Series deleted successfully
/// * `400 Bad Request` - Series has associated titles (cannot delete)
/// * `404 Not Found` - Series doesn't exist
/// * `500 Internal Server Error` - Database query failed
///
/// # Error Response Example
///
/// ```json
/// {
///   "error": "Cannot delete series: 7 title(s) are associated with this series"
/// }
/// ```
///
/// # Alternative Approach
///
/// To delete a series with titles:
/// 1. First update all associated titles to remove series_id (set to NULL)
/// 2. Then delete the series
///
/// # Side Effects
///
/// - Removes series from database permanently
/// - No cascade deletion (titles remain with NULL series_id due to ON DELETE SET NULL)
pub async fn delete_series(data: web::Data<AppState>, series_id: web::Path<String>) -> impl Responder {
    let series_id = series_id.into_inner();

    // Check if series has titles associated with it
    let count_query = "SELECT COUNT(*) as count FROM titles WHERE series_id = ?";
    match sqlx::query_scalar::<_, i64>(count_query)
        .bind(&series_id)
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(count) if count > 0 => {
            return HttpResponse::BadRequest().body(format!(
                "Cannot delete series: {} title(s) are associated with this series",
                count
            ));
        }
        Err(e) => {
            eprintln!("Failed to check series usage: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to check series usage");
        }
        _ => {}
    }

    // Delete the series
    match sqlx::query("DELETE FROM series WHERE id = ?")
        .bind(&series_id)
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().body(format!("Series with id {} not found", series_id))
            } else {
                HttpResponse::Ok().body("Series deleted successfully")
            }
        }
        Err(e) => {
            eprintln!("Failed to delete series: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete series")
        }
    }
}
