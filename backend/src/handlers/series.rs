use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::{CreateSeriesRequest, Series, SeriesWithTitleCount, UpdateSeriesRequest};
use crate::AppState;

/// GET /api/series - List all series with title counts
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

/// GET /api/series/{id} - Get a single series by ID
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

/// POST /api/series - Create a new series
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

/// PUT /api/series/{id} - Update an existing series
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

/// DELETE /api/series/{id} - Delete a series
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
