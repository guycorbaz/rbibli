use actix_web::{web, HttpResponse, Responder};
use log::{info, error};
use serde::{Deserialize, Serialize};
use crate::google_books;

/// Response structure for ISBN lookup
#[derive(Debug, Serialize, Deserialize)]
pub struct IsbnLookupResponse {
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: Option<String>,
    pub isbn: String,
    pub summary: Option<String>,
    pub categories: Vec<String>,
    /// Base64-encoded cover image data
    pub cover_image_data: Option<String>,
    pub cover_image_mime_type: Option<String>,
}

/// Lookup book information by ISBN from Google Books API
///
/// # Parameters
/// - isbn: The ISBN-10 or ISBN-13 number (path parameter)
///
/// # Response
/// - 200 OK with JSON containing book data and base64-encoded cover image
/// - 404 Not Found if ISBN is not found
/// - 500 Internal Server Error if lookup fails
///
/// # Example
/// GET /api/v1/isbn/9780134685991
pub async fn lookup_isbn(isbn: web::Path<String>) -> impl Responder {
    info!("POST /api/v1/isbn/{} - Looking up ISBN", isbn);

    // Fetch book data from Google Books API
    let book_data = match google_books::fetch_book_by_isbn(&isbn).await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to fetch book data for ISBN {}: {}", isbn, e);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "ISBN_NOT_FOUND",
                    "message": format!("No book found for ISBN: {}", isbn)
                }
            }));
        }
    };

    // Download cover image if available
    let (cover_image_data, cover_image_mime_type) = if let Some(url) = &book_data.cover_image_url {
        match google_books::download_cover_image(url).await {
            Ok((bytes, mime_type)) => {
                use base64::{engine::general_purpose::STANDARD, Engine};
                let base64_data = STANDARD.encode(&bytes);
                (Some(base64_data), Some(mime_type))
            }
            Err(e) => {
                error!("Failed to download cover image: {}", e);
                (None, None)
            }
        }
    } else {
        (None, None)
    };

    let response = IsbnLookupResponse {
        title: book_data.title,
        subtitle: book_data.subtitle,
        authors: book_data.authors,
        publisher: book_data.publisher,
        publication_year: book_data.publication_year,
        pages: book_data.pages,
        language: book_data.language,
        isbn: book_data.isbn,
        summary: book_data.summary,
        categories: book_data.categories,
        cover_image_data,
        cover_image_mime_type,
    };

    info!("Successfully looked up ISBN: {} - {}", isbn, response.title);
    HttpResponse::Ok().json(response)
}
