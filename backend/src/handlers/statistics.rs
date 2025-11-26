//! API handlers for library statistics.
//!
//! This module provides HTTP handlers for retrieving various statistics about the
//! library, such as total counts of titles, volumes, authors, and loans, as well as
//! breakdowns by genre and location.

use actix_web::{web, HttpResponse, Responder};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

/// Statistics for volumes per genre
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GenreStatistic {
    pub genre_id: Option<String>,
    pub genre_name: String,
    pub volume_count: i64,
    pub title_count: i64,
}

/// Statistics for volumes per location
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationStatistic {
    pub location_id: Option<String>,
    pub location_name: String,
    pub location_path: String,
    pub volume_count: i64,
}

/// Statistics for loan status
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoanStatistic {
    pub status: String,
    pub count: i64,
}

/// Overall library statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryStatistics {
    pub total_titles: i64,
    pub total_volumes: i64,
    pub total_authors: i64,
    pub total_publishers: i64,
    pub total_genres: i64,
    pub total_locations: i64,
    pub total_borrowers: i64,
    pub active_loans: i64,
    pub overdue_loans: i64,
}

/// Retrieves statistics of volumes per genre.
///
/// **Endpoint**: `GET /api/v1/statistics/volumes-per-genre`
///
/// Returns a list of genres with their volume and title counts, ordered by volume count descending.
/// Useful for visualizing the collection distribution across genres.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of `GenreStatistic` objects on success
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn get_volumes_per_genre(data: web::Data<AppState>) -> impl Responder {
    debug!("Fetching volumes per genre statistics");

    let query = r#"
        SELECT
            g.id as genre_id,
            g.name as genre_name,
            COUNT(DISTINCT t.id) as title_count,
            COUNT(v.id) as volume_count
        FROM genres g
        LEFT JOIN titles t ON t.genre_id = g.id
        LEFT JOIN volumes v ON v.title_id = t.id
        GROUP BY g.id, g.name
        ORDER BY volume_count DESC, title_count DESC
    "#;

    match sqlx::query_as::<_, GenreStatistic>(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(stats) => {
            debug!("Found {} genre statistics", stats.len());
            HttpResponse::Ok().json(stats)
        }
        Err(e) => {
            error!("Failed to fetch genre statistics: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch genre statistics",
                "details": e.to_string()
            }))
        }
    }
}

/// Retrieves statistics of volumes per location.
///
/// **Endpoint**: `GET /api/v1/statistics/volumes-per-location`
///
/// Returns a list of locations with their volume counts, ordered by volume count descending.
/// Includes the full hierarchical path for each location.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of `LocationStatistic` objects on success
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn get_volumes_per_location(data: web::Data<AppState>) -> impl Responder {
    debug!("Fetching volumes per location statistics");

    let query = r#"
        WITH RECURSIVE location_tree AS (
            SELECT
                id,
                name,
                parent_id,
                CAST(name AS CHAR(1000)) as full_path,
                0 as level
            FROM locations
            WHERE parent_id IS NULL

            UNION ALL

            SELECT
                l.id,
                l.name,
                l.parent_id,
                CAST(CONCAT(lt.full_path, ' > ', l.name) AS CHAR(1000)) as full_path,
                lt.level + 1 as level
            FROM locations l
            INNER JOIN location_tree lt ON l.parent_id = lt.id
        )
        SELECT
            lt.id as location_id,
            lt.name as location_name,
            lt.full_path as location_path,
            COUNT(v.id) as volume_count
        FROM location_tree lt
        LEFT JOIN volumes v ON v.location_id = lt.id
        GROUP BY lt.id, lt.name, lt.full_path
        HAVING volume_count > 0
        ORDER BY volume_count DESC
    "#;

    match sqlx::query_as::<_, LocationStatistic>(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(stats) => {
            debug!("Found {} location statistics", stats.len());
            HttpResponse::Ok().json(stats)
        }
        Err(e) => {
            error!("Failed to fetch location statistics: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch location statistics",
                "details": e.to_string()
            }))
        }
    }
}

/// Retrieves loan status statistics.
///
/// **Endpoint**: `GET /api/v1/statistics/loans`
///
/// Returns counts for each loan status (Available, Loaned, Overdue, etc.).
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of `LoanStatistic` objects on success
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn get_loan_statistics(data: web::Data<AppState>) -> impl Responder {
    debug!("Fetching loan status statistics");

    let query = r#"
        SELECT
            loan_status as status,
            COUNT(*) as count
        FROM volumes
        GROUP BY loan_status
        ORDER BY count DESC
    "#;

    match sqlx::query_as::<_, LoanStatistic>(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(stats) => {
            debug!("Found {} loan statistics", stats.len());
            HttpResponse::Ok().json(stats)
        }
        Err(e) => {
            error!("Failed to fetch loan statistics: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch loan statistics",
                "details": e.to_string()
            }))
        }
    }
}

/// Retrieves overall library statistics.
///
/// **Endpoint**: `GET /api/v1/statistics/library`
///
/// Returns general counts for all major entities in the library, including:
/// - Total titles, volumes, authors, publishers, genres, locations, borrowers
/// - Active and overdue loan counts
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with `LibraryStatistics` object on success
/// * `HttpResponse::InternalServerError` if any database query fails (though individual failures default to 0)
pub async fn get_library_statistics(data: web::Data<AppState>) -> impl Responder {
    debug!("Fetching library statistics");

    // Get counts for all entities
    let total_titles = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM titles")
        .fetch_one(&data.db_pool)
        .await
        .unwrap_or(0);

    let total_volumes = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM volumes")
        .fetch_one(&data.db_pool)
        .await
        .unwrap_or(0);

    let total_authors = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM authors")
        .fetch_one(&data.db_pool)
        .await
        .unwrap_or(0);

    let total_publishers = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM publishers")
        .fetch_one(&data.db_pool)
        .await
        .unwrap_or(0);

    let total_genres = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM genres")
        .fetch_one(&data.db_pool)
        .await
        .unwrap_or(0);

    let total_locations = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM locations")
        .fetch_one(&data.db_pool)
        .await
        .unwrap_or(0);

    let total_borrowers = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM borrowers")
        .fetch_one(&data.db_pool)
        .await
        .unwrap_or(0);

    let active_loans = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM loans WHERE return_date IS NULL"
    )
    .fetch_one(&data.db_pool)
    .await
    .unwrap_or(0);

    let overdue_loans = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM loans WHERE return_date IS NULL AND due_date < CURDATE()"
    )
    .fetch_one(&data.db_pool)
    .await
    .unwrap_or(0);

    let stats = LibraryStatistics {
        total_titles,
        total_volumes,
        total_authors,
        total_publishers,
        total_genres,
        total_locations,
        total_borrowers,
        active_loans,
        overdue_loans,
    };

    debug!("Library statistics: {:?}", stats);
    HttpResponse::Ok().json(stats)
}
