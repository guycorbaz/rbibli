//! Frontend data models module.
//!
//! This module defines the data structures used in the frontend application,
//! mirroring the backend models and providing types for UI state management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the abstract metadata of a book title.
///
/// A `Title` corresponds to a specific book (e.g., "The Hobbit") but not a physical copy.
/// Physical copies are represented by `Volume`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    /// Unique UUID
    pub id: String,
    /// Main title text
    pub title: String,
    /// Optional subtitle
    pub subtitle: Option<String>,
    /// ISBN-10 or ISBN-13
    pub isbn: Option<String>,
    /// Publisher name (denormalized for display)
    pub publisher: Option<String>,
    /// Publisher UUID
    pub publisher_id: Option<String>,
    /// Year of publication
    pub publication_year: Option<i32>,
    /// Number of pages
    pub pages: Option<i32>,
    /// Language code (e.g., "en", "fr")
    pub language: String,
    /// Dewey Decimal Classification code
    pub dewey_code: Option<String>,
    /// Genre name (denormalized)
    pub genre: Option<String>,
    /// Genre UUID
    pub genre_id: Option<String>,
    /// Series name (denormalized)
    pub series_name: Option<String>,
    /// Series UUID
    pub series_id: Option<String>,
    /// Position in the series (e.g., "1", "Vol. 2")
    pub series_number: Option<String>,
    /// Plot summary or description
    pub summary: Option<String>,
    /// URL to cover image (base64 data URI or external URL)
    pub cover_url: Option<String>,
    /// Creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
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

pub use shared::dtos::titles::CreateTitleRequest;

pub use shared::dtos::titles::UpdateTitleRequest;

/// Represents a collection of related titles (e.g., "Harry Potter").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// SeriesWithTitleCount includes the title count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesWithTitleCount {
    #[serde(flatten)]
    pub series: Series,
    pub title_count: i64,
}

pub use shared::dtos::series::CreateSeriesRequest;

pub use shared::dtos::series::UpdateSeriesRequest;

/// Represents a physical location where volumes can be stored (e.g., "Shelf A", "Room 1").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// LocationWithPath includes the full hierarchical path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationWithPath {
    #[serde(flatten)]
    pub location: Location,
    pub full_path: String,
    pub level: i32,
    pub child_count: i32,
    pub volume_count: i32,
}

pub use shared::dtos::locations::CreateLocationRequest;

pub use shared::dtos::locations::UpdateLocationRequest;

/// Represents a book author.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub biography: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub nationality: Option<String>,
    pub website_url: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// AuthorWithTitleCount includes the title count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithTitleCount {
    #[serde(flatten)]
    pub author: Author,
    pub title_count: i64,
}

pub use shared::dtos::authors::CreateAuthorRequest;

pub use shared::dtos::authors::UpdateAuthorRequest;

pub use shared::models::enums::AuthorRole;

/// AuthorWithRole includes role and display order for a title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithRole {
    #[serde(flatten)]
    pub author: Author,
    pub role: AuthorRole,
    pub display_order: i32,
}

pub use shared::dtos::authors::AddAuthorToTitleRequest;

/// Represents a book publisher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publisher {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub country: Option<String>,
    pub founded_year: Option<i32>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// PublisherWithTitleCount includes the title count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherWithTitleCount {
    #[serde(flatten)]
    pub publisher: Publisher,
    pub title_count: i64,
}

pub use shared::dtos::publishers::CreatePublisherRequest;

pub use shared::dtos::publishers::UpdatePublisherRequest;

/// Represents a book genre or category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// GenreWithTitleCount includes the title count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreWithTitleCount {
    #[serde(flatten)]
    pub genre: Genre,
    pub title_count: i64,
}

pub use shared::dtos::genres::CreateGenreRequest;

pub use shared::dtos::genres::UpdateGenreRequest;

pub use shared::models::enums::VolumeCondition;

pub use shared::models::enums::LoanStatus;

