use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// BorrowerGroup defines loan policies
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BorrowerGroup {
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    pub name: String,
    pub loan_duration_days: i32,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Borrower (library member)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Borrower {
    #[sqlx(try_from = "String")]
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

/// Borrower with group information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowerWithGroup {
    #[serde(flatten)]
    pub borrower: Borrower,
    pub group_name: Option<String>,
    pub loan_duration_days: Option<i32>,
    pub active_loan_count: i32,
}

/// CreateBorrowerGroupRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBorrowerGroupRequest {
    pub name: String,
    pub loan_duration_days: i32,
    pub description: Option<String>,
}

/// UpdateBorrowerGroupRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBorrowerGroupRequest {
    pub name: Option<String>,
    pub loan_duration_days: Option<i32>,
    pub description: Option<String>,
}

/// CreateBorrowerRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBorrowerRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub group_id: Option<String>,
}

/// UpdateBorrowerRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBorrowerRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub group_id: Option<String>,
}
