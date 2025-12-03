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
pub use shared::models::loans::Loan;
pub use shared::models::loans::LoanDetail;

pub use shared::dtos::loans::CreateLoanRequest;

pub use shared::dtos::loans::ReturnLoanRequest;
