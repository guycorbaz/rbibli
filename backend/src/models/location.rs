use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Location represents a physical location where volumes can be stored
/// Supports hierarchical organization (location can be inside another location)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// LocationWithPath includes the full hierarchical path
/// Example: "House > Room A > Shelf 1"
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
    pub parent_id: Option<String>, // UUID as string
}

/// UpdateLocationRequest for updating a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>, // UUID as string, null to remove parent
}
