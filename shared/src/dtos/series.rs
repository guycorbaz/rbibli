use serde::{Deserialize, Serialize};

/// Request payload for creating a new series.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSeriesRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Request payload for updating an existing series.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSeriesRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
