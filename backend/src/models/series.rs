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
pub use shared::models::series::Series;
pub use shared::models::series::SeriesWithTitleCount;

pub use shared::dtos::series::CreateSeriesRequest;

pub use shared::dtos::series::UpdateSeriesRequest;
