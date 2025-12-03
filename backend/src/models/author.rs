//! Author management models.
//!
//! This module defines the data structures for managing book authors.
//! It supports a many-to-many relationship with titles, allowing books to have multiple authors
//! with different roles (e.g., main author, illustrator).
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
pub use shared::models::authors::Author;
pub use shared::models::authors::AuthorWithTitleCount;
pub use shared::models::authors::TitleAuthor;

use shared::models::enums::AuthorRole;

pub use shared::dtos::authors::AddAuthorToTitleRequest;
pub use shared::dtos::authors::CreateAuthorRequest;
pub use shared::dtos::authors::UpdateAuthorRequest;
