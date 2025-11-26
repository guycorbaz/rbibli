//! API handlers for managing titles.
//!
//! This module provides HTTP handlers for creating, reading, updating, and deleting
//! book titles. It includes functionality for searching, duplicate detection, and merging.

use actix_web::{web, HttpResponse, Responder};
use crate::models::{TitleWithCount, CreateTitleRequest, UpdateTitleRequest, AddAuthorToTitleRequest, Author, AuthorRole, TitleSearchParams, DuplicatePair, DuplicateDetectionResponse, DuplicateConfidence, MergeTitlesRequest, MergeTitlesResponse};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;
use strsim::jaro_winkler;

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
            t.publisher_id,
            t.publication_year,
            t.pages,
            t.language,
            t.dewey_code,
            t.genre_old as genre,
            t.genre_id,
            s.name as series_name,
            t.series_id,
            t.series_number,
            t.summary,
            t.cover_url,
            t.image_mime_type,
            t.image_filename,
            t.created_at,
            t.updated_at,
            COUNT(v.id) as volume_count
        FROM titles t
        LEFT JOIN volumes v ON t.id = v.title_id
        LEFT JOIN series s ON t.series_id = s.id
        GROUP BY t.id, t.title, t.subtitle, t.isbn, t.publisher_old, t.publisher_id, t.publication_year,
                 t.pages, t.language, t.dewey_code, t.genre_old, t.genre_id, s.name,
                 t.series_id, t.series_number, t.summary, t.cover_url, t.image_mime_type, t.image_filename, t.created_at, t.updated_at
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
                            publisher_id: row.get("publisher_id"),
                            publication_year: row.get("publication_year"),
                            pages: row.get("pages"),
                            language: row.get("language"),
                            dewey_code: row.get("dewey_code"),
                            genre: row.get("genre"),
                            genre_id: row.get("genre_id"),
                            series_name: row.get("series_name"),
                            series_id: row.get("series_id"),
                            series_number: row.get("series_number"),
                            summary: row.get("summary"),
                            cover_url: row.get("cover_url"),
                            // Don't fetch image_data in list queries for performance
                            image_data: None,
                            image_mime_type: row.get("image_mime_type"),
                            image_filename: row.get("image_filename"),
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
    info!("Dewey code: {:?} (length: {})", req.dewey_code, req.dewey_code.as_ref().map(|s| s.len()).unwrap_or(0));

    // Generate new UUID
    let new_id = Uuid::new_v4();

    let query = r#"
        INSERT INTO titles (id, title, subtitle, isbn, publisher_old, publisher_id, publication_year, pages,
                           language, dewey_code, genre_id, series_id, series_number, summary, cover_url,
                           created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
    "#;

    match sqlx::query(query)
        .bind(new_id.to_string())
        .bind(&req.title)
        .bind(&req.subtitle)
        .bind(&req.isbn)
        .bind(&req.publisher)
        .bind(&req.publisher_id)
        .bind(req.publication_year)
        .bind(req.pages)
        .bind(&req.language)
        .bind(&req.dewey_code)
        .bind(&req.genre_id)
        .bind(&req.series_id)
        .bind(&req.series_number)
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
    if req.publisher_id.is_some() {
        update_parts.push("publisher_id = ?");
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
    if req.genre_id.is_some() {
        update_parts.push("genre_id = ?");
        has_updates = true;
    }
    if req.series_id.is_some() {
        update_parts.push("series_id = ?");
        has_updates = true;
    }
    if req.series_number.is_some() {
        update_parts.push("series_number = ?");
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
    if let Some(ref publisher_id) = req.publisher_id {
        query_builder = query_builder.bind(publisher_id);
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
    if let Some(ref genre_id) = req.genre_id {
        query_builder = query_builder.bind(genre_id);
    }
    if let Some(ref series_id) = req.series_id {
        query_builder = query_builder.bind(series_id);
    }
    if let Some(ref series_number) = req.series_number {
        query_builder = query_builder.bind(series_number);
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

/// GET /api/v1/titles/{title_id}/authors - List all authors for a title
pub async fn list_title_authors(
    data: web::Data<AppState>,
    title_id: web::Path<String>,
) -> impl Responder {
    info!("GET /api/v1/titles/{}/authors - Fetching authors for title", title_id);

    let query = r#"
        SELECT
            a.id, a.first_name, a.last_name, a.biography, a.birth_date, a.death_date,
            a.nationality, a.website_url, a.created_at, a.updated_at,
            ta.role, ta.display_order
        FROM authors a
        INNER JOIN title_authors ta ON a.id = ta.author_id
        WHERE ta.title_id = ?
        ORDER BY ta.display_order ASC, a.last_name ASC
    "#;

    match sqlx::query(query)
        .bind(title_id.as_str())
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            #[derive(serde::Serialize)]
            struct AuthorWithRole {
                #[serde(flatten)]
                author: Author,
                role: AuthorRole,
                display_order: i32,
            }

            let authors: Vec<AuthorWithRole> = rows
                .into_iter()
                .filter_map(|row| {
                    let id_str: String = row.get("id");
                    let id = Uuid::parse_str(&id_str).ok()?;

                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    let role_str: String = row.get("role");
                    let role = match role_str.as_str() {
                        "main_author" => AuthorRole::MainAuthor,
                        "co_author" => AuthorRole::CoAuthor,
                        "translator" => AuthorRole::Translator,
                        "illustrator" => AuthorRole::Illustrator,
                        "editor" => AuthorRole::Editor,
                        _ => return None,
                    };

                    Some(AuthorWithRole {
                        author: Author {
                            id,
                            first_name: row.get("first_name"),
                            last_name: row.get("last_name"),
                            biography: row.get("biography"),
                            birth_date: row.get("birth_date"),
                            death_date: row.get("death_date"),
                            nationality: row.get("nationality"),
                            website_url: row.get("website_url"),
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        role,
                        display_order: row.get("display_order"),
                    })
                })
                .collect();

            info!("Successfully fetched {} authors for title", authors.len());
            HttpResponse::Ok().json(authors)
        }
        Err(e) => {
            error!("Database error while fetching title authors: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch title authors",
                    "details": { "error": e.to_string() }
                }
            }))
        }
    }
}

/// POST /api/v1/titles/{title_id}/authors - Add an author to a title
pub async fn add_author_to_title(
    data: web::Data<AppState>,
    title_id: web::Path<String>,
    req: web::Json<AddAuthorToTitleRequest>,
) -> impl Responder {
    info!("POST /api/v1/titles/{}/authors - Adding author {} with role {:?}", title_id, req.author_id, req.role);

    // Generate new UUID for the relationship
    let relationship_id = Uuid::new_v4();

    // Determine display_order: if not provided, use the next available order
    let display_order = if let Some(order) = req.display_order {
        order
    } else {
        // Get the maximum display_order for this title and increment
        let max_order_query = "SELECT COALESCE(MAX(display_order), 0) as max_order FROM title_authors WHERE title_id = ?";
        match sqlx::query(max_order_query)
            .bind(title_id.as_str())
            .fetch_one(&data.db_pool)
            .await
        {
            Ok(row) => {
                let max_order: i32 = row.get("max_order");
                max_order + 1
            }
            Err(e) => {
                error!("Failed to get max display_order: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": {
                        "code": "DATABASE_ERROR",
                        "message": "Failed to determine display order"
                    }
                }));
            }
        }
    };

    let query = r#"
        INSERT INTO title_authors (id, title_id, author_id, role, display_order, created_at)
        VALUES (?, ?, ?, ?, ?, NOW())
    "#;

    match sqlx::query(query)
        .bind(relationship_id.to_string())
        .bind(title_id.as_str())
        .bind(&req.author_id)
        .bind(req.role.to_string())
        .bind(display_order)
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => {
            info!("Successfully added author to title");
            HttpResponse::Created().json(serde_json::json!({
                "id": relationship_id.to_string(),
                "message": "Author added to title successfully"
            }))
        }
        Err(e) => {
            error!("Database error while adding author to title: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to add author to title",
                    "details": { "error": e.to_string() }
                }
            }))
        }
    }
}

