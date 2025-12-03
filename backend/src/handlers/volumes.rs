//! API handlers for managing volumes.
//!
//! This module provides HTTP handlers for creating, reading, updating, and deleting
//! physical volumes (copies) of a title. It tracks volume condition, status, and location.

use actix_web::{web, HttpResponse, Responder};
use crate::models::{Volume, VolumeCondition, VolumeLoanStatus, CreateVolumeRequest, UpdateVolumeRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;

/// Lists all volumes for a specific title.
///
/// **Endpoint**: `GET /api/v1/titles/{title_id}/volumes`
///
/// This handler retrieves all physical copies (volumes) associated with a given title.
/// It returns details like condition, location, loan status, and individual notes.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `title_id` - Path parameter containing the title's UUID
///
/// # Returns
///
/// * `HttpResponse::Ok` with JSON array of `Volume` objects on success
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn list_volumes_by_title(
    data: web::Data<AppState>,
    title_id: web::Path<String>,
) -> impl Responder {
    info!("GET /api/v1/titles/{}/volumes - Fetching volumes for title", title_id);

    let query = r#"
        SELECT
            id,
            title_id,
            copy_number,
            barcode,
            `condition`,
            location_id,
            loan_status,
            individual_notes,
            created_at,
            updated_at
        FROM volumes
        WHERE title_id = ?
        ORDER BY copy_number ASC
    "#;

    match sqlx::query(query)
        .bind(title_id.as_str())
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            debug!("Query successful, fetched {} volumes", rows.len());
            let volumes: Vec<Volume> = rows
                .into_iter()
                .filter_map(|row| {
                    let id_str: String = row.get("id");
                    let title_id_str: String = row.get("title_id");
                    let location_id_str: Option<String> = row.get("location_id");

                    let id = match Uuid::parse_str(&id_str) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            warn!("Failed to parse volume UUID '{}': {}", id_str, e);
                            return None;
                        }
                    };

                    let title_id = match Uuid::parse_str(&title_id_str) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            warn!("Failed to parse title UUID '{}': {}", title_id_str, e);
                            return None;
                        }
                    };

                    let location_id = location_id_str;

                    let condition_str: String = row.get("condition");
                    let condition = match condition_str.as_str() {
                        "excellent" => VolumeCondition::Excellent,
                        "good" => VolumeCondition::Good,
                        "fair" => VolumeCondition::Fair,
                        "poor" => VolumeCondition::Poor,
                        "damaged" => VolumeCondition::Damaged,
                        _ => {
                            warn!("Unknown condition: {}", condition_str);
                            VolumeCondition::Good
                        }
                    };

                    let loan_status_str: String = row.get("loan_status");
                    let loan_status = match loan_status_str.as_str() {
                        "available" => VolumeLoanStatus::Available,
                        "loaned" => VolumeLoanStatus::Loaned,
                        "overdue" => VolumeLoanStatus::Overdue,
                        "lost" => VolumeLoanStatus::Lost,
                        "maintenance" => VolumeLoanStatus::Maintenance,
                        _ => {
                            warn!("Unknown loan status: {}", loan_status_str);
                            VolumeLoanStatus::Available
                        }
                    };

                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    Some(Volume {
                        id,
                        title_id,
                        copy_number: row.get("copy_number"),
                        barcode: row.get("barcode"),
                        condition,
                        location_id,
                        loan_status,
                        individual_notes: row.get("individual_notes"),
                        created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                        updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                    })
                })
                .collect();

            info!("Successfully returning {} volumes for title {}", volumes.len(), title_id);
            HttpResponse::Ok().json(volumes)
        }
        Err(e) => {
            error!("Database error while fetching volumes: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch volumes",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Retrieves a single volume by its ID.
///
/// **Endpoint**: `GET /api/v1/volumes/{id}`
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the volume's UUID
///
/// # Returns
///
/// * `HttpResponse::Ok` with `Volume` object on success
/// * `HttpResponse::NotFound` if the volume does not exist
/// * `HttpResponse::InternalServerError` if the database query fails
pub async fn get_volume(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    info!("GET /api/v1/volumes/{} - Fetching volume", id);

    let query = r#"
        SELECT
            id,
            title_id,
            copy_number,
            barcode,
            `condition`,
            location_id,
            loan_status,
            individual_notes,
            created_at,
            updated_at
        FROM volumes
        WHERE id = ?
    "#;

    match sqlx::query(query)
        .bind(id.as_str())
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => {
            let id_str: String = row.get("id");
            let title_id_str: String = row.get("title_id");
            let location_id_str: Option<String> = row.get("location_id");

            let volume_id = match Uuid::parse_str(&id_str) {
                Ok(uuid) => uuid,
                Err(e) => {
                    error!("Failed to parse UUID '{}': {}", id_str, e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": {
                            "code": "INTERNAL_ERROR",
                            "message": "Failed to parse volume ID"
                        }
                    }));
                }
            };

            let title_id = match Uuid::parse_str(&title_id_str) {
                Ok(uuid) => uuid,
                Err(e) => {
                    error!("Failed to parse title UUID '{}': {}", title_id_str, e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": {
                            "code": "INTERNAL_ERROR",
                            "message": "Failed to parse title ID"
                        }
                    }));
                }
            };

            let location_id = location_id_str;

            let condition_str: String = row.get("condition");
            let condition = match condition_str.as_str() {
                "excellent" => VolumeCondition::Excellent,
                "good" => VolumeCondition::Good,
                "fair" => VolumeCondition::Fair,
                "poor" => VolumeCondition::Poor,
                "damaged" => VolumeCondition::Damaged,
                _ => VolumeCondition::Good,
            };

            let loan_status_str: String = row.get("loan_status");
            let loan_status = match loan_status_str.as_str() {
                "available" => VolumeLoanStatus::Available,
                "loaned" => VolumeLoanStatus::Loaned,
                "overdue" => VolumeLoanStatus::Overdue,
                "lost" => VolumeLoanStatus::Lost,
                "maintenance" => VolumeLoanStatus::Maintenance,
                _ => VolumeLoanStatus::Available,
            };

            let created_at: chrono::NaiveDateTime = row.get("created_at");
            let updated_at: chrono::NaiveDateTime = row.get("updated_at");

            let volume = Volume {
                id: volume_id,
                title_id,
                copy_number: row.get("copy_number"),
                barcode: row.get("barcode"),
                condition,
                location_id,
                loan_status,
                individual_notes: row.get("individual_notes"),
                created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
            };

            info!("Successfully fetched volume: {}", id);
            HttpResponse::Ok().json(volume)
        }
        Err(sqlx::Error::RowNotFound) => {
            warn!("Volume {} not found", id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Volume not found"
                }
            }))
        }
        Err(e) => {
            error!("Database error while fetching volume: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch volume",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Creates a new volume (copy) for a title.
///
/// **Endpoint**: `POST /api/v1/volumes`
///
/// This handler adds a new physical copy to the library inventory.
/// It automatically calculates the next available copy number for the title.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `req` - JSON request body containing volume details
///
/// # Request Body
///
/// ```json
/// {
///   "title_id": "uuid-string",
///   "barcode": "123456789",
///   "condition": "good",
///   "location_id": "uuid-string",
///   "individual_notes": "Optional notes"
/// }
/// ```
///
/// # Returns
///
/// * `HttpResponse::Created` (201) with new volume ID and copy number on success
/// * `HttpResponse::BadRequest` if barcode format is invalid
/// * `HttpResponse::Conflict` if barcode already exists
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn create_volume(
    data: web::Data<AppState>,
    req: web::Json<CreateVolumeRequest>,
) -> impl Responder {
    info!("POST /api/v1/volumes - Creating new volume for title {}", req.title_id);

    let new_id = Uuid::new_v4();

    // Validate barcode format (numeric only)
    if req.barcode.is_empty() || !req.barcode.chars().all(|c| c.is_ascii_digit()) {
        warn!("Invalid barcode format: {}", req.barcode);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_BARCODE",
                "message": "Barcode must be a numeric value (e.g., 123456)"
            }
        }));
    }

    // Check if barcode already exists
    let check_barcode_query = "SELECT COUNT(*) as count FROM volumes WHERE barcode = ?";
    match sqlx::query(check_barcode_query)
        .bind(&req.barcode)
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => {
            let count: i64 = row.get("count");
            if count > 0 {
                warn!("Barcode {} already exists", req.barcode);
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": {
                        "code": "DUPLICATE_BARCODE",
                        "message": "Barcode already exists",
                        "details": {
                            "barcode": req.barcode
                        }
                    }
                }));
            }
        }
        Err(e) => {
            error!("Database error while checking barcode: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to validate barcode"
                }
            }));
        }
    }

    // Auto-calculate copy_number (get max copy_number for this title + 1)
    let copy_number_query = "SELECT COALESCE(MAX(copy_number), 0) + 1 as next_copy_number FROM volumes WHERE title_id = ?";
    let copy_number: i32 = match sqlx::query(copy_number_query)
        .bind(&req.title_id)
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => row.get("next_copy_number"),
        Err(e) => {
            error!("Database error while calculating copy_number: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to calculate copy number"
                }
            }));
        }
    };

    debug!("Calculated copy_number: {} for title {}", copy_number, req.title_id);

    // Convert condition enum to string
    let condition_str = match req.condition {
        VolumeCondition::Excellent => "excellent",
        VolumeCondition::Good => "good",
        VolumeCondition::Fair => "fair",
        VolumeCondition::Poor => "poor",
        VolumeCondition::Damaged => "damaged",
    };

    let insert_query = r#"
        INSERT INTO volumes (id, title_id, copy_number, barcode, `condition`, location_id, loan_status, individual_notes, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, 'available', ?, NOW(), NOW())
    "#;

    match sqlx::query(insert_query)
        .bind(new_id.to_string())
        .bind(&req.title_id)
        .bind(copy_number)
        .bind(&req.barcode)
        .bind(condition_str)
        .bind(&req.location_id)
        .bind(&req.individual_notes)
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => {
            info!("Successfully created volume with ID: {}", new_id);
            HttpResponse::Created().json(serde_json::json!({
                "id": new_id.to_string(),
                "copy_number": copy_number,
                "message": "Volume created successfully"
            }))
        }
        Err(e) => {
            error!("Database error while creating volume: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to create volume",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Updates an existing volume.
///
/// **Endpoint**: `PUT /api/v1/volumes/{id}`
///
/// Updates mutable fields of a volume. Only provided fields are updated.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the volume's UUID
/// * `req` - JSON request body with fields to update
///
/// # Request Body
///
/// All fields are optional:
/// ```json
/// {
///   "barcode": "new-barcode",
///   "condition": "excellent",
///   "location_id": "new-location-uuid",
///   "loan_status": "available",
///   "individual_notes": "Updated notes"
/// }
/// ```
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::NotFound` if volume does not exist
/// * `HttpResponse::BadRequest` if no fields provided or validation fails
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn update_volume(
    data: web::Data<AppState>,
    id: web::Path<String>,
    req: web::Json<UpdateVolumeRequest>,
) -> impl Responder {
    info!("PUT /api/v1/volumes/{} - Updating volume", id);

    let mut update_parts = Vec::new();
    let mut has_updates = false;

    if req.barcode.is_some() {
        update_parts.push("barcode = ?");
        has_updates = true;
    }
    if req.condition.is_some() {
        update_parts.push("`condition` = ?");
        has_updates = true;
    }
    if req.location_id.is_some() {
        update_parts.push("location_id = ?");
        has_updates = true;
    }
    if req.loan_status.is_some() {
        update_parts.push("loan_status = ?");
        has_updates = true;
    }
    if req.individual_notes.is_some() {
        update_parts.push("individual_notes = ?");
        has_updates = true;
    }

    if !has_updates {
        warn!("No fields to update for volume {}", id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "NO_UPDATES",
                "message": "No fields provided for update"
            }
        }));
    }

    update_parts.push("updated_at = NOW()");
    let update_clause = update_parts.join(", ");
    let query = format!("UPDATE volumes SET {} WHERE id = ?", update_clause);

    debug!("Update query: {}", query);

    let mut query_builder = sqlx::query(&query);

    if let Some(ref barcode) = req.barcode {
        // Validate barcode format (numeric only)
        if barcode.is_empty() || !barcode.chars().all(|c| c.is_ascii_digit()) {
            warn!("Invalid barcode format: {}", barcode);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "INVALID_BARCODE",
                    "message": "Barcode must be a numeric value (e.g., 123456)"
                }
            }));
        }
        query_builder = query_builder.bind(barcode);
    }
    if let Some(ref condition) = req.condition {
        let condition_str = match condition {
            VolumeCondition::Excellent => "excellent",
            VolumeCondition::Good => "good",
            VolumeCondition::Fair => "fair",
            VolumeCondition::Poor => "poor",
            VolumeCondition::Damaged => "damaged",
        };
        query_builder = query_builder.bind(condition_str);
    }
    if let Some(ref location_id) = req.location_id {
        query_builder = query_builder.bind(location_id);
    }
    if let Some(ref loan_status) = req.loan_status {
        let status_str = match loan_status {
            VolumeLoanStatus::Available => "available",
            VolumeLoanStatus::Loaned => "loaned",
            VolumeLoanStatus::Overdue => "overdue",
            VolumeLoanStatus::Lost => "lost",
            VolumeLoanStatus::Maintenance => "maintenance",
        };
        query_builder = query_builder.bind(status_str);
    }
    if let Some(ref notes) = req.individual_notes {
        query_builder = query_builder.bind(notes);
    }

    query_builder = query_builder.bind(id.as_str());

    match query_builder.execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Volume {} not found", id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Volume not found"
                    }
                }))
            } else {
                info!("Successfully updated volume {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Volume updated successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while updating volume: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to update volume",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// Deletes a volume from the library.
///
/// **Endpoint**: `DELETE /api/v1/volumes/{id}`
///
/// Removes a physical copy from inventory.
///
/// # Business Rules
///
/// - Cannot delete a volume that has active or overdue loans.
/// - Automatically deletes associated loan history (returned loans) before deleting the volume.
///
/// # Arguments
///
/// * `data` - Application state containing the database connection pool
/// * `id` - Path parameter containing the volume's UUID
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::NotFound` if volume does not exist
/// * `HttpResponse::Conflict` if volume has active loans
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn delete_volume(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    info!("DELETE /api/v1/volumes/{} - Attempting to delete volume", id);

    // First, check if volume exists
    let check_volume_query = "SELECT id FROM volumes WHERE id = ?";
    match sqlx::query(check_volume_query)
        .bind(id.as_str())
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(None) => {
            warn!("Volume {} not found", id);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Volume not found"
                }
            }));
        }
        Err(e) => {
            error!("Database error while checking volume existence: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to check volume",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }));
        }
        Ok(Some(_)) => {
            debug!("Volume {} found, checking for active loans", id);
        }
    }

    // Check if there are any active or overdue loans for this volume
    let check_loans_query = r#"
        SELECT COUNT(*) as count
        FROM loans
        WHERE volume_id = ? AND status IN ('active', 'overdue')
    "#;

    match sqlx::query(check_loans_query)
        .bind(id.as_str())
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => {
            let active_loans: i64 = row.get("count");

            if active_loans > 0 {
                warn!("Cannot delete volume {} - has {} active/overdue loan(s)", id, active_loans);
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": {
                        "code": "VOLUME_LOANED",
                        "message": "Cannot delete volume that has active or overdue loans",
                        "details": {
                            "active_loans": active_loans
                        }
                    }
                }));
            }

            debug!("Volume {} has no active loans, proceeding with deletion", id);
        }
        Err(e) => {
            error!("Database error while checking active loans: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to check loan status",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }));
        }
    }

    // Delete any returned loan records first (to satisfy foreign key constraint)
    let delete_returned_loans_query = "DELETE FROM loans WHERE volume_id = ? AND status = 'returned'";

    match sqlx::query(delete_returned_loans_query)
        .bind(id.as_str())
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            let deleted_loans = result.rows_affected();
            if deleted_loans > 0 {
                info!("Deleted {} returned loan record(s) for volume {}", deleted_loans, id);
            }
        }
        Err(e) => {
            error!("Database error while deleting returned loans: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete loan history",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }));
        }
    }

    // Delete the volume
    let delete_query = "DELETE FROM volumes WHERE id = ?";

    match sqlx::query(delete_query)
        .bind(id.as_str())
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Volume {} not found during deletion", id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Volume not found"
                    }
                }))
            } else {
                info!("Successfully deleted volume {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Volume deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while deleting volume: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete volume",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}
