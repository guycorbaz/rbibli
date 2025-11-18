use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Series represents a collection of related titles
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Series {
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// SeriesWithTitleCount is returned by the list endpoint, includes title count
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