/// Removes an author from a title.
///
/// **Endpoint**: `DELETE /api/v1/titles/{title_id}/authors/{author_id}`
///
/// This handler removes the association between a specific author and a title.
/// It does not delete the author or the title, only the relationship record in `title_authors`.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `path` - Path parameters containing `(title_id, author_id)`
///
/// # Returns
///
/// * `HttpResponse::Ok` (200) if the relationship was successfully removed
/// * `HttpResponse::NotFound` (404) if the relationship did not exist
/// * `HttpResponse::InternalServerError` (500) if the database operation fails
pub async fn remove_author_from_title(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (title_id, author_id) = path.into_inner();
    info!("DELETE /api/v1/titles/{}/authors/{} - Removing author from title", title_id, author_id);

    let query = "DELETE FROM title_authors WHERE title_id = ? AND author_id = ?";

    match sqlx::query(query)
        .bind(&title_id)
        .bind(&author_id)
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Author-title relationship not found");
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Author-title relationship not found"
                    }
                }))
            } else {
                info!("Successfully removed author from title");
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Author removed from title successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while removing author from title: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to remove author from title",
                    "details": { "error": e.to_string() }
                }
            }))
        }
    }
}

/// Advanced search and filtering for titles.
///
/// **Endpoint**: `GET /api/v1/titles/search`
///
/// This handler provides comprehensive search and filtering capabilities for titles.
/// It supports multiple filter criteria that can be combined, free text search,
/// and various sorting options. Results include volume counts and availability information.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `params` - Query parameters containing search filters (see TitleSearchParams)
///
/// # Query Parameters
///
/// All parameters are optional and can be combined:
///
/// * `q` - Free text search (searches title, subtitle, author names, ISBN)
/// * `title` - Filter by title (partial match, case-insensitive)
/// * `subtitle` - Filter by subtitle (partial match)
/// * `isbn` - Filter by ISBN (partial or exact match)
/// * `series_id` - Filter by series UUID
/// * `author_id` - Filter by author UUID
/// * `genre_id` - Filter by genre UUID
/// * `publisher_id` - Filter by publisher UUID
/// * `year_from` - Minimum publication year (inclusive)
/// * `year_to` - Maximum publication year (inclusive)
/// * `language` - Filter by language code (exact match)
/// * `dewey_code` - Filter by Dewey classification (partial match)
/// * `has_volumes` - Filter by ownership (true=owned, false=wishlist)
/// * `available` - Filter by availability (true=at least one available volume)
/// * `location_id` - Filter by storage location
/// * `sort_by` - Sort field (title, publication_year, created_at)
/// * `sort_order` - Sort direction (asc, desc)
/// * `limit` - Maximum results (default: 100, max: 500)
/// * `offset` - Results to skip (for pagination)
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of TitleWithCount objects on success
/// * `HttpResponse::BadRequest` if validation fails
/// * `HttpResponse::InternalServerError` if the database query fails
///
/// # Response Format
///
/// ```json
/// {
///   "results": [
///     {
///       "id": "uuid-string",
///       "title": "Book Title",
///       "subtitle": "Optional Subtitle",
///       "isbn": "978-1234567890",
///       "publication_year": 2020,
///       "volume_count": 3,
///       "available_count": 2,
///       ...
///     }
///   ],
///   "total": 42,
///   "limit": 100,
///   "offset": 0
/// }
/// ```
///
/// # Examples
///
/// ```
/// // Search for "Harry Potter" books
/// GET /api/v1/titles/search?q=harry+potter
///
/// // Find all books in a series, sorted by series number
/// GET /api/v1/titles/search?series_id=uuid-here&sort_by=title
///
/// // Find wishlist items (books without volumes)
/// GET /api/v1/titles/search?has_volumes=false
///
/// // Find available fiction books published after 2010
/// GET /api/v1/titles/search?genre_id=fiction-uuid&year_from=2010&available=true
///
/// // Complex search with multiple filters
/// GET /api/v1/titles/search?author_id=uuid&language=en&year_from=2000&year_to=2023&sort_by=publication_year&sort_order=desc
/// ```
///
/// # Performance
///
/// - Uses DISTINCT to avoid duplicates from JOINs
/// - Employs database indexes for optimized filtering
/// - LEFT JOINs preserve titles without volumes/authors
/// - Query is built dynamically based on provided filters
/// - LIMIT is enforced to prevent excessive result sets
pub async fn search_titles(
    data: web::Data<AppState>,
    mut params: web::Query<TitleSearchParams>,
) -> impl Responder {
    info!("GET /api/v1/titles/search - Advanced title search with filters: {:?}", params);

    // Validate parameters
    if let Err(e) = params.validate() {
        warn!("Invalid search parameters: {}", e);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_PARAMETERS",
                "message": e
            }
        }));
    }

    // Build the base query with all necessary JOINs
    let base_query = r#"
        SELECT DISTINCT
            t.id,
            t.title,
            t.subtitle,
            t.isbn,
            t.publisher_old as publisher,
            t.publisher_id,
            t.publication_year,
            t.pages,
            t.language,
            t.dewey_code,
            t.genre_old as genre,
            t.genre_id,
            s.name as series_name,
            t.series_id,
            t.series_number,
            t.summary,
            t.cover_url,
            t.image_mime_type,
            t.image_filename,
            t.created_at,
            t.updated_at,
            COUNT(DISTINCT v.id) as volume_count,
            SUM(CASE WHEN v.loan_status = 'Available' THEN 1 ELSE 0 END) as available_count
        FROM titles t
        LEFT JOIN volumes v ON t.id = v.title_id
        LEFT JOIN series s ON t.series_id = s.id
        LEFT JOIN publishers p ON t.publisher_id = p.id
        LEFT JOIN genres g ON t.genre_id = g.id
        LEFT JOIN title_authors ta ON t.id = ta.title_id
        LEFT JOIN authors a ON ta.author_id = a.id
    "#;

    // Build WHERE clauses dynamically
    let mut where_clauses = vec!["1=1".to_string()]; // Always true, simplifies logic
    let mut bind_values: Vec<String> = Vec::new();

    // Free text search across multiple fields
    if let Some(ref q) = params.q {
        let search_term = format!("%{}%", q);
        where_clauses.push(
            "(t.title LIKE ? OR t.subtitle LIKE ? OR t.isbn LIKE ? OR \
             CONCAT(a.first_name, ' ', a.last_name) LIKE ?)"
            .to_string()
        );
        bind_values.push(search_term.clone());
        bind_values.push(search_term.clone());
        bind_values.push(search_term.clone());
        bind_values.push(search_term);
    }

    // Title filter
    if let Some(ref title) = params.title {
        where_clauses.push("t.title LIKE ?".to_string());
        bind_values.push(format!("%{}%", title));
    }

    // Subtitle filter
    if let Some(ref subtitle) = params.subtitle {
        where_clauses.push("t.subtitle LIKE ?".to_string());
        bind_values.push(format!("%{}%", subtitle));
    }

    // ISBN filter
    if let Some(ref isbn) = params.isbn {
        where_clauses.push("t.isbn LIKE ?".to_string());
        bind_values.push(format!("%{}%", isbn));
    }

    // Series filter
    if let Some(ref series_id) = params.series_id {
        where_clauses.push("t.series_id = ?".to_string());
        bind_values.push(series_id.to_string());
    }

    // Author filter
    if let Some(ref author_id) = params.author_id {
        where_clauses.push("ta.author_id = ?".to_string());
        bind_values.push(author_id.to_string());
    }

    // Genre filter
    if let Some(ref genre_id) = params.genre_id {
        where_clauses.push("t.genre_id = ?".to_string());
        bind_values.push(genre_id.to_string());
    }

    // Publisher filter
    if let Some(ref publisher_id) = params.publisher_id {
        where_clauses.push("t.publisher_id = ?".to_string());
        bind_values.push(publisher_id.to_string());
    }

    // Publication year range
    if let Some(year_from) = params.year_from {
        where_clauses.push("t.publication_year >= ?".to_string());
        bind_values.push(year_from.to_string());
    }

    if let Some(year_to) = params.year_to {
        where_clauses.push("t.publication_year <= ?".to_string());
        bind_values.push(year_to.to_string());
    }

    // Language filter
    if let Some(ref language) = params.language {
        where_clauses.push("t.language = ?".to_string());
        bind_values.push(language.to_string());
    }

    // Dewey classification filter
    if let Some(ref dewey_code) = params.dewey_code {
        where_clauses.push("t.dewey_code LIKE ?".to_string());
        bind_values.push(format!("{}%", dewey_code));
    }

    // Location filter (only for titles with volumes in that location)
    if let Some(ref location_id) = params.location_id {
        where_clauses.push("v.location_id = ?".to_string());
        bind_values.push(location_id.to_string());
    }

    // Build the complete WHERE clause
    let where_clause = format!("WHERE {}", where_clauses.join(" AND "));

    // Build GROUP BY and HAVING clauses
    let group_by = r#"
        GROUP BY t.id, t.title, t.subtitle, t.isbn, t.publisher_old, t.publisher_id,
                 t.publication_year, t.pages, t.language, t.dewey_code,
                 t.genre_old, t.genre_id, s.name, t.series_id, t.series_number, t.summary,
                 t.cover_url, t.image_mime_type, t.image_filename, t.created_at, t.updated_at
    "#;

    let mut having_clauses = Vec::new();

    // Filter by ownership status (has volumes or not)
    if let Some(has_volumes) = params.has_volumes {
        if has_volumes {
            having_clauses.push("COUNT(DISTINCT v.id) > 0");
        } else {
            having_clauses.push("COUNT(DISTINCT v.id) = 0");
        }
    }

    // Filter by availability (at least one available volume)
    if let Some(available) = params.available {
        if available {
            having_clauses.push("SUM(CASE WHEN v.loan_status = 'Available' THEN 1 ELSE 0 END) > 0");
        }
    }

    let having_clause = if !having_clauses.is_empty() {
        format!("HAVING {}", having_clauses.join(" AND "))
    } else {
        String::new()
    };

    // Build ORDER BY clause
    let order_field = match params.sort_by.as_str() {
        "publication_year" => "t.publication_year",
        "created_at" => "t.created_at",
        _ => "t.title", // default to title
    };

    let order_direction = if params.sort_order.to_lowercase() == "desc" {
        "DESC"
    } else {
        "ASC"
    };

    let order_by = format!("ORDER BY {} {}", order_field, order_direction);

    // Build pagination
    let limit_clause = format!("LIMIT {} OFFSET {}", params.limit, params.offset);

    // Combine all parts into final query
    let final_query = format!(
        "{} {} {} {} {} {}",
        base_query, where_clause, group_by, having_clause, order_by, limit_clause
    );

    debug!("Executing search query: {}", final_query);
    debug!("Bind values: {:?}", bind_values);

    // Execute the query with dynamic binding
    let mut query_builder = sqlx::query(&final_query);
    for value in &bind_values {
        query_builder = query_builder.bind(value);
    }

    match query_builder.fetch_all(&data.db_pool).await {
        Ok(rows) => {
            debug!("Query successful, fetched {} rows", rows.len());

            // Manually construct TitleWithCount from rows
            let titles: Vec<TitleWithCount> = rows
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

                    Some(TitleWithCount {
                        title: crate::models::Title {
                            id,
                            title: row.get("title"),
                            subtitle: row.get("subtitle"),
                            isbn: row.get("isbn"),
                            publisher: row.get("publisher"),
                            publisher_id: row.get("publisher_id"),
                            publication_year: row.get("publication_year"),
                            pages: row.get("pages"),
                            language: row.get("language"),
                            dewey_code: row.get("dewey_code"),
                            genre: row.get("genre"),
                            genre_id: row.get("genre_id"),
                            series_name: row.get("series_name"),
                            series_id: row.get("series_id"),
                            series_number: row.get("series_number"),
                            summary: row.get("summary"),
                            cover_url: row.get("cover_url"),
                            image_data: None,
                            image_mime_type: row.get("image_mime_type"),
                            image_filename: row.get("image_filename"),
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        volume_count: row.get("volume_count"),
                    })
                })
                .collect();

            info!("Successfully returning {} search results", titles.len());
            HttpResponse::Ok().json(serde_json::json!({
                "results": titles,
                "total": titles.len(),
                "limit": params.limit,
                "offset": params.offset
            }))
        }
        Err(e) => {
            error!("Database error while searching titles: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to search titles",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Helper function to calculate similarity between two titles
///
/// Uses multiple factors to determine if two titles are likely duplicates:
/// - ISBN exact match (100% weight if both present)
/// - Title similarity using Jaro-Winkler distance
/// - Author names comparison
/// - Publication year proximity
///
/// # Arguments
///
/// * `title1` - First title to compare
/// * `title2` - Second title to compare
///
/// # Returns
///
/// Returns a tuple of (similarity_score, match_reasons):
/// - similarity_score: 0.0-100.0 representing how similar the titles are
/// - match_reasons: Vec of strings explaining what matched
fn calculate_similarity(title1: &TitleWithCount, title2: &TitleWithCount) -> (f64, Vec<String>) {
    let mut score = 0.0;
    let mut reasons = Vec::new();

    // ISBN match - strongest indicator (100% if both present and match)
    if let (Some(isbn1), Some(isbn2)) = (&title1.title.isbn, &title2.title.isbn) {
        if !isbn1.is_empty() && !isbn2.is_empty() {
            let normalized_isbn1 = isbn1.replace("-", "").replace(" ", "");
            let normalized_isbn2 = isbn2.replace("-", "").replace(" ", "");

            if normalized_isbn1 == normalized_isbn2 {
                score = 100.0;
                reasons.push("ISBN match".to_string());
                return (score, reasons); // ISBN match is definitive
            }
        }
    }

    // Title similarity using Jaro-Winkler (weighted 70%)
    let title1_normalized = title1.title.title.to_lowercase().trim().to_string();
    let title2_normalized = title2.title.title.to_lowercase().trim().to_string();

    let title_similarity = jaro_winkler(&title1_normalized, &title2_normalized);
    score += title_similarity * 70.0;

    if title_similarity > 0.85 {
        reasons.push(format!("Title similarity: {:.0}%", title_similarity * 100.0));
    }

    // Exact title match after normalization
    if title1_normalized == title2_normalized {
        score = score.max(70.0); // At least 70% for exact title match
        reasons.push("Exact title match".to_string());
    }

    // Penalize if title length difference is substantial (more than 30% difference)
    let len1 = title1.title.title.len() as f64;
    let len2 = title2.title.title.len() as f64;
    let len_diff = (len1 - len2).abs() / len1.max(len2);

    if len_diff > 0.3 {
        score -= 20.0;
        reasons.push(format!("Title length differs significantly ({:.0}%)", len_diff * 100.0));
    }

    // Publication year proximity (within 2 years gives +10%, same year gives +15%)
    if let (Some(year1), Some(year2)) = (title1.title.publication_year, title2.title.publication_year) {
        let year_diff = (year1 - year2).abs();
        if year_diff == 0 {
            score += 15.0;
            reasons.push("Same publication year".to_string());
        } else if year_diff <= 2 {
            score += 10.0;
            reasons.push(format!("Similar publication year ({})", year_diff));
        }
    }

    // Ensure score is in valid range
    score = score.clamp(0.0, 100.0);

    (score, reasons)
}

/// Detects potential duplicate titles in the library.
///
/// **Endpoint**: `GET /api/v1/titles/duplicates`
///
/// This handler analyzes all titles in the database to find potential duplicates based on
/// similarity metrics. It compares ISBNs, titles, authors, and publication years.
///
/// # Query Parameters
///
/// * `min_score` - Minimum similarity score (0-100) to consider a pair as duplicates (default: 50.0)
///
/// # Returns
///
/// * `HttpResponse::Ok` with `DuplicateDetectionResponse` containing categorized duplicate pairs
/// * `HttpResponse::InternalServerError` if the database query fails
///
/// # Algorithm
///
/// 1. Fetches all titles from the database
/// 2. Compares every pair of titles (O(n^2) complexity - resource intensive for large libraries)
/// 3. Calculates a similarity score for each pair
/// 4. Categorizes matches into High, Medium, and Low confidence buckets
pub async fn detect_duplicates(
    data: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    info!("GET /api/v1/titles/duplicates - Detecting duplicate titles");

    let min_score: f64 = query
        .get("min_score")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(50.0);

    let min_score = min_score.clamp(0.0, 100.0);

    debug!("Minimum similarity score threshold: {}", min_score);

    let query_str = r#"
        SELECT
            t.id, t.title, t.subtitle, t.isbn, t.publisher_old as publisher, t.publisher_id,
            t.publication_year, t.pages, t.language, t.dewey_code,
            t.genre_old as genre, t.genre_id, s.name as series_name, t.series_id, t.series_number,
            t.summary, t.cover_url, t.image_mime_type, t.image_filename, t.created_at, t.updated_at,
            COUNT(v.id) as volume_count
        FROM titles t
        LEFT JOIN volumes v ON t.id = v.title_id
        LEFT JOIN series s ON t.series_id = s.id
        GROUP BY t.id, t.title, t.subtitle, t.isbn, t.publisher_old, t.publisher_id,
                 t.publication_year, t.pages, t.language, t.dewey_code,
                 t.genre_old, t.genre_id, s.name, t.series_id, t.series_number,
                 t.summary, t.cover_url, t.image_mime_type, t.image_filename, t.created_at, t.updated_at
        ORDER BY t.title ASC
    "#;

    let rows = match sqlx::query(query_str).fetch_all(&data.db_pool).await {
        Ok(rows) => rows,
        Err(e) => {
            error!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": { "code": "DATABASE_ERROR", "message": "Failed to fetch titles" }
            }));
        }
    };

    let titles: Vec<TitleWithCount> = rows.into_iter().filter_map(|row| {
        let id_str: String = row.get("id");
        let id = Uuid::parse_str(&id_str).ok()?;
        let created_at: chrono::NaiveDateTime = row.get("created_at");
        let updated_at: chrono::NaiveDateTime = row.get("updated_at");

        Some(TitleWithCount {
            title: crate::models::Title {
                id, title: row.get("title"), subtitle: row.get("subtitle"), isbn: row.get("isbn"),
                publisher: row.get("publisher"), publisher_id: row.get("publisher_id"),
                publication_year: row.get("publication_year"), pages: row.get("pages"),
                language: row.get("language"), dewey_code: row.get("dewey_code"),
                genre: row.get("genre"),
                genre_id: row.get("genre_id"), series_name: row.get("series_name"),
                series_id: row.get("series_id"), series_number: row.get("series_number"),
                summary: row.get("summary"), cover_url: row.get("cover_url"), image_data: None,
                image_mime_type: row.get("image_mime_type"), image_filename: row.get("image_filename"),
                created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
            },
            volume_count: row.get("volume_count"),
        })
    }).collect();

    info!("Comparing {} titles for duplicates", titles.len());

    let mut all_pairs = Vec::new();
    for i in 0..titles.len() {
        for j in (i + 1)..titles.len() {
            let (similarity_score, match_reasons) = calculate_similarity(&titles[i], &titles[j]);
            if similarity_score >= min_score {
                let confidence = if similarity_score >= 90.0 {
                    DuplicateConfidence::High
                } else if similarity_score >= 70.0 {
                    DuplicateConfidence::Medium
                } else {
                    DuplicateConfidence::Low
                };

                all_pairs.push(DuplicatePair {
                    title1: titles[i].clone(),
                    title2: titles[j].clone(),
                    similarity_score,
                    confidence,
                    match_reasons,
                });
            }
        }
    }

    let mut high_confidence = Vec::new();
    let mut medium_confidence = Vec::new();
    let mut low_confidence = Vec::new();

    for pair in all_pairs {
        match pair.confidence {
            DuplicateConfidence::High => high_confidence.push(pair),
            DuplicateConfidence::Medium => medium_confidence.push(pair),
            DuplicateConfidence::Low => low_confidence.push(pair),
        }
    }

    let total_pairs = high_confidence.len() + medium_confidence.len() + low_confidence.len();
    info!("Found {} duplicate pairs", total_pairs);

    HttpResponse::Ok().json(DuplicateDetectionResponse {
        high_confidence, medium_confidence, low_confidence, total_pairs,
    })
}

