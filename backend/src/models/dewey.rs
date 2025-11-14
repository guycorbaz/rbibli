use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Dewey Decimal Classification entry
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeweyClassification {
    #[sqlx(try_from = "String")]
    pub id: Uuid,
    pub code: String,
    pub level: i32,
    pub parent_code: Option<String>,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

/// Simplified Dewey result for search responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeweySearchResult {
    pub code: String,
    pub name: String,
    pub level: i32,
    pub description: Option<String>,
    pub relevance: Option<f32>,
}

/// Query parameters for Dewey search
#[derive(Debug, Deserialize)]
pub struct DeweySearchQuery {
    pub q: String,  // Search query
    #[serde(default = "default_limit")]
    pub limit: i32, // Max results (default 20)
}

fn default_limit() -> i32 {
    20
}

/// Query parameters for browsing Dewey classifications
#[derive(Debug, Deserialize)]
pub struct DeweyBrowseQuery {
    pub parent_code: Option<String>,  // Parent code to browse children
}
