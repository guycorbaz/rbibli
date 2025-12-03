//! Title management models.
//!
//! This module defines the data structures for managing book titles (abstract metadata).
//! It includes the main `Title` entity, search parameters, and duplicate detection models.
//!
//! # Key Features
//!
//! - **Abstract Metadata**: `Title` represents the book info (ISBN, author, etc.) separate from physical copies.
//! - **Search & Filtering**: `TitleSearchParams` supports complex queries including fuzzy search.
//! - **Duplicate Detection**: Models for identifying and merging duplicate entries.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Title represents the abstract book metadata shared across all physical copies.
///
/// A `Title` contains information like the book's name, ISBN, publisher, and summary.
/// It does *not* represent a specific physical book on a shelf; that is handled by the `Volume` entity.
///
/// # Database Structure
///
/// Mapped to the `titles` table in the database.
pub use shared::models::titles::Title;

/// TitleWithCount is returned by the list endpoint and includes the volume count.
///
/// This struct flattens the `Title` fields and adds a `volume_count` field,
/// allowing the frontend to display how many physical copies exist for each title.
pub use shared::models::titles::TitleWithCount;

pub use shared::dtos::titles::CreateTitleRequest;

pub use shared::dtos::titles::UpdateTitleRequest;

pub use shared::dtos::titles::TitleSearchParams;

use shared::models::enums::DuplicateConfidence;

/// Represents a potential duplicate title pair.
///
/// Contains two titles that may be duplicates, along with a similarity score
/// and confidence level to help users make merge decisions.
///
/// # Example Response
///
/// ```json
/// {
///   "title1": {
///     "id": "uuid1",
///     "title": "Harry Potter and the Sorcerer's Stone",
///     "isbn": "978-0439708180",
///     "volume_count": 3
///   },
///   "title2": {
///     "id": "uuid2",
///     "title": "Harry Potter and the Philosopher's Stone",
///     "isbn": "978-0439708180",
///     "volume_count": 2
///   },
///   "similarity_score": 95.5,
///   "confidence": "high",
///   "match_reasons": ["ISBN match", "Title similarity: 87%"]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePair {
    /// First title in the pair
    pub title1: TitleWithCount,
    /// Second title in the pair
    pub title2: TitleWithCount,
    /// Similarity score from 0.0 to 100.0
    pub similarity_score: f64,
    /// Confidence level categorization
    pub confidence: DuplicateConfidence,
    /// Reasons why these titles matched
    pub match_reasons: Vec<String>,
}

/// Response for duplicate detection endpoint.
///
/// Groups potential duplicates by confidence level for easier review.
///
/// # Example Response
///
/// ```json
/// {
///   "high_confidence": [
///     { "title1": {...}, "title2": {...}, "similarity_score": 95.5, ... }
///   ],
///   "medium_confidence": [],
///   "low_confidence": [],
///   "total_pairs": 1
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDetectionResponse {
    /// High confidence duplicates (≥90% similarity)
    pub high_confidence: Vec<DuplicatePair>,
    /// Medium confidence duplicates (70-89% similarity)
    pub medium_confidence: Vec<DuplicatePair>,
    /// Low confidence duplicates (50-69% similarity)
    pub low_confidence: Vec<DuplicatePair>,
    /// Total number of duplicate pairs found
    pub total_pairs: usize,
}

pub use shared::dtos::titles::{MergeTitlesRequest, MergeTitlesResponse};
