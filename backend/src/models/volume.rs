use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// VolumeCondition represents the physical condition of a volume
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum VolumeCondition {
    #[sqlx(rename = "excellent")]
    Excellent,
    #[sqlx(rename = "good")]
    Good,
    #[sqlx(rename = "fair")]
    Fair,
    #[sqlx(rename = "poor")]
    Poor,
    #[sqlx(rename = "damaged")]
    Damaged,
}

/// LoanStatus represents the current loan status of a volume
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum LoanStatus {
    #[sqlx(rename = "available")]
    Available,
    #[sqlx(rename = "loaned")]
    Loaned,
    #[sqlx(rename = "overdue")]
    Overdue,
    #[sqlx(rename = "lost")]
    Lost,
    #[sqlx(rename = "maintenance")]
    Maintenance,
}

/// Volume represents a physical copy of a title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub id: Uuid,
    pub title_id: Uuid,
    pub copy_number: i32,
    pub barcode: String,
    pub condition: VolumeCondition,
    pub location_id: Option<Uuid>,
    pub loan_status: LoanStatus,
    pub individual_notes: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// CreateVolumeRequest for creating a new volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVolumeRequest {
    pub title_id: String,  // UUID as string
    pub barcode: String,
    pub condition: VolumeCondition,
    pub location_id: Option<String>,  // UUID as string
    pub individual_notes: Option<String>,
}

/// UpdateVolumeRequest for updating an existing volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVolumeRequest {
    pub barcode: Option<String>,
    pub condition: Option<VolumeCondition>,
    pub location_id: Option<String>,  // UUID as string, or empty to remove location
    pub loan_status: Option<LoanStatus>,
    pub individual_notes: Option<String>,
}
