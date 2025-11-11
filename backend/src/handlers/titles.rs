use actix_web::{web, HttpResponse, Responder};
use crate::models::TitleWithCount;
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;

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
            t.publisher,
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
        GROUP BY t.id, t.title, t.subtitle, t.isbn, t.publisher, t.publication_year,
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
