use actix_web::{web, HttpResponse, Responder};
use crate::models::{TitleWithCount, CreateTitleRequest};
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
            t.genre,
            t.summary,
            t.cover_url,
            t.created_at,
            t.updated_at,
            COUNT(v.id) as volume_count
        FROM titles t
        LEFT JOIN volumes v ON t.id = v.title_id
        GROUP BY t.id, t.title, t.subtitle, t.isbn, t.publisher_old, t.publication_year,
                 t.pages, t.language, t.dewey_code, t.dewey_category, t.genre,
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
                           language, dewey_code, dewey_category, genre, summary, cover_url,
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
