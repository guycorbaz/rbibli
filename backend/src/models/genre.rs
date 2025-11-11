use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Genre represents a book genre/category
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Genre {
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// GenreWithTitleCount includes the count of titles in this genre
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

/// UpdateGenreRequest for updating an existing genre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGenreRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
