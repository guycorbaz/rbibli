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
pub use shared::models::genres::Genre;
pub use shared::models::genres::GenreWithTitleCount;

pub use shared::dtos::genres::CreateGenreRequest;

pub use shared::dtos::genres::UpdateGenreRequest;
