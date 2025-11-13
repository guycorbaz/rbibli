use actix_web::{web, HttpResponse, Responder};
use crate::models::{BorrowerGroup, CreateBorrowerGroupRequest, UpdateBorrowerGroupRequest};
use crate::AppState;
use log::{info, error};
use uuid::Uuid;

/// GET /api/v1/borrower-groups - List all borrower groups
pub async fn list_borrower_groups(data: web::Data<AppState>) -> impl Responder {
    info!("GET /api/v1/borrower-groups - Fetching all borrower groups");

    match sqlx::query_as::<_, BorrowerGroup>("SELECT * FROM borrower_groups ORDER BY name")
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(groups) => {
            info!("Successfully fetched {} borrower groups", groups.len());
            HttpResponse::Ok().json(groups)
        }
        Err(e) => {
            error!("Failed to fetch borrower groups: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch borrower groups"
            }))
        }
    }
}

/// POST /api/v1/borrower-groups - Create a new borrower group
pub async fn create_borrower_group(
    request: web::Json<CreateBorrowerGroupRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("POST /api/v1/borrower-groups - Creating borrower group: {}", request.name);

    let id = Uuid::new_v4().to_string();

    match sqlx::query(
        "INSERT INTO borrower_groups (id, name, loan_duration_days, description)
         VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&request.name)
    .bind(request.loan_duration_days)
    .bind(&request.description)
    .execute(&data.db_pool)
    .await
    {
        Ok(_) => {
            info!("Successfully created borrower group: {}", request.name);
            HttpResponse::Created().json(serde_json::json!({ "id": id }))
        }
        Err(e) => {
            error!("Failed to create borrower group: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create borrower group"
            }))
        }
    }
}

/// PUT /api/v1/borrower-groups/{id} - Update a borrower group
pub async fn update_borrower_group(
    id: web::Path<String>,
    request: web::Json<UpdateBorrowerGroupRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("PUT /api/v1/borrower-groups/{} - Updating borrower group", id);

    let mut query_builder = sqlx::QueryBuilder::new("UPDATE borrower_groups SET ");
    let mut updates = Vec::new();

    if let Some(name) = &request.name {
        updates.push(format!("name = '{}'", name));
    }
    if let Some(loan_duration_days) = request.loan_duration_days {
        updates.push(format!("loan_duration_days = {}", loan_duration_days));
    }
    if let Some(description) = &request.description {
        updates.push(format!("description = '{}'", description));
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
                    "error": "Borrower group not found"
                }))
            } else {
                info!("Successfully updated borrower group: {}", id);
                HttpResponse::Ok().json(serde_json::json!({ "id": id.as_str() }))
            }
        }
        Err(e) => {
            error!("Failed to update borrower group: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update borrower group"
            }))
        }
    }
}

/// DELETE /api/v1/borrower-groups/{id} - Delete a borrower group
pub async fn delete_borrower_group(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("DELETE /api/v1/borrower-groups/{} - Deleting borrower group", id);

    match sqlx::query("DELETE FROM borrower_groups WHERE id = ?")
        .bind(id.as_str())
        .execute(&data.db_pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Borrower group not found"
                }))
            } else {
                info!("Successfully deleted borrower group: {}", id);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Borrower group deleted successfully"
                }))
            }
        }
        Err(e) => {
            error!("Failed to delete borrower group: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete borrower group"
            }))
        }
    }
}
