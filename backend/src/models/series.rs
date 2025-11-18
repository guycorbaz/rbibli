//! Series models for the rbibli library management system.
//!
//! This module defines data structures for managing book series (collections),
//! such as comic series (Asterix, Tintin), book series (Harry Potter),
//! and magazine collections.
//!
//! # Series Management
//!
//! The series system supports:
//! - One-to-many relationship: one series contains multiple titles
//! - Each title can belong to at most one series (or no series)
//! - Optional descriptions for additional context
//! - Title count tracking for each series
//! - Delete protection: series with associated titles cannot be deleted

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a book series or collection.
///
/// A series is a named collection of related titles. Examples include:
/// - Comic series: Asterix, Tintin, Calvin and Hobbes
/// - Book series: Harry Potter, Lord of the Rings, Foundation
/// - Educational series: "For Dummies" series
/// - Magazine collections: Time Magazine, National Geographic
///
/// # Fields
///
/// * `id` - Unique identifier (UUID)
/// * `name` - Series name (e.g., "Asterix", "Harry Potter")
/// * `description` - Optional detailed description
/// * `created_at` - Record creation timestamp
/// * `updated_at` - Last modification timestamp
///
/// # Relationship with Titles
///
/// Titles reference series via `series_id` foreign key. The relationship
/// uses ON DELETE SET NULL, so deleting a series (if allowed) sets
/// associated titles' series_id to NULL rather than cascading deletion.
///
/// # Delete Protection
///
/// Series with associated titles cannot be deleted. The API will return
/// a 409 Conflict error if deletion is attempted while titles reference
/// the series.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Series {
    /// Unique series identifier
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    /// Series name (e.g., "Asterix")
    pub name: String,
    /// Optional description providing additional context
    pub description: Option<String>,
    /// Record creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Extended series information including title count.
///
/// This structure is used when listing series to provide both the
/// series details and the number of titles associated with it.
/// The title count is useful for:
/// - Displaying collection size in the UI
/// - Implementing delete protection logic
/// - Sorting series by popularity
///
/// # Fields
///
/// * `series` - The core series record (flattened into this struct)
/// * `title_count` - Number of titles belonging to this series
///
/// # Usage
///
/// Returned by the list series endpoint (GET /api/v1/series).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesWithTitleCount {
    /// Core series record (fields are flattened into this struct)
    #[serde(flatten)]
    pub series: Series,
    /// Number of titles in this series
    pub title_count: i64,
}

/// Request payload for creating a new series.
///
/// # Fields
///
/// * `name` - Series name (required, e.g., "Harry Potter")
/// * `description` - Optional description
///
/// # Example
///
/// ```json
/// {
///   "name": "Harry Potter",
///   "description": "Fantasy series by J.K. Rowling featuring the young wizard Harry Potter"
/// }
/// ```
///
/// # Validation
///
/// - Name must not be empty
/// - Name should be reasonably unique (no strict enforcement)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSeriesRequest {
    /// Series name (required)
    pub name: String,
    /// Optional description
    pub description: Option<String>,
}

/// Request payload for updating an existing series.
///
/// All fields are optional, allowing partial updates. Only provided
/// fields will be updated in the database.
///
/// # Fields
///
/// * `name` - Optional new series name
/// * `description` - Optional new description (can set to None to clear)
///
/// # Example
///
/// ```json
/// {
///   "name": "Updated Series Name",
///   "description": "Updated description"
/// }
/// ```
///
/// # Behavior
///
/// - If name is None, the existing name is preserved
/// - If description is None, the existing description is preserved
/// - To clear a description, explicitly set it to empty string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSeriesRequest {
    /// Optional new name for the series
    pub name: Option<String>,
    /// Optional new description (or None to keep existing)
    pub description: Option<String>,
}
