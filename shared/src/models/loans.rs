use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::enums::LoanRecordStatus as LoanStatus;

/// Core loan record representing a volume loaned to a borrower.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Loan {
    /// Unique loan identifier
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    /// ID of the title being loaned
    pub title_id: String,
    /// ID of the specific volume being loaned
    pub volume_id: String,
    /// ID of the borrower
    pub borrower_id: String,
    /// Loan creation date
    #[serde(with = "chrono::serde::ts_seconds")]
    pub loan_date: DateTime<Utc>,
    /// Due date
    #[serde(with = "chrono::serde::ts_seconds")]
    pub due_date: DateTime<Utc>,
    /// Extension count
    pub extension_count: i32,
    /// Return date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "optional_ts_seconds")]
    pub return_date: Option<DateTime<Utc>>,
    /// Current status
    pub status: LoanStatus,
    /// Creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Helper module for optional timestamp serialization
mod optional_ts_seconds {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_i64(dt.timestamp()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<i64> = Option::deserialize(deserializer)?;
        Ok(opt.map(|ts| DateTime::from_timestamp(ts, 0).unwrap()))
    }
}

/// Extended loan information with borrower and title details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanDetail {
    #[serde(flatten)]
    pub loan: Loan,
    pub title: String,
    pub barcode: String,
    pub borrower_name: String,
    pub borrower_email: Option<String>,
    pub is_overdue: bool,
}
