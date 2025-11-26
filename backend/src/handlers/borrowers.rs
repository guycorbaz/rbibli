//! API handlers for managing borrowers.
//!
//! This module provides HTTP handlers for creating, reading, updating, and deleting
//! borrower records. It handles borrower details including their group association.

use actix_web::{web, HttpResponse, Responder};
use crate::models::{BorrowerWithGroup, CreateBorrowerRequest, UpdateBorrowerRequest};
use crate::AppState;
use log::{info, error};
use sqlx::Row;
use uuid::Uuid;

/// Lists all borrowers with their group information.
///
/// **Endpoint**: `GET /api/v1/borrowers`
///
/// Retrieves a list of all borrowers, including their associated group details (name, loan duration)
/// and a count of their currently active loans.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of `BorrowerWithGroup` objects on success
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn list_borrowers(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/borrowers - Fetching all borrowers");

    let query = "
        SELECT
            b.id, b.name, b.email, b.phone, b.address, b.city, b.zip, b.group_id, b.created_at, b.updated_at,
            g.name as group_name, g.loan_duration_days,
            COALESCE((SELECT COUNT(*) FROM loans WHERE borrower_id = b.id AND status = 'active'), 0) as active_loan_count
        FROM borrowers b
        LEFT JOIN borrower_groups g ON b.group_id = g.id
        ORDER BY b.name
    ";

    match sqlx::query(query).fetch_all(&data.db_pool).await {
        Ok(rows) => {
            let borrowers: Vec<BorrowerWithGroup> = rows
                .iter()
                .map(|row| {
                    let id: String = row.get("id");
                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    BorrowerWithGroup {
                        borrower: crate::models::Borrower {
                            id: Uuid::parse_str(&id).unwrap(),
                            name: row.get("name"),
                            email: row.get("email"),
                            phone: row.get("phone"),
                            address: row.get("address"),
                            city: row.get("city"),
                            zip: row.get("zip"),
                            group_id: row.get("group_id"),
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        group_name: row.get("group_name"),
                        loan_duration_days: row.get("loan_duration_days"),
                        active_loan_count: row.get::<i64, _>("active_loan_count") as i32,
                    }
                })
                .collect();

            info!("Successfully fetched {} borrowers", borrowers.len());
            HttpResponse::Ok().json(borrowers)
        }
        Err(e) => {
            error!("Failed to fetch borrowers: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch borrowers"
            }))
        }
    }
}

/// Creates a new borrower.
///
/// **Endpoint**: `POST /api/v1/borrowers`
///
/// # Arguments
///
/// * `request` - JSON request body containing borrower details
/// * `data` - Application state containing the database connection pool
///
/// # Request Body
///
/// ```json
/// {
///   "name": "Jane Doe",
///   "email": "jane@example.com",
///   "phone": "555-0123",
///   "address": "123 Library Lane",
///   "city": "Booktown",
///   "zip": "12345",
///   "group_id": "uuid-string"
/// }
/// ```
///
/// # Returns
///
/// * `HttpResponse::Created` (201) with new borrower ID on success
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn create_borrower(
    request: web::Json<CreateBorrowerRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("POST /api/v1/borrowers - Creating borrower: {}", request.name);

    let id = Uuid::new_v4().to_string();

    match sqlx::query(
        "INSERT INTO borrowers (id, name, email, phone, address, city, zip, group_id)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&request.name)
    .bind(&request.email)
    .bind(&request.phone)
    .bind(&request.address)
    .bind(&request.city)
    .bind(&request.zip)
    .bind(&request.group_id)
    .execute(&data.db_pool)
    .await
    {
        Ok(_) => {
            info!("Successfully created borrower: {}", request.name);
            HttpResponse::Created().json(serde_json::json!({ "id": id }))
        }
        Err(e) => {
            error!("Failed to create borrower: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create borrower"
            }))
        }
    }
}

/// Updates an existing borrower.
///
/// **Endpoint**: `PUT /api/v1/borrowers/{id}`
///
/// Updates mutable fields of a borrower. Only provided fields are updated.
///
/// # Arguments
///
/// * `id` - Path parameter containing the borrower's UUID
/// * `request` - JSON request body with fields to update
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::NotFound` if borrower does not exist
/// * `HttpResponse::BadRequest` if no fields provided
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn update_borrower(
    id: web::Path<String>,
    request: web::Json<UpdateBorrowerRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("PUT /api/v1/borrowers/{} - Updating borrower", id);

    let mut query_builder = sqlx::QueryBuilder::new("UPDATE borrowers SET ");
    let mut updates = Vec::new();

    if let Some(name) = &request.name {
        updates.push(format!("name = '{}'", name));
    }
    if let Some(email) = &request.email {
        updates.push(format!("email = '{}'", email));
    }
    if let Some(phone) = &request.phone {
        updates.push(format!("phone = '{}'", phone));
    }
    if let Some(address) = &request.address {
        updates.push(format!("address = '{}'", address));
    }
    if let Some(city) = &request.city {
        updates.push(format!("city = '{}'", city));
    }
    if let Some(zip) = &request.zip {
        updates.push(format!("zip = '{}'", zip));
    }
    if let Some(group_id) = &request.group_id {
        updates.push(format!("group_id = '{}'", group_id));
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    query_builder.push(updates.join(", "));
    query_builder.push(" WHERE id = '");
    query_builder.push(id.as_str());
    query_builder.push("'");

    match query_builder.build().execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Borrower not found"
                }))
            } else {
                info!("Successfully updated borrower: {}", id);
                HttpResponse::Ok().json(serde_json::json!({ "id": id.as_str() }))
            }
        }
        Err(e) => {
            error!("Failed to update borrower: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update borrower"
            }))
        }
    }
}

/// Deletes a borrower.
///
/// **Endpoint**: `DELETE /api/v1/borrowers/{id}`
///
/// Removes a borrower record.
///
/// # Business Rules
///
/// - Cannot delete a borrower who has active loans (must return books first).
///
/// # Arguments
///
/// * `id` - Path parameter containing the borrower's UUID
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::NotFound` if borrower does not exist
/// * `HttpResponse::Conflict` if borrower has active loans
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn delete_borrower(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("DELETE /api/v1/borrowers/{} - Deleting borrower", id);

    // Check if borrower has active loans
    let loan_check_query = "SELECT COUNT(*) as count FROM loans WHERE borrower_id = ? AND status = 'active'";
    match sqlx::query(loan_check_query)
        .bind(id.as_str())
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => {
            let count: i64 = row.get("count");
            if count > 0 {
                error!("Cannot delete borrower {} - has {} active loan(s)", id, count);
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": {
                        "code": "HAS_ACTIVE_LOANS",
                        "message": format!("Cannot delete borrower: they have {} active loan(s). Return all loaned volumes first.", count)
                    }
                }));
            }
        }
        Err(e) => {
            error!("Database error checking active loans: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to check for active loans"
                }
            }));
        }
    }

    // All checks passed - proceed with deletion
    match sqlx::query("DELETE FROM borrowers WHERE id = ?")
        .bind(id.as_str())
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Borrower not found"
                }))
            } else {
                info!("Successfully deleted borrower: {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Borrower deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Failed to delete borrower: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete borrower"
            }))
        }
    }
}
