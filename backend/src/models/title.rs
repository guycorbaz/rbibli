use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Title represents the abstract book metadata shared across all physical copies
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Title {
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publisher_id: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    pub genre: Option<String>,
    pub genre_id: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<String>,
    pub series_number: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "base64_option")]
    pub image_data: Option<Vec<u8>>,
    pub image_mime_type: Option<String>,
    pub image_filename: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Helper module for base64 encoding/decoding of image data
mod base64_option {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(data: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match data {
            Some(bytes) => {
                use base64::{engine::general_purpose::STANDARD, Engine};
                serializer.serialize_str(&STANDARD.encode(bytes))
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use base64::{engine::general_purpose::STANDARD, Engine};
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => STANDARD
                .decode(&s)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

/// TitleWithCount is returned by the list endpoint, includes volume count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithCount {
    #[serde(flatten)]
    pub title: Title,
    pub volume_count: i64,
}

/// CreateTitleRequest for creating a new title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTitleRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publisher_id: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub series_id: Option<String>,
    pub series_number: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
}

/// UpdateTitleRequest for updating an existing title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTitleRequest {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publisher_id: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: Option<String>,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub series_id: Option<String>,
    pub series_number: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
}

/// TitleSearchParams for advanced search and filtering
///
/// All fields are optional. When a field is provided, it will be used as a filter criterion.
/// Multiple fields are combined with AND logic.
///
/// # Query Parameters
///
/// * `q` - Free text search across title, subtitle, author names, and ISBN
/// * `title` - Partial match on title (case-insensitive, uses LIKE %value%)
/// * `subtitle` - Partial match on subtitle
/// * `isbn` - Partial or exact match on ISBN
/// * `series_id` - Filter by series UUID
/// * `author_id` - Filter by author UUID (searches through title_authors junction)
/// * `genre_id` - Filter by genre UUID
/// * `publisher_id` - Filter by publisher UUID
/// * `year_from` - Minimum publication year (inclusive)
/// * `year_to` - Maximum publication year (inclusive)
/// * `language` - Exact match on language code (e.g., "en", "fr")
/// * `dewey_code` - Partial match on Dewey classification code
/// * `has_volumes` - Filter by ownership status (true = owned, false = wishlist, None = all)
/// * `available` - Filter by availability (true = at least one available volume)
/// * `location_id` - Filter by storage location (for titles with volumes)
/// * `sort_by` - Field to sort by (title, publication_year, created_at)
/// * `sort_order` - Sort direction (asc, desc)
/// * `limit` - Maximum number of results (default: 100, max: 500)
/// * `offset` - Number of results to skip (for pagination)
///
/// # Examples
///
/// ```
/// // Search for Harry Potter books in English
/// ?q=harry potter&language=en
///
/// // Find all books in a series
/// ?series_id=uuid-here
///
/// // Find books by author published between 2000-2010
/// ?author_id=uuid-here&year_from=2000&year_to=2010
///
/// // Find wishlist items (books without volumes)
/// ?has_volumes=false
///
/// // Find available books by genre
/// ?genre_id=uuid-here&available=true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSearchParams {
    /// Free text search across multiple fields
    pub q: Option<String>,
    /// Filter by title (partial match)
    pub title: Option<String>,
    /// Filter by subtitle (partial match)
    pub subtitle: Option<String>,
    /// Filter by ISBN (partial or exact match)
    pub isbn: Option<String>,
    /// Filter by series UUID
    pub series_id: Option<String>,
    /// Filter by author UUID
    pub author_id: Option<String>,
    /// Filter by genre UUID
    pub genre_id: Option<String>,
    /// Filter by publisher UUID
    pub publisher_id: Option<String>,
    /// Minimum publication year (inclusive)
    pub year_from: Option<i32>,
    /// Maximum publication year (inclusive)
    pub year_to: Option<i32>,
    /// Filter by language code
    pub language: Option<String>,
    /// Filter by Dewey classification (partial match)
    pub dewey_code: Option<String>,
    /// Filter by ownership status (true=owned, false=wishlist)
    pub has_volumes: Option<bool>,
    /// Filter by availability (true=at least one available volume)
    pub available: Option<bool>,
    /// Filter by storage location UUID
    pub location_id: Option<String>,
    /// Sort field (title, publication_year, created_at)
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    /// Sort direction (asc, desc)
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
    /// Maximum number of results
    #[serde(default = "default_limit")]
    pub limit: i64,
    /// Number of results to skip (pagination)
    #[serde(default)]
    pub offset: i64,
}

fn default_sort_by() -> String {
    "title".to_string()
}

fn default_sort_order() -> String {
    "asc".to_string()
}

fn default_limit() -> i64 {
    100
}

impl TitleSearchParams {
    /// Validates and sanitizes the search parameters
    pub fn validate(&mut self) -> Result<(), String> {
        // Validate sort_by field
        match self.sort_by.as_str() {
            "title" | "publication_year" | "created_at" => {},
            _ => return Err(format!("Invalid sort_by field: {}. Must be one of: title, publication_year, created_at", self.sort_by)),
        }

        // Validate sort_order
        match self.sort_order.as_str() {
            "asc" | "desc" => {},
            _ => return Err(format!("Invalid sort_order: {}. Must be asc or desc", self.sort_order)),
        }

        // Validate and cap limit
        if self.limit < 1 {
            self.limit = 1;
        }
        if self.limit > 500 {
            self.limit = 500;
        }

        // Validate offset
        if self.offset < 0 {
            self.offset = 0;
        }

        // Validate year range
        if let (Some(from), Some(to)) = (self.year_from, self.year_to) {
            if from > to {
                return Err("year_from cannot be greater than year_to".to_string());
            }
        }

        Ok(())
    }
}
