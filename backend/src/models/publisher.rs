//! Publisher management models.
//!
//! This module defines the data structures for managing book publishers.
//! Publishers are entities that produce and distribute books.
//!
//! # Key Features
//!
//! - **Company Info**: Stores name, description, website, and country.
//! - **History**: Tracks the founding year.
//! - **Integration**: Linked to titles to organize the library by publisher.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Publisher represents a company or entity that publishes books.
///
/// # Database Structure
///
/// Mapped to the `publishers` table in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publisher {
    /// Unique identifier (UUID)
    pub id: Uuid,
    /// Name of the publisher (required)
    pub name: String,
    /// Description or history of the publisher
    pub description: Option<String>,
    /// Official website URL
    pub website_url: Option<String>,
    /// Country of origin or headquarters
    pub country: Option<String>,
    /// Year the publisher was founded
    pub founded_year: Option<i32>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// PublisherWithTitleCount includes the number of titles associated with this publisher.
///
/// Returned by list endpoints to show how many books from this publisher are in the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherWithTitleCount {
    /// The core publisher data (flattened)
    #[serde(flatten)]
    pub publisher: Publisher,
    /// Number of titles associated with this publisher
    pub title_count: i64,
}

pub use shared::dtos::publishers::CreatePublisherRequest;

pub use shared::dtos::publishers::UpdatePublisherRequest;
