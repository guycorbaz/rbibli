//! API handlers for file uploads.
//!
//! This module provides HTTP handlers for uploading files, such as book cover images.
//! It handles multipart form data and saves files to the configured upload directory.

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures_util::StreamExt;
use log::{error, info};
use crate::AppState;

/// Uploads a cover image for a title.
///
/// **Endpoint**: `POST /api/v1/uploads/cover`
///
/// Accepts multipart/form-data payload containing the title ID and the image file.
/// The image is stored directly in the database as a BLOB.
///
/// # Request Format
///
/// Content-Type: `multipart/form-data`
///
/// Fields:
/// - `title_id`: UUID of the title (text field)
/// - `cover`: Image file (file field)
///
/// # Constraints
///
/// - **Allowed Extensions**: jpg, jpeg, png, gif, webp
/// - **Max File Size**: 5MB
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::BadRequest` if file is invalid, too large, or missing fields
/// * `HttpResponse::NotFound` if title does not exist
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn upload_cover(
    mut payload: Multipart,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("POST /api/v1/uploads/cover - Uploading cover image to database");

    let mut title_id: Option<String> = None;
    let mut image_data: Option<Vec<u8>> = None;
    let mut image_mime_type: Option<String> = None;
    let mut image_filename: Option<String> = None;

    // Process the multipart stream
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                error!("Error reading multipart field: {}", e);
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": {
                        "code": "INVALID_MULTIPART",
                        "message": "Invalid multipart data"
                    }
                }));
            }
        };

        // Get the content disposition to extract field name
        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue,
        };

        let field_name = content_disposition.get_name().unwrap_or("");

        // Handle title_id field
        if field_name == "title_id" {
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Error reading title_id field: {}", e);
                        return HttpResponse::BadRequest().json(serde_json::json!({
                            "error": {
                                "code": "INVALID_TITLE_ID",
                                "message": "Error reading title_id field"
                            }
                        }));
                    }
                };
                bytes.extend_from_slice(&data);
            }
            title_id = Some(String::from_utf8_lossy(&bytes).to_string());
            continue;
        }

        // Handle cover field
        if field_name == "cover" {
            let original_filename = content_disposition
                .get_filename()
                .unwrap_or("image.jpg")
                .to_string();

            // Validate file extension
            let extension = original_filename
                .rsplit('.')
                .next()
                .unwrap_or("")
                .to_lowercase();

            if !matches!(extension.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp") {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": {
                        "code": "INVALID_FILE_TYPE",
                        "message": "Only jpg, jpeg, png, gif, and webp files are allowed"
                    }
                }));
            }

            // Determine MIME type
            let mime_type = match extension.as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "webp" => "image/webp",
                _ => "application/octet-stream",
            };

            image_mime_type = Some(mime_type.to_string());
            image_filename = Some(original_filename);

            // Read file data into memory
            let mut bytes = Vec::new();
            let mut total_size = 0;
            const MAX_SIZE: usize = 5 * 1024 * 1024; // 5MB

            while let Some(chunk) = field.next().await {
                let chunk_data = match chunk {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Error reading file chunk: {}", e);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": {
                                "code": "READ_ERROR",
                                "message": "Error reading uploaded file"
                            }
                        }));
                    }
                };

                total_size += chunk_data.len();
                if total_size > MAX_SIZE {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "error": {
                            "code": "FILE_TOO_LARGE",
                            "message": "File size exceeds 5MB limit"
                        }
                    }));
                }

                bytes.extend_from_slice(&chunk_data);
            }

            image_data = Some(bytes);
        }
    }

    // Validate that we have both title_id and image data
    let title_id = match title_id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "MISSING_TITLE_ID",
                    "message": "title_id field is required"
                }
            }));
        }
    };

    let image_data = match image_data {
        Some(data) => data,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": {
                    "code": "NO_FILE",
                    "message": "No file provided in 'cover' field"
                }
            }));
        }
    };

    // Update the title in the database with the image data
    let result = sqlx::query(
        "UPDATE titles
         SET image_data = ?, image_mime_type = ?, image_filename = ?
         WHERE id = ?"
    )
    .bind(&image_data)
    .bind(&image_mime_type)
    .bind(&image_filename)
    .bind(&title_id)
    .execute(&data.db_pool)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "TITLE_NOT_FOUND",
                        "message": "Title not found"
                    }
                }));
            }

            info!("Successfully uploaded cover image for title: {}", title_id);
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Image uploaded successfully",
                "title_id": title_id
            }))
        }
        Err(e) => {
            error!("Failed to save image to database: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to save image to database"
                }
            }))
        }
    }
}

/// Retrieves a cover image for a title.
///
/// **Endpoint**: `GET /api/v1/uploads/cover/{title_id}`
///
/// Serves the binary image data with the appropriate `Content-Type` header.
///
/// # Arguments
///
/// * `title_id` - UUID of the title (path parameter)
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` with image binary data
/// * `HttpResponse::NotFound` if title doesn't exist or has no image
/// * `HttpResponse::InternalServerError` if database query fails
pub async fn get_cover(
    title_id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("GET /api/v1/uploads/cover/{} - Retrieving cover image from database", title_id);

    // Query for the image data and MIME type
    let result = sqlx::query!(
        "SELECT image_data, image_mime_type
         FROM titles
         WHERE id = ?",
        title_id.as_str()
    )
    .fetch_optional(&data.db_pool)
    .await;

    match result {
        Ok(Some(row)) => {
            match (row.image_data, row.image_mime_type) {
                (Some(image_data), Some(mime_type)) => {
                    info!("Successfully retrieved cover image for title: {}", title_id);
                    HttpResponse::Ok()
                        .content_type(mime_type)
                        .body(image_data)
                }
                _ => {
                    HttpResponse::NotFound().json(serde_json::json!({
                        "error": {
                            "code": "NO_IMAGE",
                            "message": "Title has no cover image"
                        }
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "TITLE_NOT_FOUND",
                    "message": "Title not found"
                }
            }))
        }
        Err(e) => {
            error!("Failed to retrieve cover image from database: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to retrieve cover image"
                }
            }))
        }
    }
}

/// Deletes a cover image from a title.
///
/// **Endpoint**: `DELETE /api/v1/uploads/cover/{title_id}`
///
/// Removes the image data from the title record (sets fields to NULL).
///
/// # Arguments
///
/// * `title_id` - UUID of the title (path parameter)
/// * `data` - Application state containing the database connection pool
///
/// # Returns
///
/// * `HttpResponse::Ok` on success
/// * `HttpResponse::NotFound` if title doesn't exist
/// * `HttpResponse::InternalServerError` if database operation fails
pub async fn delete_cover(
    title_id: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("DELETE /api/v1/uploads/cover/{} - Deleting cover image from database", title_id);

    // Set image fields to NULL for the specified title
    let result = sqlx::query(
        "UPDATE titles
         SET image_data = NULL, image_mime_type = NULL, image_filename = NULL
         WHERE id = ?"
    )
    .bind(title_id.as_str())
    .execute(&data.db_pool)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": {
                        "code": "TITLE_NOT_FOUND",
                        "message": "Title not found"
                    }
                }));
            }

            info!("Successfully deleted cover image for title: {}", title_id);
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Cover image deleted successfully"
            }))
        }
        Err(e) => {
            error!("Failed to delete cover image from database: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Failed to delete cover image"
                }
            }))
        }
    }
}
