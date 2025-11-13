use actix_web::{web, HttpResponse, Responder};
use crate::models::{TitleWithCount, CreateTitleRequest, UpdateTitleRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;

/// Lists all titles with their volume counts.
///
/// **Endpoint**: `GET /api/v1/titles`
///
/// This handler retrieves all titles from the database along with the count of physical
/// volumes (copies) for each title. The titles are ordered alphabetically by title name.
/// This query uses a `LEFT JOIN` to include titles even if they have zero volumes (wishlist items).
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of `TitleWithCount` objects on success
/// * `HttpResponse::InternalServerError` with error details if the database query fails
///
/// # Response Format
///
/// ```json
/// [
///   {
///     "id": "uuid-string",
///     "title": "Book Title",
///     "subtitle": "Optional Subtitle",
///     "isbn": "978-1234567890",
///     "publisher": "Publisher Name",
///     "publication_year": 2020,
///     "pages": 350,
///     "language": "en",
///     "genre_id": "genre-uuid",
///     "volume_count": 3,
///     ...
///   }
/// ]
/// ```
///
/// # Database Query
///
/// Executes a SQL query that:
/// - Joins the `titles` table with the `volumes` table
/// - Groups by title ID to count volumes
/// - Returns 0 for titles with no volumes (wishlist functionality)
/// - Orders results alphabetically by title
pub async fn list_titles(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/titles - Fetching all titles with volume counts");
    // Query to get all titles with their volume counts
    let query = r#"
        SELECT
            t.id,
            t.title,
            t.subtitle,
            t.isbn,
            t.publisher_old as publisher,
            t.publication_year,
            t.pages,
            t.language,
            t.dewey_code,
            t.dewey_category,
            t.genre_old as genre,
            t.genre_id,
            t.summary,
            t.cover_url,
            t.created_at,
            t.updated_at,
            COUNT(v.id) as volume_count
        FROM titles t
        LEFT JOIN volumes v ON t.id = v.title_id
        GROUP BY t.id, t.title, t.subtitle, t.isbn, t.publisher_old, t.publication_year,
                 t.pages, t.language, t.dewey_code, t.dewey_category, t.genre_old, t.genre_id,
                 t.summary, t.cover_url, t.created_at, t.updated_at
        ORDER BY t.title ASC
    "#;

    debug!("Executing query to fetch titles");
    match sqlx::query(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            debug!("Query successful, fetched {} rows", rows.len());
            // Manually construct TitleWithCount from rows
            let titles: Vec<TitleWithCount> = rows
                .into_iter()
                .filter_map(|row| {
                    // Parse UUID from string
                    let id_str: String = row.get("id");
                    let id = match uuid::Uuid::parse_str(&id_str) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            warn!("Failed to parse UUID '{}': {}", id_str, e);
                            return None;
                        }
                    };

                    // Parse dates
                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    debug!("Processing title: {}", row.get::<String, _>("title"));
                    Some(TitleWithCount {
                        title: crate::models::Title {
                            id,
                            title: row.get("title"),
                            subtitle: row.get("subtitle"),
                            isbn: row.get("isbn"),
                            publisher: row.get("publisher"),
                            publication_year: row.get("publication_year"),
                            pages: row.get("pages"),
                            language: row.get("language"),
                            dewey_code: row.get("dewey_code"),
                            dewey_category: row.get("dewey_category"),
                            genre: row.get("genre"),
                            genre_id: row.get("genre_id"),
                            summary: row.get("summary"),
                            cover_url: row.get("cover_url"),
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        volume_count: row.get("volume_count"),
                    })
                })
                .collect();

            info!("Successfully returning {} titles", titles.len());
            HttpResponse::Ok().json(titles)
        }
        Err(e) => {
            error!("Database error while fetching titles: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch titles",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Creates a new title in the library.
///
/// **Endpoint**: `POST /api/v1/titles`
///
/// This handler creates a new title with the provided metadata. The title is created
/// without any physical volumes initially (volume_count = 0), making it suitable for
/// wishlist items. A new UUID is automatically generated for the title.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `req` - JSON request body containing the title metadata
///
/// # Request Body
///
/// ```json
/// {
///   "title": "Book Title (required)",
///   "subtitle": "Optional subtitle",
///   "isbn": "978-1234567890",
///   "publisher": "Publisher name",
///   "publication_year": 2020,
///   "pages": 350,
///   "language": "en (required)",
///   "dewey_code": "000.00",
///   "dewey_category": "Computer Science",
///   "genre_id": "genre-uuid",
///   "summary": "Book description",
///   "cover_url": "https://example.com/cover.jpg"
/// }
/// ```
///
/// # Returns
///
/// * `HttpResponse::Created` (201) with the new title's UUID on success
/// * `HttpResponse::InternalServerError` (500) if the database insertion fails
///
/// # Response Format
///
/// ```json
/// {
///   "id": "newly-generated-uuid",
///   "message": "Title created successfully"
/// }
/// ```
///
/// # Database Operations
///
/// - Generates a new UUID v4 for the title
/// - Sets `created_at` and `updated_at` to current timestamp
/// - Inserts all provided metadata into the `titles` table
pub async fn create_title(
    data: web::Data<AppState>,
    req: web::Json<CreateTitleRequest>,
) -> impl Responder {
    info!("POST /api/v1/titles - Creating new title: {}", req.title);

    // Generate new UUID
    let new_id = Uuid::new_v4();

    let query = r#"
        INSERT INTO titles (id, title, subtitle, isbn, publisher_old, publication_year, pages,
                           language, dewey_code, dewey_category, genre_id, summary, cover_url,
                           created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
    "#;

    match sqlx::query(query)
        .bind(new_id.to_string())
        .bind(&req.title)
        .bind(&req.subtitle)
        .bind(&req.isbn)
        .bind(&req.publisher)
        .bind(req.publication_year)
        .bind(req.pages)
        .bind(&req.language)
        .bind(&req.dewey_code)
        .bind(&req.dewey_category)
        .bind(&req.genre_id)
        .bind(&req.summary)
        .bind(&req.cover_url)
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => {
            info!("Successfully created title with ID: {}", new_id);
            HttpResponse::Created().json(serde_json::json!({
                "id": new_id.to_string(),
                "message": "Title created successfully"
            }))
        }
        Err(e) => {
            error!("Database error while creating title: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to create title",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Updates an existing title's metadata.
///
/// **Endpoint**: `PUT /api/v1/titles/{id}`
///
/// This handler updates one or more fields of an existing title. Only the fields present
/// in the request (non-null) are updated; other fields remain unchanged. The update
/// automatically sets the `updated_at` timestamp to the current time.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the title's UUID
/// * `req` - JSON request body with fields to update (all fields are optional)
///
/// # Request Body
///
/// All fields are optional. Only include fields you want to update:
///
/// ```json
/// {
///   "title": "New Title",
///   "subtitle": "New Subtitle",
///   "isbn": "978-1234567890",
///   "publisher": "New Publisher",
///   "publication_year": 2021,
///   "pages": 400,
///   "language": "en",
///   "dewey_code": "100.00",
///   "dewey_category": "Philosophy",
///   "genre_id": "new-genre-uuid",
///   "summary": "Updated description",
///   "cover_url": "https://example.com/new-cover.jpg"
/// }
/// ```
///
/// # Returns
///
/// * `HttpResponse::Ok` (200) if the title was found and updated successfully
/// * `HttpResponse::NotFound` (404) if no title exists with the given ID
/// * `HttpResponse::BadRequest` (400) if no fields were provided for update
/// * `HttpResponse::InternalServerError` (500) if the database update fails
///
/// # Response Format
///
/// Success:
/// ```json
/// {
///   "message": "Title updated successfully"
/// }
/// ```
///
/// # Database Operations
///
/// - Builds a dynamic UPDATE query based on provided fields
/// - Automatically updates the `updated_at` timestamp
/// - Uses parameterized queries to prevent SQL injection
/// - Returns the number of rows affected to detect if title exists
pub async fn update_title(
    data: web::Data<AppState>,
    id: web::Path<String>,
    req: web::Json<UpdateTitleRequest>,
) -> impl Responder {
    info!("PUT /api/v1/titles/{} - Updating title", id);

    // Build dynamic UPDATE query based on provided fields
    let mut update_parts = Vec::new();
    let mut has_updates = false;

    if req.title.is_some() {
        update_parts.push("title = ?");
        has_updates = true;
    }
    if req.subtitle.is_some() {
        update_parts.push("subtitle = ?");
        has_updates = true;
    }
    if req.isbn.is_some() {
        update_parts.push("isbn = ?");
        has_updates = true;
    }
    if req.publisher.is_some() {
        update_parts.push("publisher_old = ?");
        has_updates = true;
    }
    if req.publication_year.is_some() {
        update_parts.push("publication_year = ?");
        has_updates = true;
    }
    if req.pages.is_some() {
        update_parts.push("pages = ?");
        has_updates = true;
    }
    if req.language.is_some() {
        update_parts.push("language = ?");
        has_updates = true;
    }
    if req.dewey_code.is_some() {
        update_parts.push("dewey_code = ?");
        has_updates = true;
    }
    if req.dewey_category.is_some() {
        update_parts.push("dewey_category = ?");
        has_updates = true;
    }
    if req.genre_id.is_some() {
        update_parts.push("genre_id = ?");
        has_updates = true;
    }
    if req.summary.is_some() {
        update_parts.push("summary = ?");
        has_updates = true;
    }
    if req.cover_url.is_some() {
        update_parts.push("cover_url = ?");
        has_updates = true;
    }

    if !has_updates {
        warn!("No fields to update for title {}", id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "NO_UPDATES",
                "message": "No fields provided for update"
            }
        }));
    }

    update_parts.push("updated_at = NOW()");
    let update_clause = update_parts.join(", ");
    let query = format!("UPDATE titles SET {} WHERE id = ?", update_clause);

    debug!("Update query: {}", query);

    let mut query_builder = sqlx::query(&query);

    // Bind parameters in the same order as update_parts
    if let Some(ref title) = req.title {
        query_builder = query_builder.bind(title);
    }
    if let Some(ref subtitle) = req.subtitle {
        query_builder = query_builder.bind(subtitle);
    }
    if let Some(ref isbn) = req.isbn {
        query_builder = query_builder.bind(isbn);
    }
    if let Some(ref publisher) = req.publisher {
        query_builder = query_builder.bind(publisher);
    }
    if let Some(publication_year) = req.publication_year {
        query_builder = query_builder.bind(publication_year);
    }
    if let Some(pages) = req.pages {
        query_builder = query_builder.bind(pages);
    }
    if let Some(ref language) = req.language {
        query_builder = query_builder.bind(language);
    }
    if let Some(ref dewey_code) = req.dewey_code {
        query_builder = query_builder.bind(dewey_code);
    }
    if let Some(ref dewey_category) = req.dewey_category {
        query_builder = query_builder.bind(dewey_category);
    }
    if let Some(ref genre_id) = req.genre_id {
        query_builder = query_builder.bind(genre_id);
    }
    if let Some(ref summary) = req.summary {
        query_builder = query_builder.bind(summary);
    }
    if let Some(ref cover_url) = req.cover_url {
        query_builder = query_builder.bind(cover_url);
    }

    query_builder = query_builder.bind(id.as_str());

    match query_builder.execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Title {} not found", id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Title not found"
                    }
                }))
            } else {
                info!("Successfully updated title {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Title updated successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while updating title: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to update title",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Deletes a title from the library.
///
/// **Endpoint**: `DELETE /api/v1/titles/{id}`
///
/// This handler deletes a title from the database. A title can only be deleted if it has
/// no physical volumes (copies) associated with it. This business rule ensures that
/// physical inventory is not lost by accidentally deleting a title that still has copies.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the title's UUID to delete
///
/// # Returns
///
/// * `HttpResponse::Ok` (200) if the title was successfully deleted
/// * `HttpResponse::NotFound` (404) if no title exists with the given ID
/// * `HttpResponse::Conflict` (409) if the title has volumes and cannot be deleted
/// * `HttpResponse::InternalServerError` (500) if the database operation fails
///
/// # Response Format
///
/// Success:
/// ```json
/// {
///   "message": "Title deleted successfully"
/// }
/// ```
///
/// Error (has volumes):
/// ```json
/// {
///   "error": {
///     "code": "HAS_VOLUMES",
///     "message": "Cannot delete title with existing volumes",
///     "details": {
///       "volume_count": 3
///     }
///   }
/// }
/// ```
///
/// # Business Rules
///
/// - A title can only be deleted if `volume_count == 0`
/// - Titles with volumes must have all volumes deleted first
/// - This prevents accidental data loss of physical inventory
pub async fn delete_title(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    info!("DELETE /api/v1/titles/{} - Attempting to delete title", id);

    // First, check if the title has any volumes
    let check_query = r#"
        SELECT COUNT(v.id) as volume_count
        FROM titles t
        LEFT JOIN volumes v ON t.id = v.title_id
        WHERE t.id = ?
        GROUP BY t.id
    "#;

    match sqlx::query(check_query)
        .bind(id.as_str())
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => {
            let volume_count: i64 = row.get("volume_count");

            if volume_count > 0 {
                warn!("Cannot delete title {} - has {} volumes", id, volume_count);
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": {
                        "code": "HAS_VOLUMES",
                        "message": "Cannot delete title with existing volumes",
                        "details": {
                            "volume_count": volume_count
                        }
                    }
                }));
            }

            // Title has no volumes, proceed with deletion
            debug!("Title {} has no volumes, proceeding with deletion", id);
        }
        Ok(None) => {
            // Title not found
            warn!("Title {} not found", id);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Title not found"
                }
            }));
        }
        Err(e) => {
            error!("Database error while checking title volumes: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to check title volumes",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }));
        }
    }

    // Delete the title
    let delete_query = "DELETE FROM titles WHERE id = ?";

    match sqlx::query(delete_query)
        .bind(id.as_str())
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Title {} not found during deletion", id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Title not found"
                    }
                }))
            } else {
                info!("Successfully deleted title {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Title deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while deleting title: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete title",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}
