use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::enums::AuthorRole;

/// Author represents a person who writes or contributes to books.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Author {
    /// Unique identifier (UUID)
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    /// First name
    pub first_name: String,
    /// Last name
    pub last_name: String,
    /// Short biography
    pub biography: Option<String>,
    /// Date of birth (YYYY-MM-DD)
    pub birth_date: Option<NaiveDate>,
    /// Date of death (YYYY-MM-DD)
    pub death_date: Option<NaiveDate>,
    /// Nationality
    pub nationality: Option<String>,
    /// Website URL
    pub website_url: Option<String>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// AuthorWithTitleCount includes the number of titles associated with this author.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithTitleCount {
    #[serde(flatten)]
    pub author: Author,
    pub title_count: i64,
}

/// Represents the relationship between a title and an author.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct TitleAuthor {
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub title_id: Uuid,
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub author_id: Uuid,
    pub role: AuthorRole,
    pub display_order: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}
