use serde::{Deserialize, Serialize};

/// Request payload for creating a new publisher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePublisherRequest {
    pub name: String,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub country: Option<String>,
    pub founded_year: Option<i32>,
}

/// Request payload for updating an existing publisher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePublisherRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub country: Option<String>,
    pub founded_year: Option<i32>,
}
