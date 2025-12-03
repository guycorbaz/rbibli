use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Title represents the abstract book metadata shared across all physical copies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Title {
    /// Unique identifier (UUID)
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
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
    #[cfg_attr(feature = "backend", sqlx(rename = "genre_old"))]
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
    /// Raw image data (base64 encoded) - typically not stored in DB main table
    #[cfg_attr(feature = "backend", sqlx(skip))]
    pub image_data: Option<String>,
    /// MIME type of the stored image
    pub image_mime_type: Option<String>,
    /// Filename of the stored image
    pub image_filename: Option<String>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Extends `Title` with the count of physical volumes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithCount {
    /// The base title data
    #[serde(flatten)]
    pub title: Title,
    /// Number of physical copies (volumes) associated with this title
    pub volume_count: i64,
}
