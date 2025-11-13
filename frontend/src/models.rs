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
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    pub genre: Option<String>,
    pub genre_id: Option<String>,
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
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: String,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
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
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: Option<String>,
    pub dewey_code: Option<String>,
    pub dewey_category: Option<String>,
    #[serde(alias = "genre")]
    pub genre_id: Option<String>,
    pub summary: Option<String>,
    pub cover_url: Option<String>,
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
}

/// CreateLocationRequest for creating a new location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
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
