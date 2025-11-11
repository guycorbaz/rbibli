use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Publisher represents a book publisher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publisher {
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

/// PublisherWithTitleCount includes the count of titles from this publisher
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

/// UpdatePublisherRequest for updating an existing publisher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublisherRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub country: Option<String>,
    pub founded_year: Option<i32>,
}
