//! Author management models.
//!
//! This module defines the data structures for managing book authors.
//! It supports a many-to-many relationship with titles, allowing books to have multiple authors
//! with different roles (e.g., main author, translator, illustrator).
//!
//! # Key Features
//!
//! - **Biographical Data**: Stores name, birth/death dates, nationality, and biography.
//! - **Roles**: Distinguishes between main authors, co-authors, translators, etc.
//! - **Relationships**: Manages the link between titles and authors via `TitleAuthor`.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Author represents a person who writes or contributes to books.
///
/// # Database Structure
///
/// Mapped to the `authors` table in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    /// Unique identifier (UUID)
    pub id: Uuid,
    /// First name (optional in some contexts, but usually present)
    pub first_name: String,
    /// Last name (required)
    pub last_name: String,
    /// Short biography or description
    pub biography: Option<String>,
    /// Date of birth (YYYY-MM-DD)
    pub birth_date: Option<NaiveDate>,
    /// Date of death (YYYY-MM-DD), if applicable
    pub death_date: Option<NaiveDate>,
    /// Nationality or country of origin
    pub nationality: Option<String>,
    /// Personal website or profile URL
    pub website_url: Option<String>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// AuthorWithTitleCount includes the number of titles associated with this author.
///
/// Returned by list endpoints to show how many books by this author are in the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorWithTitleCount {
    /// The core author data (flattened)
    #[serde(flatten)]
    pub author: Author,
    /// Number of titles associated with this author
    pub title_count: i64,
}

/// Request payload for creating a new author.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthorRequest {
    /// First name
    pub first_name: String,
    /// Last name
    pub last_name: String,
    /// Biography
    pub biography: Option<String>,
    /// Date of birth (ISO format: YYYY-MM-DD)
    pub birth_date: Option<String>,
    /// Date of death (ISO format: YYYY-MM-DD)
    pub death_date: Option<String>,
    /// Nationality
    pub nationality: Option<String>,
    /// Website URL
    pub website_url: Option<String>,
}

/// Request payload for updating an existing author.
///
/// All fields are optional; only provided fields will be updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthorRequest {
    /// New first name
    pub first_name: Option<String>,
    /// New last name
    pub last_name: Option<String>,
    /// New biography
    pub biography: Option<String>,
    /// New birth date (ISO format: YYYY-MM-DD)
    pub birth_date: Option<String>,
    /// New death date (ISO format: YYYY-MM-DD)
    pub death_date: Option<String>,
    /// New nationality
    pub nationality: Option<String>,
    /// New website URL
    pub website_url: Option<String>,
}

/// Represents the relationship between a title and an author.
///
/// This is the junction table entity that links `titles` and `authors`,
/// including the specific role the author played for that title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAuthor {
    /// Unique identifier for this relationship
    pub id: Uuid,
    /// UUID of the title
    pub title_id: Uuid,
    /// UUID of the author
    pub author_id: Uuid,
    /// Role of the author (e.g., MainAuthor, Translator)
    pub role: AuthorRole,
    /// Display order for listing authors (1, 2, 3...)
    pub display_order: i32,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

use shared::models::enums::AuthorRole;

/// Request payload for associating an author with a title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAuthorToTitleRequest {
    /// UUID of the author to add
    pub author_id: String,
    /// Role of the author for this title
    pub role: AuthorRole,
    /// Optional display order (defaults to next available)
    pub display_order: Option<i32>,
}
