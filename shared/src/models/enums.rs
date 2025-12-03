use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use sqlx::Type;

/// Represents the physical condition of a volume.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "VARCHAR", rename_all = "lowercase"))]
#[serde(rename_all = "PascalCase")] // Frontend uses PascalCase for display, but let's check frontend models again.
// Frontend models:
// VolumeCondition: rename="Excellent", etc. (PascalCase)
// Backend models: rename="excellent" (lowercase) for sqlx.
// This is a conflict. The frontend expects "Excellent", the backend DB stores "excellent".
// Usually APIs return lowercase or snake_case.
// Let's look at frontend/src/models.rs again.
// It has #[serde(rename = "Excellent")] for VolumeCondition.
// And backend has #[sqlx(rename = "excellent")].
// Serde default is usually same as variant name (PascalCase).
// If I use `#[serde(rename_all = "lowercase")]` it will serialize to "excellent".
// If the frontend expects "Excellent", I might break it.
// Let's check frontend `VolumeCondition` again.
// It has explicit `#[serde(rename = "Excellent")]`.
// Wait, if I change it to lowercase in shared, I need to update frontend to expect lowercase.
// Or I can keep it PascalCase for Serde and lowercase for SQLx.
// Let's stick to what the backend does for SQLx (lowercase) and what the frontend does for Serde (PascalCase? No, let's standardize on lowercase for API if possible, but if frontend relies on PascalCase for display, that's an issue).
// Actually, `impl Display` in frontend uses "Excellent".
// Let's look at `frontend/src/models.rs` again.
// `#[serde(rename = "Excellent")]` -> This means it expects "Excellent" in JSON.
// Backend `volume.rs`: `#[sqlx(rename = "excellent")]`. It doesn't have serde rename attributes, so it uses default (PascalCase) for JSON?
// Let's check `backend/src/models/volume.rs` again.
// `#[derive(..., Serialize, Deserialize, ...)]`
// `pub enum VolumeCondition { ... Excellent, ... }`
// No `#[serde(rename_all = ...)]`.
// So backend sends "Excellent" (PascalCase) in JSON.
// SQLx uses "excellent" (lowercase) in DB.
// So I should NOT use `#[serde(rename_all = "lowercase")]`.
// I should use `#[sqlx(rename = "excellent")]` for backend.
// And for Serde, default is fine (PascalCase).
//
// Wait, `LoanStatus` in `volume.rs` has `#[sqlx(rename = "available")]`.
// And `LoanStatus` in `loan.rs` has `#[serde(rename_all = "lowercase")]`.
// So `LoanStatus` (loan record) is lowercase in JSON.
// `LoanStatus` (volume) is PascalCase in JSON (default).
//
// I should probably standardize everything to snake_case or lowercase for API, but that would be a breaking change for frontend.
// I will try to preserve existing behavior.

pub enum VolumeCondition {
    #[cfg_attr(feature = "backend", sqlx(rename = "excellent"))]
    #[serde(rename = "Excellent")]
    Excellent,
    #[cfg_attr(feature = "backend", sqlx(rename = "good"))]
    #[serde(rename = "Good")]
    Good,
    #[cfg_attr(feature = "backend", sqlx(rename = "fair"))]
    #[serde(rename = "Fair")]
    Fair,
    #[cfg_attr(feature = "backend", sqlx(rename = "poor"))]
    #[serde(rename = "Poor")]
    Poor,
    #[cfg_attr(feature = "backend", sqlx(rename = "damaged"))]
    #[serde(rename = "Damaged")]
    Damaged,
}

impl std::fmt::Display for VolumeCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeCondition::Excellent => write!(f, "Excellent"),
            VolumeCondition::Good => write!(f, "Good"),
            VolumeCondition::Fair => write!(f, "Fair"),
            VolumeCondition::Poor => write!(f, "Poor"),
            VolumeCondition::Damaged => write!(f, "Damaged"),
        }
    }
}

/// Represents the current loan status of a volume.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "VARCHAR", rename_all = "lowercase"))]
pub enum LoanStatus {
    #[cfg_attr(feature = "backend", sqlx(rename = "available"))]
    #[serde(rename = "Available")]
    Available,
    #[cfg_attr(feature = "backend", sqlx(rename = "loaned"))]
    #[serde(rename = "Loaned")]
    Loaned,
    #[cfg_attr(feature = "backend", sqlx(rename = "overdue"))]
    #[serde(rename = "Overdue")]
    Overdue,
    #[cfg_attr(feature = "backend", sqlx(rename = "lost"))]
    #[serde(rename = "Lost")]
    Lost,
    #[cfg_attr(feature = "backend", sqlx(rename = "maintenance"))]
    #[serde(rename = "Maintenance")]
    Maintenance,
}

impl std::fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanStatus::Available => write!(f, "Available"),
            LoanStatus::Loaned => write!(f, "Loaned"),
            LoanStatus::Overdue => write!(f, "Overdue"),
            LoanStatus::Lost => write!(f, "Lost"),
            LoanStatus::Maintenance => write!(f, "Maintenance"),
        }
    }
}

/// Represents the status of a loan record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "VARCHAR", rename_all = "lowercase"))]
#[serde(rename_all = "lowercase")] // Backend loan.rs has this
pub enum LoanRecordStatus {
    #[cfg_attr(feature = "backend", sqlx(rename = "active"))]
    Active,
    #[cfg_attr(feature = "backend", sqlx(rename = "returned"))]
    Returned,
    #[cfg_attr(feature = "backend", sqlx(rename = "overdue"))]
    Overdue,
}

impl std::fmt::Display for LoanRecordStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanRecordStatus::Active => write!(f, "Active"),
            LoanRecordStatus::Returned => write!(f, "Returned"),
            LoanRecordStatus::Overdue => write!(f, "Overdue"),
        }
    }
}

/// Defines the role of an author in relation to a specific title.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuthorRole {
    MainAuthor,
    CoAuthor,
    Translator,
    Illustrator,
    Editor,
}

impl std::fmt::Display for AuthorRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorRole::MainAuthor => write!(f, "main_author"),
            AuthorRole::CoAuthor => write!(f, "co_author"),
            AuthorRole::Translator => write!(f, "translator"),
            AuthorRole::Illustrator => write!(f, "illustrator"),
            AuthorRole::Editor => write!(f, "editor"),
        }
    }
}

/// Confidence level for duplicate detection matches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DuplicateConfidence {
    High,
    Medium,
    Low,
}
