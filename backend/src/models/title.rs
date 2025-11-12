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
