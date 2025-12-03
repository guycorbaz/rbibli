use serde::{Deserialize, Serialize};

/// Request payload for creating a new loan by barcode scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoanRequest {
    pub borrower_id: String,
    pub barcode: String,
}

/// Request payload for returning a loaned volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnLoanRequest {
    pub loan_id: String,
}

/// Response from creating a loan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoanResponse {
    pub id: String,
    pub due_date: i64,
    pub loan_duration_days: i32,
}
