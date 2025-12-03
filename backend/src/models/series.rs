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

pub use shared::dtos::series::CreateSeriesRequest;

pub use shared::dtos::series::UpdateSeriesRequest;
