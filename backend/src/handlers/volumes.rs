use actix_web::{web, HttpResponse, Responder};
use crate::models::{Volume, VolumeCondition, VolumeLoanStatus, CreateVolumeRequest, UpdateVolumeRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;

/// GET /api/v1/titles/{title_id}/volumes - List all volumes for a specific title
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

                    let location_id = location_id_str.and_then(|s| {
                        Uuid::parse_str(&s).ok()
                    });

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

/// GET /api/v1/volumes/{id} - Get a single volume by ID
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

            let location_id = location_id_str.and_then(|s| Uuid::parse_str(&s).ok());

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

/// POST /api/v1/volumes - Create a new volume
pub async fn create_volume(
    data: web::Data<AppState>,
    req: web::Json<CreateVolumeRequest>,
) -> impl Responder {
    info!("POST /api/v1/volumes - Creating new volume for title {}", req.title_id);

    let new_id = Uuid::new_v4();

    // Validate barcode format (VOL-XXXXXX)
    if !req.barcode.starts_with("VOL-") || req.barcode.len() < 5 {
        warn!("Invalid barcode format: {}", req.barcode);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_BARCODE",
                "message": "Barcode must start with 'VOL-' and have at least one digit"
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

/// PUT /api/v1/volumes/{id} - Update a volume
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
        // Validate barcode format
        if !barcode.starts_with("VOL-") || barcode.len() < 5 {
            warn!("Invalid barcode format: {}", barcode);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "INVALID_BARCODE",
                    "message": "Barcode must start with 'VOL-' and have at least one digit"
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

/// DELETE /api/v1/volumes/{id} - Delete a volume
/// Business rule: Cannot delete if volume is loaned or overdue
pub async fn delete_volume(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    info!("DELETE /api/v1/volumes/{} - Attempting to delete volume", id);

    // First, check loan status
    let check_query = r#"
        SELECT loan_status
        FROM volumes
        WHERE id = ?
    "#;

    match sqlx::query(check_query)
        .bind(id.as_str())
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => {
            let loan_status_str: String = row.get("loan_status");

            if loan_status_str == "loaned" || loan_status_str == "overdue" {
                warn!("Cannot delete volume {} - status: {}", id, loan_status_str);
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": {
                        "code": "VOLUME_LOANED",
                        "message": "Cannot delete volume that is loaned or overdue",
                        "details": {
                            "loan_status": loan_status_str
                        }
                    }
                }));
            }

            debug!("Volume {} has loan status: {}, proceeding with deletion", id, loan_status_str);
        }
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
            error!("Database error while checking volume loan status: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to check volume status",
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
