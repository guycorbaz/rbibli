use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Title represents the abstract book metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub publisher_id: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    pub genre: Option<String>,
    pub genre_id: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<String>,
    pub series_number: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// TitleWithCount includes the volume count
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
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub series_id: Option<String>,
    pub series_number: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
}

/// Series represents a collection of related titles
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

/// CreateSeriesRequest for creating a new series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSeriesRequest {
    pub name: String,
    pub description: Option<String>,
}

/// UpdateSeriesRequest for updating an existing series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSeriesRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Location represents a physical location where volumes can be stored
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

/// CreateLocationRequest for creating a new location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

/// UpdateLocationRequest for updating an existing location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

/// Author represents a book author
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

/// CreateAuthorRequest for creating a new author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthorRequest {
    pub first_name: String,
    pub last_name: String,
    pub biography: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub nationality: Option<String>,
    pub website_url: Option<String>,
}

/// UpdateAuthorRequest for updating an existing author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthorRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub biography: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub nationality: Option<String>,
    pub website_url: Option<String>,
}

/// AuthorRole enum for different author roles in a title
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuthorRole {
    MainAuthor,
    CoAuthor,
    Translator,
    Illustrator,
    Editor,
}

/// AuthorWithRole includes role and display order for a title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithRole {
    #[serde(flatten)]
    pub author: Author,
    pub role: AuthorRole,
    pub display_order: i32,
}

/// AddAuthorToTitleRequest for associating an author with a title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAuthorToTitleRequest {
    pub author_id: String,
    pub role: AuthorRole,
    pub display_order: Option<i32>,
}

/// Publisher represents a book publisher
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

/// CreatePublisherRequest for creating a new publisher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePublisherRequest {
    pub name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub country: Option<String>,
    pub founded_year: Option<i32>,
}

/// UpdatePublisherRequest for updating a publisher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublisherRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub country: Option<String>,
    pub founded_year: Option<i32>,
}

/// Genre represents a book genre/category
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

/// CreateGenreRequest for creating a new genre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGenreRequest {
    pub name: String,
    pub description: Option<String>,
}

/// UpdateGenreRequest for updating a genre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGenreRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// VolumeCondition represents the physical condition of a volume
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VolumeCondition {
    #[serde(rename = "Excellent")]
    Excellent,
    #[serde(rename = "Good")]
    Good,
    #[serde(rename = "Fair")]
    Fair,
    #[serde(rename = "Poor")]
    Poor,
    #[serde(rename = "Damaged")]
    Damaged,
}

impl std::fmt::Display for VolumeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeCondition::Excellent => write!(f, "Excellent"),
            VolumeCondition::Good => write!(f, "Good"),
            VolumeCondition::Fair => write!(f, "Fair"),
            VolumeCondition::Poor => write!(f, "Poor"),
            VolumeCondition::Damaged => write!(f, "Damaged"),
        }
    }
}

/// LoanStatus represents the current loan status of a volume
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoanStatus {
    #[serde(rename = "Available")]
    Available,
    #[serde(rename = "Loaned")]
    Loaned,
    #[serde(rename = "Overdue")]
    Overdue,
    #[serde(rename = "Lost")]
    Lost,
    #[serde(rename = "Maintenance")]
    Maintenance,
}

impl std::fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanStatus::Available => write!(f, "Available"),
            LoanStatus::Loaned => write!(f, "Loaned"),
            LoanStatus::Overdue => write!(f, "Overdue"),
            LoanStatus::Lost => write!(f, "Lost"),
            LoanStatus::Maintenance => write!(f, "Maintenance"),
        }
    }
}

/// Volume represents a physical copy of a title
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

/// CreateVolumeRequest for creating a new volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVolumeRequest {
    pub title_id: String,
    pub barcode: String,
    pub condition: VolumeCondition,
    pub location_id: Option<String>,
    pub individual_notes: Option<String>,
}

/// UpdateVolumeRequest for updating an existing volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVolumeRequest {
    pub barcode: Option<String>,
    pub condition: Option<VolumeCondition>,
    pub location_id: Option<String>,
    pub loan_status: Option<LoanStatus>,
    pub individual_notes: Option<String>,
}

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

/// Borrower represents a person who can borrow volumes
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

/// CreateBorrowerRequest for creating a new borrower
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBorrowerRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub group_id: Option<String>,
}

/// UpdateBorrowerRequest for updating a borrower
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBorrowerRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub group_id: Option<String>,
}

/// LoanRecordStatus represents the status of a loan record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoanRecordStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "returned")]
    Returned,
    #[serde(rename = "overdue")]
    Overdue,
}

impl std::fmt::Display for LoanRecordStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanRecordStatus::Active => write!(f, "Active"),
            LoanRecordStatus::Returned => write!(f, "Returned"),
            LoanRecordStatus::Overdue => write!(f, "Overdue"),
        }
    }
}

/// Loan represents a loan record
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

/// CreateLoanRequest for creating a loan by barcode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoanRequest {
    pub borrower_id: String,
    pub barcode: String,
}

/// Response from creating a loan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoanResponse {
    pub id: String,
    pub due_date: i64,
    pub loan_duration_days: i32,
}


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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DuplicateConfidence {
    High,
    Medium,
    Low,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeTitlesRequest {
    pub confirm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeTitlesResponse {
    pub success: bool,
    pub primary_title_id: String,
    pub volumes_moved: i64,
    pub secondary_title_deleted: bool,
    pub message: String,
}
