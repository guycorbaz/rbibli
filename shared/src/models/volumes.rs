use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::enums::{VolumeCondition, LoanStatus};

/// Volume represents a specific physical copy of a title.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Volume {
    /// Unique identifier (UUID)
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    /// Foreign key to the parent Title
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub title_id: Uuid,
    /// Sequential number of this copy
    pub copy_number: i32,
    /// Unique barcode string
    pub barcode: String,
    /// Physical condition
    pub condition: VolumeCondition,
    /// Foreign key to the storage Location
    pub location_id: Option<String>,
    /// Current availability status
    pub loan_status: LoanStatus,
    /// Individual notes
    pub individual_notes: Option<String>,
    /// Timestamp of creation
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp of last update
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}
