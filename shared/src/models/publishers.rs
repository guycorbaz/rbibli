use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Publisher represents a company or entity that publishes books.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Publisher {
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
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

/// PublisherWithTitleCount includes the number of titles associated with this publisher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherWithTitleCount {
    #[serde(flatten)]
    pub publisher: Publisher,
    pub title_count: i64,
}
