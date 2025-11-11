use actix_web::{web, HttpResponse, Responder};
use crate::models::{Author, AuthorWithTitleCount, CreateAuthorRequest, UpdateAuthorRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;
use chrono::NaiveDate;

/// GET /api/v1/authors - List all authors with title counts
pub async fn list_authors(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/authors - Fetching all authors");

    let query = r#"
        SELECT
            a.id,
            a.first_name,
            a.last_name,
            a.biography,
            a.birth_date,
            a.death_date,
            a.nationality,
            a.website_url,
            a.created_at,
            a.updated_at,
            COUNT(ta.id) as title_count
        FROM authors a
        LEFT JOIN title_authors ta ON a.id = ta.author_id
        GROUP BY a.id
        ORDER BY a.last_name ASC, a.first_name ASC
    "#;

    debug!("Executing query to fetch authors");
    match sqlx::query(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            debug!("Query successful, fetched {} rows", rows.len());
            let authors: Vec<AuthorWithTitleCount> = rows
                .into_iter()
                .filter_map(|row| {
                    let id_str: String = row.get("id");
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            warn!("Failed to parse author UUID '{}': {}", id_str, e);
                            return None;
                        }
                    };

                    let birth_date: Option<NaiveDate> = row.get("birth_date");
                    let death_date: Option<NaiveDate> = row.get("death_date");
                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    debug!("Processing author: {} {}", row.get::<String, _>("first_name"), row.get::<String, _>("last_name"));
                    Some(AuthorWithTitleCount {
                        author: Author {
                            id,
                            first_name: row.get("first_name"),
                            last_name: row.get("last_name"),
                            biography: row.get("biography"),
                            birth_date,
                            death_date,
                            nationality: row.get("nationality"),
                            website_url: row.get("website_url"),
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        title_count: row.get("title_count"),
                    })
                })
                .collect();

            info!("Successfully returning {} authors", authors.len());
            HttpResponse::Ok().json(authors)
        }
        Err(e) => {
            error!("Database error while fetching authors: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch authors",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// GET /api/v1/authors/{id} - Get a single author by ID
pub async fn get_author(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let author_id = path.into_inner();
    info!("GET /api/v1/authors/{} - Fetching author", author_id);

    // Validate UUID
    let uuid = match Uuid::parse_str(&author_id) {
        Ok(u) => u,
        Err(_) => {
            warn!("Invalid UUID format: {}", author_id);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "INVALID_UUID",
                    "message": "Invalid author ID format"
                }
            }));
        }
    };

    let query = r#"
        SELECT id, first_name, last_name, biography, birth_date, death_date,
               nationality, website_url, created_at, updated_at
        FROM authors
        WHERE id = ?
    "#;

    match sqlx::query(query)
        .bind(&author_id)
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => {
            let birth_date: Option<NaiveDate> = row.get("birth_date");
            let death_date: Option<NaiveDate> = row.get("death_date");
            let created_at: chrono::NaiveDateTime = row.get("created_at");
            let updated_at: chrono::NaiveDateTime = row.get("updated_at");

            let author = Author {
                id: uuid,
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                biography: row.get("biography"),
                birth_date,
                death_date,
                nationality: row.get("nationality"),
                website_url: row.get("website_url"),
                created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
            };

            info!("Successfully found author: {} {}", author.first_name, author.last_name);
            HttpResponse::Ok().json(author)
        }
        Ok(None) => {
            warn!("Author not found: {}", author_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Author not found"
                }
            }))
        }
        Err(e) => {
            error!("Database error while fetching author: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch author",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// POST /api/v1/authors - Create a new author
pub async fn create_author(
    data: web::Data<AppState>,
    req: web::Json<CreateAuthorRequest>,
) -> impl Responder {
    info!("POST /api/v1/authors - Creating new author: {} {}", req.first_name, req.last_name);

    // Generate new UUID
    let new_id = Uuid::new_v4();

    // Parse dates if provided
    let birth_date = req.birth_date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let death_date = req.death_date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let query = r#"
        INSERT INTO authors (id, first_name, last_name, biography, birth_date, death_date,
                           nationality, website_url, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
    "#;

    match sqlx::query(query)
        .bind(new_id.to_string())
        .bind(&req.first_name)
        .bind(&req.last_name)
        .bind(&req.biography)
        .bind(birth_date)
        .bind(death_date)
        .bind(&req.nationality)
        .bind(&req.website_url)
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => {
            info!("Successfully created author with ID: {}", new_id);
            HttpResponse::Created().json(serde_json::json!({
                "id": new_id.to_string(),
                "message": "Author created successfully"
            }))
        }
        Err(e) => {
            error!("Database error while creating author: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to create author",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// PUT /api/v1/authors/{id} - Update an author
pub async fn update_author(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateAuthorRequest>,
) -> impl Responder {
    let author_id = path.into_inner();
    info!("PUT /api/v1/authors/{} - Updating author", author_id);

    // Validate UUID
    if Uuid::parse_str(&author_id).is_err() {
        warn!("Invalid UUID format: {}", author_id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_UUID",
                "message": "Invalid author ID format"
            }
        }));
    }

    // Build dynamic UPDATE query
    let mut updates = Vec::new();
    let mut query = "UPDATE authors SET ".to_string();

    if req.first_name.is_some() {
        updates.push("first_name = ?");
    }
    if req.last_name.is_some() {
        updates.push("last_name = ?");
    }
    if req.biography.is_some() {
        updates.push("biography = ?");
    }
    if req.birth_date.is_some() {
        updates.push("birth_date = ?");
    }
    if req.death_date.is_some() {
        updates.push("death_date = ?");
    }
    if req.nationality.is_some() {
        updates.push("nationality = ?");
    }
    if req.website_url.is_some() {
        updates.push("website_url = ?");
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

    if let Some(ref first_name) = req.first_name {
        sql_query = sql_query.bind(first_name);
    }
    if let Some(ref last_name) = req.last_name {
        sql_query = sql_query.bind(last_name);
    }
    if let Some(ref biography) = req.biography {
        sql_query = sql_query.bind(biography);
    }
    if let Some(ref birth_date_str) = req.birth_date {
        let birth_date = NaiveDate::parse_from_str(birth_date_str, "%Y-%m-%d").ok();
        sql_query = sql_query.bind(birth_date);
    }
    if let Some(ref death_date_str) = req.death_date {
        let death_date = NaiveDate::parse_from_str(death_date_str, "%Y-%m-%d").ok();
        sql_query = sql_query.bind(death_date);
    }
    if let Some(ref nationality) = req.nationality {
        sql_query = sql_query.bind(nationality);
    }
    if let Some(ref website_url) = req.website_url {
        sql_query = sql_query.bind(website_url);
    }

    sql_query = sql_query.bind(&author_id);

    match sql_query.execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Author not found: {}", author_id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Author not found"
                    }
                }))
            } else {
                info!("Successfully updated author: {}", author_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Author updated successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while updating author: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to update author",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// DELETE /api/v1/authors/{id} - Delete an author
pub async fn delete_author(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let author_id = path.into_inner();
    info!("DELETE /api/v1/authors/{} - Deleting author", author_id);

    // Validate UUID
    if Uuid::parse_str(&author_id).is_err() {
        warn!("Invalid UUID format: {}", author_id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_UUID",
                "message": "Invalid author ID format"
            }
        }));
    }

    let query = "DELETE FROM authors WHERE id = ?";

    match sqlx::query(query)
        .bind(&author_id)
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Author not found: {}", author_id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Author not found"
                    }
                }))
            } else {
                info!("Successfully deleted author: {}", author_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Author deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while deleting author: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete author",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}
