//! Genre management models.
//!
//! This module defines the data structures for managing book genres (categories).
//! Genres help organize the library by subject matter or literary style.
//!
//! # Key Features
//!
//! - **Categorization**: Simple name and description for grouping books.
//! - **Statistics**: Tracks how many titles belong to each genre.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Genre represents a category or classification for books.
///
/// # Database Structure
///
/// Mapped to the `genres` table in the database.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Genre {
    /// Unique identifier (UUID)
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    /// Name of the genre (e.g., "Science Fiction", "History")
    pub name: String,
    /// Optional description of what this genre encompasses
    pub description: Option<String>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// GenreWithTitleCount includes the number of titles associated with this genre.
///
/// Returned by list endpoints to show distribution of books across categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreWithTitleCount {
    /// The core genre data (flattened)
    #[serde(flatten)]
    pub genre: Genre,
    /// Number of titles associated with this genre
    pub title_count: i64,
}

/// Request payload for creating a new genre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGenreRequest {
    /// Genre name (required)
    pub name: String,
    /// Description
    pub description: Option<String>,
}

/// Request payload for updating an existing genre.
///
/// All fields are optional; only provided fields will be updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGenreRequest {
    /// New name
    pub name: Option<String>,
    /// New description
    pub description: Option<String>,
}
