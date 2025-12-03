use serde::{Deserialize, Serialize};

/// Request payload for creating a new genre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGenreRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Request payload for updating an existing genre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGenreRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
