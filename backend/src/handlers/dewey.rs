use actix_web::{web, HttpResponse, Responder};
use crate::models::{DeweySearchResult, DeweySearchQuery, DeweyBrowseQuery};
use crate::AppState;
use log::{info, error};
use sqlx::Row;

/// GET /api/v1/dewey/search?q=mathematics&limit=20
/// Search Dewey classifications by keyword (full-text search)
pub async fn search_dewey(
    query: web::Query<DeweySearchQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("GET /api/v1/dewey/search?q={}&limit={}", query.q, query.limit);

    if query.q.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Search query cannot be empty"
        }));
    }

    // Full-text search with relevance scoring
    let search_sql = "
        SELECT
            code,
            name,
            level,
            description,
            MATCH(name, description) AGAINST(? IN NATURAL LANGUAGE MODE) as relevance
        FROM dewey_classifications
        WHERE MATCH(name, description) AGAINST(? IN NATURAL LANGUAGE MODE)
        ORDER BY relevance DESC, level ASC, code ASC
        LIMIT ?
    ";

    match sqlx::query(search_sql)
        .bind(&query.q)
        .bind(&query.q)
        .bind(query.limit)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            let results: Vec<DeweySearchResult> = rows
                .iter()
                .map(|row| DeweySearchResult {
                    code: row.get("code"),
                    name: row.get("name"),
                    level: row.get("level"),
                    description: row.get("description"),
                    relevance: Some(row.get::<f32, _>("relevance")),
                })
                .collect();

            info!("Found {} results for query: {}", results.len(), query.q);
            HttpResponse::Ok().json(results)
        }
        Err(e) => {
            error!("Failed to search Dewey classifications: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to search classifications"
            }))
        }
    }
}

/// GET /api/v1/dewey/browse?parent_code=500
/// Browse Dewey classifications hierarchically
/// If parent_code is not provided, returns main classes (level 1)
pub async fn browse_dewey(
    query: web::Query<DeweyBrowseQuery>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("GET /api/v1/dewey/browse?parent_code={:?}", query.parent_code);

    let sql = if let Some(ref parent_code) = query.parent_code {
        // Get children of the specified parent
        "SELECT code, name, level, description
         FROM dewey_classifications
         WHERE parent_code = ?
         ORDER BY code"
    } else {
        // Get main classes (level 1)
        "SELECT code, name, level, description
         FROM dewey_classifications
         WHERE level = 1
         ORDER BY code"
    };

    let results = if let Some(ref parent_code) = query.parent_code {
        sqlx::query(sql)
            .bind(parent_code)
            .fetch_all(&data.db_pool)
            .await
    } else {
        sqlx::query(sql)
            .fetch_all(&data.db_pool)
            .await
    };

    match results {
        Ok(rows) => {
            let classifications: Vec<DeweySearchResult> = rows
                .iter()
                .map(|row| DeweySearchResult {
                    code: row.get("code"),
                    name: row.get("name"),
                    level: row.get("level"),
                    description: row.get("description"),
                    relevance: None,
                })
                .collect();

            info!("Found {} classifications", classifications.len());
            HttpResponse::Ok().json(classifications)
        }
        Err(e) => {
            error!("Failed to browse Dewey classifications: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to browse classifications"
            }))
        }
    }
}

/// GET /api/v1/dewey/{code}
/// Get a specific Dewey classification by code
pub async fn get_dewey_by_code(
    code: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("GET /api/v1/dewey/{}", code);

    let sql = "SELECT code, name, level, description
               FROM dewey_classifications
               WHERE code = ?";

    match sqlx::query(sql)
        .bind(code.as_str())
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => {
            let classification = DeweySearchResult {
                code: row.get("code"),
                name: row.get("name"),
                level: row.get("level"),
                description: row.get("description"),
                relevance: None,
            };

            HttpResponse::Ok().json(classification)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Classification not found"
            }))
        }
        Err(e) => {
            error!("Failed to get Dewey classification: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get classification"
            }))
        }
    }
}
