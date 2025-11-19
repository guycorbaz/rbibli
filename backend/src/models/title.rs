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

/// Confidence level for duplicate detection matches
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DuplicateConfidence {
    /// High confidence (≥90%): Likely the same title
    High,
    /// Medium confidence (70-89%): Possibly the same title
    Medium,
    /// Low confidence (50-69%): Might be related
    Low,
}

/// Represents a potential duplicate title pair
///
/// Contains two titles that may be duplicates, along with a similarity score
/// and confidence level to help users make merge decisions.
///
/// # Fields
///
/// * `title1` - First title in the potential duplicate pair
/// * `title2` - Second title in the potential duplicate pair
/// * `similarity_score` - Numeric similarity score (0.0-100.0)
/// * `confidence` - Categorical confidence level (High/Medium/Low)
/// * `match_reasons` - List of reasons why these titles matched (e.g., "ISBN match", "Title similarity: 95%")
///
/// # Example Response
///
/// ```json
/// {
///   "title1": {
///     "id": "uuid1",
///     "title": "Harry Potter and the Sorcerer's Stone",
///     "isbn": "978-0439708180",
///     "volume_count": 3
///   },
///   "title2": {
///     "id": "uuid2",
///     "title": "Harry Potter and the Philosopher's Stone",
///     "isbn": "978-0439708180",
///     "volume_count": 2
///   },
///   "similarity_score": 95.5,
///   "confidence": "high",
///   "match_reasons": ["ISBN match", "Title similarity: 87%"]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePair {
    /// First title in the pair
    pub title1: TitleWithCount,
    /// Second title in the pair
    pub title2: TitleWithCount,
    /// Similarity score from 0.0 to 100.0
    pub similarity_score: f64,
    /// Confidence level categorization
    pub confidence: DuplicateConfidence,
    /// Reasons why these titles matched
    pub match_reasons: Vec<String>,
}

/// Response for duplicate detection endpoint
///
/// Groups potential duplicates by confidence level for easier review.
///
/// # Fields
///
/// * `high_confidence` - Pairs with high similarity (≥90%)
/// * `medium_confidence` - Pairs with medium similarity (70-89%)
/// * `low_confidence` - Pairs with low similarity (50-69%)
/// * `total_pairs` - Total number of duplicate pairs found
///
/// # Example Response
///
/// ```json
/// {
///   "high_confidence": [
///     { "title1": {...}, "title2": {...}, "similarity_score": 95.5, ... }
///   ],
///   "medium_confidence": [],
///   "low_confidence": [],
///   "total_pairs": 1
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDetectionResponse {
    /// High confidence duplicates (≥90% similarity)
    pub high_confidence: Vec<DuplicatePair>,
    /// Medium confidence duplicates (70-89% similarity)
    pub medium_confidence: Vec<DuplicatePair>,
    /// Low confidence duplicates (50-69% similarity)
    pub low_confidence: Vec<DuplicatePair>,
    /// Total number of duplicate pairs found
    pub total_pairs: usize,
}

/// Request to merge a secondary title into a primary title
///
/// All volumes from the secondary title will be moved to the primary title,
/// and the secondary title will be deleted.
///
/// # Fields
///
/// * `confirm` - Must be true to proceed with merge (safety check)
///
/// # Example Request
///
/// ```json
/// {
///   "confirm": true
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeTitlesRequest {
    /// Confirmation flag - must be true to proceed
    pub confirm: bool,
}

/// Response from merging two titles
///
/// Provides information about the merge operation results.
///
/// # Fields
///
/// * `success` - Whether the merge completed successfully
/// * `primary_title_id` - ID of the title that was kept
/// * `volumes_moved` - Number of volumes moved from secondary to primary
/// * `secondary_title_deleted` - Whether the secondary title was deleted
/// * `message` - Human-readable success message
///
/// # Example Response
///
/// ```json
/// {
///   "success": true,
///   "primary_title_id": "uuid1",
///   "volumes_moved": 2,
///   "secondary_title_deleted": true,
///   "message": "Successfully merged 2 volumes from secondary title into primary title"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeTitlesResponse {
    /// Whether the operation succeeded
    pub success: bool,
    /// ID of the primary title (kept)
    pub primary_title_id: String,
    /// Number of volumes moved
    pub volumes_moved: i64,
    /// Whether secondary title was deleted
    pub secondary_title_deleted: bool,
    /// Success message
    pub message: String,
}
