use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Loan status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LoanStatus {
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "returned")]
    Returned,
    #[sqlx(rename = "overdue")]
    Overdue,
}

/// Loan record
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Loan {
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    pub title_id: String,
    pub volume_id: String,
    pub borrower_id: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub loan_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub due_date: DateTime<Utc>,
    pub extension_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "optional_ts_seconds")]
    pub return_date: Option<DateTime<Utc>>,
    pub status: LoanStatus,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
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

/// Loan with detailed information
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

/// CreateLoanRequest - creates loan by barcode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoanRequest {
    pub borrower_id: String,
    pub barcode: String,
}

/// ReturnLoanRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnLoanRequest {
    pub loan_id: String,
}
