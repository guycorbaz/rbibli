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
pub use shared::models::titles::Title;

/// Extends `Title` with the count of physical volumes.
pub use shared::models::titles::TitleWithCount;

pub use shared::dtos::titles::CreateTitleRequest;

pub use shared::dtos::titles::UpdateTitleRequest;

/// Represents a collection of related titles (e.g., "Harry Potter").
pub use shared::models::series::Series;
pub use shared::models::series::SeriesWithTitleCount;

pub use shared::dtos::series::CreateSeriesRequest;

pub use shared::dtos::series::UpdateSeriesRequest;

pub use shared::models::locations::Location;
pub use shared::models::locations::LocationWithPath;

pub use shared::dtos::locations::CreateLocationRequest;

pub use shared::dtos::locations::UpdateLocationRequest;

pub use shared::models::authors::Author;
pub use shared::models::authors::AuthorWithTitleCount;

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

pub use shared::models::publishers::Publisher;
pub use shared::models::publishers::PublisherWithTitleCount;

pub use shared::dtos::publishers::CreatePublisherRequest;

pub use shared::dtos::publishers::UpdatePublisherRequest;

pub use shared::models::genres::Genre;
pub use shared::models::genres::GenreWithTitleCount;

pub use shared::dtos::genres::CreateGenreRequest;

pub use shared::dtos::genres::UpdateGenreRequest;

pub use shared::models::enums::VolumeCondition;

pub use shared::models::enums::LoanStatus;

pub use shared::models::volumes::Volume;

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

pub use shared::models::borrowers::BorrowerGroup;

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

pub use shared::models::borrowers::Borrower;
pub use shared::models::borrowers::BorrowerWithGroup;

pub use shared::dtos::borrowers::CreateBorrowerRequest;

pub use shared::dtos::borrowers::UpdateBorrowerRequest;

pub use shared::models::enums::LoanRecordStatus;

pub use shared::models::loans::Loan;
pub use shared::models::loans::LoanDetail;

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
