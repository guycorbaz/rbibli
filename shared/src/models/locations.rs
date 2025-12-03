use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Location represents a physical place where volumes can be stored.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Location {
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// LocationWithPath includes the full hierarchical path and statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationWithPath {
    #[serde(flatten)]
    pub location: Location,
    pub full_path: String,
    pub level: i32,
    pub child_count: i32,
    pub volume_count: i32,
}
