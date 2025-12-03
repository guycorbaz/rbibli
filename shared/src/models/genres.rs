use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Genre represents a category or classification for books.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Genre {
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// GenreWithTitleCount includes the number of titles associated with this genre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreWithTitleCount {
    #[serde(flatten)]
    pub genre: Genre,
    pub title_count: i64,
}
