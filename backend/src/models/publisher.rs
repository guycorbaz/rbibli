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
pub use shared::models::publishers::Publisher;
pub use shared::models::publishers::PublisherWithTitleCount;

pub use shared::dtos::publishers::CreatePublisherRequest;

pub use shared::dtos::publishers::UpdatePublisherRequest;
