//! Volume management models.
//!
//! This module defines the data structures for managing physical book copies (volumes).
//! Each volume is a unique physical item identified by a barcode, distinct from the abstract `Title`.
//!
//! # Key Features
//!
//! - **Physical Tracking**: Tracks condition, location, and unique barcodes.
//! - **Loan Status**: Manages the availability of each copy (Available, Loaned, Lost, etc.).
//! - **Copy Numbering**: Automatically assigns sequential copy numbers (1, 2, 3...) per title.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::models::enums::{VolumeCondition, LoanStatus};

/// Volume represents a specific physical copy of a title.
///
/// Unlike `Title`, which is abstract metadata, `Volume` corresponds to a tangible object
/// on a shelf. It has a unique barcode and specific physical attributes.
///
/// # Database Structure
///
/// Mapped to the `volumes` table in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    /// Unique identifier (UUID)
    pub id: Uuid,
    /// Foreign key to the parent Title
    pub title_id: Uuid,
    /// Sequential number of this copy for the title (1, 2, 3...)
    pub copy_number: i32,
    /// Unique barcode string (e.g., "VOL-000001")
    pub barcode: String,
    /// Physical condition of the book
    pub condition: VolumeCondition,
    /// Foreign key to the storage Location (optional)
    pub location_id: Option<Uuid>,
    /// Current availability status
    pub loan_status: LoanStatus,
    /// Notes specific to this physical copy (e.g., "Signed by author", "Missing page 42")
    pub individual_notes: Option<String>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

pub use shared::dtos::volumes::CreateVolumeRequest;

pub use shared::dtos::volumes::UpdateVolumeRequest;