/// Represents a physical copy of a title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub id: String,
    pub title_id: String,
    pub copy_number: i32,
    pub barcode: String,
    pub condition: VolumeCondition,
    pub location_id: Option<String>,
    pub loan_status: LoanStatus,
    pub individual_notes: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

pub use shared::dtos::volumes::CreateVolumeRequest;

pub use shared::dtos::volumes::UpdateVolumeRequest;

/// Response from ISBN lookup containing book data from Google Books API
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// BorrowerGroup represents a group of borrowers with specific loan policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowerGroup {
    pub id: String,
    pub name: String,
    pub loan_duration_days: i32,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// CreateBorrowerGroupRequest for creating a new borrower group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBorrowerGroupRequest {
    pub name: String,
    pub loan_duration_days: i32,
    pub description: Option<String>,
}

/// UpdateBorrowerGroupRequest for updating a borrower group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBorrowerGroupRequest {
    pub name: Option<String>,
    pub loan_duration_days: Option<i32>,
    pub description: Option<String>,
}

/// Represents a person who can borrow volumes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Borrower {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub group_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// BorrowerWithGroup includes the group information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowerWithGroup {
    #[serde(flatten)]
    pub borrower: Borrower,
    pub group_name: Option<String>,
    pub loan_duration_days: Option<i32>,
    pub active_loan_count: i32,
}

pub use shared::dtos::borrowers::CreateBorrowerRequest;

pub use shared::dtos::borrowers::UpdateBorrowerRequest;

pub use shared::models::enums::LoanRecordStatus;

/// Represents a loan transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    pub id: String,
    pub title_id: String,
    pub volume_id: String,
    pub borrower_id: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub loan_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub due_date: DateTime<Utc>,
    pub extension_count: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "optional_ts_seconds")]
    pub return_date: Option<DateTime<Utc>>,
    pub status: LoanRecordStatus,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Helper module for optional timestamp serialization
mod optional_ts_seconds {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_i64(dt.timestamp()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<i64> = Option::deserialize(deserializer)?;
        Ok(opt.map(|ts| DateTime::from_timestamp(ts, 0).unwrap()))
    }
}

/// LoanDetail includes full information about a loan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanDetail {
    #[serde(flatten)]
    pub loan: Loan,
    pub title: String,
    pub barcode: String,
    pub borrower_name: String,
    pub borrower_email: Option<String>,
    pub is_overdue: bool,
}

pub use shared::dtos::loans::CreateLoanRequest;

pub use shared::dtos::loans::CreateLoanResponse;


// ============================================================================
// Statistics Models
// ============================================================================

/// Statistics for volumes per genre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreStatistic {
    pub genre_id: Option<String>,
    pub genre_name: String,
    pub volume_count: i64,
    pub title_count: i64,
}

/// Statistics for volumes per location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationStatistic {
    pub location_id: Option<String>,
    pub location_name: String,
    pub location_path: String,
    pub volume_count: i64,
}

/// Statistics for loan status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanStatistic {
    pub status: String,
    pub count: i64,
}

/// Overall library statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStatistics {
    pub total_titles: i64,
    pub total_volumes: i64,
    pub total_authors: i64,
    pub total_publishers: i64,
    pub total_genres: i64,
    pub total_locations: i64,
    pub total_borrowers: i64,
    pub active_loans: i64,
    pub overdue_loans: i64,
}

// ============================================================================
// Duplicate Detection Models
// ============================================================================

pub use shared::models::enums::DuplicateConfidence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePair {
    pub title1: TitleWithCount,
    pub title2: TitleWithCount,
    pub similarity_score: f64,
    pub confidence: DuplicateConfidence,
    pub match_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDetectionResponse {
    pub high_confidence: Vec<DuplicatePair>,
    pub medium_confidence: Vec<DuplicatePair>,
    pub low_confidence: Vec<DuplicatePair>,
    pub total_pairs: usize,
}

pub use shared::dtos::titles::MergeTitlesRequest;

pub use shared::dtos::titles::MergeTitlesResponse;
