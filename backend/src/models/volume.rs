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

/// Represents the physical condition of a volume.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum VolumeCondition {
    /// Like new, no visible wear.
    #[sqlx(rename = "excellent")]
    Excellent,
    /// Minor wear, but structurally sound.
    #[sqlx(rename = "good")]
    Good,
    /// Noticeable wear, but readable.
    #[sqlx(rename = "fair")]
    Fair,
    /// Significant wear, loose pages, or markings.
    #[sqlx(rename = "poor")]
    Poor,
    /// Severe damage, may be unusable.
    #[sqlx(rename = "damaged")]
    Damaged,
}

/// Represents the current loan status of a volume.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum LoanStatus {
    /// Available for borrowing.
    #[sqlx(rename = "available")]
    Available,
    /// Currently checked out by a borrower.
    #[sqlx(rename = "loaned")]
    Loaned,
    /// Checked out and past the due date.
    #[sqlx(rename = "overdue")]
    Overdue,
    /// Reported lost or missing.
    #[sqlx(rename = "lost")]
    Lost,
    /// Removed from circulation for repair or review.
    #[sqlx(rename = "maintenance")]
    Maintenance,
}

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

/// Request payload for creating a new volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVolumeRequest {
    /// UUID of the parent title
    pub title_id: String,
    /// Unique barcode (must be globally unique)
    pub barcode: String,
    /// Initial physical condition
    pub condition: VolumeCondition,
    /// UUID of the initial storage location (optional)
    pub location_id: Option<String>,
    /// Initial notes
    pub individual_notes: Option<String>,
}

/// Request payload for updating an existing volume.
///
/// All fields are optional; only provided fields will be updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVolumeRequest {
    /// New barcode (must be unique if changed)
    pub barcode: Option<String>,
    /// New physical condition
    pub condition: Option<VolumeCondition>,
    /// New storage location UUID (or empty to remove location)
    pub location_id: Option<String>,
    /// New loan status (careful: changing this manually may bypass loan logic)
    pub loan_status: Option<LoanStatus>,
    /// New notes
    pub individual_notes: Option<String>,
}
