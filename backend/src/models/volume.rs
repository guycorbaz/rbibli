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
pub use shared::models::volumes::Volume;

pub use shared::dtos::volumes::CreateVolumeRequest;

pub use shared::dtos::volumes::UpdateVolumeRequest;
