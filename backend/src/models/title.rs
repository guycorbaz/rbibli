//! Title management models.
//!
//! This module defines the data structures for managing book titles (abstract metadata).
//! It includes the main `Title` entity, search parameters, and duplicate detection models.
//!
//! # Key Features
//!
//! - **Abstract Metadata**: `Title` represents the book info (ISBN, author, etc.) separate from physical copies.
//! - **Search & Filtering**: `TitleSearchParams` supports complex queries including fuzzy search.
//! - **Duplicate Detection**: Models for identifying and merging duplicate entries.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Title represents the abstract book metadata shared across all physical copies.
///
/// A `Title` contains information like the book's name, ISBN, publisher, and summary.
/// It does *not* represent a specific physical book on a shelf; that is handled by the `Volume` entity.
///
/// # Database Structure
///
/// Mapped to the `titles` table in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Title {
    /// Unique identifier (UUID)
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    /// Main title of the book
    pub title: String,
    /// Optional subtitle
    pub subtitle: Option<String>,
    /// International Standard Book Number (10 or 13 digits)
    pub isbn: Option<String>,
    /// Name of the publisher
    pub publisher: Option<String>,
    /// UUID of the publisher entity
    pub publisher_id: Option<String>,
    /// Year of publication
    pub publication_year: Option<i32>,
    /// Number of pages
    pub pages: Option<i32>,
    /// Language code (e.g., "en", "fr")
    pub language: String,
    /// Dewey Decimal Classification code (e.g., "005.133")
    pub dewey_code: Option<String>,
    /// Genre name
    pub genre: Option<String>,
    /// UUID of the genre entity
    pub genre_id: Option<String>,
    /// Series name
    pub series_name: Option<String>,
    /// UUID of the series entity
    pub series_id: Option<String>,
    /// Number within the series (e.g., "1", "Vol. 2")
    pub series_number: Option<String>,
    /// Plot summary or description
    pub summary: Option<String>,
    /// URL to the cover image
    pub cover_url: Option<String>,
    /// Base64 encoded image data (optional, for direct storage)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "base64_option")]
    pub image_data: Option<Vec<u8>>,
    /// MIME type of the image data (e.g., "image/jpeg")
    pub image_mime_type: Option<String>,
    /// Original filename of the uploaded image
    pub image_filename: Option<String>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
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

/// TitleWithCount is returned by the list endpoint and includes the volume count.
///
/// This struct flattens the `Title` fields and adds a `volume_count` field,
/// allowing the frontend to display how many physical copies exist for each title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithCount {
    /// The core title metadata (flattened)
    #[serde(flatten)]
    pub title: Title,
    /// Number of physical volumes associated with this title
    pub volume_count: i64,
}

pub use shared::dtos::titles::CreateTitleRequest;

pub use shared::dtos::titles::UpdateTitleRequest;

pub use shared::dtos::titles::TitleSearchParams;

use shared::models::enums::DuplicateConfidence;

/// Represents a potential duplicate title pair.
///
/// Contains two titles that may be duplicates, along with a similarity score
/// and confidence level to help users make merge decisions.
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

/// Response for duplicate detection endpoint.
///
/// Groups potential duplicates by confidence level for easier review.
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

pub use shared::dtos::titles::{MergeTitlesRequest, MergeTitlesResponse};
