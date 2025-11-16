use actix_web::{web, HttpResponse, Responder};
use crate::models::{Location, LocationWithPath, CreateLocationRequest, UpdateLocationRequest};
use crate::AppState;
use log::{info, warn, error, debug};
use sqlx::Row;
use uuid::Uuid;

/// GET /api/v1/locations - List all locations with hierarchical paths
pub async fn list_locations(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/locations - Fetching all locations");

    // Recursive query to build full paths with child and volume counts
    let query = r#"
        WITH RECURSIVE location_path AS (
            -- Base case: root locations (no parent)
            SELECT
                id,
                name,
                description,
                parent_id,
                created_at,
                updated_at,
                name as path,
                0 as level
            FROM locations
            WHERE parent_id IS NULL

            UNION ALL

            -- Recursive case: child locations
            SELECT
                l.id,
                l.name,
                l.description,
                l.parent_id,
                l.created_at,
                l.updated_at,
                CONCAT(lp.path, ' > ', l.name) as path,
                lp.level + 1 as level
            FROM locations l
            INNER JOIN location_path lp ON l.parent_id = lp.id
        )
        SELECT
            lp.*,
            COALESCE((SELECT COUNT(*) FROM locations WHERE parent_id = lp.id), 0) as child_count,
            COALESCE((SELECT COUNT(*) FROM volumes WHERE location_id = lp.id), 0) as volume_count
        FROM location_path lp
        ORDER BY lp.path ASC
    "#;

    debug!("Executing recursive query to fetch locations with paths");
    match sqlx::query(query)
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(rows) => {
            debug!("Query successful, fetched {} rows", rows.len());
            let locations: Vec<LocationWithPath> = rows
                .into_iter()
                .filter_map(|row| {
                    let id_str: String = row.get("id");
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(uuid) => uuid,
                        Err(e) => {
                            warn!("Failed to parse location UUID '{}': {}", id_str, e);
                            return None;
                        }
                    };

                    let parent_id_str: Option<String> = row.get("parent_id");
                    let parent_id = parent_id_str.and_then(|s| Uuid::parse_str(&s).ok());

                    let created_at: chrono::NaiveDateTime = row.get("created_at");
                    let updated_at: chrono::NaiveDateTime = row.get("updated_at");

                    debug!("Processing location: {}", row.get::<String, _>("name"));
                    Some(LocationWithPath {
                        location: Location {
                            id,
                            name: row.get("name"),
                            description: row.get("description"),
                            parent_id,
                            created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                            updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
                        },
                        full_path: row.get("path"),
                        level: row.get("level"),
                        child_count: row.get::<i64, _>("child_count") as i32,
                        volume_count: row.get::<i64, _>("volume_count") as i32,
                    })
                })
                .collect();

            info!("Successfully returning {} locations", locations.len());
            HttpResponse::Ok().json(locations)
        }
        Err(e) => {
            error!("Database error while fetching locations: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch locations",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// GET /api/v1/locations/{id} - Get a single location by ID
pub async fn get_location(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let location_id = path.into_inner();
    info!("GET /api/v1/locations/{} - Fetching location", location_id);

    // Validate UUID
    let uuid = match Uuid::parse_str(&location_id) {
        Ok(u) => u,
        Err(_) => {
            warn!("Invalid UUID format: {}", location_id);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "INVALID_UUID",
                    "message": "Invalid location ID format"
                }
            }));
        }
    };

    let query = r#"
        SELECT id, name, description, parent_id, created_at, updated_at
        FROM locations
        WHERE id = ?
    "#;

    match sqlx::query(query)
        .bind(&location_id)
        .fetch_optional(&data.db_pool)
        .await
    {
        Ok(Some(row)) => {
            let _id_str: String = row.get("id");
            let parent_id_str: Option<String> = row.get("parent_id");
            let parent_id = parent_id_str.and_then(|s| Uuid::parse_str(&s).ok());

            let created_at: chrono::NaiveDateTime = row.get("created_at");
            let updated_at: chrono::NaiveDateTime = row.get("updated_at");

            let location = Location {
                id: uuid,
                name: row.get("name"),
                description: row.get("description"),
                parent_id,
                created_at: chrono::DateTime::from_naive_utc_and_offset(created_at, chrono::Utc),
                updated_at: chrono::DateTime::from_naive_utc_and_offset(updated_at, chrono::Utc),
            };

            info!("Successfully found location: {}", location.name);
            HttpResponse::Ok().json(location)
        }
        Ok(None) => {
            warn!("Location not found: {}", location_id);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Location not found"
                }
            }))
        }
        Err(e) => {
            error!("Database error while fetching location: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to fetch location",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// POST /api/v1/locations - Create a new location
pub async fn create_location(
    data: web::Data<AppState>,
    req: web::Json<CreateLocationRequest>,
) -> impl Responder {
    info!("POST /api/v1/locations - Creating new location: {}", req.name);

    // Generate new UUID
    let new_id = Uuid::new_v4();

    // Validate parent_id if provided
    let parent_uuid = if let Some(parent_id_str) = &req.parent_id {
        match Uuid::parse_str(parent_id_str) {
            Ok(u) => Some(u),
            Err(_) => {
                warn!("Invalid parent UUID format: {}", parent_id_str);
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": {
                        "code": "INVALID_PARENT_UUID",
                        "message": "Invalid parent location ID format"
                    }
                }));
            }
        }
    } else {
        None
    };

    let query = r#"
        INSERT INTO locations (id, name, description, parent_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, NOW(), NOW())
    "#;

    match sqlx::query(query)
        .bind(new_id.to_string())
        .bind(&req.name)
        .bind(&req.description)
        .bind(parent_uuid.map(|u| u.to_string()))
        .execute(&data.db_pool)
        .await
    {
        Ok(_) => {
            info!("Successfully created location with ID: {}", new_id);
            HttpResponse::Created().json(serde_json::json!({
                "id": new_id.to_string(),
                "message": "Location created successfully"
            }))
        }
        Err(e) => {
            error!("Database error while creating location: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to create location",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// PUT /api/v1/locations/{id} - Update a location
pub async fn update_location(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateLocationRequest>,
) -> impl Responder {
    let location_id = path.into_inner();
    info!("PUT /api/v1/locations/{} - Updating location", location_id);

    // Validate UUID
    let _uuid = match Uuid::parse_str(&location_id) {
        Ok(u) => u,
        Err(_) => {
            warn!("Invalid UUID format: {}", location_id);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "INVALID_UUID",
                    "message": "Invalid location ID format"
                }
            }));
        }
    };

    // Build dynamic UPDATE query based on provided fields
    let mut updates = Vec::new();
    let mut query = "UPDATE locations SET ".to_string();

    if req.name.is_some() {
        updates.push("name = ?");
    }
    if req.description.is_some() {
        updates.push("description = ?");
    }
    if req.parent_id.is_some() {
        updates.push("parent_id = ?");
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

    if let Some(ref name) = req.name {
        sql_query = sql_query.bind(name);
    }
    if let Some(ref description) = req.description {
        sql_query = sql_query.bind(description);
    }
    if let Some(ref parent_id_str) = req.parent_id {
        let parent_uuid = match Uuid::parse_str(parent_id_str) {
            Ok(u) => u,
            Err(_) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": {
                        "code": "INVALID_PARENT_UUID",
                        "message": "Invalid parent location ID format"
                    }
                }));
            }
        };
        sql_query = sql_query.bind(parent_uuid.to_string());
    }

    sql_query = sql_query.bind(&location_id);

    match sql_query.execute(&data.db_pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Location not found: {}", location_id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Location not found"
                    }
                }))
            } else {
                info!("Successfully updated location: {}", location_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Location updated successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while updating location: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to update location",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}

/// DELETE /api/v1/locations/{id} - Delete a location
pub async fn delete_location(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let location_id = path.into_inner();
    info!("DELETE /api/v1/locations/{} - Deleting location", location_id);

    // Validate UUID
    if Uuid::parse_str(&location_id).is_err() {
        warn!("Invalid UUID format: {}", location_id);
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": {
                "code": "INVALID_UUID",
                "message": "Invalid location ID format"
            }
        }));
    }

    // Check if location has child locations
    let child_check_query = "SELECT COUNT(*) as count FROM locations WHERE parent_id = ?";
    match sqlx::query(child_check_query)
        .bind(&location_id)
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => {
            let count: i64 = row.get("count");
            if count > 0 {
                warn!("Cannot delete location {} - has {} child locations", location_id, count);
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": {
                        "code": "HAS_CHILD_LOCATIONS",
                        "message": format!("Cannot delete location: it has {} child location(s). Delete or move child locations first.", count)
                    }
                }));
            }
        }
        Err(e) => {
            error!("Database error checking child locations: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to check for child locations"
                }
            }));
        }
    }

    // Check if location has volumes allocated to it
    let volume_check_query = "SELECT COUNT(*) as count FROM volumes WHERE location_id = ?";
    match sqlx::query(volume_check_query)
        .bind(&location_id)
        .fetch_one(&data.db_pool)
        .await
    {
        Ok(row) => {
            let count: i64 = row.get("count");
            if count > 0 {
                warn!("Cannot delete location {} - has {} volumes allocated", location_id, count);
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": {
                        "code": "HAS_VOLUMES",
                        "message": format!("Cannot delete location: it has {} volume(s) allocated to it. Move or delete volumes first.", count)
                    }
                }));
            }
        }
        Err(e) => {
            error!("Database error checking volumes: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to check for allocated volumes"
                }
            }));
        }
    }

    // All checks passed - proceed with deletion
    let query = "DELETE FROM locations WHERE id = ?";

    match sqlx::query(query)
        .bind(&location_id)
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                warn!("Location not found: {}", location_id);
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "Location not found"
                    }
                }))
            } else {
                info!("Successfully deleted location: {}", location_id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Location deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Database error while deleting location: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete location",
                    "details": {
                        "error": e.to_string()
                    }
                }
            }))
        }
    }
}
