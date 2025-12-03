use serde::{Deserialize, Serialize};

/// Request payload for creating a new borrower group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBorrowerGroupRequest {
    pub name: String,
    pub loan_duration_days: i32,
    pub description: Option<String>,
}

/// Request payload for updating an existing borrower group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBorrowerGroupRequest {
    pub name: Option<String>,
    pub loan_duration_days: Option<i32>,
    pub description: Option<String>,
}

/// Request payload for creating a new borrower.
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

/// Request payload for updating an existing borrower.
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
