//! Location management models.
//!
//! This module defines the data structures for managing physical storage locations.
//! Locations are hierarchical, allowing for structures like "Room A > Shelf 1 > Bin 3".
//!
//! # Key Features
//!
//! - **Hierarchy**: Supports parent-child relationships for nested locations.
//! - **Path Generation**: Can generate full paths (e.g., "Library > Fiction > A-M").
//! - **Statistics**: Tracks volume counts per location.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Location represents a physical place where volumes can be stored.
///
/// Locations can be nested to create a hierarchy (e.g., Building -> Room -> Shelf).
///
/// # Database Structure
///
/// Mapped to the `locations` table in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Unique identifier (UUID)
    pub id: Uuid,
    /// Name of the location (e.g., "Living Room")
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// UUID of the parent location (None for top-level locations)
    pub parent_id: Option<Uuid>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// LocationWithPath includes the full hierarchical path and statistics.
///
/// Returned by list endpoints to provide context and usage data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationWithPath {
    /// The core location data (flattened)
    #[serde(flatten)]
    pub location: Location,
    /// Full hierarchical path string (e.g., "House > Living Room > Bookshelf")
    pub full_path: String,
    /// Depth level in the hierarchy (0 = root)
    pub level: i32,
    /// Number of direct sub-locations
    pub child_count: i32,
    /// Number of volumes stored directly in this location
    pub volume_count: i32,
}

/// Request payload for creating a new location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    /// Location name (required)
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Parent location UUID (optional)
    pub parent_id: Option<String>,
}

/// Request payload for updating an existing location.
///
/// All fields are optional; only provided fields will be updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    /// New name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New parent location UUID (or empty to move to root)
    pub parent_id: Option<String>,
}
