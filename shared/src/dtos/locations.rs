use serde::{Deserialize, Serialize};

/// Request payload for creating a new location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}

/// Request payload for updating an existing location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
}
