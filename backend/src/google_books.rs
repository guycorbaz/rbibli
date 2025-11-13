use serde::{Deserialize, Serialize};
use log::{info, error, debug};
use std::error::Error;

/// Google Books API client for fetching book metadata by ISBN
///
/// This module provides functionality to query the Google Books API
/// and retrieve comprehensive book information including:
/// - Title and subtitle
/// - Authors
/// - Publisher and publication date
/// - ISBN numbers
/// - Page count
/// - Description/summary
/// - Cover image URLs
/// - Language

const GOOGLE_BOOKS_API_URL: &str = "https://www.googleapis.com/books/v1/volumes";

/// Response from Google Books API
#[derive(Debug, Deserialize)]
pub struct GoogleBooksResponse {
    pub items: Option<Vec<BookItem>>,
    #[serde(rename = "totalItems")]
    pub total_items: i32,
}

/// Individual book item from the API response
#[derive(Debug, Deserialize)]
pub struct BookItem {
    pub id: String,
    #[serde(rename = "volumeInfo")]
    pub volume_info: VolumeInfo,
}

/// Volume information containing all book metadata
#[derive(Debug, Deserialize)]
pub struct VolumeInfo {
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Option<Vec<String>>,
    pub publisher: Option<String>,
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,
    #[serde(rename = "industryIdentifiers")]
    pub industry_identifiers: Option<Vec<IndustryIdentifier>>,
    #[serde(rename = "pageCount")]
    pub page_count: Option<i32>,
    pub language: Option<String>,
    #[serde(rename = "imageLinks")]
    pub image_links: Option<ImageLinks>,
    pub description: Option<String>,
    pub categories: Option<Vec<String>>,
}

/// ISBN and other industry identifiers
#[derive(Debug, Deserialize)]
pub struct IndustryIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub identifier: String,
}

/// Links to cover images
#[derive(Debug, Deserialize)]
pub struct ImageLinks {
    pub thumbnail: Option<String>,
    #[serde(rename = "smallThumbnail")]
    pub small_thumbnail: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    #[serde(rename = "extraLarge")]
    pub extra_large: Option<String>,
}

/// Simplified book data structure for our application
#[derive(Debug, Serialize, Deserialize)]
pub struct BookData {
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
    pub cover_image_url: Option<String>,
}

/// Fetches book data from Google Books API by ISBN
///
/// # Arguments
/// * `isbn` - The ISBN-10 or ISBN-13 number to search for
///
/// # Returns
/// * `Ok(BookData)` - Book information if found
/// * `Err(Box<dyn Error>)` - If the book is not found or an error occurs
pub async fn fetch_book_by_isbn(isbn: &str) -> Result<BookData, Box<dyn Error>> {
    info!("Fetching book data from Google Books API for ISBN: {}", isbn);

    // Clean ISBN (remove hyphens and spaces)
    let clean_isbn = isbn.replace("-", "").replace(" ", "");

    // Build the API URL
    let url = format!("{}?q=isbn:{}", GOOGLE_BOOKS_API_URL, clean_isbn);
    debug!("Google Books API URL: {}", url);

    // Make the request
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rbibli/1.0")
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        error!("Google Books API returned error status: {}", status);
        return Err(format!("Google Books API error: {}", status).into());
    }

    let api_response: GoogleBooksResponse = response.json().await?;

    if api_response.total_items == 0 || api_response.items.is_none() {
        error!("No books found for ISBN: {}", isbn);
        return Err(format!("No books found for ISBN: {}", isbn).into());
    }

    let items = api_response.items.unwrap();
    let book_item = &items[0]; // Take the first result
    let volume_info = &book_item.volume_info;

    // Extract publication year from published_date (format: YYYY, YYYY-MM, or YYYY-MM-DD)
    let publication_year = volume_info.published_date.as_ref().and_then(|date| {
        date.split('-').next().and_then(|year| year.parse::<i32>().ok())
    });

    // Get the best available cover image URL (prefer larger images)
    let cover_image_url = volume_info.image_links.as_ref().and_then(|links| {
        links.extra_large.clone()
            .or_else(|| links.large.clone())
            .or_else(|| links.medium.clone())
            .or_else(|| links.small.clone())
            .or_else(|| links.thumbnail.clone())
            .or_else(|| links.small_thumbnail.clone())
    });

    let book_data = BookData {
        title: volume_info.title.clone(),
        subtitle: volume_info.subtitle.clone(),
        authors: volume_info.authors.clone().unwrap_or_default(),
        publisher: volume_info.publisher.clone(),
        publication_year,
        pages: volume_info.page_count,
        language: volume_info.language.clone(),
        isbn: clean_isbn,
        summary: volume_info.description.clone(),
        categories: volume_info.categories.clone().unwrap_or_default(),
        cover_image_url,
    };

    info!("Successfully fetched book data: {}", book_data.title);
    Ok(book_data)
}

/// Downloads a cover image from a URL
///
/// # Arguments
/// * `url` - The URL of the image to download
///
/// # Returns
/// * `Ok((Vec<u8>, String))` - The image bytes and MIME type
/// * `Err(Box<dyn Error>)` - If download fails
pub async fn download_cover_image(url: &str) -> Result<(Vec<u8>, String), Box<dyn Error>> {
    info!("Downloading cover image from: {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "rbibli/1.0")
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        error!("Failed to download cover image: {}", status);
        return Err(format!("Image download error: {}", status).into());
    }

    // Get MIME type from Content-Type header
    let mime_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string();

    let bytes = response.bytes().await?.to_vec();

    info!("Successfully downloaded cover image: {} bytes, type: {}", bytes.len(), mime_type);
    Ok((bytes, mime_type))
}
