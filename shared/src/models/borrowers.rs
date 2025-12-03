use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Borrower group defining loan duration policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct BorrowerGroup {
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    pub name: String,
    pub loan_duration_days: i32,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Library member who can borrow books.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Borrower {
    #[cfg_attr(feature = "backend", sqlx(try_from = "String"))]
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub group_id: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Extended borrower information with group details and loan count.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowerWithGroup {
    #[serde(flatten)]
    pub borrower: Borrower,
    pub group_name: Option<String>,
    pub loan_duration_days: Option<i32>,
    pub active_loan_count: i32,
}
