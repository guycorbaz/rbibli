//! Loan models for the rbibli library management system.
//!
//! This module defines the data structures for managing book loans, including
//! loan records, loan status tracking, and loan extension functionality.
//!
//! # Loan Management
//!
//! The loan system supports:
//! - Creating loans by barcode scanning
//! - Tracking loan status (active, returned, overdue)
//! - Extending loans (max 1 extension per loan)
//! - Automatic overdue detection
//! - Borrower group-based loan duration policies

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::models::enums::LoanRecordStatus as LoanStatus;

/// Core loan record representing a volume loaned to a borrower.
///
/// # Fields
///
/// * `id` - Unique identifier (UUID)
/// * `title_id` - Reference to the title being loaned
/// * `volume_id` - Specific volume/copy being loaned
/// * `borrower_id` - Person borrowing the volume
/// * `loan_date` - When the loan was created
/// * `due_date` - When the volume should be returned
/// * `extension_count` - Number of times loan has been extended (max 1)
/// * `return_date` - Actual return date (None if not returned)
/// * `status` - Current loan status
/// * `created_at` - Record creation timestamp
/// * `updated_at` - Last modification timestamp
///
/// # Extension Policy
///
/// Loans can be extended once by adding the same duration as the original loan period.
/// For example, a 21-day loan can be extended by 21 more days.
///
/// # Timestamps
///
/// All DateTime fields are serialized as Unix timestamps (seconds since epoch) for API responses.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Loan {
    /// Unique loan identifier
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    /// ID of the title being loaned
    pub title_id: String,
    /// ID of the specific volume being loaned
    pub volume_id: String,
    /// ID of the borrower
    pub borrower_id: String,
    /// Date and time when the loan was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub loan_date: DateTime<Utc>,
    /// Date and time when the volume is due back
    #[serde(with = "chrono::serde::ts_seconds")]
    pub due_date: DateTime<Utc>,
    /// Number of times this loan has been extended (0 or 1)
    pub extension_count: i32,
    /// Date and time when the volume was actually returned (None if not returned)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "optional_ts_seconds")]
    pub return_date: Option<DateTime<Utc>>,
    /// Current status of the loan
    pub status: LoanStatus,
    /// Record creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Helper module for serializing optional timestamps as Unix timestamps.
///
/// Converts `Option<DateTime<Utc>>` to/from optional Unix timestamps (i64).
/// Used for the `return_date` field which may be None if the loan is not returned.
mod optional_ts_seconds {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes optional DateTime to optional Unix timestamp
    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_i64(dt.timestamp()),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes optional Unix timestamp to optional DateTime
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<i64> = Option::deserialize(deserializer)?;
        Ok(opt.map(|ts| DateTime::from_timestamp(ts, 0).unwrap()))
    }
}

/// Extended loan information with borrower and title details.
///
/// This structure combines the core loan record with additional context
/// needed for displaying loans in the UI. It's typically used for
/// listing active loans with full details.
///
/// # Fields
///
/// * `loan` - The core loan record (flattened into this struct)
/// * `title` - Human-readable title of the book
/// * `barcode` - Volume barcode for identification
/// * `borrower_name` - Name of the person who borrowed the volume
/// * `borrower_email` - Contact email (optional)
/// * `is_overdue` - Computed field indicating if loan is past due date
///
/// # Usage
///
/// Returned by the list active loans endpoint and used to populate
/// the loan management UI with all necessary information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanDetail {
    /// Core loan record (fields are flattened into this struct)
    #[serde(flatten)]
    pub loan: Loan,
    /// Title of the book being loaned
    pub title: String,
    /// Barcode of the specific volume
    pub barcode: String,
    /// Name of the borrower
    pub borrower_name: String,
    /// Email address of the borrower (optional)
    pub borrower_email: Option<String>,
    /// Whether this loan is past its due date
    pub is_overdue: bool,
}

/// Request payload for creating a new loan by barcode scanning.
///
/// # Fields
///
/// * `borrower_id` - UUID of the borrower (must exist in database)
/// * `barcode` - Volume barcode to loan (format: numeric, e.g., 123456)
///
/// # Workflow
///
/// 1. System looks up volume by barcode
/// 2. Validates volume is loanable and available
/// 3. Determines loan duration from borrower's group settings
/// 4. Creates loan record and updates volume status
///
/// # Validation
///
/// The system will reject the request if:
/// - Borrower ID doesn't exist
/// - Barcode is not found
/// - Volume is already loaned
/// - Volume is marked as not loanable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoanRequest {
    /// ID of the borrower
    pub borrower_id: String,
    /// Barcode of the volume to loan (e.g., "123456")
    pub barcode: String,
}

/// Request payload for returning a loaned volume.
///
/// # Fields
///
/// * `loan_id` - UUID of the loan to mark as returned
///
/// # Workflow
///
/// 1. Validates loan exists and is active
/// 2. Sets return_date to current timestamp
/// 3. Updates loan status to 'returned'
/// 4. Updates volume status to 'available'
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnLoanRequest {
    /// ID of the loan to return
    pub loan_id: String,
}
