use serde::{Deserialize, Serialize};
use crate::models::enums::AuthorRole;

/// Request payload for creating a new author.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthorRequest {
    pub first_name: String,
    pub last_name: String,
    pub biography: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub nationality: Option<String>,
    pub website_url: Option<String>,
}

/// Request payload for updating an existing author.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthorRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub biography: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub nationality: Option<String>,
    pub website_url: Option<String>,
}

/// Request payload for associating an author with a title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAuthorToTitleRequest {
    pub author_id: String,
    pub role: AuthorRole,
    pub display_order: Option<i32>,
}
