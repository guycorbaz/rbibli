use actix_web::{web, HttpResponse, Responder};
use crate::models::{TitleWithCount, CreateTitleRequest, UpdateTitleRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;

/// GET /api/v1/titles - List all titles with volume counts
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
            t.summary,
            t.cover_url,
            t.created_at,
            t.updated_at,
            COUNT(v.id) as volume_count
        FROM titles t
        LEFT JOIN volumes v ON t.id = v.title_id
        GROUP BY t.id, t.title, t.subtitle, t.isbn, t.publisher_old, t.publication_year,
                 t.pages, t.language, t.dewey_code, t.dewey_category, t.genre_old,
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

/// POST /api/v1/titles - Create a new title
pub async fn create_title(
    data: web::Data<AppState>,
    req: web::Json<CreateTitleRequest>,
) -> impl Responder {
    info!("POST /api/v1/titles - Creating new title: {}", req.title);

    // Generate new UUID
    let new_id = Uuid::new_v4();

    let query = r#"
        INSERT INTO titles (id, title, subtitle, isbn, publisher_old, publication_year, pages,
                           language, dewey_code, dewey_category, genre_old, summary, cover_url,
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
        .bind(&req.genre)
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

/// PUT /api/v1/titles/{id} - Update a title
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
    if req.genre.is_some() {
        update_parts.push("genre_old = ?");
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
    if let Some(ref genre) = req.genre {
        query_builder = query_builder.bind(genre);
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
