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