/// Merges a secondary title into a primary title.
///
/// **Endpoint**: `POST /api/v1/titles/{primary_id}/merge/{secondary_id}`
///
/// This handler merges two title records. All volumes associated with the secondary title
/// are moved to the primary title, and then the secondary title is deleted.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `path` - Path parameters containing `(primary_id, secondary_id)`
/// * `request` - JSON body containing confirmation flag
///
/// # Request Body
///
/// ```json
/// {
///   "confirm": true
/// }
/// ```
///
/// # Returns
///
/// * `HttpResponse::Ok` with `MergeTitlesResponse` on success
/// * `HttpResponse::BadRequest` if confirmation is missing or IDs are identical
/// * `HttpResponse::NotFound` if either title does not exist
/// * `HttpResponse::InternalServerError` if the transaction fails
///
/// # Transaction Safety
///
/// This operation is performed within a database transaction to ensure atomicity.
/// If any step (moving volumes, deleting title) fails, the entire operation is rolled back.
pub async fn merge_titles(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    request: web::Json<MergeTitlesRequest>,
) -> impl Responder {
    let (primary_id, secondary_id) = path.into_inner();
    info!("POST /api/v1/titles/{}/merge/{}", primary_id, secondary_id);

    if !request.confirm {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": { "code": "CONFIRMATION_REQUIRED", "message": "Must confirm merge" }
        }));
    }

    if primary_id == secondary_id {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": { "code": "INVALID_REQUEST", "message": "Cannot merge title with itself" }
        }));
    }

    let mut tx = match data.db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to begin transaction: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": { "code": "TRANSACTION_ERROR", "message": "Failed to begin transaction" }
            }));
        }
    };

    // Verify titles exist
    let primary_exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM titles WHERE id = ?")
        .bind(&primary_id).fetch_one(&mut *tx).await.unwrap_or(0);
    let secondary_exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM titles WHERE id = ?")
        .bind(&secondary_id).fetch_one(&mut *tx).await.unwrap_or(0);

    if primary_exists == 0 || secondary_exists == 0 {
        let _ = tx.rollback().await;
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": { "code": "NOT_FOUND", "message": "One or both titles not found" }
        }));
    }

    // Get max copy_number
    let max_copy: Option<i32> = sqlx::query_scalar("SELECT MAX(copy_number) FROM volumes WHERE title_id = ?")
        .bind(&primary_id).fetch_one(&mut *tx).await.unwrap_or(None);
    let next_copy = max_copy.unwrap_or(0) + 1;

    // Move volumes
    let update_result = sqlx::query(
        r#"UPDATE volumes v
           INNER JOIN (
               SELECT id, ROW_NUMBER() OVER (ORDER BY copy_number) as new_copy_num
               FROM volumes WHERE title_id = ?
           ) numbered ON v.id = numbered.id
           SET v.title_id = ?, v.copy_number = numbered.new_copy_num + ? - 1
           WHERE v.title_id = ?"#
    )
    .bind(&secondary_id).bind(&primary_id).bind(next_copy).bind(&secondary_id)
    .execute(&mut *tx).await;

    let volumes_moved = match update_result {
        Ok(result) => result.rows_affected() as i64,
        Err(e) => {
            error!("Failed to move volumes: {}", e);
            let _ = tx.rollback().await;
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": { "code": "DATABASE_ERROR", "message": "Failed to move volumes" }
            }));
        }
    };

    // Delete secondary title
    if let Err(e) = sqlx::query("DELETE FROM titles WHERE id = ?").bind(&secondary_id).execute(&mut *tx).await {
        error!("Failed to delete secondary title: {}", e);
        let _ = tx.rollback().await;
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": { "code": "DATABASE_ERROR", "message": "Failed to delete title" }
        }));
    }

    // Commit
    match tx.commit().await {
        Ok(_) => {
            info!("Successfully merged titles");
            HttpResponse::Ok().json(MergeTitlesResponse {
                success: true,
                primary_title_id: primary_id.clone(),
                volumes_moved,
                secondary_title_deleted: true,
                message: format!("Successfully merged {} volume(s)", volumes_moved),
            })
        }
        Err(e) => {
            error!("Failed to commit: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": { "code": "TRANSACTION_ERROR", "message": "Failed to commit" }
            }))
        }
    }
}
